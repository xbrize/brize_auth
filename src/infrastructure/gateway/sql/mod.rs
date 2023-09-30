use sqlx::mysql::MySqlPool;

use crate::domain::{Session, SessionRecordId};

pub struct MySqlGateway {
    pub pool: MySqlPool,
}

impl MySqlGateway {
    pub async fn new() -> Self {
        let url = "mysql://root:my-secret-pw@localhost:3306/mysql";
        let pool = MySqlPool::connect(&url)
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
}

impl MySqlGateway {
    pub async fn create_session(&self, session: &Session) {
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
        .await
        .unwrap();
    }
    pub async fn get_session_by_id(&self, session_id: &SessionRecordId) {
        let row: Session = sqlx::query_as(
            r#"
        SELECT id, created_at, expires_at
        FROM sessions
        WHERE id = ?
        "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        dbg!(row);
    }
}
