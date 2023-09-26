use super::{SessionRecordId, UserRecordId};
use crate::domain::{RepositoryError, Session};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: SessionRecordId,
    pub user_record_link: UserRecordId,
    pub session: Session,
}

#[async_trait]
pub trait SessionRepository {
    async fn get_session(
        &self,
        session_record_id: SessionRecordId,
    ) -> Result<SessionRecord, RepositoryError>;
    async fn create_session(
        &self,
        user_record_link: UserRecordId,
    ) -> Result<SessionRecordId, RepositoryError>;
}
