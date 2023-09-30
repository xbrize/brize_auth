use super::Expiry;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type SessionRecordId = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionState {
    Valid,
    Invalid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: SessionRecordId,
    pub created_at: u64,
    pub expires_at: u64,
}

impl Session {
    pub fn new(session_duration: Expiry) -> Self {
        let now = Expiry::now();
        let duration = session_duration.time();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            expires_at: duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Expiry::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_entity() {
        let session = Session::new(Expiry::Second(1));
        assert!(!session.is_expired());
    }
}
