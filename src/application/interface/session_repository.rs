use crate::domain::{RepoResult, Session, SessionRecordId};
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository {
    async fn store_session(&mut self, session: &Session) -> RepoResult<SessionRecordId>;
    async fn get_session_by_id(&mut self, session_id: &SessionRecordId) -> RepoResult<Session>;
    async fn delete_session(&mut self, session_record_id: &SessionRecordId) -> RepoResult<()>;
}
