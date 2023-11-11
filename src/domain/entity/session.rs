use serde::{Deserialize, Serialize};

use crate::domain::config::Expiry;

use base64::{engine::general_purpose, Engine};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub type SessionToken = String;
pub type CsrfToken = String;

fn generate_csrf_token() -> String {
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

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: SessionToken,
    pub csrf_token: CsrfToken,
    pub user_identity: String,
    pub created_at: u64,
    pub expires_at: u64,
}

impl Session {
    pub fn new(duration: &Expiry, user_identity: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Expiry::now(),
            expires_at: duration.time(),
            user_identity: user_identity.to_string(),
            csrf_token: generate_csrf_token(),
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
        assert_eq!(session.csrf_token.len(), 44);
        assert_eq!(session.user_identity, "user_identity@mail.com");
    }
}
