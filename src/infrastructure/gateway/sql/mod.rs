use std::{error::Error, fmt::format};

use crate::{
    application::{Authenticate, SessionRepository, UserRepository},
    domain::{Session, SessionRecordId, User},
    infrastructure::gateway::sql,
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
    async fn register(&self, fields: Vec<(&str, &str)>, unique_fields: Vec<&str>) -> bool {
        // TODO lookup unique fields first
        // let mut where_clause = String::from("WHERE");
        // for (index, unique_field) in unique_fields.iter().enumerate() {
        //     let is_last_field = unique_fields.len() - 1 == index;

        //     if is_last_field {
        //         let field_name = format!("{} = ?;", unique_field);
        //         where_clause.push_str(&field_name);
        //     } else {
        //         let field_name = format!("{} = ? OR", unique_field);
        //         where_clause.push_str(&field_name);
        //     }
        // }

        // let sql = format!("SELECT 1 FROM users {}", where_clause);
        // let mut query_builder = query_builder::QueryBuilder::new(sql);

        // for field in fields.iter() {
        //     query_builder.push_bind(field.1);
        // }

        // let query = query_builder.build();
        // let qiz = query.execute(&self.pool).await;
        // dbg!(qiz);

        // ------------------------
        let mut insert_statement = String::from("INSERT INTO users (");

        for (index, field) in fields.iter().enumerate() {
            let is_last_field = fields.len() - 1 == index;

            if is_last_field {
                let column_name = format!("{})", field.0);
                insert_statement.push_str(&column_name);
            } else {
                let column_name = format!("{},", field.0);
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
        query.execute(&self.pool).await.unwrap();

        true
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
