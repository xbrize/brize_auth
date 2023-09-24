use crate::{entities::User, interface_adapters::user_repository::UserRepository};

pub async fn login_user(repository: UserRepository, email: &str, password: &str) -> Option<User> {
    match repository.find_user_by_email(&email).await {
        Some(user_record) => {
            if user_record.user.get_password() == password {
                println!("Login Successful");
                return Some(user_record.user);
            } else {
                println!("Password Did Not Match");
                return None;
            }
        }
        None => {
            println!("Username Not Found");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{interface_adapters::initialize_test_database, use_cases::register_user};

    #[tokio::test]
    async fn test_register_use_case() {
        // Start database
        let db = initialize_test_database().await;
        let user_repo = UserRepository::new(db);

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-two@email.com";
        let new_user = User::new(username, password, email);
        register_user(&user_repo, &new_user).await;

        let login_attempt = login_user(user_repo, email, password).await;
        assert!(login_attempt.is_some());
    }
}
