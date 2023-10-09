use brize_auth::{
    auth::{Auth, AuthConfig, GatewayConfig},
    domain::Expiry,
};

#[tokio::main]
async fn main() {
    let config = AuthConfig::new().set_credentials_gateway(GatewayConfig::MySqlGateway(
        "mysql://root:my-secret-pw@localhost:3306/mysql".to_string(),
    ));

    let mut auth = Auth::new(config).await.unwrap();

    let user_identity = "test@gmail.com";
    let raw_password = "plokij1234!";

    let session = auth.register(user_identity, raw_password).await;
    dbg!(session);
}
