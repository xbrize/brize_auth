use serde::{Deserialize, Serialize};

use crate::domain::config::Expiry;

pub type SessionToken = String;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: SessionToken,
    pub created_at: u64,
    pub expires_at: u64,
    pub user_identity: String,
    pub csrf_token: String,
}

impl Session {
    pub fn new(duration: &Expiry, user_identity: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Expiry::now(),
            expires_at: duration.time(),
            user_identity: user_identity.to_string(),
            csrf_token: uuid::Uuid::new_v4().to_string(),
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
        let session = Session::new(&Expiry::Second(1), "user_identity@mail.com");
        assert!(!session.is_expired());
        assert_eq!(session.user_identity, "user_identity@mail.com");
    }
}
