use serde::{Deserialize, Serialize};

use super::Expiry;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

impl Claims {
    pub fn new(user_email: &str, expiry: Expiry) -> Self {
        Self {
            sub: user_email.to_owned(),
            exp: expiry.time(),
        }
    }
}
