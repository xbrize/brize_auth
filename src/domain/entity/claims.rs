use serde::{Deserialize, Serialize};

use crate::domain::config::Expiry;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(user_identity: &str, expiry: &Expiry) -> Self {
        Self {
            sub: user_identity.to_string(),
            exp: expiry.time(),
        }
    }
}
