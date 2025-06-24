.PHONY: setup
setup:
	@command -v sqlx >/dev/null 2>&1 || { \
		echo >&2 "Error: sqlx command not found"; \
		exit 1; \
	}
	@command -v docker-compose >/dev/null 2>&1 || { \
		echo >&2 "Error: docker-compose command not found"; \
		exit 1; \
	}
	cp .env.example .env
	docker-compose up -d
	sqlx db setup

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: run-dev
run-dev:
	env RUST_LOG=debug cargo run
