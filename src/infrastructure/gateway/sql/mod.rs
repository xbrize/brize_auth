use anyhow::{Context, Result};
use sqlx::mysql::MySqlPool;

use crate::{
    application::interface::{CredentialsRepository, SessionRepository},
    domain::{
        config::DatabaseConfig,
        entity::{Credentials, Session, SessionId},
    },
};

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let addr = format!(
            "mysql://{}:{}@{}/{}",
            config.user_name, config.password, config.host, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with MySql database");

        Self { pool }
    }

    pub async fn _create_session_table(&self) {
        sqlx::query(
            r#"
            CREATE TABLE sessions (
                id CHAR(36) PRIMARY KEY,  
                created_at BIGINT UNSIGNED NOT NULL,
                expires_at BIGINT UNSIGNED NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn _create_credentials_table(&self) {
        sqlx::query(
            r#"
            CREATE TABLE credentials (
                id CHAR(36) PRIMARY KEY,
                user_identity VARCHAR(255) NOT NULL,
                hashed_password VARCHAR(255) NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}

impl MySqlGateway {}

#[async_trait::async_trait]
impl SessionRepository for MySqlGateway {
    async fn store_session(&mut self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, created_at, expires_at)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(session.id.as_str())
        .bind(session.created_at as i64) // Converting usize to i64 for compatibility
        .bind(session.expires_at as i64)
        .execute(&self.pool)
        .await
        .context("Failed to store session in MySql")?;

        Ok(())
    }

    async fn get_session_by_id(&mut self, session_id: &SessionId) -> Result<Session> {
        let session: Session = sqlx::query_as(
            r#"
            SELECT id, created_at, expires_at
            FROM sessions
            WHERE id = ?
            "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to get session by id from MySql")?;

        Ok(session)
    }

    async fn delete_session(&mut self, session_id: &SessionId) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM sessions 
            WHERE id = ?
            "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .context("Failed to delete session from MySql")?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl CredentialsRepository for MySqlGateway {
    async fn insert_credentials(&self, credentials: &Credentials) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO credentials (id, user_identity, hashed_password)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(&credentials.id)
        .bind(&credentials.user_identity)
        .bind(&credentials.hashed_password)
        .execute(&self.pool)
        .await
        .context("Failed to insert credentials into MySql")?;

        Ok(())
    }

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials> {
        let credentials: Credentials = sqlx::query_as(
            r#"
            SELECT id, user_identity, hashed_password
            FROM credentials
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find credentials by id from MySql")?;

        Ok(credentials)
    }

    async fn find_credentials_by_user_identity(&self, user_identity: &str) -> Result<Credentials> {
        let credentials: Credentials = sqlx::query_as(
            r#"
            SELECT id, user_identity, hashed_password
            FROM credentials
            WHERE user_identity = ?
            "#,
        )
        .bind(user_identity)
        .fetch_one(&self.pool)
        .await
        .context("Failed to find credentials by user identity from MySql")?;

        Ok(credentials)
    }

    async fn update_user_identity(&self, user_identity: &str, new_identity: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE credentials
            SET user_identity = ?
            WHERE user_identity = ?
            "#,
        )
        .bind(new_identity)
        .bind(user_identity)
        .execute(&self.pool)
        .await
        .context("Failed to update user identity in MySql")?;

        Ok(())
    }

    async fn update_user_password(
        &self,
        user_identity: &str,
        new_hashed_password: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE credentials
            SET hashed_password = ?
            WHERE user_identity = ?
            "#,
        )
        .bind(&new_hashed_password)
        .bind(user_identity)
        .execute(&self.pool)
        .await
        .context("Failed to update user password in MySql")?;

        Ok(())
    }

    async fn delete_credentials_by_user_identity(&self, user_identity: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM credentials
            WHERE user_identity = ?
            "#,
        )
        .bind(user_identity)
        .execute(&self.pool)
        .await
        .context("Failed to delete credentials by user identity from MySql")?;

        Ok(())
    }

    async fn delete_credentials_by_id(&self, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM credentials
            WHERE id = ?
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .context("Failed to delete credentials by id from MySql")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::config::Expiry;

    #[tokio::test]
    async fn test_mysql_session_repo() {
        let db_config = DatabaseConfig {
            host: "localhost:3306".to_string(),
            password: "my-secret-pw".to_string(),
            db_name: "mysql".to_string(),
            user_name: "root".to_string(),
        };

        let mut repo = MySqlGateway::new(&db_config).await;
        repo._create_session_table().await;

        let session = &Session::new(&Expiry::Day(1));
        let query = repo.store_session(session).await;
        assert!(query.is_ok());

        let session_from_repo = repo.get_session_by_id(&session.id).await.unwrap();
        assert_eq!(session_from_repo.is_expired(), false);
        assert_eq!(session_from_repo.id, session.id);

        repo.delete_session(&session.id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.id).await;
        assert!(session_from_repo.is_err());
    }

    #[tokio::test]
    async fn test_mysql_credentials_repo() {
        let db_config = DatabaseConfig {
            host: "localhost:3306".to_string(),
            password: "my-secret-pw".to_string(),
            db_name: "mysql".to_string(),
            user_name: "root".to_string(),
        };

        let repo = MySqlGateway::new(&db_config).await;
        repo._create_credentials_table().await;

        let password = "test-pass-word";
        let email = "test@email.com";

        // Create new credentials
        let credentials = Credentials::new(email, password);
        repo.insert_credentials(&credentials).await.unwrap();

        // Test getting credentials
        let creds = repo.find_credentials_by_user_identity(email).await.unwrap();
        assert_eq!(creds.user_identity, email);

        // Test changing credentials
        let new_identity = "updatedidentity@gmail.com";
        let new_password = "the-updated-password";
        repo.update_user_identity(&credentials.user_identity, new_identity)
            .await
            .unwrap();
        repo.update_user_password(&new_identity, new_password)
            .await
            .unwrap();

        let creds = repo.find_credentials_by_id(&credentials.id).await.unwrap();
        assert_eq!(creds.user_identity, new_identity);
        assert_eq!(creds.hashed_password, new_password);

        // Delete credentials by user identity
        repo.delete_credentials_by_user_identity(&creds.user_identity)
            .await
            .unwrap();
        let creds = repo.find_credentials_by_id(&credentials.id).await;
        assert!(creds.is_err());

        // Delete credentials by id
        let credentials = Credentials::new(email, password);
        repo.insert_credentials(&credentials).await.unwrap();

        repo.delete_credentials_by_id(&credentials.id)
            .await
            .unwrap();
        let creds = repo.find_credentials_by_id(&credentials.id).await;
        assert!(creds.is_err());
    }
}
