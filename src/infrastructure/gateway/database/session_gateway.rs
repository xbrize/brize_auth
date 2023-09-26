use crate::application::{SessionRecordId, UserRecordId};
use crate::infrastructure::DataStore;
use crate::{
    application::{SessionRecord, SessionRepository},
    domain::RepositoryError,
};
use async_trait::async_trait;

#[async_trait]
impl SessionRepository for DataStore {
    async fn create_session(
        &self,
        user_record_link: UserRecordId,
    ) -> Result<SessionRecordId, RepositoryError> {
        let sql = "
        CREATE session:uuid() CONTENT {
            user_record_link: $user,
            session: {
                created_at: time::now(),
                updated_at: <future> {time::now()},
                expires_at: time::now() + 2w,
                is_expired: <future> {expires_at < updated_at}
            },
        }
        ";

        let query_result = self
            .database
            .query(sql)
            .bind(("user", user_record_link))
            .await;

        match query_result {
            Ok(mut response) => {
                let session_record: Option<SessionRecord> = match response.take(0) {
                    Ok(session_record) => session_record,
                    Err(error) => {
                        println!("{error}");
                        return Err(RepositoryError::NotFound);
                    }
                };

                if let Some(record) = session_record {
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

    async fn get_session(
        &self,
        session_record_id: SessionRecordId,
    ) -> Result<SessionRecord, RepositoryError> {
        match self.database.select(session_record_id).await {
            Ok(session) => {
                let session: Option<SessionRecord> = match session {
                    Some(session) => session,
                    None => return Err(RepositoryError::NotFound),
                };

                if let Some(record) = session {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_gateway() {
        let session_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

        let email = "test@email.com";

        let new_session_id = session_repo
            .create_session(SessionRecordId::from(("user", email)))
            .await;
        assert!(new_session_id.is_ok());

        // Test get session
        let session = session_repo.get_session(new_session_id.unwrap()).await;
        assert!(session.is_ok());
    }
}
