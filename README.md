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
use brize_auth::{Auth, Config};

#[tokio::main]
fn main {
    let auth = Auth::new();

    auth.credentials_database(Config::SurrealDB); // Default is MySql
    auth.credential_table(Config::TableName("my_creds_table")) // Default is credentials

    // If you want a session based user auth
    auth.session_database(Config::Redis); // Default is same database as credentials
    auth.session_table(Config::TableName("my_sesh_table")) // Default is sessions
    // --- OR ---
    // If you want a JWT based user auth
    auth.use_jwt(Expiry::Month(2)); // Will return JWT tokens with a 3 month TTL


    let user_identity = "brizey@gmail.com"; // This has to be something unique
    let raw_password = "plokij1234!";

    // If successful, get your session id or jwt token back, user is auto logged in
    let session_id_or_jwt_token = auth.register(user_identity, raw_password).await.unwrap();

    // After registering, will be able to log in
    let session_id_or_jwt_token = auth.login(user_identity, raw_password).await.unwrap();

    // When user tries something
    let bool = auth.validate_session(session_id).await.unrwap();
    // --- OR ---
    let bool = auth.validate_token(jwt_token).await.unwrap();

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
