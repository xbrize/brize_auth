use crate::{
    application::interface::{CredentialsRepository, SessionRepository},
    domain::{
        config::DatabaseConfig,
        entity::{Credentials, Session, SessionRecordId},
    },
};
use sqlx::mysql::MySqlPool;
use std::error::Error;

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

    async fn delete_session(&mut self, session_id: &SessionRecordId) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
        DELETE FROM sessions 
        WHERE id = ?
        "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await?;

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

    async fn update_user_identity(
        &self,
        current_identity: &str,
        new_identity: &str,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            UPDATE credentials
            SET user_identity = ?
            WHERE user_identity = ?
            "#,
        )
        .bind(new_identity)
        .bind(current_identity)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_user_password(
        &self,
        user_identity: &str,
        new_raw_password: &str,
    ) -> Result<(), Box<dyn Error>> {
        // TODO hash password here
        sqlx::query(
            r#"
            UPDATE credentials
            SET hashed_password = ?
            WHERE user_identity = ?
            "#,
        )
        .bind(new_raw_password)
        .bind(user_identity)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_credentials_by_user_identity(
        &self,
        user_identity: &str,
    ) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            DELETE FROM credentials
            WHERE user_identity = ?
            "#,
        )
        .bind(user_identity)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::config::Expiry;

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
        assert_eq!(creds.unwrap().user_identity, email);

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
        // TODO this will fail after hashing password
        assert_eq!(creds.hashed_password, new_password);

        // Delete credentials
        repo.delete_credentials_by_user_identity(&creds.user_identity)
            .await
            .unwrap();
        let creds = repo.find_credentials_by_id(&credentials.id).await;
        assert!(creds.is_err());
    }
}
