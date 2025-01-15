CREATE TABLE IF NOT EXISTS users
(
    uuid       UUID PRIMARY KEY,
    name       VARCHAR(255)                          NOT NULL,
    email      VARCHAR(255)                          NOT NULL,
    password   VARCHAR(255)                          NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX ON users (email);