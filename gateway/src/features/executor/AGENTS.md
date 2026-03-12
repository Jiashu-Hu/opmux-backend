# EXECUTOR FEATURE KNOWLEDGE BASE

## OVERVIEW

LLM execution with vendor registry, retry/backoff, fallback strategy, and parameter extraction.

## WHERE TO LOOK

| Task            | Location                                         | Notes                           |
| --------------- | ------------------------------------------------ | ------------------------------- |
| Service logic   | `gateway/src/features/executor/service.rs`       | Retry, fallback, extract params |
| Vendor registry | `gateway/src/features/executor/repository.rs`    | Vendor init + dispatch          |
| Vendor config   | `gateway/src/features/executor/config.rs`        | Env-driven config               |
| Error types     | `gateway/src/features/executor/error.rs`         | Error mapping + HTTP response   |
| Vendor impls    | `gateway/src/features/executor/vendors/`         | OpenAI integration              |
| Tests           | `gateway/src/features/executor/service_tests.rs` | Retry, fallback, health         |

## CONVENTIONS

- Repository does direct API calls only; retries/fallbacks live in service.
- Vendor implementations must conform to `LLMVendor` trait.
- Retry uses full-jitter backoff in service; retry timing is intentionally non-deterministic.
- Health checks run concurrently; panic/error ordering can vary, so avoid order-sensitive
  assertions.

## ANTI-PATTERNS

- Do not implement retry logic in vendor implementations.
- Do not add vendor-specific branching in service layer.
