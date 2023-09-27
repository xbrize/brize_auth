#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionState {
    Valid,
    Invalid,
    ExpiresSoon,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    created_at: String,
    updated_at: String,
    expires_at: String,
    pub is_expired: bool,
}
