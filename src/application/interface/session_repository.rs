use std::error::Error;

use crate::domain::{Session, SessionRecordId};
use async_trait::async_trait;

#[async_trait]
pub trait SessionRepository {
    async fn store_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>>;
    async fn get_session_by_id(
        &mut self,
        session_id: &SessionRecordId,
    ) -> Result<Session, Box<dyn Error>>;
    async fn delete_session(
        &mut self,
        session_record_id: &SessionRecordId,
    ) -> Result<(), Box<dyn Error>>;
}
