#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type CredentialsId = String;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Credentials {
    pub credentials_id: CredentialsId,
    pub unique_identifier: String,
    pub hashed_password: String,
}

impl Credentials {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            credentials_id: uuid::Uuid::new_v4().to_string(),
            unique_identifier: email.to_string(),
            hashed_password: password.to_string(),
        }
    }

    pub fn match_password(&self, password: &str) -> bool {
        &self.hashed_password == password
    }
}
