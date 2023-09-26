use surrealdb::opt::RecordId;

use super::DataStore;
use crate::{
    application::{UserRecord, UserRepository},
    domain::RepositoryError,
};

#[async_trait::async_trait]
impl UserRepository for DataStore {
    async fn find_user_by_email(&self, email: &str) -> Result<UserRecord, RepositoryError> {
        match self.database.select(("user", email)).await {
            Ok(user_record) => {
                let user_record: Option<UserRecord> = match user_record {
                    Some(user_record) => user_record,
                    None => return Err(RepositoryError::NotFound),
                };

                if let Some(record) = user_record {
                    return Ok(record);
                } else {
                    return Err(RepositoryError::NotFound);
                }
            }
            Err(surreal_error) => {
                println!("Error while finding user by email:\n{}", surreal_error);
                return Err(RepositoryError::QueryFail);
            }
        }
    }

    async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
    ) -> Result<RecordId, RepositoryError> {
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

        let query_result = self
            .database
            .query(sql)
            .bind(("id", email))
            .bind(("username", username))
            .bind(("password", password))
            .bind(("email", email))
            .await;

        match query_result {
            Ok(mut response) => {
                let user_record: Option<UserRecord> = match response.take(0) {
                    Ok(user_record) => user_record,
                    Err(error) => {
                        println!("{error}");
                        return Err(RepositoryError::NotFound);
                    }
                };

                if let Some(record) = user_record {
                    return Ok(record.id);
                } else {
                    return Err(RepositoryError::NotFound);
                }
            }
            Err(surreal_error) => {
                println!("{surreal_error}");
                Err(RepositoryError::QueryFail)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_repository() {
        let username = "test-user-name";
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let user_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

        // Create new user
        user_repo
            .create_user(username, password, email)
            .await
            .unwrap();

        // Test getting user
        let user_record = user_repo.find_user_by_email(email).await.unwrap();
        assert_eq!(user_record.user.get_email(), email);
    }
}
