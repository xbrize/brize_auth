use std::error::Error;

use crate::{
    application::SessionRepository,
    domain::{Expiry, Session, SessionRecordId},
};

pub async fn start_session<T: SessionRepository>(
    repository: &mut T,
    session_duration: Expiry,
) -> Result<SessionRecordId, Box<dyn Error>> {
    let session = Session::new(session_duration);
    repository.store_session(&session).await?;

    Ok(session.id)
}

pub async fn validate_session<T: SessionRepository>(
    repository: &mut T,
    session_record_id: &SessionRecordId,
) -> Result<bool, Box<dyn Error>> {
    let session = repository.get_session_by_id(session_record_id).await?;

    if session.is_expired() {
        repository.delete_session(session_record_id).await?;
        Ok(false)
    } else {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::{DatabaseConfig, RedisGateway};

    #[tokio::test]
    async fn test_session_commands_with_reddis() {
        let config = DatabaseConfig {
            host: "localhost".to_string(),
            password: "mypassword".to_string(),
            user_name: "".to_string(),
            db_name: "".to_string(),
        };

        let mut repo = RedisGateway::new(&config).await;

        let session_id = start_session(&mut repo, Expiry::Day(1)).await;
        assert!(session_id.is_ok());

        let session_id = session_id.unwrap();

        let is_valid = validate_session(&mut repo, &session_id).await.unwrap();
        assert!(is_valid);
    }
}
