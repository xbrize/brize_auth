use crate::{
    application::SessionRepository,
    domain::{Expiry, RepoResult, Session, SessionRecordId},
};

pub async fn start_session<T: SessionRepository>(
    repository: &mut T,
) -> RepoResult<SessionRecordId> {
    let session = Session::new(Expiry::Day(1));
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
