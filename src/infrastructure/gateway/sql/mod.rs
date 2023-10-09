use std::error::Error;

use crate::{
    application::{CredentialsRepository, SessionRepository},
    domain::{Credentials, DatabaseConfig, Session, SessionRecordId},
};
use sqlx::mysql::MySqlPool;

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
            .expect("Failed to connect to SqlDb");

        Self { pool }
    }

    pub async fn create_session_table(&self) {
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

    pub async fn create_credentials_table(&self) {
        sqlx::query(
            r#"
            CREATE TABLE credentials (
                id CHAR(36) PRIMARY KEY,
                user_identity CHAR(36) NOT NULL,
                hashed_password CHAR(36) NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}

#[async_trait::async_trait]
impl SessionRepository for MySqlGateway {
    async fn store_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, created_at, expires_at)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(&session.id)
        .bind(session.created_at as i64) // Converting usize to i64 for compatibility
        .bind(session.expires_at as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_session_by_id(
        &mut self,
        session_id: &SessionRecordId,
    ) -> Result<Session, Box<dyn Error>> {
        let session: Session = sqlx::query_as(
            r#"
        SELECT id, created_at, expires_at
        FROM sessions
        WHERE id = ?
        "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn delete_session(
        &mut self,
        _session_record_id: &SessionRecordId,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl CredentialsRepository for MySqlGateway {
    async fn insert_credentials(&self, credentials: &Credentials) -> Result<(), Box<dyn Error>> {
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
        .await?;

        Ok(())
    }
    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials, Box<dyn Error>> {
        let creds: Credentials = sqlx::query_as(
            r#"
            SELECT id, user_identity, hashed_password
            FROM credentials
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(creds)
    }

    async fn find_credentials_by_user_identity(
        &self,
        user_identity: &str,
    ) -> Result<Option<Credentials>, Box<dyn Error>> {
        let creds_query: Result<Credentials, sqlx::Error> = sqlx::query_as(
            r#"
            SELECT id, user_identity, hashed_password
            FROM credentials
            WHERE user_identity = ?
            "#,
        )
        .bind(user_identity)
        .fetch_one(&self.pool)
        .await;

        match creds_query {
            Ok(creds) => {
                println!("User Credentials Found");
                Ok(Some(creds))
            }
            Err(e) => match e {
                sqlx::Error::RowNotFound => {
                    println!("User Credentials Not Found");
                    Ok(None)
                }
                _ => Err(Box::new(e)),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Expiry;

    use super::*;

    #[tokio::test]
    async fn test_mysql_session_repo() {
        let db_config = DatabaseConfig {
            host: "localhost:3306".to_string(),
            password: "my-secret-pw".to_string(),
            db_name: "mysql".to_string(),
            user_name: "root".to_string(),
        };

        let mut repo = MySqlGateway::new(&db_config).await;
        repo.create_session_table().await;

        let session = &Session::new(Expiry::Day(1));
        let query = repo.store_session(session).await;
        assert!(query.is_ok());

        let session_from_repo = repo.get_session_by_id(&session.id).await.unwrap();
        assert_eq!(session_from_repo.is_expired(), false);
        assert_eq!(session_from_repo.id, session.id);
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
        repo.create_credentials_table().await;

        let password = "test-pass-word";
        let email = "test@email.com";

        // Create new credentials
        let credentials = Credentials::new(email, password);
        repo.insert_credentials(&credentials).await.unwrap();

        // Test getting credentials
        let creds = repo.find_credentials_by_user_identity(email).await.unwrap();
        assert_eq!(creds.unwrap().user_identity, email);
    }
}
