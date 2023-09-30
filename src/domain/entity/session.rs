#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use super::Expiry;

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionState {
    Valid,
    Invalid,
    ExpiresSoon,
}

pub type SessionRecordId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionRecordId,
    pub created_at: usize,
    pub expires_at: usize,
}

impl Session {
    pub fn new(session_duration: Expiry) -> Self {
        let now = Expiry::now();
        let duration = session_duration.time();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now as usize,
            expires_at: duration,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Expiry::now() as usize
    }
}
