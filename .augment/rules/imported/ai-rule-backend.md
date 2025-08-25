---
type: 'manual'
---

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

## Development Process

- Development Planning: Do not start development directly; first plan which modules to use and how
  to structure; if there is missing information, ask the user to supplement it first

- Code Review: After development is completed, review which areas do not comply with the
  specifications
