use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use crate::domain::session::Session;

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    id: RecordId,
    user_record_link: RecordId,
    session: Session,
}

#[async_trait]
pub trait SessionRepository {
    async fn get_session(&self, session_record_id: RecordId) -> Option<SessionRecord>;
    async fn create_session(&self, user_record_link: RecordId) -> Option<RecordId>;
}
