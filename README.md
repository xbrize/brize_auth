# Brize Auth :construction:

A simple to use async basic auth and session library for MySql and SurrealDB.

Still a WIP, not in a usable state.

## Setup

Start Development Database, Non-Persist, No-Auth

```bash
cargo add brize_auth
```

## Usage

```rust
use brize_auth::{Auth, AuthConfig, GatewayType, DatabaseConfig, Expiry};

#[tokio::main]
fn main {
    // Set your database params
    let db_config = DatabaseConfig {
        host: "localhost:3306".to_string(),
        db_name: "mysql".to_string(),
        user_name: "root".to_string(),
        password: "my-secret-pw".to_string(),
    };

    // Start your auth config
    let config = AuthConfig::new()
        .set_credentials_gateway(GatewayType::MySql(db_config))
        .set_session_duration(Expiry::Day(1));

    // Init auth with configs
    let mut auth = Auth::new(config).await.unwrap();

    // Get user credentials from a request
    let user_identity = "test@gmail.com";
    let raw_password = "plokij1234!";

    // Create a new set of credentials..
    // .. returns the id of the credentials row, use this as a foreign key on your user table
    let user_key: Option<String> = auth.register(user_identity, raw_password).await;

    // Log user in and get a session token back
    let session_token: Result<String> = auth.login(user_identity, raw_password).await;

    // Validate token later for user
    let validation: Result<bool> = auth.validate_session(session.as_str()).await;

    // Log out and kill session
    auth.log_out(user_identity, session_id_or_jwt_token);

    // Change user credentials
    let new_identity = "brizzz@gmail.com";
    let new_password = "vbnm1234!";
    auth.change_user_identity(user_identity, new_identity);
    auth.change_user_password(raw_password, new_password);

    // Delete user
    auth.delete_credentials(user_identity, raw_password);

    // Delete session
    auth.delete_session(session_id);

    // Request fresh session
    let new_session = auth.get_fresh_session(old_session_id);

    // Request fresh token
    let new_token = auth.get_fresh_token(old_token);
}
```

## Config

Brize Auth has some opinions, but can be configured to your use case.

```rust
use brize_auth::{Auth, AuthConfig, GatewayType, DatabaseConfig, Expiry};

pub struct DatabaseConfig {
    pub db_name: String,
    pub password: String,
    pub user_name: String,
    pub host: String,
}

enum GatewayType {
    MySql(DatabaseConfig),
    Surreal(DatabaseConfig),
    Redis(DatabaseConfig),
}

enum Expiry {
    Second(u64),
    Day(u64),
    Week(u64),
    Month(u64),
    Year(u64),
}

let config = AuthConfig::new()
    // Set your preferred database tech for the credentials table
    .set_credentials_gateway(GatewayType::MySql(DatabaseConfig))
    // Table sessions are the default, and it defaults to your above GatewayType...
    //.. Set your session duration with the Expiry module
    .set_session_duration(Expiry::Month(1));
    // Override the default session GatewayType from above
    .set_session_gateway(GatewayType::Redis(DatabaseConfig))
    // Use JWT sessions instead of table sessions
    .set_jwt_session(Expiry::Day(1), SUPER_SECRET_FOR_ENCODING_DECODING);
    // Don't use the session feature at all
    .disable_sessions()

let auth = Auth::new(config).await;
```

## Roadmap

- [ ] Database
  - [ ] Authentication
    - [x] User Schema
    - [x] User Registration
    - [x] Login
    - [x] Session Management
    - [ ] Logout
    - [ ] Delete User
  - Authorization
    - [ ] Roles Schema
