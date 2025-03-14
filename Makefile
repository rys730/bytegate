migrate:
	sqlx migrate run --source ./src/migrations

status:
	sqlx migrate info

new-migration:
	@if [ -z "$(name)" ]; then \
		echo "Error: Migration name is required. Usage: make new-migration name=<migration_name>"; \
		exit 1; \
	fi
	cargo sqlx migrate add $(name) --source ./src/migrations
