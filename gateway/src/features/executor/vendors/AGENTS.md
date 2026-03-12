# EXECUTOR VENDORS KNOWLEDGE BASE

## OVERVIEW

Vendor-specific LLM API integrations that implement the `LLMVendor` trait.

## WHERE TO LOOK

| Task           | Location                                          | Notes                           |
| -------------- | ------------------------------------------------- | ------------------------------- |
| Vendor trait   | `gateway/src/features/executor/vendors/traits.rs` | Contract for vendors            |
| OpenAI client  | `gateway/src/features/executor/vendors/openai.rs` | Chat completions + health check |
| Module exports | `gateway/src/features/executor/vendors/mod.rs`    | Vendor registry exports         |

## CONVENTIONS

- Implement `health_check` with a lightweight endpoint and no retries.
- Use shared `reqwest::Client` per vendor for pooling.

## ANTI-PATTERNS

- Do not leak vendor-specific errors past the `ExecutorError` boundary.
