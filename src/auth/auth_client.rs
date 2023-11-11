use crate::domain::entity::{Credentials, CredentialsId};
use crate::{
    application::interface::CredentialsRepository,
    infrastructure::services::password_hash::{hash_raw_password, verify_password},
};
use crate::{config::DatabaseConfig, infrastructure::gateway};
use anyhow::{Context, Result};

pub struct AuthClient<C: CredentialsRepository> {
    pub gateway: C,
}

#[cfg(feature = "mysql")]
impl AuthClient<gateway::mysql::MySqlGateway> {
    pub async fn new(db_configs: &DatabaseConfig) -> AuthClient<gateway::mysql::MySqlGateway> {
        let gateway = gateway::mysql::MySqlGateway::new(db_configs).await;

        Self { gateway }
    }
}

#[cfg(feature = "surreal")]
impl AuthClient<gateway::surreal::SurrealGateway> {
    pub async fn _new(db_configs: &DatabaseConfig) -> AuthClient<gateway::surreal::SurrealGateway> {
        let gateway = gateway::surreal::SurrealGateway::new(db_configs).await;

        Self { gateway }
    }
}

impl<C: CredentialsRepository> AuthClient<C> {
    /// Register a new user and insert them into the database if user does not already exist
    pub async fn register(
        &mut self,
        user_identity: &str,
        raw_password: &str,
    ) -> Result<CredentialsId> {
        match self
            .gateway
            .find_credentials_by_user_identity(user_identity)
            .await
        {
            Ok(_) => {
                return Err(anyhow::anyhow!(
                    "Registration failed, credentials already exist"
                ))
            }
            Err(_) => {
                let hashed_password = hash_raw_password(raw_password);

                let credentials = Credentials::new(user_identity, hashed_password.as_str());

                self.gateway
                    .insert_credentials(&credentials)
                    .await
                    .context("Registration failed, repository error")?;

                return Ok(credentials.id);
            }
        };
    }

    /// Matches credentials provided by the user with the what is in the database
    pub async fn verify_credentials(&self, user_identity: &str, raw_password: &str) -> Result<()> {
        let creds = self
            .gateway
            .find_credentials_by_user_identity(&user_identity)
            .await?;

        if verify_password(raw_password, &creds.hashed_password).is_ok() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Username or Password did not match"))
        }
    }

    /// Deletes credentials from table
    pub async fn destroy_credentials(&mut self, user_identity: &str) -> Result<()> {
        self.gateway
            .delete_credentials_by_user_identity(user_identity)
            .await
    }
}
