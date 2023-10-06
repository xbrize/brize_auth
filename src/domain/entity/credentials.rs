#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type CredentialsId = String;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Credentials {
    pub id: CredentialsId,
    pub email: String,
    pub password: String,
}

impl Credentials {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            password: password.to_string(),
            email: email.to_string(),
        }
    }

    pub fn match_password(&self, password: &str) -> bool {
        &self.password == password
    }
}
