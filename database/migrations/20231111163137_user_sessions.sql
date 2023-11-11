CREATE TABLE user_sessions (
    session_id CHAR(36) PRIMARY KEY,  
    created_at BIGINT UNSIGNED NOT NULL,
    expires_at BIGINT UNSIGNED NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    csrf_token CHAR(44) NOT NULL
);