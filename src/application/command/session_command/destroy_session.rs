use crate::{
    application::{SessionRecordId, SessionRepository},
    domain::RepositoryError,
};

pub async fn destroy_session<T: SessionRepository>(
    repository: &T,
    session_record_id: &SessionRecordId,
) -> Result<(), RepositoryError> {
    repository.delete_session(session_record_id).await
}
