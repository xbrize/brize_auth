use anyhow::{Context, Result};
use sqlx::{
    mysql::{MySqlPool, MySqlRow},
    FromRow, Row,
};

use crate::{
    application::interface::{CredentialsRepository, SessionRepository},
    domain::{
        config::DatabaseConfig,
        entity::Credentials,
        entity::{Session, SessionToken},
    },
};

impl FromRow<'_, MySqlRow> for Session {
    fn from_row(row: &MySqlRow) -> sqlx::Result<Self> {
        Ok(Self {
            session_id: row.try_get("session_id")?,
            created_at: row.try_get("created_at")?,
            expires_at: row.try_get("expires_at")?,
            user_id: row.try_get("user_id")?,
            csrf_token: row.try_get("csrf_token")?,
        })
    }
}

impl FromRow<'_, MySqlRow> for Credentials {
    fn from_row(row: &MySqlRow) -> sqlx::Result<Self> {
        Ok(Self {
            credentials_id: row.try_get("credentials_id")?,
            hashed_password: row.try_get("hashed_password")?,
            user_name: row.try_get("user_name")?,
        })
    }
}

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let addr = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.user_name, config.password, config.host, config.port, config.db_name
        );
        let pool = MySqlPool::connect(addr.as_str())
            .await
            .expect("Failed connection with MySql database");

        Self { pool }
    }
}

impl MySqlGateway {}

#[async_trait::async_trait]
impl SessionRepository for MySqlGateway {
    async fn insert_session(&self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_sessions (session_id, created_at, expires_at, user_id, csrf_token)
            VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(session.session_id.as_str())
        .bind(session.created_at as i64) // Converting usize to i64 for compatibility
        .bind(session.expires_at as i64)
        .bind(session.user_id.as_str())
        .bind(session.csrf_token.as_str())
        .execute(&self.pool)
        .await
        .context("Failed to store session in MySql")?;

        Ok(())
    }

    async fn get_session_by_id(&self, session_id: &SessionToken) -> Result<Session> {
        let session: Session = sqlx::query_as(
            r#"
            SELECT session_id, created_at, expires_at, user_id, csrf_token
            FROM user_sessions
            WHERE session_id = ?
            "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to get session by id ")?;

        Ok(session)
    }

    async fn delete_session(&self, session_id: &SessionToken) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_sessions 
            WHERE session_id = ?
            "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .context("Failed to delete session ")?;

        Ok(())
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

    async fn update_user_name(&self, user_name: &str, new_identity: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_credentials
            SET user_name = ?
            WHERE user_name = ?
            "#,
        )
        .bind(new_identity)
        .bind(user_name)
        .execute(&self.pool)
        .await
        .context("Failed to update user name")?;

        Ok(())
    }

    async fn update_user_password(&self, user_name: &str, new_hashed_password: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE user_credentials
            SET hashed_password = ?
            WHERE user_name = ?
            "#,
        )
        .bind(&new_hashed_password)
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
    use super::*;
    use crate::{domain::config::Expiry, helpers::mysql_configs};

    #[tokio::test]
    async fn test_mysql_session_repo() {
        let db_config = mysql_configs();
        let repo = MySqlGateway::new(&db_config).await;

        let session = &Session::new(&Expiry::Day(1), "848hfhs0-88ryh-eohrnf-odsiru");
        let query = repo.insert_session(session).await;
        assert!(query.is_ok());

        let session_from_repo = repo.get_session_by_id(&session.session_id).await.unwrap();
        assert_eq!(session_from_repo.is_expired(), false);
        assert_eq!(session_from_repo.session_id, session.session_id);
        assert_eq!(session_from_repo.csrf_token, session.csrf_token);

        repo.delete_session(&session.session_id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.session_id).await;
        assert!(session_from_repo.is_err());
    }

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
