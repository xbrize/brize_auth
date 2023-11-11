use serde::{Deserialize, Serialize};

use crate::domain::config::Expiry;

use base64::{engine::general_purpose, Engine};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub type SessionToken = String;
pub type CsrfToken = String;

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub session_id: SessionToken,
    pub csrf_token: CsrfToken,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
}

impl Session {
    pub fn new(duration: &Expiry, user_id: &str) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            created_at: Expiry::now(),
            expires_at: duration.time(),
            user_id: user_id.to_string(),
            csrf_token: generate_csrf_token(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Expiry::now()
    }
}

fn generate_csrf_token() -> CsrfToken {
    // Generate a random alphanumeric string of length 32
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // Encode the random string using base64 for URL safety
    let token = general_purpose::STANDARD.encode(random_string.as_bytes());
    token
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_entity() {
        let session = Session::new(&Expiry::Second(1), "user_identity@mail.com");
        assert!(!session.is_expired());
        assert_eq!(session.csrf_token.len(), 44);
        assert_eq!(session.user_id, "user_identity@mail.com");
    }
}
