CREATE TABLE url_maps (
    id SERIAL PRIMARY KEY,
    created_at timestamptz NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    updated_at timestamptz NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    expired_at timestamptz,
    destination_url varchar(255) NOT NULL,
    short_url varchar(255) NOT NULL
)