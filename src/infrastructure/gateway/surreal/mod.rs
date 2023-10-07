use std::error::Error;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::application::{CredentialsRepository, SessionRepository};
use crate::domain::{Credentials, Session, SessionRecordId};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealSessionRecord {
    pub id: Thing,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealCredentialRecord {
    pub id: Thing,
    pub user_identity: String,
    pub hashed_password: String,
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
impl CredentialsRepository for SurrealGateway {
    async fn find_credentials_by_user_identity(
        &self,
        user_identity: &str,
    ) -> Result<Credentials, Box<dyn Error>> {
        let sql = "
        SELECT * FROM credentials WHERE user_identity = $user_identity
        ";

        let mut query_result = self
            .database
            .query(sql)
            .bind(("user_identity", user_identity))
            .await?;

        let credentials_record: Vec<SurrealCredentialRecord> = query_result.take(0)?;
        let credentials = Credentials::new(
            &credentials_record[0].user_identity,
            &credentials_record[0].hashed_password,
        );
        Ok(credentials)
    }

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials, Box<dyn Error>> {
        let cred_record: Option<SurrealCredentialRecord> =
            self.database.select(("session", id)).await?;

        if let Some(record) = cred_record {
            let cred = Credentials::new(&record.user_identity, &record.hashed_password);
            return Ok(cred);
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No Credentials record found",
            )));
        }
    }

    async fn insert_credentials(&self, user: &Credentials) -> Result<(), Box<dyn Error>> {
        self.database
            .create::<Vec<SurrealCredentialRecord>>("credentials")
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
    async fn test_surreal_creds_repository() {
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let creds_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        // Create new creds
        let creds = Credentials::new(email, password);
        creds_repo.insert_credentials(&creds).await.unwrap();

        // Test getting creds
        let user_cred = creds_repo
            .find_credentials_by_user_identity(email)
            .await
            .unwrap();
        assert_eq!(user_cred.user_identity, email);
    }
}
