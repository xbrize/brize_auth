use std::error::Error;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::application::{SessionRepository, UserRepository};
use crate::domain::{Session, SessionRecordId, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealSessionRecord {
    pub id: Thing,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealUserRecord {
    pub id: Thing,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct SurrealGateway {
    pub database: Surreal<Client>,
}

impl SurrealGateway {
    pub async fn new(addr: &str, namespace: &str, database_name: &str) -> Self {
        let db = Surreal::new::<Ws>(addr)
            .await
            .expect("Could not connect to database:");
        db.use_ns(namespace)
            .use_db(database_name)
            .await
            .expect("Could not connect to database:");

        Self { database: db }
    }
}

#[async_trait::async_trait]
impl SessionRepository for SurrealGateway {
    async fn get_session_by_id(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<Session, Box<dyn Error>> {
        let session: Option<SurrealSessionRecord> =
            self.database.select(("session", session_record_id)).await?;

        if let Some(record) = session {
            let session = Session {
                id: record.id.id.to_raw(),
                expires_at: record.expires_at,
                created_at: record.created_at,
            };
            return Ok(session);
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No session record found",
            )));
        }
    }

    async fn store_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>> {
        self.database
            .create::<Vec<SurrealSessionRecord>>("session")
            .content(&session)
            .await?;
        Ok(())
    }

    async fn delete_session(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<(), Box<dyn Error>> {
        self.database
            .delete::<Option<SurrealSessionRecord>>(("session", session_record_id))
            .await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl UserRepository for SurrealGateway {
    async fn find_user_by_email(&self, email: &str) -> Result<User, Box<dyn Error>> {
        let sql = "
        SELECT * from user where email = $email
        ";

        let mut query_result = self
            .database
            .query(sql)
            .bind(("email", email))
            .await
            .unwrap();

        // TODO do not return password
        let user_record: Vec<SurrealUserRecord> = query_result.take(0).unwrap();
        let user = User::new(
            &user_record[0].username,
            &user_record[0].password,
            &user_record[0].email,
        );
        Ok(user)
    }

    async fn store_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
        self.database
            .create::<Vec<SurrealUserRecord>>("user")
            .content(&user)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Expiry;

    use super::*;

    #[tokio::test]
    async fn test_surreal_session_repository() {
        let mut repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        let session = Session::new(Expiry::Day(1));
        let query = repo.store_session(&session).await;
        assert!(query.is_ok());

        let session_from_storage = repo.get_session_by_id(&session.id).await.unwrap();
        assert!(!session_from_storage.is_expired());
        assert_eq!(session_from_storage.id, session.id);
    }

    #[tokio::test]
    async fn test_surreal_user_repository() {
        let username = "test-user-name";
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let user_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        // Create new user
        let user = User::new(username, password, email);
        user_repo.store_user(&user).await.unwrap();

        // Test getting user
        let user_record = user_repo.find_user_by_email(email).await.unwrap();
        dbg!(&user_record);
        assert_eq!(user_record.email, email);
    }
}
