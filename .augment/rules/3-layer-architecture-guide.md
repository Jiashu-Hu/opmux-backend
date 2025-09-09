---
type: 'manual'
---

# 3-Layer Architecture Guide

## Overview

This guide explains our 3-layer architecture pattern used throughout the Gateway service. This
pattern ensures clean separation of concerns, testability, and maintainability at both the
microservice level and individual feature level.

## Architecture Perspectives

### Microservice Level - Gateway as Service Coordinator

At the microservice architecture level, our Gateway service acts as the **central coordinator** that
orchestrates communication between multiple specialized microservices:

```
Client Request → Gateway Service → Memory Service
                      ↓              Router Service
                 Orchestration  →   Rewrite Service
                      ↓              Validation Service
                Client Response ← Aggregated Results
```

**Gateway's Role in Microservice Architecture:**

- **Single Entry Point**: All client requests flow through Gateway
- **Service Orchestration**: Coordinates calls to Memory, Router, Rewrite, Validation services
- **Context Management**: Retrieves and passes context data between services
- **Response Aggregation**: Combines results from multiple services into unified responses
- **Cross-Cutting Concerns**: Handles authentication, tracing, error handling for all services

### Feature Level - Internal Module Organization

Within the Gateway service, each feature module (like `ingress`, `auth`, `health`) implements the
3-layer pattern:

```
Feature Module Structure:
├── handler.rs      # HTTP Layer - Request/Response processing
├── service.rs      # Business Layer - Feature-specific logic
├── repository.rs   # Data Layer - External service communication
├── models.rs       # Data structures
└── error.rs        # Feature errors
```

## The Three Layers

### Handler Layer (Entry Point)

**Microservice Perspective**: Gateway's HTTP interface to external clients **Feature Perspective**:
Individual endpoint handlers within each feature

- **What it does**:
  - Receives HTTP requests from clients
  - Validates request format and parameters
  - Delegates business logic to Service layer
  - Converts Service results to HTTP responses
  - Handles HTTP status codes and error formatting
- **What it doesn't do**:
  - No business logic
  - No direct microservice calls
  - No external service communication

### Service Layer (Business Logic)

**Microservice Perspective**: Gateway's orchestration engine for microservice coordination **Feature
Perspective**: Feature-specific business rules and workflows

- **What it does**:
  - Implements business rules and workflows
  - Orchestrates multiple Repository calls (which map to different microservices)
  - Handles business error scenarios
  - Processes and transforms data
  - Coordinates complex operations across services
- **What it doesn't do**:
  - No HTTP request/response handling
  - No direct gRPC/external service calls
  - No data access implementation

### Repository Layer (Data Access)

**Microservice Perspective**: Gateway's communication interface to other microservices **Feature
Perspective**: External data access and service communication for each feature

- **What it does**:
  - gRPC client management for microservice communication
  - Database operations (via Supabase)
  - File system access
  - External API calls
  - Mock implementations for development
- **What it doesn't do**:
  - No business logic
  - No HTTP handling
  - No data processing beyond access

## Data Flow Examples

### Microservice Level Flow (Ingress Feature)

```
Client → Gateway Handler → Gateway Service → Repository Layer
                                ↓
                          Memory Service (get context)
                                ↓
                          Router Service (process request)
                                ↓
                          Memory Service (update context)
                                ↓
Gateway Response ← Gateway Service ← Repository Layer
```

### Feature Level Flow (Auth Feature)

```
Auth Request → Auth Handler → Auth Service → Auth Repository
                                ↓              ↓
                          Validate API Key → Supabase DB
                                ↓              ↓
Auth Response ← Auth Handler ← Auth Service ← Auth Repository
```

## Implementation Examples

### Microservice Coordination (Ingress Repository)

```rust
/// Repository coordinates multiple microservices for ingress processing
impl IngressRepository {
    /// Coordinates with Memory Service for context
    pub async fn get_context(&self, user_id: &str) -> Result<ContextData, IngressError> {
        // gRPC call to Memory Service
        self.memory_client.get_context(user_id).await
    }

    /// Coordinates with Router Service for AI processing
    pub async fn route_request(&self, prompt: &str, context: &ContextData) -> Result<RouterResponse, IngressError> {
        // gRPC call to Router Service
        self.router_client.process_request(prompt, context).await
    }
}
```

### Feature-Level Business Logic (Ingress Service)

```rust
/// Service orchestrates the complete ingress workflow
impl IngressService {
    pub async fn process_request(&self, request: IngressRequest, user_id: String) -> Result<IngressResponse, IngressError> {
        // Step 1: Get context from Memory Service (via repository)
        let context = self.repository.get_context(&user_id).await?;

        // Step 2: Apply business logic (feature-specific)
        let processed_prompt = self.process_prompt(&request.prompt, &request.metadata).await?;

        // Step 3: Route to Router Service (via repository)
        let response = self.repository.route_request(&processed_prompt, &context, &request.metadata).await?;

        // Step 4: Update context in Memory Service (via repository)
        self.repository.update_context(&user_id, &request.prompt, &response.ai_response).await?;

        Ok(response)
    }
}
```

## Benefits at Both Levels

### Microservice Level Benefits

- **Service Isolation**: Each microservice can be developed, deployed, and scaled independently
- **Clear Boundaries**: Gateway handles coordination, other services focus on their domain
- **Fault Tolerance**: Repository layer can implement circuit breakers and retries
- **Unified Interface**: Clients only need to know Gateway API, not individual services

### Feature Level Benefits

- **Code Organization**: Each feature is self-contained with clear structure
- **Independent Development**: Teams can work on different features simultaneously
- **Easy Testing**: Mock repositories for unit tests, real repositories for integration tests
- **Gradual Migration**: Replace mocks with real implementations incrementally

## Development Guidelines

### Microservice Integration

1. **Repository First**: Define microservice communication interfaces
2. **Service Orchestration**: Implement business workflows that coordinate multiple services
3. **Handler Simplicity**: Keep HTTP handlers thin, delegate to services

### Feature Development

1. **Start Small**: Begin with mock repositories for rapid prototyping
2. **Layer by Layer**: Build repository → service → handler
3. **Maintain Boundaries**: Never bypass layers or mix responsibilities

### When Adding New Microservices

- Add new client in Repository layer
- Update Service layer to orchestrate new service
- Handler layer remains unchanged

### When Adding New Features

- Create new feature module with 3-layer structure
- Reuse existing microservice clients in Repository layer
- Implement feature-specific business logic in Service layer

## File Structure - Dual Perspective

### Microservice Level Structure (Gateway Service as Coordinator)

```
gateway/src/
├── main.rs                     # Application entry point
├── lib.rs                      # Library exports
├── core/                       # Shared primitives across all features
│   ├── mod.rs
│   ├── error.rs               # Global error types
│   └── tracing.rs             # Distributed tracing setup
├── middleware/                 # Cross-cutting concerns for ALL microservice communication
│   ├── mod.rs
│   ├── auth.rs                # Unified authentication middleware
│   ├── tracing.rs             # Request tracing middleware
│   └── metrics.rs             # Metrics collection middleware
├── grpc/                      # Microservice Communication Layer (Shared Repository Components)
│   ├── mod.rs
│   ├── clients.rs             # gRPC client pool management
│   ├── memory.rs              # Memory Service client
│   ├── router.rs              # Router Service client
│   ├── rewrite.rs             # Rewrite Service client
│   └── validation.rs          # Validation Service client
└── features/                  # Feature modules (each implementing 3-layer internally)
    ├── ingress/               # AI request processing feature
    ├── auth/                  # Authentication feature
    └── health/                # Health check feature
```

**Microservice Level Responsibilities:**

- `grpc/` = **Repository Layer** for microservice communication
- `features/` = **Service Layer** for business orchestration
- `middleware/` = **Handler Layer** cross-cutting concerns
- `main.rs` = **Handler Layer** HTTP server setup

### Feature Level Structure (Each Feature Module)

```
features/ingress/              # Example: Ingress Feature (3-Layer Architecture)
├── mod.rs                     # Feature exports and wiring
├── handler.rs                 # Handler Layer - HTTP request/response processing
├── service.rs                 # Service Layer - Ingress business logic
├── repository.rs              # Repository Layer - Data access & microservice calls
├── models.rs                  # Data models and structs
├── types.rs                   # Type definitions and enums
├── constants.rs               # Feature constants
└── error.rs                   # Feature-specific error types

features/auth/                 # Example: Auth Feature (3-Layer Architecture)
├── mod.rs                     # Feature exports and wiring
├── handler.rs                 # Handler Layer - Auth endpoints
├── service.rs                 # Service Layer - Auth business logic
├── repository.rs              # Repository Layer - Supabase client & API key validation
├── models.rs                  # Auth data structures
├── types.rs                   # Auth type definitions
├── constants.rs               # Auth constants
└── error.rs                   # Auth-specific errors

features/health/               # Example: Health Feature (3-Layer Architecture)
├── mod.rs                     # Feature exports and wiring
├── handler.rs                 # Handler Layer - Health check endpoints
├── service.rs                 # Service Layer - Health check logic
└── models.rs                  # Health check data structures
```

**Feature Level Responsibilities:**

- `handler.rs` = **Handler Layer** for this specific feature
- `service.rs` = **Service Layer** for this specific feature
- `repository.rs` = **Repository Layer** for this specific feature

## Layer Mapping Between Perspectives

### Repository Layer Mapping

```
Microservice Level:     Feature Level:
grpc/memory.rs    →    features/ingress/repository.rs (uses Memory client)
grpc/router.rs    →    features/ingress/repository.rs (uses Router client)
grpc/rewrite.rs   →    features/ingress/repository.rs (uses Rewrite client)
                       features/auth/repository.rs (uses Supabase client)
```

### Service Layer Mapping

```
Microservice Level:     Feature Level:
features/ (orchestration) → features/ingress/service.rs (business logic)
                           features/auth/service.rs (auth logic)
                           features/health/service.rs (health logic)
```

### Handler Layer Mapping

```
Microservice Level:     Feature Level:
main.rs (HTTP server)  → features/ingress/handler.rs (ingress endpoints)
middleware/ (cross-cutting) → features/auth/handler.rs (auth endpoints)
                             features/health/handler.rs (health endpoints)
```

## Data Flow Through File Structure

### Microservice Coordination Flow

```
Client Request
    ↓
main.rs (HTTP Server)
    ↓
middleware/auth.rs (Authentication)
    ↓
features/ingress/handler.rs (Route to feature)
    ↓
features/ingress/service.rs (Business orchestration)
    ↓
features/ingress/repository.rs (Uses shared gRPC clients)
    ↓
grpc/memory.rs + grpc/router.rs (Microservice calls)
    ↓
External Services (Memory, Router, etc.)
```

### Feature-Specific Flow

```
HTTP Request for /api/v1/route
    ↓
features/ingress/handler.rs (HTTP processing)
    ↓
features/ingress/service.rs (Ingress business logic)
    ↓
features/ingress/repository.rs (Data access)
    ↓
grpc/memory.rs (Memory Service client)
grpc/router.rs (Router Service client)
```

## Current Implementation Status

### Implemented Files

```
gateway/src/
├── main.rs                    ✅ HTTP server setup
├── lib.rs                     ✅ Library exports
├── features/
│   ├── ingress/
│   │   ├── mod.rs            ✅ Module exports
│   │   ├── handler.rs        ✅ HTTP handler (selected file)
│   │   ├── service.rs        ✅ Business logic
│   │   ├── repository.rs     ✅ Mock data access
│   │   ├── models.rs         ✅ Data structures
│   │   └── error.rs          ✅ Error types
│   └── health/
│       ├── mod.rs            ✅ Module exports
│       └── handler.rs        ✅ Health endpoint
└── middleware/
    └── auth.rs               ✅ Auth middleware
```

### Planned Files

```
├── grpc/                     🔄 Microservice clients (future)
│   ├── memory.rs            ⏳ Memory Service client
│   ├── router.rs            ⏳ Router Service client
│   └── rewrite.rs           ⏳ Rewrite Service client
├── features/
│   └── auth/                🔄 Auth feature (in development)
│       ├── handler.rs       ⏳ Auth endpoints
│       ├── service.rs       ⏳ Auth business logic
│       └── repository.rs    ⏳ Supabase client
```

This dual-perspective file structure ensures clean separation at both the microservice coordination
level and individual feature implementation level.
