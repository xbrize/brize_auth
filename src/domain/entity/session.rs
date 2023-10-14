use crate::domain::config::Expiry;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type SessionId = String;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: SessionId,
    pub created_at: u64,
    pub expires_at: u64,
}

impl Session {
    pub fn new(duration: &Expiry) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Expiry::now(),
            expires_at: duration.time(),
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
        let session = Session::new(&Expiry::Second(1));
        assert!(!session.is_expired());
    }
}
