use crate::domain::{RepositoryError, User, UserRecordId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: UserRecordId,
    pub user: User,
    pub created_at: String,
}

// TODO run results through a filter as a safety guard for sensitive data
#[async_trait]
pub trait UserRepository {
    async fn find_user_by_email(&self, user_email: &str) -> Result<User, RepositoryError>;
    // async fn find_user_by_username(&self, user_email: &str) -> Result<UserRecord, RepositoryError>;
    // async fn find_user_by_id(&self, user_email: &str) -> Result<UserRecord, RepositoryError>;

    async fn store_user(&self, user: &User) -> Result<UserRecordId, RepositoryError>;
}
