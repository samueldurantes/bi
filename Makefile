.PHONY: setup
setup:
	@command -v docker-compose >/dev/null 2>&1 || { \
		echo >&2 "Error: docker-compose command not found"; \
		exit 1; \
	}
	cp .env.example .env
	docker-compose up -d

.PHONY: migrate
migrate:
	@command -v sqlx >/dev/null 2>&1 || { \
		echo >&2 "Error: sqlx command not found"; \
		exit 1; \
	}
	sqlx db setup

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: test
test:
	cargo test

.PHONY: run-dev
run-dev:
	env RUST_LOG=debug cargo run
