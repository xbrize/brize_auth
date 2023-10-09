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

    // let user_key = auth.register(user_identity, raw_password).await;
    // let session = auth.login(user_identity, raw_password).await;
    let validation = auth.validate_session("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ0ZXN0QGdtYWlsLmNvbSIsImV4cCI6MTY5OTQ4ODMxM30.SGZdzy9W_JtWl3QC5k0EyAQuGR6vvKJrkJSRLvTSIBg").await;
    dbg!(validation);
}
