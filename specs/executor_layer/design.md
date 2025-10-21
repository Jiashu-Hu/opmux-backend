# Executor Layer - Technical Design

## Overview

The Executor Layer is an internal module within Gateway Service responsible for executing actual LLM
API calls based on routing decisions from RouterService. It is **not a separate microservice** but a
well-encapsulated module within the Gateway codebase.

## Responsibilities

### What Executor Layer Does ✅

- Execute LLM API calls based on RoutePlan (vendor_id, model_id)
- Extract execution parameters from original_payload (temperature, max_tokens, etc.)
- Integrate multiple LLM vendor SDKs (OpenAI, Anthropic, Cohere, etc.)
- Handle HTTP calls, retries, timeouts
- Calculate actual token usage and costs
- Support streaming responses (future: SSE/WebSocket)

### What Executor Layer Does NOT Do ❌

- Routing decisions (RouterService's responsibility)
- Prompt rewriting (RewriteService's responsibility)
- Context management (MemoryService's responsibility)
- Authentication (Gateway middleware's responsibility)

## Architecture

### Module Structure

```
gateway/src/executor/
├── mod.rs              # Module exports and re-exports
├── config.rs           # Configuration data structures (no business logic)
├── error.rs            # Executor-specific error types
├── models.rs           # Data models (ExecutionParams, ExecutionResult, etc.)
├── service.rs          # ExecutorService orchestration layer
└── vendors/            # Vendor-specific implementations
    ├── mod.rs          # Vendor module exports
    ├── traits.rs       # LLMVendor trait definition
    ├── openai.rs       # OpenAI Chat Completions API
    └── anthropic.rs    # Anthropic Messages API (future)
```

### Component Responsibilities

#### 1. Configuration Layer (`config.rs`)

**Responsibility**: Store and load configuration data ONLY

**Structures**:

- `ModelPricing`: Pricing per 1000 tokens (prompt + completion)
- `OpenAIConfig`: OpenAI vendor configuration
- `ExecutorConfig`: Top-level executor configuration

**Methods**:

- `from_env()`: Load configuration from environment variables
- `validate()`: Validate configuration completeness (may log for convenience)
- `Default`: Provide default values

**Design Principle**: Configuration layer does NOT contain business logic. No model support checks,
no cost calculations. Only data storage, loading, and validation.

#### 2. Vendor Trait (`vendors/traits.rs`)

**LLMVendor Trait**: Unified interface for all LLM vendors

```rust
#[async_trait]
pub trait LLMVendor: Send + Sync {
    /// Execute LLM API call
    async fn execute(&self, model: &str, params: ExecutionParams) -> Result<ExecutionResult, ExecutorError>;

    /// Return vendor identifier
    fn vendor_id(&self) -> &str;

    /// Check if vendor supports model (business logic)
    fn supports_model(&self, model: &str) -> bool;

    /// Calculate cost based on token usage (business logic)
    fn calculate_cost(&self, prompt_tokens: i64, completion_tokens: i64, model: &str) -> f64;
}
```

#### 3. Vendor Implementations (`vendors/*.rs`)

**Responsibility**: Business logic and API integration

**OpenAIVendor** (`vendors/openai.rs`):

- Integrates with OpenAI Chat Completions API
- Supported models: gpt-4, gpt-4-turbo, gpt-3.5-turbo
- Business logic:
  - `supports_model()`: Check if model is in config.supported_models
  - `calculate_cost()`: Calculate cost using config.pricing
- API integration:
  - Serialize request to OpenAI format
  - Handle authentication (Bearer token)
  - Parse response and extract tokens/cost
  - Error handling (rate limits, auth failures, etc.)

**Future Vendors**:

- `AnthropicVendor`: Claude models
- `CohereVendor`: Command models

#### 4. Service Layer (`service.rs`)

**ExecutorService**: Orchestrates LLM execution workflow

**Responsibilities**:

- Vendor registry and selection (by vendor_id)
- Parameter extraction from original_payload
- Error handling and retry logic
- Streaming response support (future)

**Status**: Placeholder implementation, not yet complete

#### 5. Error Handling (`error.rs`)

**ExecutorError**: Business operation focused errors

- `UnsupportedVendor(String)`: Vendor not configured
- `UnsupportedModel(String, String)`: Model not supported by vendor
- `InvalidPayload(String)`: Request format invalid
- `ApiCallFailed(String)`: LLM API call failed
- `RateLimitExceeded(String)`: Vendor rate limit hit
- `AuthenticationFailed(String)`: Invalid API key
- `TimeoutError(u64)`: Request timeout
- `NetworkError(String)`: Network connectivity issues

**HTTP Status Mapping**:

- 400: UnsupportedVendor, UnsupportedModel, InvalidPayload
- 401: AuthenticationFailed
- 429: RateLimitExceeded
- 500: ApiCallFailed, NetworkError, TimeoutError

## Configuration

### Environment Variables

**OpenAI Configuration**:

- `OPENAI_API_KEY`: API key (required)
- `OPENAI_BASE_URL`: Custom endpoint (optional, default: https://api.openai.com/v1)
- `OPENAI_TIMEOUT_MS`: Request timeout (optional, default: 30000ms)

**Executor Configuration**:

- `EXECUTOR_TIMEOUT_MS`: Global timeout (optional, default: 30000ms)
- `EXECUTOR_MAX_RETRIES`: Max retry attempts (optional, default: 3)

**Future Vendors**:

- `ANTHROPIC_API_KEY`: Anthropic API key
- `COHERE_API_KEY`: Cohere API key

### Pricing Configuration

Pricing is configured in code (default values) but can be overridden in the future:

**OpenAI Pricing** (as of 2024):

- GPT-4: $0.03/1K prompt tokens, $0.06/1K completion tokens
- GPT-4 Turbo: $0.01/1K prompt tokens, $0.03/1K completion tokens
- GPT-3.5 Turbo: $0.0005/1K prompt tokens, $0.0015/1K completion tokens

**Future Enhancement**: Load pricing from database or configuration file for dynamic updates.

## Integration Flow

### Request Flow

```
Client Request
    ↓
Ingress Handler (HTTP)
    ↓
Ingress Service (Business Logic)
    ↓
Repository.optimize_route() → RouterService (gRPC)
    ↓ (returns RoutePlan: { vendor_id: "openai", model_id: "gpt-4" })
Repository.execute_llm_call() → ExecutorService
    ↓
ExecutorService.execute(plan, payload)
    ↓
1. Select vendor by vendor_id → OpenAIVendor
2. Extract params from payload → ExecutionParams { messages, temperature, max_tokens, ... }
3. Validate model support → OpenAIVendor.supports_model("gpt-4")
    ↓
OpenAIVendor.execute("gpt-4", params)
    ↓
1. Build ChatCompletionRequest
2. POST to OpenAI API
3. Parse ChatCompletionResponse
4. Calculate cost using pricing config
    ↓
ExecutionResult { content, model_used, prompt_tokens, completion_tokens, total_cost, finish_reason }
    ↓
Return to Ingress Service
    ↓
Return to Client
```

### Error Handling Flow

```
OpenAI API Error
    ↓
Parse HTTP status code
    ↓
401 → ExecutorError::AuthenticationFailed
429 → ExecutorError::RateLimitExceeded
5xx → ExecutorError::ApiCallFailed
Timeout → ExecutorError::TimeoutError
    ↓
Convert to HTTP Response (IntoResponse)
    ↓
Return error JSON to client
```

## Design Principles

### 1. Configuration vs Business Logic Separation

**Configuration Layer** (`config.rs`):

- ✅ Load from environment variables
- ✅ Store configuration data (API keys, URLs, pricing, models)
- ✅ Validate configuration completeness
- ✅ Provide default values
- ❌ NO business logic (model checks, cost calculations)

**Vendor Layer** (`vendors/*.rs`):

- ✅ Business logic (model support checks, cost calculations)
- ✅ API integration and error handling
- ✅ Access configuration data directly (e.g., `self.config.pricing.get(model)`)

**Rationale**: Configuration should be pure data. Business logic belongs in the layer that uses the
data.

### 2. Vendor Abstraction

All vendors implement the same `LLMVendor` trait, allowing:

- Easy addition of new vendors
- Vendor selection at runtime
- Consistent error handling
- Unified testing interface

### 3. Error Design

Errors are modeled by **business operation** (what failed), not technical cause:

- ✅ `UnsupportedModel`: Tells us the operation (model selection) failed
- ❌ `HttpError`: Only tells us the technical cause, loses business context

## Current Implementation Status

### ✅ Completed

- Configuration system (ModelPricing, OpenAIConfig, ExecutorConfig)
- Environment variable loading and validation
- LLMVendor trait definition
- OpenAIVendor structure and implementation
- Error handling (ExecutorError with HTTP mapping)
- Cost calculation based on configurable pricing
- Configuration/business logic separation

### ⏸️ In Progress / Not Started

- ExecutorService orchestration layer (placeholder only)
- Real API testing (not tested with actual OpenAI API key)
- Integration with ingress service (deferred)
- Anthropic vendor implementation
- Streaming response support
- Retry logic and circuit breaker

## Future Enhancements

### Short-term (Post-MVP)

1. Complete ExecutorService implementation
2. Test with real OpenAI API
3. Integrate with ingress service
4. Add Anthropic vendor support
5. Implement retry logic

### Medium-term

1. Load pricing from database (dynamic updates)
2. Support streaming responses (SSE/WebSocket)
3. Add more vendors (Cohere, Google, etc.)
4. Implement circuit breaker pattern
5. Add request/response caching

### Long-term

1. Multi-region vendor support
2. Automatic failover between vendors
3. Cost optimization strategies
4. A/B testing support
5. Custom model fine-tuning integration

## Testing Strategy

### Unit Tests

- Configuration loading and validation
- Model support checks
- Cost calculation accuracy
- Error handling and conversion

### Integration Tests

- Real API calls (with test API keys)
- Error scenarios (rate limits, auth failures)
- Timeout handling
- Retry logic

### Mock Testing

- Mock vendor implementations for testing
- Simulate API failures
- Test error propagation
