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
    user_identity: &str,
    raw_password: &str,
) -> Option<CredentialsId> {
    // TODO this does not cover the case of a db error. Needs a rewrite
    match repository
        .find_credentials_by_user_identity(user_identity)
        .await
    {
        Ok(user_record) => {
            println!("Credentials {} Already Exists", user_record.user_identity);
            return None;
        }
        Err(_) => {
            let credentials = Credentials::new(user_identity, raw_password);
            repository.insert_credentials(&credentials).await.unwrap();

            return Some(credentials.id);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::MySqlGateway;

    use super::*;

    #[tokio::test]
    async fn test_register_command() {
        // Start database
        let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
        let repo = MySqlGateway::new(url).await;
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
