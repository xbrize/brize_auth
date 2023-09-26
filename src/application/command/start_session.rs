use crate::application::{SessionRecordId, SessionRepository, UserRecordId};

pub async fn start_session<T: SessionRepository>(
    session_repository: &T,
    user_record_id: UserRecordId,
) -> Option<SessionRecordId> {
    match session_repository.create_session(user_record_id).await {
        Ok(record_id) => Some(record_id),
        Err(_) => None,
    }
}
