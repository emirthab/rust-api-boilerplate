CREATE TABLE user_logins
(
    id            SERIAL PRIMARY KEY,
    user_id       SERIAL REFERENCES users(id),
    device_id     VARCHAR(255) NOT NULL UNIQUE,
    ip_address    VARCHAR(255) NOT NULL,
    created_at    TIMESTAMP NOT NULL,
    updated_at    TIMESTAMP NOT NULL
)