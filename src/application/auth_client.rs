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
    pub async fn new_surreal_client(
        db_configs: &DatabaseConfig,
    ) -> AuthClient<gateway::surreal::SurrealGateway> {
        let gateway = gateway::surreal::SurrealGateway::new(db_configs).await;

        Self { gateway }
    }
}

impl<C: CredentialsRepository> AuthClient<C> {
    /// Register a new user and insert them into the database if user does not already exist
    pub async fn register(&self, user_name: &str, raw_password: &str) -> Result<CredentialsId> {
        match self.gateway.find_credentials_by_user_name(user_name).await {
            Ok(_) => {
                return Err(anyhow::anyhow!(
                    "Registration failed, credentials already exist"
                ))
            }
            Err(_) => {
                let hashed_password = hash_raw_password(raw_password);

                let credentials = Credentials::new(user_name, hashed_password.as_str());

                self.gateway
                    .insert_credentials(&credentials)
                    .await
                    .context("Registration failed, repository error")?;

                return Ok(credentials.credentials_id);
            }
        };
    }

    /// Matches credentials provided by the user with the what is in the database
    pub async fn verify_credentials(&self, user_name: &str, raw_password: &str) -> Result<()> {
        let creds = self
            .gateway
            .find_credentials_by_user_name(&user_name)
            .await?;

        verify_password(raw_password, &creds.hashed_password)
            .context("Username or Password did not match")
    }

    /// Deletes credentials from table
    pub async fn destroy_credentials(&mut self, user_name: &str) -> Result<()> {
        self.gateway
            .delete_credentials_by_user_name(user_name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::mysql_configs;
    #[cfg(feature = "surreal")]
    use crate::helpers::surreal_configs;

    #[tokio::test]
    async fn test_mysql_auth() {
        let db_configs = mysql_configs();
        let auth = AuthClient::new(&db_configs).await;

        // create random user creds
        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";
        let creds_id = auth.register(email, password).await.unwrap();
        assert_eq!(creds_id.len(), 36);

        // login attempt
        auth.verify_credentials(email, password).await.unwrap();
    }

    #[cfg(feature = "surreal")]
    #[tokio::test]
    async fn test_surreal_auth() {
        let db_configs = surreal_configs();
        let auth = AuthClient::new_surreal_client(&db_configs).await;

        // create random user creds
        let random_str = &uuid::Uuid::new_v4().to_string();
        let email = &random_str[..10];
        let password = "secret-test-password";
        let creds_id = auth.register(email, password).await.unwrap();
        assert_eq!(creds_id.len(), 36);

        // login attempt
        auth.verify_credentials(email, password).await.unwrap();
    }
}
