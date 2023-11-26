use super::MySqlGateway;
use crate::{entity::Credentials, interface::CredentialsRepository};
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlRow, FromRow, Row};

impl FromRow<'_, MySqlRow> for Credentials {
    fn from_row(row: &MySqlRow) -> sqlx::Result<Self> {
        Ok(Self {
            credentials_id: row.try_get("credentials_id")?,
            hashed_password: row.try_get("hashed_password")?,
            user_name: row.try_get("user_name")?,
        })
    }
}

#[async_trait::async_trait]
impl CredentialsRepository for MySqlGateway {
    async fn insert_credentials(&self, credentials: &Credentials) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_credentials (credentials_id, user_name, hashed_password)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(&credentials.credentials_id)
        .bind(&credentials.user_name)
        .bind(&credentials.hashed_password)
        .execute(&self.pool)
        .await
        .context("Failed to insert credentials")?;

        Ok(())
    }

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials> {
        let credentials: Credentials = sqlx::query_as(
            r#"
            SELECT credentials_id, user_name, hashed_password
            FROM user_credentials
            WHERE credentials_id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find credentials by id")?;

        Ok(credentials)
    }

    async fn find_credentials_by_user_name(&self, user_name: &str) -> Result<Credentials> {
        let credentials: Credentials = sqlx::query_as(
            r#"
            SELECT credentials_id, user_name, hashed_password
            FROM user_credentials
            WHERE user_name = ?
            "#,
        )
        .bind(user_name)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find credentials by user name")?;

        Ok(credentials)
    }

    async fn update_user_name(&self, user_name: &str, new_user_name: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_credentials
            SET user_name = ?
            WHERE user_name = ?
            "#,
        )
        .bind(new_user_name)
        .bind(user_name)
        .execute(&self.pool)
        .await
        .context("Failed to update user name")?;

        Ok(())
    }

    async fn update_user_password(&self, user_name: &str, new_password: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_credentials
            SET hashed_password = ?
            WHERE user_name = ?
            "#,
        )
        .bind(&new_password)
        .bind(user_name)
        .execute(&self.pool)
        .await
        .context("Failed to update user password")?;

        Ok(())
    }

    async fn delete_credentials_by_user_name(&self, user_name: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_credentials
            WHERE user_name = ?
            "#,
        )
        .bind(user_name)
        .execute(&self.pool)
        .await
        .context("Failed to delete credentials by user name")?;

        Ok(())
    }

    async fn delete_credentials_by_id(&self, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_credentials
            WHERE credentials_id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to delete credentials by id")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::mysql_configs;

    use super::*;

    #[tokio::test]
    async fn test_mysql_credentials_repo() {
        let db_config = mysql_configs();
        let repo = MySqlGateway::new(&db_config).await;

        let password = "test-pass-word";
        let email = "test@email.com";

        // Create new credentials
        let credentials = Credentials::new(email, password);
        repo.insert_credentials(&credentials).await.unwrap();

        // Test getting credentials
        let creds = repo.find_credentials_by_user_name(email).await.unwrap();
        assert_eq!(creds.user_name, email);

        // Test changing credentials
        let new_identity = "updatedidentity@gmail.com";
        let new_password = "the-updated-password";
        repo.update_user_name(&credentials.user_name, new_identity)
            .await
            .unwrap();
        repo.update_user_password(&new_identity, new_password)
            .await
            .unwrap();

        let creds = repo
            .find_credentials_by_id(&credentials.credentials_id)
            .await
            .unwrap();
        assert_eq!(creds.user_name, new_identity);
        assert_eq!(creds.hashed_password, new_password);

        // Delete credentials by user name
        repo.delete_credentials_by_user_name(&creds.user_name)
            .await
            .unwrap();
        let creds = repo
            .find_credentials_by_id(&credentials.credentials_id)
            .await;
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
