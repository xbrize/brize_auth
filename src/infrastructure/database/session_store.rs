use super::DatabaseClient;
use crate::domain::session::Session;
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    id: RecordId,
    user_record_link: RecordId,
    session: Session,
}

pub struct SessionRepository<'a> {
    database: &'a DatabaseClient,
}

impl<'a> SessionRepository<'a> {
    pub fn new(database: &'a DatabaseClient) -> Self {
        Self { database }
    }
    pub async fn create_session(&self, user_record_link: RecordId) -> Option<RecordId> {
        let sql = "
        RETURN (CREATE session:uuid() CONTENT {
            user_record_link: $user,
            session: {
                created_at: time::now(),
                updated_at: <future> {time::now()},
                expires_at: time::now() + 2w,
                is_expired: <future> {expires_at < updated_at}
            },
        }).id
        ";

        match self
            .database
            .query(sql)
            .bind(("user", user_record_link))
            .await
        {
            Ok(mut response) => match response.take(0) {
                Ok(session_id) => session_id,
                Err(e) => {
                    println!("No session data in response:\n{}", e);
                    None
                }
            },
            Err(e) => {
                println!("Failed to create session:\n{}", e);
                None
            }
        }
    }

    pub async fn get_session(&self, session_record_id: RecordId) -> Option<SessionRecord> {
        match self.database.select(session_record_id).await {
            Ok(session) => session,
            Err(e) => {
                println!("Error getting session:\n{}", e);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::initialize_test_database;

    #[tokio::test]
    async fn test_session_model() {
        let db = initialize_test_database().await;
        let session_repo = SessionRepository::new(&db);

        let email = "test@email.com";

        let new_session_id = session_repo
            .create_session(RecordId::from(("user", email)))
            .await;
        assert!(new_session_id.is_some());

        // Test get session
        let session = session_repo.get_session(new_session_id.unwrap()).await;
        assert!(session.is_some());
    }
}
