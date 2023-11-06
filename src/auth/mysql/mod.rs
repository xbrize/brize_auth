use crate::{infrastructure::gateway::mysql::MySqlGateway, DatabaseConfig, SessionType};
use anyhow::Result;

use super::Auth;

pub async fn init_auth(
    db_config: DatabaseConfig,
    session_type: SessionType,
) -> Result<Auth<MySqlGateway, MySqlGateway>> {
    let credentials_gateway = MySqlGateway::new(&db_config).await;

    match session_type {
        SessionType::None => Ok(Auth {
            credentials_gateway,
            session_gateway: None,
            session_type: SessionType::None,
        }),
        SessionType::Session(duration) => {
            let session_gateway = MySqlGateway::new(&db_config).await;

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
//     async fn test_auth_mysql() {
//         let db_config = DatabaseConfig {
//             host: "localhost".to_string(),
//             port: "3306".to_string(),
//             db_name: "mysql".to_string(),
//             user_name: "root".to_string(),
//             password: "my-secret-pw".to_string(),
//             namespace: None,
//         };

//         let repo = MySqlGateway::new(&db_config).await;
//         repo._create_credentials_table().await;
//         repo._create_session_table().await;

//         let config = AuthConfig::new()
//             .set_credentials_gateway(GatewayType::MySql(db_config))
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
