# Rust API Playground

A learning project for building APIs with Rust, Axum, and PostgreSQL.

## Quick Start

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install Docker and Docker Compose: https://docs.docker.com/get-docker/
3. Install libpq (required by Diesel):
    - On Ubuntu/Debian: `sudo apt-get install libpq-dev`
    - On macOS with Homebrew: `brew install libpq`
    - On Windows: Install PostgreSQL, which includes libpq
4. Clone and set up the project:
   ```
   git clone https://github.com/yourusername/hello_cargo.git
   cd hello_cargo
   ```
5. Install Diesel CLI:
   ```
   cargo install diesel_cli --no-default-features --features postgres
   ```
6. Start the PostgreSQL database:
   ```
   docker-compose up -d
   ```
7. Set up the database:
   ```
   diesel setup
   diesel migration run
   ```
8. Run the application:
   ```
   cargo run
   ```
9. API docs: http://localhost:8080/swagger-ui/

## Structure

- `src/api.rs`: API logic and routes
- `src/main.rs`: Entry point
- `src/models.rs`: Data models
- `src/repositories.rs`: Data access layer
- `src/services.rs`: Business logic
- `src/schema.rs`: Database schema
- `src/config.rs`: Configuration management
- `src/logging.rs`: Logging setup
- `tests/integration_test.rs`: API tests

## Commands

- Build: `cargo build`
- Test: `cargo test`
- Run: `cargo run`
- Start PostgreSQL: `docker-compose up -d`
- Stop PostgreSQL: `docker-compose down`
- Create a new migration: `diesel migration generate <name>`
- Run migrations: `diesel migration run`
- Revert migrations: `diesel migration revert`

## Features

- CRUD operations for users
- Swagger UI documentation
- Configuration management
- Logging
- PostgreSQL database integration
- Database migrations with Diesel
- Integration tests

## Database Management

- PostgreSQL runs on `localhost:5432`
- Database name: `hello_cargo`
- Username: `hello_cargo`
- Password: `NotSoStrongPassword`

## Environment Variables

- `RUN_MODE`: Set to `production` for production settings
- `APP_SERVER_PORT`: Override the server port
- `APP_LOG_LEVEL`: Set the log level (e.g., DEBUG, INFO)
- `APP_DATABASE_URL`: Set the database connection string

## Learning Goals

- Rust ownership and borrowing
- Async programming with Tokio
- API development with Axum
- Testing in Rust
- Database integration with Diesel
- Docker for development environments
- Configuration management
- Logging in Rust applications

## Next Steps

- Implement authentication and authorization
- Add more complex database queries and relationships
- Implement caching
- Set up CI/CD pipeline
- Explore performance optimizations

## Troubleshooting

### libpq issues

If you encounter errors related to `libpq` when building the project, ensure that you have the PostgreSQL client libraries installed on your system. The installation process varies depending on your operating system:

- **Ubuntu/Debian**:
  ```
  sudo apt-get update
  sudo apt-get install libpq-dev
  ```

- **macOS (using Homebrew)**:
  ```
  brew install libpq
  ```
  After installation, you may need to add the following to your `~/.cargo/config` file:
  ```toml
  [target.x86_64-apple-darwin]
  rustflags = ["-L", "/usr/local/opt/libpq/lib"]
  ```

- **Windows**:
  Install PostgreSQL from the official website: https://www.postgresql.org/download/windows/
  After installation, ensure that the PostgreSQL bin directory is in your system PATH.

After installing `libpq`, try rebuilding the project with `cargo build`.

## Resources

- Rust Book: https://doc.rust-lang.org/book/
- Axum Docs: https://docs.rs/axum/latest/axum/
- Tokio Tutorial: https://tokio.rs/tokio/tutorial
- Diesel Guide: https://diesel.rs/guides/getting-started
- Docker Documentation: https://docs.docker.com/

Feel free to experiment and modify this project as you learn. Happy coding!