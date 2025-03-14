# Bytegate
URL Shortener using rust axum + sqlx. 
serves as learning purposes about rust backend development

## DB Migration

`cargo run --bin migrate`

## SQLx migration
`cargo sqlx prepare --database-url postgres://{DB_USER}:{DB_PASS}@{DB_HOST}:{DB_PORT}/{DB_NAME}`
