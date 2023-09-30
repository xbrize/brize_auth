use crate::{
    application::SessionRepository,
    domain::{Expiry, RepositoryError, Session, SessionRecordId},
};

pub async fn start_session<T: SessionRepository>(
    repository: &T,
) -> Result<SessionRecordId, RepositoryError> {
    let session = Session::new(Expiry::Day(1));
    let record_id = repository.store_session(&session).await?;

    Ok(record_id)
}

pub async fn validate_session<T: SessionRepository>(
    repository: &T,
    session_record_id: &SessionRecordId,
) -> bool {
    match repository.get_session(session_record_id).await {
        Ok(session) => {
            if session.is_expired() {
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
    use crate::{
        domain::{Expiry, Session},
        infrastructure::SurrealGateway,
    };

    #[tokio::test]
    async fn test_session_commands() {
        let session_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;
        let email = "test@email.com";

        // Test starting session
        let session = Session::new(Expiry::Day(1));
        let session = start_session(&session_repo).await;
        assert!(session.is_ok());

        // Test validating session
        let session = session.unwrap();
        let is_valid = validate_session(&session_repo, &session).await;
        assert_eq!(is_valid, true);
    }
}
