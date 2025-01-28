use crate::domain::entity::{Session, SessionToken};
use anyhow::Result;

#[trait_variant::make(HttpService: Send)]
pub trait SessionRepository: Send + Sync {
    async fn insert_session(&self, session: &Session) -> Result<()>;

    async fn get_session_by_id(&self, session_id: &SessionToken) -> Result<Session>;

    async fn delete_session(&self, session_id: &SessionToken) -> Result<()>;
}
