use crate::domain::entity::{Credentials, CredentialsId};
use crate::infrastructure::gateway;
use crate::{
    application::interface::CredentialsRepository,
    infrastructure::services::password_hash::{hash_raw_password, verify_password},
};
use anyhow::{Context, Result};

pub struct AuthClient<C: CredentialsRepository> {
    pub gateway: C,
}

impl AuthClient<gateway::mysql::MySqlGateway> {
    pub async fn new_mysql_client(database_url: &str) -> AuthClient<gateway::mysql::MySqlGateway> {
        let gateway = gateway::mysql::MySqlGateway::new(database_url).await;

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
    pub async fn destroy_credentials(&self, user_name: &str) -> Result<()> {
        self.gateway
            .delete_credentials_by_user_name(user_name)
            .await
    }

    /// Update user name
    pub async fn update_user_name(
        &self,
        current_user_name: &str,
        new_user_name: &str,
    ) -> Result<()> {
        self.gateway
            .update_user_name(current_user_name, new_user_name)
            .await
    }

    /// Update user password
    pub async fn update_password(&self, user_name: &str, new_raw_password: &str) -> Result<()> {
        let new_hashed_password = hash_raw_password(new_raw_password);
        self.gateway
            .update_user_password(user_name, new_hashed_password.as_str())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::mysql_configs;

    #[tokio::test]
    async fn test_mysql_auth() {
        let db_configs = mysql_configs();
        let auth = AuthClient::new_mysql_client(&db_configs.mysql_connection_string()).await;

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
