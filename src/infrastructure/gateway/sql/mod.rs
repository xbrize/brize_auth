use std::{error::Error, fmt::format};

use crate::{
    application::{Authenticate, SessionRepository, UserRepository},
    domain::{Session, SessionRecordId, User},
};
use sqlx::{mysql::MySqlPool, query_builder, Execute, MySql, QueryBuilder};

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new(addr: &str) -> Self {
        let pool = MySqlPool::connect(addr)
            .await
            .expect("Failed to connect to SqlDb");

        Self { pool }
    }

    pub async fn create_session_table(&self) {
        sqlx::query(
            r#"
            CREATE TABLE sessions (
                id CHAR(36) PRIMARY KEY,  
                created_at BIGINT UNSIGNED NOT NULL,
                expires_at BIGINT UNSIGNED NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn create_user_table(&self) {
        sqlx::query(
            r#"
            CREATE TABLE users (
                id CHAR(36) PRIMARY KEY,  
                username CHAR(36) NOT NULL,
                password CHAR(36) NOT NULL,
                email CHAR(36) NOT NULL UNIQUE
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .unwrap();
    }
}

#[async_trait::async_trait]
impl SessionRepository for MySqlGateway {
    async fn store_session(&mut self, session: &Session) -> Result<(), Box<dyn Error>> {
        sqlx::query(
            r#"
            INSERT INTO sessions (id, created_at, expires_at)
            VALUES (?, ?, ?);
            "#,
        )
        .bind(&session.id)
        .bind(session.created_at as i64) // Converting usize to i64 for compatibility
        .bind(session.expires_at as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_session_by_id(
        &mut self,
        session_id: &SessionRecordId,
    ) -> Result<Session, Box<dyn Error>> {
        let session: Session = sqlx::query_as(
            r#"
        SELECT id, created_at, expires_at
        FROM sessions
        WHERE id = ?
        "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    async fn delete_session(
        &mut self,
        _session_record_id: &SessionRecordId,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl UserRepository for MySqlGateway {
    async fn store_user(&self, user: &User) -> Result<(), Box<dyn Error>> {
        // TODO maybe a way to do this
        // use serde::{Deserialize, Serialize};
        // use serde_json::Value;

        // #[derive(Serialize, Deserialize)]
        // struct Person {
        //     name: String,
        //     age: u8,
        // }

        // let person = Person {
        //     name: "Alice".to_string(),
        //     age: 30,
        // };
        // let value: Value = serde_json::to_value(&person).unwrap();

        // if let Value::Object(map) = value {
        //     for (key, val) in map {
        //         println!("{}: {}", key, val);
        //     }
        // }
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password, email)
            VALUES (?, ?, ?, ?);
            "#,
        )
        .bind(&user.id)
        .bind(&user.username) // Converting usize to i64 for compatibility
        .bind(&user.password)
        .bind(&user.email)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // TODO scrub out user's password
    async fn find_user_by_email(&self, email: &str) -> Result<User, Box<dyn Error>> {
        let user: User = sqlx::query_as(
            r#"
            SELECT id, username, email, password
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }
}

#[async_trait::async_trait]
impl Authenticate for MySqlGateway {
    async fn check_for_unique_fields(
        &self,
        fields: &Vec<(&str, &str, bool)>,
    ) -> Result<bool, Box<dyn Error>> {
        let mut where_query_builder: QueryBuilder<MySql> =
            QueryBuilder::new("SELECT 1 FROM users WHERE ");

        for (index, field) in fields.iter().enumerate() {
            let (key, value, is_unique) = field;

            if *is_unique {
                if index == 0 {
                    let f = format!("{key} = ");
                    where_query_builder.push(f);
                    where_query_builder.push_bind(value);
                } else {
                    let f = format!(" OR {key} = ");
                    where_query_builder.push(f);
                    where_query_builder.push_bind(value);
                }
            }
        }

        let where_sql = where_query_builder.build_query_scalar::<i64>();
        let res_sql = where_sql.fetch_one(&self.pool).await?;

        if res_sql > 0 {
            return Ok(false);
        }

        Ok(true)
    }

    async fn register(&self, fields: Vec<(&str, &str, bool)>) -> Result<bool, Box<dyn Error>> {
        if !self.check_for_unique_fields(&fields).await? {
            return Ok(false);
        }

        let mut insert_statement = String::from("INSERT INTO users (");

        for (index, field) in fields.iter().enumerate() {
            let key = field.0;
            let is_last_field = fields.len() - 1 == index;

            if is_last_field {
                let column_name = format!("{})", key);
                insert_statement.push_str(&column_name);
            } else {
                let column_name = format!("{},", key);
                insert_statement.push_str(&column_name);
            }
        }

        let start_statement = format!("{} VALUES (", insert_statement);
        let mut query_builder: QueryBuilder<MySql> =
            query_builder::QueryBuilder::new(start_statement);

        let mut seperated = query_builder.separated(", ");

        for field in fields {
            seperated.push_bind(field.1);
        }
        seperated.push_unseparated(");");

        let query = query_builder.build();
        query.execute(&self.pool).await?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Expiry;

    use super::*;

    #[tokio::test]
    async fn test_mysql_session_repo() {
        let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
        let mut repo = MySqlGateway::new(url).await;
        repo.create_session_table().await;

        let session = &Session::new(Expiry::Day(1));
        let query = repo.store_session(session).await;
        assert!(query.is_ok());

        let session_from_repo = repo.get_session_by_id(&session.id).await.unwrap();
        assert_eq!(session_from_repo.is_expired(), false);
        assert_eq!(session_from_repo.id, session.id);
    }

    #[tokio::test]
    async fn test_mysql_user_repo() {
        let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
        let repo = MySqlGateway::new(url).await;
        repo.create_user_table().await;

        let username = "test-user-name";
        let password = "test-pass-word";
        let email = "test@email.com";

        // Create new user
        let user = User::new(username, password, email);
        repo.store_user(&user).await.unwrap();

        // Test getting user
        let user_record = repo.find_user_by_email(email).await.unwrap();
        dbg!(&user_record);
        assert_eq!(user_record.email, email);
    }
}
