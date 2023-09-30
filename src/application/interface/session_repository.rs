use crate::domain::{RepositoryError, Session, SessionRecordId};
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository {
    async fn store_session(
        &mut self,
        session: &Session,
    ) -> Result<SessionRecordId, RepositoryError>;
    async fn get_session_by_id(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<Session, RepositoryError>;
    async fn delete_session(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<(), RepositoryError>;
}
