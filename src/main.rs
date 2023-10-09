use brize_auth::{
    auth::{Auth, AuthConfig, GatewayType},
    domain::Expiry,
    infrastructure::{DatabaseConfig, MySqlGateway},
};

#[tokio::main]
async fn main() {
    let db_config = DatabaseConfig {
        host: "localhost:3306".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        password: "my-secret-pw".to_string(),
    };
    // let repo = MySqlGateway::new(db_config).await;
    // repo.create_credentials_table().await;
    // repo.create_session_table().await;
    let config = AuthConfig::new()
        .set_credentials_gateway(GatewayType::MySql(db_config))
        .set_session_duration(Expiry::Day(1));

    let mut auth = Auth::new(config).await.unwrap();

    let user_identity = "test@gmail.com";
    let raw_password = "plokij1234!";

    let user_key = auth.register(user_identity, raw_password).await;
    dbg!(user_key);
    // let session = auth.login(user_identity, raw_password).await;
    // let validation = auth.validate_session(session).await;
    // dbg!(validation);
}
