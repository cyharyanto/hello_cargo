# Rust API Playground

A learning project for building APIs with Rust and Axum.

## Quick Start

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install Docker and Docker Compose: https://docs.docker.com/get-docker/
3. Clone and set up the project:
   ```
   git clone https://github.com/yourusername/hello_cargo.git
   cd hello_cargo
   ```
4. Start the PostgreSQL database:
   ```
   docker-compose up -d
   ```
5. Run the application:
   ```
   cargo run
   ```
6. API docs: http://localhost:8080/swagger-ui/

## Structure

- `src/api.rs`: API logic and routes
- `src/main.rs`: Entry point
- `src/models.rs`: Data models
- `src/repositories.rs`: Data access layer
- `src/services.rs`: Business logic
- `tests/integration_test.rs`: API tests

## Commands

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run`
- Start PostgreSQL: `docker-compose up -d`
- Stop PostgreSQL: `docker-compose down`

## Features

- CRUD operations for users
- Swagger UI documentation
- Configuration management
- Logging
- PostgreSQL database (via Docker)

## Database Management

- PostgreSQL runs on `localhost:5432`
- PgAdmin4 is available at `http://localhost:5050`
   - Email: admin@example.com
   - Password: admin

## Learning Goals

- Rust ownership and borrowing
- Async programming with Tokio
- API development with Axum
- Testing in Rust
- Database integration with Diesel
- Docker for development environments

## Next Steps

- Implement Diesel ORM for database operations
- Add authentication
- Improve error handling

## Resources

- Rust Book: https://doc.rust-lang.org/book/
- Axum Docs: https://docs.rs/axum/latest/axum/
- Tokio Tutorial: https://tokio.rs/tokio/tutorial
- Diesel Guide: https://diesel.rs/guides/getting-started

Experiment freely. Learn by doing.