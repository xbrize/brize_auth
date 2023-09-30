use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::application::{UserRecord, UserRepository};
use crate::domain::{SessionRecordId, UserRecordId};
use crate::{
    application::SessionRepository,
    domain::{RepositoryError, Session},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurrealSessionRecord {
    pub id: Thing,
    pub created_at: usize,
    pub expires_at: usize,
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
    async fn get_session(
        &self,
        session_record_id: &SessionRecordId,
    ) -> Result<Session, RepositoryError> {
        match self.database.select(("session", session_record_id)).await {
            Ok(session) => {
                let session: Option<SurrealSessionRecord> = match session {
                    Some(session) => session,
                    None => return Err(RepositoryError::NotFound),
                };

                if let Some(record) = session {
                    let session = Session {
                        id: record.id.id.to_string(),
                        expires_at: record.expires_at,
                        created_at: record.created_at,
                    };
                    return Ok(session);
                } else {
                    return Err(RepositoryError::NotFound);
                }
            }
            Err(surreal_error) => {
                println!("Error while finding user by email:\n{}", surreal_error);
                return Err(RepositoryError::QueryFail);
            }
        }
    }

    async fn store_session(&self, session: Session) -> Result<SessionRecordId, RepositoryError> {
        let query_result: Result<Option<SurrealSessionRecord>, surrealdb::Error> = self
            .database
            .create(("session", &session.id))
            .content(&session)
            .await;

        match query_result {
            Ok(session) => match session {
                Some(session) => Ok(session.id.to_string()),
                None => Err(RepositoryError::NotFound),
            },
            Err(e) => {
                println!("Surreal DB failed to store session: {}", e);
                Err(RepositoryError::QueryFail)
            }
        }
    }

    async fn delete_session(
        &self,
        session_record_id: &SessionRecordId,
    ) -> Result<(), RepositoryError> {
        match self
            .database
            .delete::<Option<SurrealSessionRecord>>(("session", session_record_id))
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("{}", err);
                return Err(RepositoryError::QueryFail);
            }
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for SurrealGateway {
    async fn find_user_by_email(&self, email: &str) -> Result<UserRecord, RepositoryError> {
        match self.database.select(("user", email)).await {
            Ok(user_record) => {
                let user_record: Option<UserRecord> = match user_record {
                    Some(user_record) => user_record,
                    None => return Err(RepositoryError::NotFound),
                };

                if let Some(record) = user_record {
                    return Ok(record);
                } else {
                    return Err(RepositoryError::NotFound);
                }
            }
            Err(surreal_error) => {
                println!("Error while finding user by email:\n{}", surreal_error);
                return Err(RepositoryError::QueryFail);
            }
        }
    }

    async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<UserRecordId, RepositoryError> {
        let sql = "
        CREATE user CONTENT {
            id: $id,
            user: {
                username: $username,
                password: $password,
                email: $email,
            },
            created_at: time::now()
        };
        ";

        let query_result = self
            .database
            .query(sql)
            .bind(("id", email))
            .bind(("username", username))
            .bind(("password", password))
            .bind(("email", email))
            .await;

        match query_result {
            Ok(mut response) => {
                let user_record: Option<UserRecord> = match response.take(0) {
                    Ok(user_record) => user_record,
                    Err(error) => {
                        println!("{error}");
                        return Err(RepositoryError::NotFound);
                    }
                };

                if let Some(record) = user_record {
                    return Ok(record.id);
                } else {
                    return Err(RepositoryError::NotFound);
                }
            }
            Err(surreal_error) => {
                println!("{surreal_error}");
                Err(RepositoryError::QueryFail)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Expiry;

    use super::*;

    #[tokio::test]
    async fn test_surreal_session_repository() {
        let session_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        let email = "test@email.com";

        // Test create session
        let session = Session::new(Expiry::Day(1));
        let new_session_id = session_repo.store_session(session).await.unwrap();

        // Test get session
        let session = session_repo.get_session(&new_session_id).await;
        assert!(session.is_ok());

        // Test delete session
        let delete_status = session_repo.delete_session(&new_session_id).await;
        assert!(delete_status.is_ok());
        let session = session_repo.get_session(&new_session_id).await;
        assert!(session.is_err());
    }

    #[tokio::test]
    async fn test_surreal_user_repository() {
        let username = "test-user-name";
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let user_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        // Create new user
        user_repo
            .create_user(username, password, email)
            .await
            .unwrap();

        // Test getting user
        let user_record = user_repo.find_user_by_email(email).await.unwrap();
        assert_eq!(user_record.user.get_email(), email);
    }
}
