use crate::{domain::User, infrastructure::user_repository::UserRepository};

pub async fn register_user(repository: &UserRepository<'_>, new_user: &User) -> bool {
    match repository.find_user_by_email(&new_user.get_email()).await {
        Some(user_record) => {
            println!("User {} Already Exists", user_record.user.get_email());
            return false;
        }
        None => {
            match repository.create_user(&new_user).await {
                Ok(_) => {
                    println!("User Has Been Created");
                    return true;
                }
                Err(e) => {
                    println!("Failed to create new user:\n {}", e);
                    return false;
                }
            };
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::initialize_test_database;

    #[tokio::test]
    async fn test_register_use_case() {
        // Start database
        let db = initialize_test_database().await;
        let user_repo = UserRepository::new(&db);

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-two@email.com";
        let new_user = User::new(username, password, email);
        let registration = register_user(&user_repo, &new_user).await;
        assert_eq!(registration, true);

        // Test registration failure
        let new_user = User::new(username, password, email);
        let registration = register_user(&user_repo, &new_user).await;
        assert_eq!(registration, false);
    }
}
