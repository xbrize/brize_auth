use brize_auth::{Auth, AuthConfig, DatabaseConfig, Expiry, GatewayType, SessionType};

#[tokio::main]
async fn main() {
    let db_config = DatabaseConfig {
        host: "localhost:3306".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        password: "my-secret-pw".to_string(),
    };

    let config = AuthConfig::new()
        .set_credentials_gateway(GatewayType::MySql(db_config))
        .set_session_type(SessionType::Session(Expiry::Month(1)));

    let mut auth = Auth::new(config).await.unwrap();

    let user_identity = "test@gmail.com";
    let raw_password = "plokij1234!";

    let user_key = auth.register(user_identity, raw_password).await;
    dbg!(user_key);
    let session = auth.login(user_identity, raw_password).await.unwrap();
    let validation = auth.validate_session(session.as_str()).await.unwrap();
    dbg!(validation);
}
