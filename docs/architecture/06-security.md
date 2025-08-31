# Security Architecture

## Overview

This document outlines the comprehensive security architecture for the Rust Todo App, covering authentication, authorization, data protection, and security controls throughout the application stack.

## Security Model

```mermaid
graph TB
    subgraph "Client Layer"
        Client[Web Client/API Consumer]
    end
    
    subgraph "Network Security"
        TLS[TLS 1.3 Encryption]
        WAF[Web Application Firewall]
        LB[Load Balancer]
    end
    
    subgraph "Application Security"
        Auth[JWT Authentication]
        AuthZ[Authorization Middleware]
        Valid[Input Validation]
        CORS[CORS Policy]
    end
    
    subgraph "Data Security"
        Hash[Password Hashing<br/>Argon2]
        Encrypt[Data Encryption]
        DB[(Encrypted Database)]
    end
    
    Client --> TLS
    TLS --> WAF
    WAF --> LB
    LB --> Auth
    Auth --> AuthZ
    AuthZ --> Valid
    Valid --> CORS
    CORS --> Hash
    Hash --> Encrypt
    Encrypt --> DB
```

## Authentication Architecture

### JWT Token-Based Authentication

```mermaid
sequenceDiagram
    participant Client
    participant App
    participant Auth as Auth Service
    participant DB as Database
    
    Client->>App: POST /signup (credentials)
    App->>Auth: hash_password()
    Auth->>DB: store user + hash
    DB-->>App: user created
    App-->>Client: 201 Created
    
    Client->>App: POST /login (credentials)
    App->>DB: get user by username
    DB-->>App: user data + hash
    App->>Auth: verify_password()
    Auth->>Auth: generate_token()
    Auth-->>App: JWT token
    App-->>Client: 200 OK + JWT
    
    Client->>App: GET /todos (Authorization: Bearer JWT)
    App->>Auth: validate_token()
    Auth-->>App: user claims
    App->>DB: get user todos
    DB-->>App: todo data
    App-->>Client: 200 OK + todos
```

### Password Security

**Argon2 Configuration**:
```rust
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}
```

**Security Properties**:
- **Algorithm**: Argon2id (memory-hard function)
- **Salt**: Cryptographically random per password
- **Iterations**: Configurable work factor
- **Memory Cost**: Resistant to GPU attacks

### JWT Token Management

**Token Structure**:
```json
{
  "header": {
    "alg": "HS256",
    "typ": "JWT"
  },
  "payload": {
    "iat": 1693494267,
    "user_id": 12345
  }
}
```

**Security Features**:
- **Session Management**: Time-based expiration using `iat` + session duration
- **Signing**: HMAC-SHA256 with secret key
- **Claims**: Minimal user information (only user_id and issued_at)
- **Stateless**: No server-side session storage
- **Automatic Extraction**: Built-in Axum extractor for request validation

## Authorization Framework

### Role-Based Access Control (RBAC)

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,      // Issued at timestamp
    pub user_id: i64,  // User identifier
}

// Automatic token extraction and validation
impl FromRequestParts<AppState> for Claims {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session_duration = state.session_duration_minutes;
        
        // Extract Bearer token from Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Decode and validate JWT token
        let token_data = decode::<Self>(
            bearer.token(),
            &state.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Check session expiration
        if token_data.claims.iat + (session_duration * 60) < Utc::now().timestamp() {
            return Err(StatusCode::UNAUTHORIZED);
        }

        Ok(token_data.claims)
    }
}
```

### Resource-Level Authorization

```rust
// Automatic user context extraction in handlers
pub async fn get_user_todos(
    claims: Claims,  // Automatically extracted and validated
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT * FROM todos WHERE user_id = $1",
        claims.user_id
    )
    .fetch_all(&app_state.db_pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(todos))
}
```

## Input Validation and Sanitization

### Request Validation

```rust
#[derive(Deserialize, Validate, ToSchema)]
pub struct SignupRequest {
    #[validate(length(min = 3, max = 50))]
    #[validate(regex = "USERNAME_REGEX")]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    #[validate(custom = "validate_password_strength")]
    pub password: String,
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new("missing_uppercase"));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new("missing_lowercase"));
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new("missing_number"));
    }
    Ok(())
}
```

### SQL Injection Prevention

**Parameterized Queries**:
```rust
// Safe: Using parameterized queries
sqlx::query_as!(
    User,
    "SELECT id, username FROM users WHERE username = $1",
    username
)
.fetch_one(pool)
.await
```

**Compile-time Verification**:
- SQLx verifies queries at compile time
- Type-safe parameter binding
- Automatic SQL injection prevention

## Data Protection

### Encryption at Rest

**Database Encryption**:
- PostgreSQL transparent data encryption (TDE)
- Encrypted storage volumes
- Encrypted database backups

**Application Secrets**:
```rust
// Environment-based secret management
let jwt_secret = std::env::var("JWT_SECRET")
    .expect("JWT_SECRET must be set");
let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
```

### Encryption in Transit

**TLS Configuration**:
- **Protocol**: TLS 1.3 minimum
- **Cipher Suites**: Strong encryption only
- **Certificate**: Valid SSL/TLS certificates
- **HSTS**: HTTP Strict Transport Security headers

**Database Connections**:
```rust
// Encrypted database connections
let database_url = "postgresql://user:pass@host:5432/db?sslmode=require";
```

## Security Headers and CORS

### HTTP Security Headers

```rust
use tower_http::set_header::SetResponseHeaderLayer;

let app = Router::new()
    .layer(SetResponseHeaderLayer::overriding(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    ))
    .layer(SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    ));
```

### CORS Policy

```rust
use tower_http::cors::{CorsLayer, Any};

let cors = CorsLayer::new()
    .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);
```

## Error Handling and Information Disclosure

### Secure Error Responses

```rust
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    // Never include sensitive details in production
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl From<ServiceError> for ErrorResponse {
    fn from(error: ServiceError) -> Self {
        match error {
            ServiceError::ValidationError(msg) => ErrorResponse {
                error: "validation_failed".to_string(),
                message: msg,
                details: None, // Never expose internal validation details
            },
            ServiceError::DatabaseError(_) => ErrorResponse {
                error: "internal_error".to_string(),
                message: "An internal error occurred".to_string(),
                details: None, // Never expose database errors
            },
        }
    }
}
```

### Logging Security

```rust
// Safe logging - no sensitive data
tracing::info!(
    user_id = %user_context.user_id,
    endpoint = %request.uri(),
    method = %request.method(),
    "Request processed successfully"
);

// Dangerous - never log passwords or tokens
// tracing::debug!("Password: {}", password); // DON'T DO THIS
```

## Container and Infrastructure Security

### Container Security

**Dockerfile Security**:
```dockerfile
# Use specific version tags
FROM rust:1.70-slim as builder

# Create non-root user
RUN addgroup --system --gid 1001 appgroup
RUN adduser --system --uid 1001 --gid 1001 appuser

# Set working directory
WORKDIR /app

# Copy and build application
COPY . .
RUN cargo build --release

# Production stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Create non-root user in production image
RUN addgroup --system --gid 1001 appgroup
RUN adduser --system --uid 1001 --gid 1001 appuser

# Copy binary and set ownership
COPY --from=builder /app/target/release/rust_todo_app /usr/local/bin/
RUN chown appuser:appgroup /usr/local/bin/rust_todo_app

# Switch to non-root user
USER appuser

EXPOSE 8000
CMD ["rust_todo_app"]
```

### Kubernetes Security

**Security Context**:
```yaml
apiVersion: apps/v1
kind: Deployment
spec:
  template:
    spec:
      securityContext:
        runAsNonRoot: true
        runAsUser: 1001
        fsGroup: 1001
      containers:
      - name: app
        securityContext:
          allowPrivilegeEscalation: false
          readOnlyRootFilesystem: true
          capabilities:
            drop:
            - ALL
        resources:
          limits:
            memory: "128Mi"
            cpu: "200m"
          requests:
            memory: "64Mi"
            cpu: "50m"
```

## Security Monitoring and Auditing

### Security Events

```rust
// Audit logging for security events
pub fn log_security_event(event_type: &str, user_id: Option<i64>, details: &str) {
    tracing::warn!(
        event_type = event_type,
        user_id = user_id,
        details = details,
        timestamp = %chrono::Utc::now(),
        "Security event detected"
    );
}

// Usage examples
log_security_event("failed_login", None, "Invalid password");
log_security_event("token_validation_failed", None, "Malformed JWT token");
log_security_event("session_expired", Some(user_id), "Token expired");
```

### Rate Limiting

```rust
use tower::limit::RateLimitLayer;
use std::time::Duration;

let app = Router::new()
    .layer(RateLimitLayer::new(100, Duration::from_secs(60))); // 100 requests per minute
```

## Vulnerability Management

### Dependency Scanning

**Cargo Audit Integration**:
```yaml
# GitHub Actions security workflow
- name: Security audit
  run: |
    cargo install cargo-audit
    cargo audit
```

**Deny Configuration** (`deny.toml`):
```toml
[advisories]
db-path = "~/.cargo/advisory-db"
db-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"
```

### Security Testing

### Automated Security Tests**:
```rust
#[tokio::test]
async fn test_jwt_token_validation() {
    let invalid_token = "invalid.jwt.token";
    let result = validate_token(invalid_token, &decoding_key);
    assert!(result.is_err()); // Should reject invalid tokens
}

#[tokio::test]
async fn test_session_expiration() {
    let expired_claims = Claims {
        iat: Utc::now().timestamp() - 7200, // 2 hours ago
        user_id: 123,
    };
    
    // Test that expired tokens are rejected
    let result = validate_session(&expired_claims, 60); // 60 minute session
    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_isolation() {
    let user1_claims = Claims { iat: Utc::now().timestamp(), user_id: 1 };
    let user2_claims = Claims { iat: Utc::now().timestamp(), user_id: 2 };
    
    // Ensure users can only access their own data
    let user1_todos = get_user_todos(user1_claims, &app_state).await.unwrap();
    let user2_todos = get_user_todos(user2_claims, &app_state).await.unwrap();
    
    assert_ne!(user1_todos, user2_todos);
}
```

## Compliance and Standards

### Security Standards Compliance

- **OWASP Top 10**: Protection against common web vulnerabilities
- **NIST Cybersecurity Framework**: Risk management approach
- **ISO 27001**: Information security management
- **SOC 2 Type II**: Security and availability controls

### Data Privacy

- **GDPR Compliance**: User data protection and privacy rights
- **Data Minimization**: Collect only necessary user information
- **Right to Deletion**: User account and data removal capabilities
- **Data Portability**: Export user data in standard formats
