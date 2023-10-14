use super::Expiry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(user_identity: &str, expiry: Expiry) -> Self {
        Self {
            sub: user_identity.to_string(),
            exp: expiry.time(),
        }
    }
}
