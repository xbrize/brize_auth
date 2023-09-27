use crate::application::{UserRecordId, UserRepository};

pub async fn login_user<T: UserRepository>(
    repository: &T,
    email: &str,
    password: &str,
) -> Option<UserRecordId> {
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
        Err(e) => {
            println!("Login user failed:{:#?}", e);
            None
        }
    }
}
