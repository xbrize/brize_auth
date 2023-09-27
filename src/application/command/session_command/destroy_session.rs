use crate::application::{SessionRecordId, SessionRepository};

pub async fn destroy_session<T: SessionRepository>(
    repository: &T,
    session_record_id: &SessionRecordId,
) {
    match repository.delete_session(session_record_id).await {
        Ok(_) => (),
        Err(e) => println!("Destroy session failed:{:#?}", e),
    }
}
