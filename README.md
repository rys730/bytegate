# Bytegate
URL Shortener using rust axum + sqlx. 
serves as learning purposes about rust backend development

## Setup

run `cargo sqlx prepare` to generate `.sqlx/` folder, which will be needed to build the docker image.

adjust env in the `docker-compose.yml` (or change it to use `.env`) then run `docker compose up`.

service will run on `0.0.0.0:3000`