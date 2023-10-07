use std::{error::Error, fmt::format};

use crate::{
    application::{Authenticate, CredentialsRepository, SessionRepository},
    domain::{Credentials, CredentialsId, Session, SessionRecordId},
};
use sqlx::{mysql::MySqlPool, query_builder, Execute, MySql, QueryBuilder};

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(addr: &str) -> Self {
        let pool = MySqlPool::connect(addr)
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
    ) -> Result<Credentials, Box<dyn Error>> {
        let creds: Credentials = sqlx::query_as(
            r#"
            SELECT id, user_identity, hashed_password
            FROM credentials
            WHERE user_identity = ?
            "#,
        )
        .bind(user_identity)
        .fetch_one(&self.pool)
        .await?;

        Ok(creds)
    }
}

#[async_trait::async_trait]
impl Authenticate for MySqlGateway {
    async fn register(&self, fields: Vec<(&str, &str, bool)>) -> Result<bool, Box<dyn Error>> {
        let mut insert_statement = String::from("INSERT INTO users (");

        for (index, field) in fields.iter().enumerate() {
            let key = field.0;
            let is_last_field = fields.len() - 1 == index;

            if is_last_field {
                let column_name = format!("{})", key);
                insert_statement.push_str(&column_name);
            } else {
                let column_name = format!("{},", key);
                insert_statement.push_str(&column_name);
            }
        }

        let start_statement = format!("{} VALUES (", insert_statement);
        let mut query_builder: QueryBuilder<MySql> =
            query_builder::QueryBuilder::new(start_statement);

        let mut seperated = query_builder.separated(", ");

        for field in fields {
            seperated.push_bind(field.1);
        }
        seperated.push_unseparated(");");

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Expiry;

    use super::*;

    #[tokio::test]
    async fn test_mysql_session_repo() {
        let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
        let mut repo = MySqlGateway::new(url).await;
        repo.create_session_table().await;

        let session = &Session::new(Expiry::Day(1));
        let query = repo.store_session(session).await;
        assert!(query.is_ok());

        let session_from_repo = repo.get_session_by_id(&session.id).await.unwrap();
        assert_eq!(session_from_repo.is_expired(), false);
        assert_eq!(session_from_repo.id, session.id);
    }

    #[tokio::test]
    async fn test_mysql_user_repo() {
        let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
        let repo = MySqlGateway::new(url).await;
        repo.create_credentials_table().await;

        let password = "test-pass-word";
        let email = "test@email.com";

        // Create new user
        let user = Credentials::new(email, password);
        repo.insert_credentials(&user).await.unwrap();

        // Test getting user
        let user_record = repo.find_credentials_by_user_identity(email).await.unwrap();
        dbg!(&user_record);
        assert_eq!(user_record.user_identity, email);
    }
}
