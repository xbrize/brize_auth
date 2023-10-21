use crate::domain::entity::{Session, SessionToken};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository {
    async fn insert_session(&mut self, session: &Session) -> Result<()>;
    async fn get_session_by_id(&mut self, session_id: &SessionToken) -> Result<Session>;
    async fn delete_session(&mut self, session_id: &SessionToken) -> Result<()>;
}
