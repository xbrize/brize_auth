use crate::domain::entity::{Session, SessionId};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository {
    async fn store_session(&mut self, session: &Session) -> Result<()>;
    async fn get_session_by_id(&mut self, session_id: &SessionId) -> Result<Session>;
    async fn delete_session(&mut self, session_record_id: &SessionId) -> Result<()>;
}
