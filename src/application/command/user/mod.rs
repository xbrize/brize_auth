use crate::{
    application::CredentialsRepository,
    domain::{Credentials, CredentialsId},
};

// TODO log out user command
pub async fn login_user<T: CredentialsRepository>(
    repository: &T,
    email: &str,
    password: &str,
) -> Option<CredentialsId> {
    match repository.find_credentials_by_user_identity(&email).await {
        Ok(user_record) => {
            if user_record.match_password(password) {
                println!("Login Successful");
                return Some(user_record.user_identity);
            } else {
                println!("Password Did Not Match");
                return None;
            }
        }
        Err(e) => {
            println!("Login user failed:{:#?}", e);
            None
        }
    }
}

pub async fn register_user<T: CredentialsRepository>(
    repository: &T,
    username: &str,
    password: &str,
    email: &str,
) -> Option<CredentialsId> {
    // TODO this does not cover the case of a db error. Needs a rewrite
    match repository.find_credentials_by_user_identity(email).await {
        Ok(user_record) => {
            println!("Credentials {} Already Exists", user_record.user_identity);
            return None;
        }
        Err(_) => {
            let user = Credentials::new(email, password);
            repository.insert_credentials(&user).await.unwrap();

            return Some(user.user_identity);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::SurrealGateway;

    use super::*;

    #[tokio::test]
    async fn test_register_command() {
        // Start database
        let user_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

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

    #[tokio::test]
    async fn test_login_command() {
        // Start database
        let user_repo = SurrealGateway::new("127.0.0.1:8000", "test", "test").await;

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-login@email.com";
        register_user(&user_repo, username, password, email).await;

        let login_attempt = login_user(&user_repo, email, password).await;
        assert!(login_attempt.is_some());
    }
}
