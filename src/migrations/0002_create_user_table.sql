CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    created_at timestamptz DEFAULT(CURRENT_TIMESTAMP),
    username varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    session varchar(255) NOT NULL
);