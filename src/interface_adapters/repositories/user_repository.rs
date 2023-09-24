use super::DatabaseClient;
use crate::entities::User;

pub struct UserRepository {
    database: DatabaseClient,
}

impl UserRepository {
    pub fn new(database: DatabaseClient) -> Self {
        Self { database }
    }

    pub async fn find_user_by_email(&self, email: &str) -> Option<User> {
        match self.database.select(("user", email)).await {
            Ok(user) => user,
            Err(e) => {
                println!("Error while getting user:\n{}", e);
                None
            }
        }
    }

    pub async fn create_user(&self, user: &User) -> surrealdb::Result<()> {
        let sql = "
        CREATE user CONTENT {
            id: $id,
            username: $username,
            password: $password,
            email: $email,
            created_at: time::now()
        };
        ";

        self.database
            .query(sql)
            .bind(("id", &user.get_email()))
            .bind(("username", &user.get_username()))
            .bind(("password", &user.get_password()))
            .bind(("email", &user.get_email()))
            .await?;

        Ok(())
    }

    pub async fn init_user_table(&self) -> surrealdb::Result<()> {
        let sql = "
            DEFINE TABLE user SCHEMAFULL;
            DEFINE FIELD username ON TABLE user TYPE string;
            DEFINE FIELD password ON TABLE user TYPE string;
            DEFINE FIELD email ON TABLE user TYPE string;
            DEFINE FIELD created_at ON TABLE user TYPE datetime;
        ";

        self.database.query(sql).await?;

        Ok(())
    }

    pub async fn drop_user_table(&self) -> surrealdb::Result<()> {
        let sql = "
            DROP TABLE user;
        ";

        self.database.query(sql).await?;

        Ok(())
    }
}
