use crate::application::{SessionRecordId, SessionRepository};

pub async fn validate_session<T: SessionRepository>(
    repository: &T,
    session_record_id: &SessionRecordId,
) -> bool {
    match repository.get_session(session_record_id).await {
        Ok(session_record) => {
            if session_record.session.is_expired {
                false
            } else {
                true
            }
        }
        Err(_) => false,
    }
}
