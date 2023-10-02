use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::application::{SessionRepository, UserRepository};
use crate::domain::{RepoResult, RepositoryError, Session, SessionRecordId, User, UserRecordId};

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
    ) -> Result<Session, RepositoryError> {
        match self.database.select(("session", session_record_id)).await {
            Ok(session) => {
                let session: Option<SurrealSessionRecord> = match session {
                    Some(session) => session,
                    None => return Err(RepositoryError::NotFound),
                };

                if let Some(record) = session {
                    let session = Session {
                        id: record.id.id.to_raw(),
                        expires_at: record.expires_at,
                        created_at: record.created_at,
                    };
                    return Ok(session);
                } else {
                    return Err(RepositoryError::NotFound);
                }
            }
            Err(surreal_error) => {
                println!("Error while finding session:\n{}", surreal_error);
                return Err(RepositoryError::QueryFail);
            }
        }
    }

    async fn store_session(&mut self, session: &Session) -> RepoResult<SessionRecordId> {
        let query_result: Result<Vec<SurrealSessionRecord>, surrealdb::Error> =
            self.database.create("session").content(&session).await;

        match query_result {
            Ok(session) => Ok(session[0].id.id.to_raw()),

            Err(e) => {
                println!("Surreal DB failed to store session: {}", e);
                Err(RepositoryError::QueryFail)
            }
        }
    }

    async fn delete_session(
        &mut self,
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
    async fn find_user_by_email(&self, email: &str) -> Result<User, RepositoryError> {
        let sql = "
        SELECT * from user where email = $email
        ";

        let mut query_result = self
            .database
            .query(sql)
            .bind(("email", email))
            .await
            .unwrap();

        let user_record: Vec<SurrealUserRecord> = query_result.take(0).unwrap();
        let user = User::new(
            &user_record[0].username,
            &user_record[0].password,
            &user_record[0].email,
        );
        Ok(user)
        //     match self.database.select(("user", email)).await {
        //         Ok(user_record) => {
        //             let user_record: Option<SurrealUserRecord> = match user_record {
        //                 Some(user_record) => user_record,
        //                 None => return Err(RepositoryError::NotFound),
        //             };

        //             if let Some(record) = user_record {
        //                 // TODO this will cause issues with created at and other timestamps
        //                 let user = User::new(&record.username, &record.password, &record.email);
        //                 return Ok(user);
        //             } else {
        //                 return Err(RepositoryError::NotFound);
        //             }
        //         }
        //         Err(surreal_error) => {
        //             println!("Error while finding user by email:\n{}", surreal_error);
        //             return Err(RepositoryError::QueryFail);
        //         }
        //     }
    }

    async fn store_user(&self, user: &User) -> Result<UserRecordId, RepositoryError> {
        let query_result: Result<Vec<SurrealUserRecord>, surrealdb::Error> =
            self.database.create("user").content(&user).await;

        match query_result {
            Ok(user) => Ok(user[0].id.id.to_raw()),

            Err(e) => {
                println!("Surreal DB failed to store session: {}", e);
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
        let mut repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        let session = Session::new(Expiry::Day(1));
        let session_id = repo.store_session(&session).await.unwrap();
        assert_eq!(session_id, session.id);

        let session_from_storage = repo.get_session_by_id(&session_id).await.unwrap();
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
