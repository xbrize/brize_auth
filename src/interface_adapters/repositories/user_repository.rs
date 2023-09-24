use serde::{Deserialize, Serialize};

use super::{DatabaseClient, RecordId};
use crate::entities::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: RecordId,
    pub user: User,
    pub created_at: String,
}

pub struct UserRepository {
    database: DatabaseClient,
}

impl UserRepository {
    pub fn new(database: DatabaseClient) -> Self {
        Self { database }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Option<UserRecord> {
        match self.database.select(("user", email)).await {
            Ok(user) => user,
            Err(e) => {
                println!("Error while finding user by email:\n{}", e);
                None
            }
        }
    }

    pub async fn create_user(&self, user: &User) -> surrealdb::Result<()> {
        let sql = "
        CREATE user CONTENT {
            id: $id,
            user: {
                username: $username,
                password: $password,
                email: $email,
            },
            created_at: time::now()
        };
        ";

        self.database
            .query(sql)
            .bind(("id", &user.get_email()))
            .bind(("username", &user.get_username()))
            .bind(("password", &user.get_password()))
            .bind(("email", &user.get_email()))
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface_adapters::initialize_test_database;

    #[tokio::test]
    async fn test_user_repository() {
        let username = "test-user-name";
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let db = initialize_test_database().await;
        let user_repo = UserRepository::new(db);

        // Create new user
        let new_user = User::new(username, password, email);
        user_repo.create_user(&new_user).await.unwrap();

        // Test getting user
        let user_record = user_repo.find_user_by_email(email).await.unwrap();
        assert_eq!(user_record.user.get_email(), new_user.get_email());
    }
}
