use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    sql::Thing,
    Surreal,
};

use crate::domain::entity::Credentials;
use crate::domain::{config::DatabaseConfig, entity::Session};
use crate::{
    application::interface::{CredentialsRepository, SessionRepository},
    domain::entity::SessionToken,
};

#[derive(Serialize, Deserialize)]
pub struct SurrealRecord<T> {
    id: Option<Thing>,
    data: T,
}

pub struct SurrealGateway {
    pub database: Surreal<Client>,
}

impl SurrealGateway {
    pub async fn new(config: &DatabaseConfig) -> Self {
        let address = format!("{}:{}", config.host, config.port);
        let db = Surreal::new::<Ws>(address.as_str())
            .await
            .expect("Failed connection with SurrealDB");

        db.signin(Root {
            username: config.user_name.as_str(),
            password: config.password.as_str(),
        })
        .await
        .expect("Failed to sign into SurrealDB");

        let namespace = match &config.namespace {
            Some(namespace) => namespace.as_str(),
            None => "",
        };

        db.use_ns(namespace)
            .use_db(config.db_name.as_str())
            .await
            .expect("Failed connection with SurrealDB");

        Self { database: db }
    }
}

#[async_trait::async_trait]
impl SessionRepository for SurrealGateway {
    async fn get_session_by_id(&self, session_id: &SessionToken) -> Result<Session> {
        let query_for_record: Option<SurrealRecord<Session>> = self
            .database
            .select(("user_sessions", session_id))
            .await
            .context("Failed to get session by id from Surreal")?;

        match query_for_record {
            Some(record) => Ok(record.data),
            None => Err(anyhow::anyhow!("Session not found by id in Surreal")),
        }
    }

    async fn insert_session(&self, session: &Session) -> Result<()> {
        let record = SurrealRecord {
            id: None,
            data: session,
        };

        self.database
            .create::<Option<SurrealRecord<Session>>>(("user_sessions", &session.session_id))
            .content(&record)
            .await
            .context("Failed to store session in Surreal")?;

        Ok(())
    }

    async fn delete_session(&self, session_id: &SessionToken) -> Result<()> {
        self.database
            .delete::<Option<SurrealRecord<Session>>>(("user_sessions", session_id))
            .await
            .context("Failed to delete session from Surreal")?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl CredentialsRepository for SurrealGateway {
    async fn find_credentials_by_user_identity(&self, user_identity: &str) -> Result<Credentials> {
        let sql = "
        SELECT * FROM user_credentials WHERE data.user_identity = $user_identity
        ";

        let mut query_result = self
            .database
            .query(sql)
            .bind(("user_identity", user_identity))
            .await
            .context("Failed to get credentials by user identity in Surreal")?;

        let mut records = query_result
            .take::<Vec<SurrealRecord<Credentials>>>(0)
            .context("Could not take from Surreal response")?;

        if !records.is_empty() {
            Ok(records.remove(0).data)
        } else {
            Err(anyhow::anyhow!(
                "Failed to find credentials by user identity in Surreal"
            ))
        }
    }

    async fn find_credentials_by_id(&self, id: &str) -> Result<Credentials> {
        let query_for_record: Option<SurrealRecord<Credentials>> =
            self.database.select(("user_credentials", id)).await?;

        match query_for_record {
            Some(record) => Ok(record.data),
            None => Err(anyhow::anyhow!(
                "Failed to find credentials by id in Surreal"
            )),
        }
    }

    async fn insert_credentials(&self, credentials: &Credentials) -> Result<()> {
        let record = SurrealRecord {
            id: None,
            data: credentials,
        };

        self.database
            .create::<Option<SurrealRecord<Credentials>>>(("user_credentials", &credentials.id))
            .content(&record)
            .await
            .context("Failed to insert credentials into Surreal")?;

        Ok(())
    }

    async fn update_user_identity(&self, current_identity: &str, new_identity: &str) -> Result<()> {
        let sql = "
            UPDATE user_credentials
            SET data.user_identity = $new_identity
            WHERE data.user_identity = $current_identity;
        ";

        self.database
            .query(sql)
            .bind(("new_identity", new_identity))
            .bind(("current_identity", current_identity))
            .await
            .context("Failed to update user identity in Surreal")?;

        Ok(())
    }

    async fn update_user_password(
        &self,
        user_identity: &str,
        new_hashed_password: &str,
    ) -> Result<()> {
        let sql = "
            UPDATE user_credentials
            SET data.hashed_password = $new_hashed_password
            WHERE data.user_identity = $user_identity;
        ";

        self.database
            .query(sql)
            .bind(("new_hashed_password", new_hashed_password))
            .bind(("user_identity", user_identity))
            .await
            .context("Failed to update password in Surreal")?;

        Ok(())
    }

    async fn delete_credentials_by_user_identity(&self, user_identity: &str) -> Result<()> {
        let sql = "
            DELETE FROM user_credentials
            WHERE data.user_identity = $user_identity;
        ";

        self.database
            .query(sql)
            .bind(("user_identity", user_identity))
            .await
            .context("Failed to delete credentials by user identity in Surreal")?;

        Ok(())
    }

    async fn delete_credentials_by_id(&self, id: &str) -> Result<()> {
        self.database
            .delete::<Option<SurrealRecord<Credentials>>>(("user_credentials", id))
            .await
            .context("Failed to delete credentials by id from Surreal")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{domain::config::Expiry, helpers::surreal_configs};

    use super::*;

    #[tokio::test]
    async fn test_surreal_session_repo() {
        let db_config = surreal_configs();
        let repo = SurrealGateway::new(&db_config).await;

        let session = Session::new(&Expiry::Day(1), "user_identity@mail.com");
        let query = repo.insert_session(&session).await;
        assert!(query.is_ok());

        let session_from_storage = repo.get_session_by_id(&session.session_id).await.unwrap();
        assert!(!session_from_storage.is_expired());
        assert_eq!(session_from_storage.session_id, session.session_id);
        assert_eq!(session_from_storage.csrf_token, session.csrf_token);

        repo.delete_session(&session.session_id).await.unwrap();
        let session_from_repo = repo.get_session_by_id(&session.session_id).await;
        assert!(session_from_repo.is_err());
    }

    #[tokio::test]
    async fn test_surreal_credentials_repo() {
        let password = "test-pass-word";
        let email = "test@email.com";

        // Start database
        let db_config = surreal_configs();
        let repo = SurrealGateway::new(&db_config).await;

        // Create new creds
        let creds = Credentials::new(email, password);
        repo.insert_credentials(&creds).await.unwrap();

        // Test getting creds
        let user_cred = repo.find_credentials_by_user_identity(email).await.unwrap();
        assert_eq!(user_cred.user_identity, email);

        // Test changing credentials
        let new_identity = "updatedidentity@gmail.com";
        let new_password = "the-updated-password";
        repo.update_user_identity(&creds.user_identity, new_identity)
            .await
            .unwrap();
        repo.update_user_password(&new_identity, new_password)
            .await
            .unwrap();

        let creds = repo.find_credentials_by_id(&creds.id).await.unwrap();
        assert_eq!(creds.user_identity, new_identity);

        // Delete credentials
        repo.delete_credentials_by_user_identity(&creds.user_identity)
            .await
            .unwrap();
        let creds = repo.find_credentials_by_id(&creds.id).await;
        assert!(creds.is_err());

        // Delete credentials by id
        let credentials = Credentials::new(email, password);
        repo.insert_credentials(&credentials).await.unwrap();

        repo.delete_credentials_by_id(&credentials.id)
            .await
            .unwrap();
        let creds = repo.find_credentials_by_id(&credentials.id).await;
        assert!(creds.is_err());
    }
}
