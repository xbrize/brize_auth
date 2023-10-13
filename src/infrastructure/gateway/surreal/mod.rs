use std::error::Error;

use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::application::{CredentialsRepository, SessionRepository};
use crate::domain::{Credentials, DatabaseConfig, Session, SessionRecordId};

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

impl SurrealCredentialRecord {
    pub fn into_credentials(&self) -> Credentials {
        Credentials {
            id: self.id.id.to_raw(),
            user_identity: self.user_identity.to_string(),
            hashed_password: self.hashed_password.to_string(),
        }
    }
}

pub struct SurrealGateway {
    pub database: Surreal<Client>,
}

impl SurrealGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let db = Surreal::new::<Ws>(config.host.as_str())
            .await
            .expect("Could not connect to database:");
        db.use_ns(config.user_name.as_str())
            .use_db(config.db_name.as_str())
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
    ) -> Result<Option<Credentials>, Box<dyn Error>> {
        let sql = "
        SELECT * FROM credentials WHERE user_identity = $user_identity
        ";

        let query_result = self
            .database
            .query(sql)
            .bind(("user_identity", user_identity))
            .await;

        match query_result {
            Ok(mut result) => match result.take::<Vec<SurrealCredentialRecord>>(0) {
                Ok(take) => {
                    if take.is_empty() {
                        println!("User Credentials Not Found");
                        Ok(None)
                    } else {
                        println!("User Credentials Found");
                        Ok(Some(take[0].into_credentials()))
                    }
                }
                Err(e) => Err(Box::new(e)),
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials, Box<dyn Error>> {
        let cred_record: Option<SurrealCredentialRecord> =
            self.database.select(("credentials", id)).await?;

        if let Some(record) = cred_record {
            let cred = record.into_credentials();
            return Ok(cred);
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Credentials record not found",
            )));
        }
    }

    async fn insert_credentials(&self, credentials: &Credentials) -> Result<(), Box<dyn Error>> {
        self.database
            .create::<Vec<SurrealCredentialRecord>>("credentials")
            .content(&credentials)
            .await?;

        Ok(())
    }

    async fn update_user_identity(
        &self,
        current_identity: &str,
        new_identity: &str,
    ) -> Result<(), Box<dyn Error>> {
        let sql = "
            UPDATE credentials
            SET user_identity = $new_identity
            WHERE user_identity = $current_identity;
        ";

        self.database
            .query(sql)
            .bind(("new_identity", new_identity))
            .bind(("current_identity", current_identity))
            .await?;

        Ok(())
    }

    async fn update_user_password(
        &self,
        user_identity: &str,
        new_raw_password: &str,
    ) -> Result<(), Box<dyn Error>> {
        // TODO hash this
        let new_hashed_password = new_raw_password;
        let sql = "
            UPDATE credentials
            SET hashed_password = $new_hashed_password
            WHERE user_identity = $user_identity;
        ";

        self.database
            .query(sql)
            .bind(("new_hashed_password", new_hashed_password))
            .bind(("user_identity", user_identity))
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
        let db_config = DatabaseConfig {
            db_name: "test".to_string(),
            host: "127.0.0.1:8000".to_string(),
            user_name: "test".to_string(),
            password: "".to_string(),
        };
        let mut repo = SurrealGateway::new(&db_config).await;

        let session = Session::new(Expiry::Day(1));
        let query = repo.store_session(&session).await;
        assert!(query.is_ok());

        let session_from_storage = repo.get_session_by_id(&session.id).await.unwrap();
        assert!(!session_from_storage.is_expired());
        assert_eq!(session_from_storage.id, session.id);

        repo.delete_session(&session.id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.id).await;
        assert!(session_from_repo.is_err());
    }

    #[tokio::test]
    async fn test_surreal_creds_repository() {
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let db_config = DatabaseConfig {
            db_name: "test".to_string(),
            host: "127.0.0.1:8000".to_string(),
            user_name: "test".to_string(),
            password: "".to_string(),
        };
        let repo = SurrealGateway::new(&db_config).await;

        // Create new creds
        let creds = Credentials::new(email, password);
        repo.insert_credentials(&creds).await.unwrap();

        // Test getting creds
        let user_cred = repo.find_credentials_by_user_identity(email).await.unwrap();
        assert_eq!(user_cred.unwrap().user_identity, email);

        // Test changing credentials
        let new_identity = "updatedidentity@gmail.com";
        let new_password = "the-updated-password";
        repo.update_user_identity(&creds.user_identity, new_identity)
            .await
            .unwrap();
        repo.update_user_password(&new_identity, new_password)
            .await
            .unwrap();

        let creds = repo.find_credentials_by_id(&creds.id).await.unwrap();
        assert_eq!(creds.user_identity, new_identity);
        // TODO this will fail after hashing password
        assert_eq!(creds.hashed_password, new_password);
    }
}
