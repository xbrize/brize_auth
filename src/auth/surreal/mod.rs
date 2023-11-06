use super::Auth;
use crate::{infrastructure::gateway::surreal::SurrealGateway, DatabaseConfig, SessionType};
use anyhow::Result;

pub async fn init_auth(
    db_config: DatabaseConfig,
    session_type: SessionType,
) -> Result<Auth<SurrealGateway, SurrealGateway>> {
    let credentials_gateway = SurrealGateway::new(&db_config).await;

    match session_type {
        SessionType::None => Ok(Auth {
            credentials_gateway,
            session_gateway: None,
            session_type: SessionType::None,
        }),
        SessionType::Session(duration) => {
            let session_gateway = SurrealGateway::new(&db_config).await;

            Ok(Auth {
                credentials_gateway,
                session_gateway: Some(session_gateway),
                session_type: SessionType::Session(duration),
            })
        }
        SessionType::JWT(duration) => Ok(Auth {
            credentials_gateway,
            session_gateway: None,
            session_type: SessionType::JWT(duration),
        }),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::domain::config::{DatabaseConfig, Expiry};

//     #[tokio::test]
//     async fn test_auth_surreal() {
//         let db_config = DatabaseConfig {
//             db_name: "test".to_string(),
//             host: "127.0.0.1".to_string(),
//             port: "8000".to_string(),
//             user_name: "root".to_string(),
//             password: "surreal_ps".to_string(),
//             namespace: Some("test".to_string()),
//         };

//         let config = AuthConfig::new()
//             .set_credentials_gateway(GatewayType::Surreal(db_config))
//             .set_session_type(SessionType::Session(Expiry::Month(1)));

//         let mut auth = Auth::new(config).await.unwrap();

//         let random_string = uuid::Uuid::new_v4().to_string();
//         let user_identity = &random_string[0..10];
//         let raw_password = &random_string[0..8];

//         auth.register(user_identity, raw_password).await.unwrap();
//         let session = auth.login(user_identity, raw_password).await.unwrap();
//         let validated_user = auth.validate_session(session.as_str()).await.unwrap();
//         assert_eq!(validated_user, user_identity);

//         auth.logout(&session).await.unwrap();
//         let validation = auth.validate_session(session.as_str()).await;
//         assert!(validation.is_err())
//     }
// }
