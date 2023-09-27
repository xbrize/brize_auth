use crate::{
    application::{destroy_session, validate_session, SessionRecordId},
    domain::SessionState,
    infrastructure::DataStore,
};

pub async fn handle_session_validation(session_record_id: SessionRecordId) -> SessionState {
    let repository = DataStore::new("127.0.0.1:8000", "test", "test").await;

    match validate_session(&repository, &session_record_id).await {
        true => SessionState::Valid,
        false => {
            destroy_session(&repository, &session_record_id).await;
            return SessionState::Invalid;
        }
    }
}
