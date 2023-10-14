#![allow(dead_code)]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub type CredentialsId = String;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Credentials {
    pub id: CredentialsId,
    pub user_identity: String,
    // #[serde(skip_serializing)]
    pub hashed_password: String,
}

impl Credentials {
    pub fn new(user_identity: &str, raw_password: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_identity: user_identity.to_string(),
            hashed_password: raw_password.to_string(),
        }
    }

    pub fn hash_password(mut self) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        self.hashed_password = argon2
            .hash_password(self.hashed_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        self
    }

    pub fn verify_password(&self, raw_password: &str) -> bool {
        let parsed_hash = PasswordHash::new(&self.hashed_password).unwrap();
        Argon2::default()
            .verify_password(raw_password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
