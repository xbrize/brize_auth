use crate::{
    application::SessionRepository,
    domain::{Expiry, RepoResult, Session, SessionRecordId},
};

pub async fn start_session<T: SessionRepository>(
    repository: &mut T,
    session_duration: Expiry,
) -> RepoResult<SessionRecordId> {
    let session = Session::new(session_duration);
    let record_id = repository.store_session(&session).await?;

    Ok(record_id)
}

pub async fn validate_session<T: SessionRepository>(
    repository: &mut T,
    session_record_id: &SessionRecordId,
) -> bool {
    match repository.get_session_by_id(session_record_id).await {
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
    use crate::infrastructure::RedisGateway;

    #[tokio::test]
    async fn test_session_commands_with_reddis() {
        let mut repo = RedisGateway::new("redis://:mypassword@localhost/").await;

        let session_id = start_session(&mut repo, Expiry::Day(1)).await;
        assert!(session_id.is_ok());

        let session_id = session_id.unwrap();

        let is_valid = validate_session(&mut repo, &session_id).await;
        assert!(is_valid);
    }
}
