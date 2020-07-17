include .env
export

DATABASE_URL := postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}

start:
	@cargo run

up:
	@docker-compose up -d

down:
	@docker-compose down
