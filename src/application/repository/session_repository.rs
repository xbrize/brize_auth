use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

use crate::domain::{session::Session, RepositoryError};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: RecordId,
    pub user_record_link: RecordId,
    pub session: Session,
}

#[async_trait]
pub trait SessionRepository {
    async fn get_session(
        &self,
        session_record_id: RecordId,
    ) -> Result<SessionRecord, RepositoryError>;
    async fn create_session(&self, user_record_link: RecordId)
        -> Result<RecordId, RepositoryError>;
}
