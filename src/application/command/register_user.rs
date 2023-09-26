use crate::application::{UserRecordId, UserRepository};
use crate::domain::User;

pub async fn register_user<T: UserRepository>(
    repository: &T,
    username: &str,
    password: &str,
    email: &str,
) -> Option<UserRecordId> {
    match repository.find_user_by_email(email).await {
        Ok(user_record) => {
            println!("User {} Already Exists", user_record.user.get_email());
            return None;
        }
        Err(_) => {
            match repository.create_user(username, password, email).await {
                Ok(record_id) => {
                    println!("User Has Been Created");
                    return Some(record_id);
                }
                Err(_) => {
                    println!("Failed to create new user");
                    return None;
                }
            };
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::DataStore;

    use super::*;

    #[tokio::test]
    async fn test_register_command() {
        // Start database
        let user_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-register@email.com";
        let registration = register_user(&user_repo, username, password, email).await;
        assert!(registration.is_some());

        // Test registration failure
        let registration = register_user(&user_repo, username, password, email).await;
        assert!(registration.is_none());
    }
}
