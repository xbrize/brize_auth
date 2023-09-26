use super::DataStore;
use crate::application::{SessionRecord, SessionRepository};
use async_trait::async_trait;
use surrealdb::opt::RecordId;

#[async_trait]
impl SessionRepository for DataStore {
    async fn create_session(&self, user_record_link: RecordId) -> Option<RecordId> {
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

    async fn get_session(&self, session_record_id: RecordId) -> Option<SessionRecord> {
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

    #[tokio::test]
    async fn test_session_model() {
        let session_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

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
