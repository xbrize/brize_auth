# Brize Auth

A tiny async authentication library.

## Summary

A tool for simplifying authentication in the Rust ecosystems. Purposefully built to be agnostic of your specific business/schema logic for managing users. Primarily controls the user **credentials** and optionally managing **sessions**. Built asynchronously with the Tokio runtime, and supports MySql and SurrealDB.

## Credentials

Brize auth **credentials** has 3 fields, an **id** for linking to your specific business/schema logic, the **user_identity** which should be a unique way to identify a user such as an email, and a **hashed_password**. This will be stored in a **user_credentials** table on your database.

## Sessions

The sessions are optional, in case you want to use some other session solution. If you do enable **sessions**, Brize auth offers classic table sessions, which have an **id** field as the token, **created_at** and **expired_at** for managing the expiration. The sessions will be stored in a **user_sessions** table on your database. A CSRF token is also available to use as **csrf_token** for form validation.

## Setup

First install the crate

```bash
cargo add brize_auth --features "mysql,sessions"
```

Next, set up the database tables with this schema, if using a SQL database

```sql
-- Credentials table
CREATE TABLE user_credentials (
    credentials_id CHAR(36) PRIMARY KEY,
    user_name VARCHAR(255) NOT NULL,
    hashed_password VARCHAR(255) NOT NULL
);

-- Sessions table
CREATE TABLE user_sessions (
    session_id CHAR(36) PRIMARY KEY,
    created_at BIGINT UNSIGNED NOT NULL,
    expires_at BIGINT UNSIGNED NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    csrf_token CHAR(44) NOT NULL
);
```

## Usage

### MySql feature

```rust
use anyhow::{Context, Result};
use brize_auth::{
    config::DatabaseConfig,
    mysql::MySqlGateway,
    AuthClient,
    SessionClient
};

#[tokio::main]
fn main {
    // Set your database params
    let db_config = DatabaseConfig {
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD not found"),
        user_name: env::var("DB_USER").expect("DB_USER not found"),
        host: env::var("DB_HOST").expect("DB_HOST not found"),
        port: env::var("DB_PORT").ok(),
        db_name: env::var("DB_NAME").expect("DB_NAME not found"),
        namespace: None,
    }

    // Start auth client
    let auth: AuthClient<MySqlGateway> = AuthClient::new_mysql_client(&db_config).await;

    // Get user credentials from a request
    let user_name = "test@gmail.com";
    let raw_password = "plokij1234!";

    // Create a new set of credentials..
    // .. returns the id of the credentials row, use this as some kind of reference key on YOUR user table
    let credentials_id: String = auth.register(user_identity, raw_password).await.unwrap();

    // Verify user credentials
    let is_valid_user = auth.verify_credentials(user_name, raw_password).await.unwrap();

    // Start session client
    let session_client: SessionClient<MySqlGateway> = SessionClient::new_mysql_client(&db_config).await;

    // Begin user session and configure expiration
    let session: Session = session_client.start_session(user_id, Expiry::Month(1)).await.unwrap();

    // Match csrf token
    let csrf_from_form = "saslfj00324-2lkjsdf-sdfksfkajlasjfngj"
    let is_valid_csrf: bool = session.match_csrf_toke(csrf_from_form);

    // End session for user
    session_client.destroy_session(&session.session_id).await.unwrap();

}
```

## Config

The preferred database and session expirations can be configured

```rust
use brize_auth::{DatabaseConfig, Expiry};

pub struct DatabaseConfig {
    pub db_name: String, // Name of database
    pub password: String, // Password for user
    pub user_name: String, // Name of user
    pub host: String, // Host IP
    pub port: Option<String>, // Port for host
    pub namespace: Option<String> // Optional namespace in db
}

enum Expiry {
    Second(u64), // Epoch seconds
    Day(u64), // Days in EPOCH
    Week(u64), // Weeks in EPOCH
    Month(u64), // Months in EPOCH
    Year(u64), // Years in EPOCH
}
```

## Supported Databases

- MySql (credentials + sessions)
- SurrealDB (credentials + sessions)

## Roadmap

### Prototype phase

- [x] User Registration
  - [x] Create user credentials if none exist
  - [x] Deny if user credentials does exist
  - [x] Return credentials foreign key
- [x] Login
  - [x] Match user credentials
  - [x] Return session token if matched (if sessions enabled)
  - [x] Deny user if no match
  - [x] Hash password
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

### Alpha testing phase

- [x] Code refactoring
  - [x] Domain module
  - [x] Application module
  - [x] Infrastructure module
  - [x] Library
- [ ] Live testing
  - [x] Secure production db testing
  - [ ] Benchmarking

### Beta features

- [x] Add feature splitting
- [x] add a port to config
- [ ] Add PostgresSQL support
- [ ] Add Sqlite support
- [ ] Configure custom table names for credentials and sessions
- [ ] Add refresh config for Session and Csrf tokens
