# Rust Todo App

A modern todo application built with Rust, Axum, and comprehensive OpenAPI documentation.

## Features

- 🚀 **Fast & Efficient**: Built with Rust and Axum for high performance
- 📚 **OpenAPI Documentation**: Comprehensive API documentation with Swagger UI
- 🏥 **Health Checks**: Built-in health monitoring endpoints
- 🔧 **Developer Friendly**: Extensive linting and code quality tools
- 📦 **Modular Architecture**: Clean, maintainable code structure

## Quick Start

### Prerequisites

- Rust 1.70+ installed
- Cargo package manager

### Installation

1. Clone the repository:
```bash
git clone https://github.com/gaspecian/rust_todo_app.git
cd rust_todo_app
```

2. Install development tools:
```bash
make install-tools
```

3. Run the application:
```bash
make run
```

The application will be available at `http://127.0.0.1:8000`

## API Documentation

Once the application is running, you can access:

- **Swagger UI**: http://127.0.0.1:8000/swagger-ui
- **OpenAPI JSON**: http://127.0.0.1:8000/api-doc/openapi.json
- **Health Check**: http://127.0.0.1:8000/health

## Development

### Available Commands

```bash
# Development
make run          # Run the application
make dev          # Run with auto-reload (requires cargo-watch)
make build        # Build the project
make test         # Run tests

# Code Quality
make lint         # Run all linting tools
make fmt          # Format code
make clippy       # Run clippy linter
make check        # Check code without building

# Maintenance
make clean        # Clean build artifacts
make audit        # Security audit
make outdated     # Check for outdated dependencies
```

### Code Quality

This project uses comprehensive linting and code quality tools:

- **Clippy**: Advanced Rust linter with pedantic rules
- **Rustfmt**: Consistent code formatting
- **Cargo Audit**: Security vulnerability scanning
- **Pre-commit Hooks**: Automated quality checks

### Project Structure

```
src/
├── main.rs                 # Application entry point
├── modules/                # Application modules
│   └── health/            # Health check module
│       ├── mod.rs         # Module definition
│       ├── routes.rs      # HTTP routes
│       ├── service.rs     # Business logic
│       └── interfaces/    # Data structures
└── swagger/               # OpenAPI configuration
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `make lint` to ensure code quality
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
