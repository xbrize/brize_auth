#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    email: String,
    created_at: String,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            created_at: String::from(""),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.to_string()
    }
    pub fn get_password(&self) -> String {
        self.password.to_string()
    }

    pub fn get_email(&self) -> String {
        self.email.to_string()
    }

    pub fn get_created_at(&self) -> String {
        self.created_at.to_string()
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

        assert_eq!(new_user.get_email(), email);
        assert_eq!(new_user.get_username(), username);
        assert_eq!(new_user.get_password(), password);
    }
}
