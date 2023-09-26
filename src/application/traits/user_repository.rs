use crate::domain::User;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

#[derive(Debug)]
pub enum UserRepoError {
    QueryFail,
    NoUserRecord,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: RecordId,
    pub user: User,
    pub created_at: String,
}

#[async_trait]
pub trait UserRepository {
    async fn find_user_by_email(&self, user_email: &str) -> Result<UserRecord, UserRepoError>;

    async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<RecordId, UserRepoError>;
}
