# Executive Summary

All changes in this project must satisfy three requirements: strictly follow the 3-layer
development scheme, practice test-driven development (TDD), and comply with the **Back-End
Development Specifications** in this document.

- The 3-layer scheme must govern design and implementation (Handler/Service/Repository, no layer
  skipping).
- Tests must be written before implementation (Red → Green → Refactor).
- All implementation and refactoring must comply with the **Back-End Development Specifications**.

# Architecture Guide

## Overview

This project follow a 3-layer pattern to keep responsibilities clear and testable.

```
handler.rs      # HTTP layer
service.rs      # Business layer
repository.rs   # Data/external access
```

## Layer Responsibilities

### Handler (HTTP)

- Parse and validate requests
- Call the service layer
- Map service results to HTTP responses
- No business logic
- No external calls

### Service (Business)

- Implement workflows and business rules
- Orchestrate multiple repository calls
- Handle business errors
- No HTTP handling
- No direct external calls

### Repository (Data/External)

- Talk to external services (gRPC, DB, APIs)
- Provide data access primitives to services
- No business logic
- No HTTP handling

## Example

**Ingress flow (simplified)**

```
Client → Handler → Service → Repository → External Service
```

```rust
// handler.rs
pub async fn handle(req: IngressRequest, svc: IngressService) -> Result<Response, IngressError> {
    let result = svc.process(req).await?;
    Ok(Response::from(result))
}

// service.rs
impl IngressService {
    pub async fn process(&self, req: IngressRequest) -> Result<IngressResult, IngressError> {
        let ctx = self.repo.get_context(&req.user_id).await?;
        let routed = self.repo.route_request(&req.prompt, &ctx).await?;
        Ok(routed)
    }
}
```

## Rules

- Do not bypass layers.
- Handlers stay thin.
- Repositories are the only place that touch external systems.

# Test-Driven Development Guide

## Overview

We use TDD to keep changes small, explicit, and verifiable. Write the failing test first, then
implement the minimal code to pass it, then refactor.

## Loop

1. **Red**: Add a test that captures the desired behavior and fails.
2. **Green**: Implement the simplest code to pass the test.
3. **Refactor**: Improve structure without changing behavior.

## What To Test

- Service layer: business logic and orchestration.
- Repository layer: integration boundaries (mocked in unit tests, real in integration tests).
- Handler layer: request/response mapping and error translation.

## Test Types

- **Unit tests**: Service and pure logic, use mocks for repositories.
- **Integration tests**: Real repository implementations, external calls guarded by env.

## Practical Guidelines

- One behavior per test.
- Prefer explicit inputs/outputs over complex fixtures.
- Use descriptive test names that encode the behavior.
- Keep tests deterministic; avoid sleeps and time-based flakiness.

## Example (Service)

```rust
#[tokio::test]
async fn process_returns_error_when_context_missing() {
    let repo = FakeIngressRepository::with_missing_context();
    let svc = IngressService::new(repo);

    let err = svc.process(IngressRequest::for_user("u1")).await.unwrap_err();

    assert!(matches!(err, IngressError::ContextMissing));
}
```

# Back-End Development Specifications

## Module Development Guidelines

- Module Priority: Refer to MODULE_LIBRARY_REFERENCE.md to select existing modules and
  implementations

- Strictly Prohibit Reinventing the Wheel: Do not hand-write implementations of existing modules or
  crates

- Proactively Evaluate Module Suitability: If existing modules do not fit the current needs,
  proactively propose and explain your alternative solutions

- Style Specifications: Use Rust's standard idioms and patterns; prefer configuration via
  environment variables or config files over hard-coded values

- Structure Reuse: Refer to the module structure of the most similar existing endpoints or services

- Routing and Handlers: Use centralized router + handler patterns, refer to ROUTING_SYSTEM.md

- Split Files: For complex services, do not cram everything into one file; split files reasonably
  based on responsibilities

## Architecture Design Patterns

### Business Logic and Data Decoupling

- Strict Separation: Handler functions should only handle request/response logic

- Data Operations: All data operations must be handled by dedicated repositories or services

- No Direct DB Calls: Handlers should never make direct database queries

- Dependency Injection: Handlers receive dependencies through structs or traits, not by creating
  them themselves

## File Structure Organization

```
ServiceName/
├── mod.rs                      # Main module - Exports and wiring
├── handlers/                   # Request handlers (API endpoints)
│   ├── get_handler.rs
│   ├── post_handler.rs
│   └── error_handler.rs
├── services/                   # Business logic layer
│   ├── domain_service.rs
│   ├── repository.rs
│   └── api/                    # External API specifications (e.g., OpenAPI if applicable)
├── models/                     # Data models and structs
│   ├── request_models.rs
│   └── response_models.rs
├── repositories/               # Data access layer
│   ├── db_repository.rs
│   └── mock_repository.rs
├── types.rs                    # Type definitions and enums
├── constants.rs                # Constants and mock data
└── lib.rs                      # Crate-level exports (if applicable)
```

## Module Hierarchy

- Core Modules: src/core/ - Reusable primitives (e.g., error types, utils)

- Feature Modules: src/features/[FeatureName]/ - Business-specific logic

- Repository Modules: src/repositories/ - Data access implementations

- Handler Modules: src/handlers/ - Top-level API handlers

## Data Service Architecture

- Registry Pattern: Main service acts as unified entry point (e.g., via a ServiceRegistry struct)

- Specialized Services: Each domain has dedicated service (e.g., user_service, auth_service,
  data_service)

- API Ready: All services have corresponding interface definitions (e.g., traits for mocking)

- Mock Data Support: Services use mock implementations that can be easily replaced with real
  databases or APIs

## Layered Architecture for Complex Modules

When creating complex modules or refactoring existing ones, use 3-Layer Separation:

### File Structure Pattern

```
ServiceName/
├── handler.rs         # Handler Layer - Request/response processing
├── service.rs         # Service Layer - Business logic abstraction
├── repository.rs      # Repository Layer - Data access & mocks
├── models.rs          # Data models and structs
├── lib.rs             # Clean exports
└── README.md          # Architecture documentation
```

### Layer Responsibilities

- Handler Layer: Only handles HTTP requests, validation, and response formatting; manages async
  flows

- Service Layer: Handles all business logic, orchestrations, and error handling

- Repository Layer: Contains data access, queries, and transformations; includes mocks

### Implementation Rules

- Handler: Import only Service methods, never direct data access

- Service: Provide async methods with consistent traits/interfaces

- Repository: Structure data exactly as real sources would return (e.g., DB results)

- Types: Define complete Rust structs and enums for all data structures, with Serde support where
  needed

### Benefits

- Easy Integration: Replace mocks with real implementations without touching handlers

- Clean Testing: Test each layer independently (unit tests for services, integration for handlers)

- Reusable Logic: Services can be shared across handlers

- Type Safety: Full Rust type system coverage across all layers

### When to Apply

- Modules with complex business requirements

- Endpoints that will integrate with databases or external APIs later

- Modules with multiple data sources

- Reusable business logic across features

## Refactoring and Migration Principles

When refactoring or migrating existing code, follow these strict principles:

### Core Refactoring Rules

- Strictly Follow Existing Interfaces: Never change function signatures or trait definitions during
  refactoring

- Minimize Changes: Only move code, do not optimize or add new features

- Avoid Over-Engineering: Do not add unnecessary abstraction layers or generics

- Near-Zero Code Increment: Refactoring should not significantly increase code volume

### Refactoring Process

1. Identify Existing Code: Locate the exact code to be moved (usually 2-10 lines)

2. Create Minimal Structure: Create the simplest possible module to hold the code

3. Move Code As-Is: Transfer code without modification or optimization

4. Update References: Update imports and usages to reference new location

5. Clean Up: Remove original code only after verification

### Code Increment Control

- Target: Net code increase should be < 20 lines for simple refactoring

- Warning: If increment > 50 lines, review for over-engineering

- Red Flag: If increment > 100 lines, restart with minimal approach

### Common Over-Engineering Mistakes to Avoid

- Adding unnecessary trait implementations

- Creating wrapper functions that didn't exist before

- Adding extensive doc comments during refactoring

- Using unnecessary async or borrow optimizations not in original code

- Creating new abstraction layers
