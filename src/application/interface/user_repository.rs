use std::error::Error;

use crate::domain::{Credentials, CredentialsId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: CredentialsId,
    pub user: Credentials,
    pub created_at: String,
}

// TODO run results through a filter as a safety guard for sensitive data
#[async_trait]
pub trait UserRepository {
    async fn find_user_by_email(&self, user_email: &str) -> Result<Credentials, Box<dyn Error>>;
    async fn check_for_unique_fields(
        &self,
        fields: &Vec<(&str, &str, bool)>,
    ) -> Result<bool, Box<dyn Error>>;
    // async fn find_user_by_username(&self, user_email: &str) -> Result<UserRecord, RepositoryError>;
    // async fn find_user_by_id(&self, user_email: &str) -> Result<UserRecord, RepositoryError>;

    async fn store_user(&self, user: &Credentials) -> Result<(), Box<dyn Error>>;
}
