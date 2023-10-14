# Brize Auth :construction:

A minimalistic and async authentication library.

Still a WIP, not in a usable state. Roadmap at bottom.

## Setup

First install the crate

```bash
cargo add brize_auth
```

Next, set up the database tables with this schema

```sql
-- Credentials table
CREATE TABLE credentials (
    id CHAR(36) PRIMARY KEY,
    user_identity CHAR(36) NOT NULL,
    hashed_password CHAR(36) NOT NULL
);

-- Sessions table
CREATE TABLE sessions (
    id CHAR(36) PRIMARY KEY,
    created_at BIGINT UNSIGNED NOT NULL,
    expires_at BIGINT UNSIGNED NOT NULL
);

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
        .set_session_type(SessionType::Session(Expiry::Month(1)));

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
    let validation: Result<bool> = auth.validate_session(session_token).await;

    // Logout user and delete the session
    let logout_status = Result<()> = auth.logout(session_token).await;
}
```

## Config

The preferred database and session type can be configured to your use case.

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

enum SessionType {
    JWT(Expiry),
    Session(Expiry),
    None,
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
    // Set your session type, TableSession, JWT, or None to disable and the duration
    .set_session_type(SessionType::Session(Expiry::Month(1)));
    // Override the default session GatewayType from above
    .set_session_gateway(GatewayType::Redis(DatabaseConfig))

let auth = Auth::new(config).await;
```

## Supported Databases

- MySql (credentials + sessions)
- SurrealDB (credentials + sessions)
- Redis (sessions only)

## Testing

### Setup

Install docker and run make sure the daemon is running

- [Link to docker](https://docs.docker.com/engine/install/)

Fork this repo

```cli
gh repo fork xbrize/brize_auth
```

### Running Tests

**All test scripts are in `./scripts` but feel free to make your own. You will need to chmod +x to the script files.**

After giving permission to execute, simply run them. Each is designed to spin up docker containers that are hosting generic databases. These are then used to run the tests against.

```bash
scripts/tests/<desired_script>.sh
```

## Roadmap

- [x] User Registration
  - [x] Create user credentials if none exist
  - [x] Deny if user credentials does exist
  - [x] Return credentials foreign key
- [x] Login
  - [x] Match user credentials
  - [x] Return session token if matched (if sessions enabled)
  - [x] Deny user if no match
  - [ ] Hash password
- [x] Session Management
  - [x] Create session
  - [x] Validate session
  - [x] Delete sessions based on age and logout
- [x] Logout
  - [x] Delete users session
- [x] Change Credentials
  - [x] Update user_identity
  - [x] Update user_password
- [x] Delete User
  - [x] Remove credentials and session from database
