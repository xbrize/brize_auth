#![allow(dead_code)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    password: String,
    email: String,
    created_at: String,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            id: format!("user:{email}"),
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            created_at: String::from(""),
        }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_username(&self) -> String {
        self.id.to_string()
    }
    pub fn get_password(&self) -> String {
        self.password.to_string()
    }

    pub fn get_email(&self) -> String {
        self.email.to_string()
    }

    pub fn get_created_at(&self) -> String {
        self.created_at.to_string()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use surrealdb::{engine::remote::ws::Ws, Surreal};

//     async fn setup_db() -> Surreal<Client> {
//         let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
//         db.use_ns("test").use_db("test").await.unwrap();
//         db
//     }

//     #[tokio::test]
//     async fn test_user_model() {
//         let username = "test-user-name";
//         let password = "test-pass-word";
//         let email = "test@email.com";

//         // Start database
//         let db = setup_db().await;

//         // Init user table
//         init_user_table(&db).await.unwrap();

//         // Create new user
//         let new_user = User::new(username, password, email);
//         create_user(&db, &new_user).await.unwrap();

//         // Test getting user
//         let user = get_user(&db, email).await.unwrap();
//         assert_eq!(user.email, new_user.email);

//         // Test registering new user
//         let username = "test-user-name-two";
//         let password = "test-pass-word-two";
//         let email = "test-two@email.com";
//         let new_user = User::new(username, password, email);
//         let registration = register_user(&db, &new_user).await;
//         assert_eq!(registration, true);

//         // Test registration failure
//         let registration = register_user(&db, &new_user).await;
//         assert_eq!(registration, false);

//         // Test login
//         let user = login_user(&db, email, password).await.unwrap();
//         assert_eq!(user.username, username);
//     }
// }
