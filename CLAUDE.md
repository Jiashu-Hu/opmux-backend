# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this
repository.

## Development Commands

### Building

- `cargo build` - Build the project in debug mode
- `cargo build --release` - Build optimized release version

### Testing

- `cargo test` - Run all tests
- `cargo test --verbose` - Run tests with verbose output

### Code Quality

- `cargo fmt` - Format Rust code (uses rustfmt with max_width = 90)
- `cargo fmt -- --check` - Check if code is properly formatted
- `cargo clippy` - Run linting and code analysis
- `cargo clippy --all-targets --all-features -- -D warnings` - Strict linting (CI configuration)
- `cargo audit` - Security audit for dependencies

### Prettier (Non-Rust files)

- `npm run format` - Format markdown, YAML, JSON, TOML files
- `npm run format:check` - Check formatting without changes
- `npm run format:md` - Format only markdown files
- `npm run format:yaml` - Format only YAML files
- `npm run format:json` - Format only JSON files
- `npm run format:toml` - Format only TOML files

## Architecture

This is a **Rust workspace** implementing an **AI API Router** system with a **Gateway Mediation
Pattern**.

### Workspace Structure

- **`common/`** - Shared utilities and common code (currently minimal)
- **`gateway/`** - AI API Router Gateway microservice (main application)

### System Architecture Design

The Gateway service implements a **layered architecture with strict separation of concerns**,
serving as the unified entry point for the AI API Router system. Key architectural patterns:

#### Gateway Mediation Pattern

- Gateway acts as the single point of coordination for all microservices
- All inter-service communication flows through Gateway
- Context data retrieved from Memory Service and passed to Router Service
- Unified error handling and retry logic across all service interactions
- Complete request tracing and monitoring through centralized flow

#### 3-Layer Architecture Pattern

Each feature module follows a strict 3-layer architecture:

- **Handler Layer** (`handler.rs`) - HTTP request/response processing
- **Service Layer** (`service.rs`) - Business logic and orchestration
- **Repository Layer** (`repository.rs`) - Data access and external service integration

### Gateway Service (`gateway/`) Structure

```
gateway/src/
├── core/                   # Reusable primitives
│   ├── error.rs           # Error handling
│   └── tracing.rs         # Distributed tracing
├── features/              # Business-specific modules (3-layer architecture)
│   ├── auth/              # Multi-method authentication (API keys, JWT, service tokens)
│   ├── ingress/           # Request ingress and orchestration
│   └── health/            # Health checks
├── middleware/            # HTTP middleware
│   ├── auth.rs           # Unified authentication middleware
│   ├── tracing.rs        # Request tracing
│   └── metrics.rs        # Metrics collection
└── grpc/                 # gRPC client abstractions for service communication
    ├── router.rs         # Router service client
    ├── memory.rs         # Memory service client
    ├── rewrite.rs        # Rewrite service client
    └── validation.rs     # Validation service client
```

### Authentication Architecture

Implements **unified multi-method authentication** supporting:

- **API Key Authentication** - Long-lived keys for B2B API clients (primary)
- **JWT Authentication** - Supabase integration for dashboard users (future)
- **Service Authentication** - Internal microservice communication (future)

Uses **moka cache** for high-performance key validation with database fallback.

### Request Processing Flow

1. **Unified Authentication Middleware** - Detect and validate authentication method
2. **Authorization Check** - Verify permissions and rate limits
3. **Service Coordination** - Retrieve context from Memory Service
4. **Conditional Routing** - Rewrite Service → Router Service (if rewrite=true) or direct to Router
5. **Response Aggregation** - Merge responses with metadata (cache hits, cost savings)
6. **Context Update** - Update Memory Service with new conversation context

### Dependencies

Uses modern Rust ecosystem with workspace-shared dependencies:

- **axum** - HTTP server framework
- **tokio** - Async runtime
- **tonic** - gRPC client for microservice communication
- **serde** - Serialization
- **tracing** - Structured logging and distributed tracing
- **jsonwebtoken** - JWT validation for Supabase
- **moka** - High-performance caching
- **anyhow/thiserror** - Error handling

## CI/CD

The project has comprehensive GitHub Actions workflows that run on `main` and `develop` branches:

- Tests on multiple Rust versions (stable, beta, nightly)
- Formatting checks (both Rust and Prettier)
- Linting with Clippy
- Security audits
- Release builds with artifact uploads

## Development Workflow and Requirements

### Requirements and Design Documentation

- **`specs/`** - Complete system specifications and design documents
  - `specs/gateway_service/` - Gateway service requirements, design, and tasks
  - `specs/unified_auth_system/` - Authentication system specifications
  - `specs/tracing_implementation_plan.md` - Observability implementation plan
- **`.augment/rules/`** - Development standards and coding guidelines (MUST READ BEFORE ANY
  DEVELOPMENT)
  - `ai-rule-backend.md` - Backend development specifications and architecture patterns
  - `ai-task-plan.md` - Development process (design phase → incremental development)
  - `ai-error-design.md` - Layered error handling standards (business operation focused)
  - `rust-documentation-guide.md` - Documentation standards and style guide
  - `ai-rule.md` - Spec workflow for new feature development

### MANDATORY Development Process (Strictly Follow These Steps)

#### Step 1: Determine Development Approach

**Strictly follow guidance from `.augment/rules/ai-rule.md`**

- **Use Full Spec Process For:** New feature development, complex architecture design, multi-module
  integration, database/UI design
- **Skip Spec Process For:** Simple fixes, document updates, configuration modifications, code
  refactoring
- **Commands Available:** `/spec` (force spec), `/no_spec` (skip spec), `/help`

#### Step 2A: Full Spec Workflow (For New Features)

**Strictly follow the spec workflow from `.augment/rules/ai-rule.md`**

1. **Requirements Document Design**
   - Use EARS syntax for requirement descriptions
   - Create `specs/[feature_name]/requirements.md`
   - Get user confirmation before proceeding
2. **Technical Solution Design**
   - Design complete technical architecture, technology stack, database/interface design
   - Use mermaid diagrams when necessary
   - Create `specs/[feature_name]/design.md`
   - Get user confirmation before proceeding
3. **Task Breakdown**
   - Create detailed implementation tasks
   - Create `specs/[feature_name]/tasks.md`
   - Get user confirmation before proceeding

#### Step 2B: Direct Implementation (For Simple Changes)

**Strictly follow guidelines from `.augment/rules/ai-task-plan.md`**

- Apply incremental development approach with business-value-first order

#### Step 3: Code Implementation Phase

**Strictly follow ALL these requirements:**

- **Architecture:** Apply patterns from `.augment/rules/ai-rule-backend.md`
  - Use 3-layer architecture (Handler/Service/Repository) for complex modules
  - Strict separation of business logic and data operations
  - No direct database calls from handlers
  - Use dependency injection patterns
- **Error Handling:** Follow standards from `.augment/rules/ai-error-design.md`
  - Model errors by business operation (not technical cause)
  - Use two-layer structure: Feature-level errors + Top-level AppError
  - Business operation focused error variants
- **Documentation:** Follow guidelines from `.augment/rules/rust-documentation-guide.md`
  - Simple, brief, informative, consistent style
  - Document business context and flow for complex functions
- **Module Development:** Follow specifications from `.augment/rules/ai-rule-backend.md`
  - Prioritize existing modules/crates (no reinventing wheel)
  - Use centralized router + handler patterns
  - Split files by responsibility for complex services

#### Step 4: Refactoring (When Needed)

**Strictly follow refactoring principles from `.augment/rules/ai-rule-backend.md`**

- Never change function signatures during refactoring
- Minimize changes - only move code, don't optimize
- Target < 20 lines net increase for simple refactoring
- Avoid over-engineering and unnecessary abstractions

### Development Standards

- **3-Layer Architecture** - Handler/Service/Repository separation for all complex modules
- **Error Handling** - Business operation focused errors (not technical cause focused)
- **Module Reuse** - Use existing modules/crates, avoid reinventing the wheel
- **Code Structure** - Split files by responsibility, use centralized routing patterns
- **Documentation** - Simple, brief, informative, consistent (see rust-documentation-guide.md)

### Refactoring Principles

- Strictly follow existing interfaces during refactoring
- Minimize changes - only move code, don't optimize or add features
- Target < 20 lines net increase for simple refactoring
- Avoid over-engineering and unnecessary abstraction layers

## Development Notes

- Code formatting is enforced in CI - always run `cargo fmt` and `npm run format` before committing
- The project uses strict linting settings that treat warnings as errors in CI
- Both Rust and non-Rust files have formatting requirements
- Always refer to `.augment/rules/` for development standards before implementing new features
- Check `specs/` directory for complete requirements and design documentation
