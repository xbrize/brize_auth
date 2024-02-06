use crate::{
    entity::{Session, SessionToken},
    interface::SessionRepository,
};
use anyhow::{Context, Result};
use sqlx::{mysql::MySqlRow, FromRow, Row};

use super::MySqlGateway;

impl FromRow<'_, MySqlRow> for Session {
    fn from_row(row: &MySqlRow) -> sqlx::Result<Self> {
        Ok(Self {
            session_id: row.try_get("session_id")?,
            created_at: row.try_get("created_at")?,
            expires_at: row.try_get("expires_at")?,
            user_id: row.try_get("user_id")?,
            csrf_token: row.try_get("csrf_token")?,
        })
    }
}

#[async_trait::async_trait]
impl SessionRepository for MySqlGateway {
    async fn insert_session(&self, session: &Session) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO user_sessions (session_id, created_at, expires_at, user_id, csrf_token)
            VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(session.session_id.as_str())
        .bind(session.created_at as i64) // Converting usize to i64 for compatibility
        .bind(session.expires_at as i64)
        .bind(session.user_id.as_str())
        .bind(session.csrf_token.as_str())
        .execute(&self.pool)
        .await
        .context("Failed to store session in MySql")?;

        Ok(())
    }

    async fn get_session_by_id(&self, session_id: &SessionToken) -> Result<Session> {
        let session: Session = sqlx::query_as(
            r#"
            SELECT session_id, created_at, expires_at, user_id, csrf_token
            FROM user_sessions
            WHERE session_id = ?
            "#,
        )
        .bind(session_id)
        .fetch_one(&self.pool)
        .await
        .context("Failed to get session by id ")?;

        Ok(session)
    }

    async fn delete_session(&self, session_id: &SessionToken) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM user_sessions 
            WHERE session_id = ?
            "#,
        )
        .bind(session_id)
        .execute(&self.pool)
        .await
        .context("Failed to delete session ")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Expiry, helpers::mysql_configs};

    #[tokio::test]
    async fn test_mysql_session_repo() {
        let db_config = mysql_configs();
        let repo = MySqlGateway::new(&db_config.mysql_connection_string()).await;

        let session = &Session::new(&Expiry::Day(1), "848hfhs0-88ryh-eohrnf-odsiru");
        let query = repo.insert_session(session).await;
        assert!(query.is_ok());

        let session_from_repo = repo.get_session_by_id(&session.session_id).await.unwrap();
        assert_eq!(session_from_repo.is_expired(), false);
        assert_eq!(session_from_repo.session_id, session.session_id);
        assert_eq!(session_from_repo.csrf_token, session.csrf_token);

        repo.delete_session(&session.session_id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.session_id).await;
        assert!(session_from_repo.is_err());
    }
}
