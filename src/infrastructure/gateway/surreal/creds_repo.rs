use super::{SurrealGateway, SurrealRecord};
use crate::{entity::Credentials, interface::CredentialsRepository};
use anyhow::{Context, Result};

#[async_trait::async_trait]
impl CredentialsRepository for SurrealGateway {
    async fn find_credentials_by_user_name(&self, user_name: &str) -> Result<Credentials> {
        let sql = "
        SELECT * FROM user_credentials WHERE data.user_name = $user_name
        ";

        let mut query_result = self
            .database
            .query(sql)
            .bind(("user_name", user_name))
            .await
            .context("Failed to get credentials by user name")?;

        let mut records = query_result
            .take::<Vec<SurrealRecord<Credentials>>>(0)
            .context("Could not take from response")?;

        if !records.is_empty() {
            Ok(records.remove(0).data)
        } else {
            Err(anyhow::anyhow!("Failed to find credentials by user name"))
        }
    }

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials> {
        let query_for_record: Option<SurrealRecord<Credentials>> =
            self.database.select(("user_credentials", id)).await?;

        match query_for_record {
            Some(record) => Ok(record.data),
            None => Err(anyhow::anyhow!("Failed to find credentials by id")),
        }
    }

    async fn insert_credentials(&self, credentials: &Credentials) -> Result<()> {
        let record = SurrealRecord {
            id: None,
            data: credentials,
        };

        self.database
            .create::<Option<SurrealRecord<Credentials>>>((
                "user_credentials",
                &credentials.credentials_id,
            ))
            .content(&record)
            .await
            .context("Failed to insert credentials into Surreal")?;

        Ok(())
    }

    async fn update_user_name(&self, user_name: &str, new_user_name: &str) -> Result<()> {
        let sql = "
            UPDATE user_credentials
            SET data.user_name = $new_identity
            WHERE data.user_name = $current_identity;
        ";

        self.database
            .query(sql)
            .bind(("new_identity", new_user_name))
            .bind(("current_identity", user_name))
            .await
            .context("Failed to update user identity in Surreal")?;

        Ok(())
    }

    async fn update_user_password(&self, user_name: &str, new_password: &str) -> Result<()> {
        let sql = "
            UPDATE user_credentials
            SET data.hashed_password = $new_password
            WHERE data.user_name = $user_name;
        ";

        self.database
            .query(sql)
            .bind(("new_password", new_password))
            .bind(("user_name", user_name))
            .await
            .context("Failed to update password in Surreal")?;

        Ok(())
    }

    async fn delete_credentials_by_user_name(&self, user_name: &str) -> Result<()> {
        let sql = "
            DELETE FROM user_credentials
            WHERE data.user_name = $user_name;
        ";

        self.database
            .query(sql)
            .bind(("user_name", user_name))
            .await
            .context("Failed to delete credentials by user identity in Surreal")?;

        Ok(())
    }

    async fn delete_credentials_by_id(&self, id: &str) -> Result<()> {
        self.database
            .delete::<Option<SurrealRecord<Credentials>>>(("user_credentials", id))
            .await
            .context("Failed to delete credentials by id from Surreal")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::surreal_configs;

    use super::*;

    #[tokio::test]
    async fn test_surreal_credentials_repo() {
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let db_config = surreal_configs();
        let repo = SurrealGateway::new(&db_config).await;

        // Create new creds
        let creds = Credentials::new(email, password);
        repo.insert_credentials(&creds).await.unwrap();

        // Test getting creds
        let user_cred = repo.find_credentials_by_user_name(email).await.unwrap();
        assert_eq!(user_cred.user_name, email);

        // Test changing credentials
        let new_identity = "updatedidentity@gmail.com";
        let new_password = "the-updated-password";
        repo.update_user_name(&creds.user_name, new_identity)
            .await
            .unwrap();
        repo.update_user_password(&new_identity, new_password)
            .await
            .unwrap();

        let creds = repo
            .find_credentials_by_id(&creds.credentials_id)
            .await
            .unwrap();
        assert_eq!(creds.user_name, new_identity);

        // Delete credentials
        repo.delete_credentials_by_user_name(&creds.user_name)
            .await
            .unwrap();
        let creds = repo.find_credentials_by_id(&creds.credentials_id).await;
        assert!(creds.is_err());

        // Delete credentials by credentials_id
        let credentials = Credentials::new(email, password);
        repo.insert_credentials(&credentials).await.unwrap();

        repo.delete_credentials_by_id(&credentials.credentials_id)
            .await
            .unwrap();
        let creds = repo
            .find_credentials_by_id(&credentials.credentials_id)
            .await;
        assert!(creds.is_err());
    }
}
