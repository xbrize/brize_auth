use crate::application::{SessionRecordId, SessionRepository};

pub async fn validate_session<T: SessionRepository>(
    repository: &T,
    session_record_id: &SessionRecordId,
) -> bool {
    match repository.get_session(session_record_id).await {
        Ok(session_record) => {
            if session_record.session.is_expired {
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
