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
        Ok(credentials_query) => match credentials_query {
            Some(credentials) => {
                if credentials.match_password(password) {
                    println!("Login Successful");
                    return Some(credentials.user_identity);
                } else {
                    println!("Password Did Not Match");
                    return None;
                }
            }
            None => {
                println!("User Credentials Not Found For Login");
                return None;
            }
        },
        Err(e) => {
            println!("Error Logging In User:{:#?}", e);
            None
        }
    }
}

pub async fn register_user<T: CredentialsRepository>(
    repository: &T,
    user_identity: &str,
    raw_password: &str,
) -> Option<CredentialsId> {
    match repository
        .find_credentials_by_user_identity(user_identity)
        .await
    {
        Ok(credentials_query) => match credentials_query {
            Some(_) => {
                println!("Credentials Already Exist, User Not Created");
                return None;
            }
            None => {
                println!("New User Created");
                let credentials = Credentials::new(user_identity, raw_password);
                repository.insert_credentials(&credentials).await.unwrap();

                return Some(credentials.id);
            }
        },
        Err(e) => {
            println!("Failed to register user:{}", e);
            return None;
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::{DatabaseConfig, MySqlGateway};

    use super::*;

    #[tokio::test]
    async fn test_register_command() {
        // Start database
        let db_config = DatabaseConfig {
            host: "localhost:3306".to_string(),
            password: "my-secret-pw".to_string(),
            db_name: "mysql".to_string(),
            user_name: "root".to_string(),
        };

        let mut repo = MySqlGateway::new(db_config).await;
        repo.create_credentials_table().await;

        let password = "test-pass-word";
        let email = "test_email@gmail.com";

        // Test registration success
        let creds_id = register_user(&repo, email, password).await;
        assert!(creds_id.is_some());

        // Test login success
        let user = login_user(&repo, email, password).await;
        assert!(user.is_some());

        // Test registration failure
        let registration = register_user(&repo, email, password).await;
        assert!(registration.is_none());

        // Test login failure
        let login = login_user(&repo, email, "jello").await;
        assert!(login.is_none());
    }
}
