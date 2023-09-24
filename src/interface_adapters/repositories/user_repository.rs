use super::{DatabaseClient, Record};
use crate::entities::User;

pub struct UserRepository {
    database: DatabaseClient,
}

impl UserRepository {
    pub fn new(database: DatabaseClient) -> Self {
        Self { database }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Option<User> {
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
            username: $username,
            password: $password,
            email: $email,
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

    pub async fn init_user_table(&self) -> surrealdb::Result<()> {
        let sql = "
            DEFINE TABLE user SCHEMAFULL;
            DEFINE FIELD username ON TABLE user TYPE string;
            DEFINE FIELD password ON TABLE user TYPE string;
            DEFINE FIELD email ON TABLE user TYPE string;
            DEFINE FIELD created_at ON TABLE user TYPE datetime;
        ";

        self.database.query(sql).await?;

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

        // Init user table
        user_repo.init_user_table().await.unwrap();

        // Create new user
        let new_user = User::new(username, password, email);
        user_repo.create_user(&new_user).await.unwrap();

        // Test getting user
        let user_record = user_repo.find_user_by_email(email).await.unwrap();
        assert_eq!(user_record.get_email(), new_user.get_email());
    }
}
