# HEALTH FEATURE KNOWLEDGE BASE

## OVERVIEW

Health and readiness endpoints with cached dependency checks.

## WHERE TO LOOK

| Task          | Location                                       | Notes                        |
| ------------- | ---------------------------------------------- | ---------------------------- |
| HTTP handlers | `gateway/src/features/health/handler.rs`       | /health and /ready endpoints |
| Health logic  | `gateway/src/features/health/service.rs`       | Caching + vendor checks      |
| Errors        | `gateway/src/features/health/error.rs`         | 503 mapping                  |
| Tests         | `gateway/src/features/health/service_tests.rs` | Cache + readiness behavior   |

## CONVENTIONS

- Cache only successful health checks; failures are not cached.
- Health check timeout and TTL are env-configured.

## ANTI-PATTERNS

- Do not cache failed health checks; it blocks recovery.
