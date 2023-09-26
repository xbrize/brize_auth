use surrealdb::opt::RecordId;

use crate::application::UserRepository;

pub async fn login_user<T: UserRepository>(
    repository: &T,
    email: &str,
    password: &str,
) -> Option<RecordId> {
    match repository.find_user_by_email(&email).await {
        Ok(user_record) => {
            if user_record.user.get_password() == password {
                println!("Login Successful");
                return Some(user_record.id);
            } else {
                println!("Password Did Not Match");
                return None;
            }
        }
        Err(_) => {
            println!("Username Not Found");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        application::register_user,
        domain::User,
        infrastructure::{database::DataStore, initialize_test_database},
    };

    #[tokio::test]
    async fn test_register_use_case() {
        // Start database
        let db = initialize_test_database().await;
        let user_repo = DataStore::new(&db);

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-login@email.com";
        let new_user = User::new(username, password, email);
        register_user(&user_repo, &new_user).await;

        let login_attempt = login_user(&user_repo, email, password).await;
        assert!(login_attempt.is_some());
    }
}
