# Makefile for Rust Todo App
# This file provides convenient commands for development and linting

.PHONY: help build test lint fmt clippy check clean run dev install-tools

# Default target
help:
	@echo "Available commands:"
	@echo "  build        - Build the project"
	@echo "  test         - Run tests"
	@echo "  lint         - Run all linting tools"
	@echo "  fmt          - Format code with rustfmt"
	@echo "  clippy       - Run clippy linter"
	@echo "  check        - Check code without building"
	@echo "  clean        - Clean build artifacts"
	@echo "  run          - Run the application"
	@echo "  dev          - Run in development mode with auto-reload"
	@echo "  install-tools - Install required development tools"
	@echo "  ci           - Run all CI checks locally"
	@echo "  docker-build - Build Docker image"
	@echo "  docker-run   - Run Docker container"
	@echo ""
	@echo "Database commands:"
	@echo "  db-up        - Start PostgreSQL database"
	@echo "  db-down      - Stop database services"
	@echo "  db-logs      - View database logs"
	@echo "  db-shell     - Connect to database shell"
	@echo "  db-reset     - Reset database (removes all data)"
	@echo "  dev-up       - Start full development environment"
	@echo "  dev-down     - Stop development environment"

# Build the project
build:
	cargo build

# Build for release
build-release:
	cargo build --release

# Run tests
test:
	cargo test

# Run all linting tools
lint: fmt clippy check
	@echo "All linting checks completed!"

# Format code
fmt:
	cargo fmt --all

# Check formatting without applying changes
fmt-check:
	cargo fmt --all -- --check

# Run clippy
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

# Run clippy with strict settings
clippy-strict:
	cargo clippy --all-targets --all-features -- \
		-W clippy::all \
		-W clippy::pedantic \
		-W clippy::nursery \
		-A clippy::multiple-crate-versions \
		-A clippy::cargo-common-metadata \
		-A clippy::module-name-repetitions \
		-A clippy::missing-errors-doc \
		-A clippy::missing-panics-doc \
		-A clippy::needless-for-each \
		-D warnings

# Check code without building
check:
	cargo check --all-targets --all-features

# Clean build artifacts
clean:
	cargo clean

# Run the application
run:
	cargo run

# Run in development mode (requires cargo-watch)
dev:
	cargo watch -x run

# Install development tools
install-tools:
	rustup component add clippy rustfmt
	cargo install cargo-watch cargo-audit cargo-outdated cargo-deny cargo-edit sqlx-cli cargo-llvm-cov

# Security audit
audit:
	cargo audit --ignore RUSTSEC-2023-0071

# Check for outdated dependencies
outdated:
	cargo outdated

# Run dependency checks
deny:
	cargo deny check

# Run comprehensive checks (CI-like) - using regular clippy for CI
ci: fmt-check clippy test audit
	@echo "All CI checks passed!"

# Run strict checks (for local development)
ci-strict: fmt-check clippy-strict test audit deny
	@echo "All strict CI checks passed!"

# Fix common issues automatically
fix:
	cargo fix --allow-dirty --allow-staged
	cargo clippy --fix --allow-dirty --allow-staged
	cargo fmt

# Generate documentation
docs:
	cargo doc --open

# Run with specific log level
run-debug:
	RUST_LOG=debug cargo run

# Run with trace logging
run-trace:
	RUST_LOG=trace cargo run

# Docker commands
docker-build:
	docker build -t rust_todo_app .

docker-run:
	docker run -p 8000:8000 rust_todo_app

# Database commands
db-up:
	docker-compose up -d postgres

db-down:
	docker-compose down

db-logs:
	docker-compose logs -f postgres

db-shell:
	docker-compose exec postgres psql -U todo_user -d todo_app

db-reset:
	docker-compose down -v
	docker-compose up -d postgres

# Full development environment
dev-up:
	docker-compose up -d
	@echo "Database and pgAdmin are now running"
	@echo "pgAdmin: http://localhost:5050 (admin@todo.local / admin)"
	@echo "PostgreSQL: localhost:5432 (todo_user / todo_password)"

dev-down:
	docker-compose down

# GitHub Actions local testing (requires act)
act-lint:
	act -j lint

act-ci:
	act -j test

# Benchmark
bench:
	cargo bench

# Coverage (requires cargo-llvm-cov)
coverage:
	cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# Release preparation
prepare-release:
	@echo "Preparing release..."
	make ci
	@echo "All checks passed! Ready for release."
