# Rust Todo App

<!-- Build and Quality Status -->
[![CI](https://github.com/gaspecian/rust_todo_app/workflows/CI/badge.svg)](https://github.com/gaspecian/rust_todo_app/actions/workflows/ci.yml)
[![Lint](https://github.com/gaspecian/rust_todo_app/workflows/Lint/badge.svg)](https://github.com/gaspecian/rust_todo_app/actions/workflows/lint.yml)
[![Security](https://github.com/gaspecian/rust_todo_app/workflows/Security/badge.svg)](https://github.com/gaspecian/rust_todo_app/actions/workflows/security.yml)
[![Dependencies](https://github.com/gaspecian/rust_todo_app/workflows/Dependencies/badge.svg)](https://github.com/gaspecian/rust_todo_app/actions/workflows/dependencies.yml)
[![Release](https://github.com/gaspecian/rust_todo_app/workflows/Release/badge.svg)](https://github.com/gaspecian/rust_todo_app/actions/workflows/release.yml)

<!-- Project Information -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)
[![GitHub release](https://img.shields.io/github/v/release/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/releases)
[![GitHub tag](https://img.shields.io/github/v/tag/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/tags)

<!-- Repository Stats -->
[![GitHub issues](https://img.shields.io/github/issues/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/pulls)
[![GitHub stars](https://img.shields.io/github/stars/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/network)

<!-- Docker and Deployment -->
[![Docker Image](https://img.shields.io/badge/docker-ghcr.io-blue.svg)](https://github.com/gaspecian/rust_todo_app/pkgs/container/rust_todo_app)
[![Docker Pulls](https://img.shields.io/docker/pulls/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/pkgs/container/rust_todo_app)

<!-- Code Quality -->
[![Lines of Code](https://img.shields.io/tokei/lines/github/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app)
[![Code Size](https://img.shields.io/github/languages/code-size/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app)
[![Repository Size](https://img.shields.io/github/repo-size/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app)
[![Last Commit](https://img.shields.io/github/last-commit/gaspecian/rust_todo_app)](https://github.com/gaspecian/rust_todo_app/commits/main)

A modern, high-performance todo application built with Rust, Axum web framework, PostgreSQL, and comprehensive OpenAPI documentation. Features enterprise-grade CI/CD, security scanning, and containerized deployment.

## 🚀 Features

- **⚡ High Performance**: Built with Rust and Axum for blazing-fast API responses
- **📚 OpenAPI Documentation**: Interactive Swagger UI with comprehensive API docs
- **🗄️ PostgreSQL Integration**: Robust database layer with connection pooling
- **🏥 Health Monitoring**: Built-in health checks and observability endpoints
- **🔒 Security First**: Automated security auditing and vulnerability scanning
- **🐳 Container Ready**: Multi-platform Docker images published to GitHub Container Registry
- **🔧 Developer Experience**: Extensive tooling, linting, and development automation
- **📦 Modular Architecture**: Clean, maintainable, and extensible code structure
- **🚦 CI/CD Pipeline**: Comprehensive GitHub Actions workflows for quality assurance

## 📋 Table of Contents

- [Quick Start](#-quick-start)
- [Installation Methods](#-installation-methods)
- [API Documentation](#-api-documentation)
- [Development](#-development)
- [Docker Usage](#-docker-usage)
- [Project Structure](#-project-structure)
- [Troubleshooting](#-troubleshooting)

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **PostgreSQL 12+** - [Install PostgreSQL](https://www.postgresql.org/download/)
- **Docker** (optional) - [Install Docker](https://docs.docker.com/get-docker/)

### Fastest Setup (Docker)

```bash
# Clone the repository
git clone https://github.com/gaspecian/rust_todo_app.git
cd rust_todo_app

# Start the full development environment
make dev-up

# The application will be available at http://localhost:8000
```

## 📦 Installation Methods

### Method 1: Local Development

```bash
# 1. Clone and setup
git clone https://github.com/gaspecian/rust_todo_app.git
cd rust_todo_app

# 2. Install development tools
make install-tools

# 3. Setup environment
cp .env.example .env
# Edit .env with your database configuration

# 4. Start database
make db-up

# 5. Run the application
make run
```

### Method 2: Docker Container

```bash
# Pull and run the latest image
docker pull ghcr.io/gaspecian/rust_todo_app:latest
docker run -p 8000:8000 \
  -e DATABASE_URL="postgresql://user:pass@host:5432/dbname" \
  ghcr.io/gaspecian/rust_todo_app:latest
```

### Method 3: Docker Compose

```bash
# Clone repository
git clone https://github.com/gaspecian/rust_todo_app.git
cd rust_todo_app

# Start everything with Docker Compose
docker-compose up -d
```

## 📚 API Documentation

Once running, access the comprehensive API documentation:

| Resource | URL | Description |
|----------|-----|-------------|
| **Swagger UI** | http://localhost:8000/swagger-ui | Interactive API documentation |
| **OpenAPI JSON** | http://localhost:8000/api-doc/openapi.json | Machine-readable API spec |


## 🛠️ Development

### Available Make Commands

```bash
# Development
make run              # Run the application
make dev              # Run with auto-reload (requires cargo-watch)
make build            # Build the project
make build-release    # Build optimized release version
make test             # Run all tests
make ci               # Run all CI checks locally

# Code Quality
make lint             # Run all linting tools
make fmt              # Format code with rustfmt
make clippy           # Run clippy linter
make check            # Check code without building

# Database Management
make db-up            # Start PostgreSQL database
make db-down          # Stop database services
make db-logs          # View database logs
make db-shell         # Connect to database shell
make db-reset         # Reset database (removes all data)

# Development Environment
make dev-up           # Start full development environment
make dev-down         # Stop development environment

# Docker Operations
make docker-build     # Build Docker image locally
make docker-run       # Run Docker container

# Maintenance
make clean            # Clean build artifacts
make audit            # Security audit
make outdated         # Check for outdated dependencies
```

### Code Quality Standards

This project maintains high code quality through:

- **🔍 Clippy**: Advanced Rust linter with pedantic rules
- **📐 Rustfmt**: Consistent code formatting
- **🔒 Cargo Audit**: Security vulnerability scanning
- **📊 Coverage**: Code coverage reporting
- **🚨 Pre-commit Hooks**: Automated quality checks
- **📋 Deny.toml**: Dependency and license compliance

### Development Workflow

1. **Setup Development Environment**:
   ```bash
   make install-tools
   make dev-up
   ```

2. **Make Changes**: Edit code with your preferred editor

3. **Run Quality Checks**:
   ```bash
   make lint          # Format and lint code
   make test          # Run tests
   make ci            # Full CI check locally
   ```

4. **Test Locally**:
   ```bash
   make run           # Start the application
   # Test your changes at http://localhost:8000
   ```

## 🐳 Docker Usage

### Pre-built Images

Images are automatically built and published to GitHub Container Registry:

```bash
# Latest stable release
docker pull ghcr.io/gaspecian/rust_todo_app:latest

# Specific version
docker pull ghcr.io/gaspecian/rust_todo_app:v1.0.0

# Development version (main branch)
docker pull ghcr.io/gaspecian/rust_todo_app:main
```

### Building Locally

```bash
# Build image
make docker-build

# Run container
make docker-run

# Or manually
docker build -t rust_todo_app .
docker run -p 8000:8000 rust_todo_app
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://postgres:password@localhost:5432/todo_app` |
| `RUST_LOG` | Logging level | `info` |
| `PORT` | Server port | `8000` |
| `HOST` | Server host | `0.0.0.0` |

## 🗄️ Database Setup

### Local PostgreSQL

```bash
# Start database with Docker Compose
make db-up

# Or install PostgreSQL locally and create database
createdb todo_app
```

### Database Configuration

Create `.env` file:

```env
DATABASE_URL=postgresql://postgres:password@localhost:5432/todo_app
RUST_LOG=debug
PORT=8000
HOST=127.0.0.1
```

## 📁 Project Structure

```
rust_todo_app/
├── 📁 src/
│   ├── 📄 main.rs                    # Application entry point
│   ├── 📁 modules/                   # Feature modules
│   │   ├── 📁 health/               # Health check endpoints
│   │   │   ├── 📄 mod.rs            # Module definition
│   │   │   ├── 📄 routes.rs         # HTTP route handlers
│   │   │   ├── 📄 service.rs        # Business logic
│   │   │   └── 📁 interfaces/       # Data structures & DTOs
│   │   └── 📁 signup/               # User registration
│   └── 📁 swagger/                  # OpenAPI configuration
│       └── 📄 doc_config.rs         # API documentation setup
├── 📁 .github/                      # GitHub Actions workflows
│   └── 📁 workflows/                # CI/CD pipelines
├── 📁 docs/                         # Additional documentation
├── 📁 db/                           # Database migrations
├── 📄 Dockerfile                    # Container configuration
├── 📄 docker-compose.yml            # Development environment
├── 📄 Makefile                      # Development commands
├── 📄 Cargo.toml                    # Rust dependencies
├── 📄 clippy.toml                   # Linter configuration
├── 📄 rustfmt.toml                  # Formatter configuration
└── 📄 deny.toml                     # Dependency policies
```

## 🔧 Troubleshooting

### Common Issues

**Database Connection Issues**:
```bash
# Check if PostgreSQL is running
make db-logs

# Reset database
make db-reset
make db-up
```

**Build Issues**:
```bash
# Clean and rebuild
make clean
make build
```

**Docker Issues**:
```bash
# Rebuild image
docker-compose down
docker-compose build --no-cache
docker-compose up
```

### Getting Help

- 📖 Check our [documentation](docs/)
- 🐛 [Report bugs](https://github.com/gaspecian/rust_todo_app/issues)
- 💬 [Ask questions](https://github.com/gaspecian/rust_todo_app/discussions)
- 📧 Contact: [your-email@example.com](mailto:your-email@example.com)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Modern web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation
- [Tokio](https://tokio.rs/) - Async runtime

---

**Built with ❤️ and Rust 🦀**
