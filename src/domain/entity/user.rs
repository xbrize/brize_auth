#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub type UserRecordId = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    password: String,
    // pub created_at: String,
    // pub last_login: String,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            // created_at: "".to_string(),
            // last_login: "".to_string(),
        }
    }

    pub fn match_password(&self, password: &str) -> bool {
        &self.password == password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_model() {
        let username = "test-user-name";
        let password = "test-pass-word";
        let email = "test@email.com";

        let new_user = User::new(username, password, email);
        assert!(new_user.match_password(password));
    }
}
