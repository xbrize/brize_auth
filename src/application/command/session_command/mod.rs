use crate::application::{SessionRecordId, SessionRepository, UserRecordId};

pub async fn start_session<T: SessionRepository>(
    repository: &T,
    user_record_id: &UserRecordId,
) -> Option<SessionRecordId> {
    match repository.create_session(user_record_id).await {
        Ok(record_id) => Some(record_id),
        Err(e) => {
            println!("Start session failed:{:#?}", e);
            None
        }
    }
}

pub async fn validate_session<T: SessionRepository>(
    repository: &T,
    session_record_id: &SessionRecordId,
) -> bool {
    match repository.get_session(session_record_id).await {
        Ok(session_record) => {
            if session_record.session.is_expired() {
                match repository.delete_session(session_record_id).await {
                    Ok(_) => (),
                    Err(e) => println!("Destroy session failed:{:#?}", e),
                }
                false
            } else {
                true
            }
        }
        Err(e) => {
            println!("Validating session failed:{:#?}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{application::UserRecordId, infrastructure::SurrealGateway};

    #[tokio::test]
    async fn test_session_commands() {
        let session_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;
        let email = "test@email.com";

        // Test starting session
        let session_id = start_session(&session_repo, &UserRecordId::from(("user", email))).await;
        assert!(session_id.is_some());

        // Test validating session
        let session_id = session_id.unwrap();
        let is_valid = validate_session(&session_repo, &session_id).await;
        assert_eq!(is_valid, true);
    }
}
