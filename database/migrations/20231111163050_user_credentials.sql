CREATE TABLE user_credentials (
    credentials_id CHAR(36) PRIMARY KEY,
    user_name VARCHAR(255) NOT NULL,
    hashed_password VARCHAR(255) NOT NULL
);