#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    created_at: String,
    updated_at: String,
    expires_at: String,
    is_expired: bool,
}
