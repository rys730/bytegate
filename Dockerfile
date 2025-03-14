FROM rust as chef
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
COPY . .

# use sqlx offline mode
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
COPY --from=builder /app/target/release/bytegate app

ENTRYPOINT [ "./app" ]