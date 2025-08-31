# Level 3: Component Diagram

## Overview

The Component diagram shows the internal structure of the Rust Todo App container, breaking down the application into its key components and their relationships.

## Component Architecture

```mermaid
C4Component
    title Component Diagram for Rust Todo App

    Container(webApp, "Web Application", "Rust, Axum", "Main application container")

    Component(router, "HTTP Router", "Axum Router", "Routes HTTP requests to appropriate handlers")
    Component(middleware, "Middleware Stack", "Axum Middleware", "Authentication, logging, CORS, error handling")
    
    Component(healthModule, "Health Module", "Rust Module", "System health checks and status reporting")
    Component(signupModule, "Signup Module", "Rust Module", "User registration and account creation")
    Component(loginModule, "Login Module", "Rust Module", "User authentication and session management")
    Component(todoModule, "Todo Module", "Rust Module", "Todo CRUD operations and business logic")
    
    Component(authService, "Auth Service", "Rust Service", "JWT token validation and user context")
    Component(dbService, "Database Service", "SQLx", "Database connection pool and query execution")
    Component(swaggerDoc, "OpenAPI Documentation", "Utoipa", "API specification and Swagger UI")
    
    ComponentDb(database, "PostgreSQL", "Database", "Data persistence layer")

    Rel(router, middleware, "Processes requests through", "HTTP")
    Rel(middleware, healthModule, "Routes health requests", "Function calls")
    Rel(middleware, signupModule, "Routes signup requests", "Function calls")
    Rel(middleware, loginModule, "Routes login requests", "Function calls")
    Rel(middleware, todoModule, "Routes todo requests", "Function calls")
    
    Rel(signupModule, authService, "Validates registration", "Function calls")
    Rel(loginModule, authService, "Authenticates users", "Function calls")
    Rel(todoModule, authService, "Validates tokens", "Function calls")
    
    Rel(signupModule, dbService, "Stores user data", "SQL queries")
    Rel(loginModule, dbService, "Retrieves user data", "SQL queries")
    Rel(todoModule, dbService, "CRUD operations", "SQL queries")
    Rel(healthModule, dbService, "Checks DB health", "SQL queries")
    
    Rel(dbService, database, "Executes queries", "PostgreSQL protocol")
    Rel(router, swaggerDoc, "Serves API docs", "HTTP")
```

## Core Components

### HTTP Router
- **Technology**: Axum Router
- **Responsibilities**:
  - Route matching and dispatch
  - Path parameter extraction
  - Method-based routing
- **Endpoints**:
  - `/health` → Health Module
  - `/signup` → Signup Module  
  - `/login` → Login Module
  - `/todos` → Todo Module
  - `/swagger-ui` → OpenAPI Documentation

### Middleware Stack
- **Technology**: Axum middleware layers
- **Components**:
  - **CORS Middleware**: Cross-origin request handling
  - **Auth Middleware**: JWT token validation
  - **Logging Middleware**: Request/response logging
  - **Error Middleware**: Centralized error handling
- **Processing Order**: CORS → Logging → Auth → Error → Route Handler

### Business Logic Modules

#### Health Module
- **Location**: `src/modules/health/`
- **Responsibilities**:
  - Application health status
  - Database connectivity checks
  - System resource monitoring
- **Endpoints**: `GET /health`
- **Dependencies**: Database Service

#### Signup Module  
- **Location**: `src/modules/signup/`
- **Responsibilities**:
  - User registration validation
  - Password strength requirements
  - Account creation workflow
- **Endpoints**: `POST /signup`
- **Dependencies**: Auth Service, Database Service

#### Login Module
- **Location**: `src/modules/login/`
- **Responsibilities**:
  - User authentication
  - Password verification
  - JWT token generation
- **Endpoints**: `POST /login`
- **Dependencies**: Auth Service, Database Service

#### Todo Module
- **Location**: `src/modules/todo/` (planned)
- **Responsibilities**:
  - Todo item CRUD operations
  - User-specific todo filtering
  - Todo status management
- **Endpoints**: 
  - `GET /todos` - List todos
  - `POST /todos` - Create todo
  - `PUT /todos/{id}` - Update todo
  - `DELETE /todos/{id}` - Delete todo
- **Dependencies**: Auth Service, Database Service

## Service Components

### Auth Service
- **Location**: `src/auth.rs`
- **Responsibilities**:
  - JWT token generation and validation
  - Password hashing (Argon2)
  - User context extraction
  - Session management
- **Key Functions**:
  - `hash_password()` - Secure password hashing
  - `verify_password()` - Password verification
  - `generate_token()` - JWT creation
  - `validate_token()` - JWT verification

### Database Service
- **Technology**: SQLx with PostgreSQL
- **Responsibilities**:
  - Connection pool management
  - Query execution and result mapping
  - Transaction management
  - Database health monitoring
- **Features**:
  - Compile-time SQL verification
  - Async query execution
  - Type-safe result mapping
  - Connection pooling

### OpenAPI Documentation
- **Technology**: Utoipa + Swagger UI
- **Responsibilities**:
  - API specification generation
  - Interactive documentation serving
  - Schema validation
- **Integration**: Compile-time API doc generation

## Data Flow Patterns

### Request Processing Flow
```
HTTP Request → Router → Middleware Stack → Module Handler → Service Layer → Database
```

### Authentication Flow
```
Request → Auth Middleware → JWT Validation → User Context → Handler
```

### Error Handling Flow
```
Error → Error Middleware → Structured Response → HTTP Client
```

## Module Structure

Each business module follows a consistent structure:

```
src/modules/{module_name}/
├── mod.rs           # Module exports and configuration
├── routes.rs        # HTTP route handlers
├── service.rs       # Business logic implementation
└── interfaces/      # Data structures and DTOs
    ├── mod.rs
    ├── requests.rs  # Request DTOs
    └── responses.rs # Response DTOs
```

## Component Dependencies

### Dependency Graph
```
Router
├── Middleware Stack
│   ├── Health Module → Database Service
│   ├── Signup Module → Auth Service, Database Service
│   ├── Login Module → Auth Service, Database Service
│   └── Todo Module → Auth Service, Database Service
├── Auth Service → Database Service
└── OpenAPI Documentation
```

### Shared Dependencies
- **Serde**: JSON serialization across all modules
- **Tokio**: Async runtime for all components
- **Tracing**: Logging infrastructure
- **SQLx**: Database access layer

## Error Handling Strategy

- **Module Level**: Business logic validation errors
- **Service Level**: Database and external service errors  
- **Middleware Level**: Authentication and authorization errors
- **Router Level**: HTTP protocol and routing errors

Each level transforms errors into appropriate HTTP responses with structured error messages.
