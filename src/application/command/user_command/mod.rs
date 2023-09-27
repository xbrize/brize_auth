mod login_user;
pub use login_user::*;

mod register_user;
pub use register_user::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::DataStore;

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

    #[tokio::test]
    async fn test_login_command() {
        // Start database
        let user_repo = DataStore::new("127.0.0.1:8000", "test", "test").await;

        // Test registering new user
        let username = "test-user-name-two";
        let password = "test-pass-word-two";
        let email = "test-login@email.com";
        register_user(&user_repo, username, password, email).await;

        let login_attempt = login_user(&user_repo, email, password).await;
        assert!(login_attempt.is_some());
    }
}
