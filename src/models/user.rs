#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Thing,
    username: String,
    password: String,
    email: String,
    created_at: String,
}

pub enum RegisterStatus {
    Accepted(User),
    Denied,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            id: Thing::from(("user", email)),
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
            created_at: String::from(""),
        }
    }
}

pub async fn init_user_table(database: &Surreal<Client>) -> surrealdb::Result<()> {
    let sql = "
        DEFINE TABLE user SCHEMAFULL;
        DEFINE FIELD username ON TABLE user TYPE string;
        DEFINE FIELD password ON TABLE user TYPE string;
        DEFINE FIELD email ON TABLE user TYPE string;
        DEFINE FIELD created_at ON TABLE user TYPE datetime;
    ";

    database.query(sql).await?;

    Ok(())
}

pub async fn drop_user_table(database: Surreal<Client>) -> surrealdb::Result<()> {
    let sql = "
        DROP TABLE user;
    ";

    database.query(sql).await?;

    Ok(())
}

pub async fn create_user(database: &Surreal<Client>, user: &User) -> surrealdb::Result<()> {
    let sql = "
    CREATE user CONTENT {
        id: $id,
        username: $username,
        password: $password,
        email: $email,
        created_at: time::now()
    };
    ";

    database
        .query(sql)
        .bind(("id", &user.id))
        .bind(("username", &user.username))
        .bind(("password", &user.password))
        .bind(("email", &user.email))
        .await?;

    Ok(())
}

pub async fn get_user(database: &Surreal<Client>, email: &str) -> Option<User> {
    match database.select(("user", email)).await {
        Ok(user) => user,
        Err(e) => {
            println!("Error while getting user:\n{}", e);
            None
        }
    }
}

pub async fn register_user(database: &Surreal<Client>, user: User) -> bool {
    let does_user_exist = get_user(database, &user.email).await;
    match does_user_exist {
        Some(found_user) => {
            println!("User {} Already Exists", found_user.email);
            return false;
        }
        None => {
            match create_user(database, &user).await {
                Ok(_) => {
                    println!("User {} Has Been Created", user.email);
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

pub async fn login_user(database: &Surreal<Client>, email: &str, password: &str) -> Option<User> {
    let lookup_user = get_user(&database, email).await;

    match lookup_user {
        Some(user) => {
            if user.password == password {
                println!("Login Successful");
                return Some(user);
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
    use surrealdb::{engine::remote::ws::Ws, Surreal};

    #[tokio::test]
    pub async fn test_user_registration() {
        let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        init_user_table(&db).await.unwrap();

        let new_user = User::new("myusrname", "mypassword", "myemail@email.com");
        let register_status = register_user(&db, new_user).await;
        assert_eq!(register_status, true);
    }
}
