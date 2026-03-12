# FEATURES KNOWLEDGE BASE

## OVERVIEW

Feature modules follow a strict 3-layer split: handler (HTTP), service (business), repository
(external/data).

## STRUCTURE

```
features/
├── auth/
├── executor/
├── health/
└── ingress/
```

## WHERE TO LOOK

| Task           | Location                               | Notes                 |
| -------------- | -------------------------------------- | --------------------- |
| HTTP endpoints | `gateway/src/features/*/handler.rs`    | Axum handlers         |
| Business logic | `gateway/src/features/*/service.rs`    | Orchestration + rules |
| External calls | `gateway/src/features/*/repository.rs` | API/gRPC access       |

## CONVENTIONS

- Each feature module defines its own error enum and models where needed.
- Service layer owns retries, fallback, and orchestration logic.

## ANTI-PATTERNS

- Do not call repositories directly from handlers.
- Do not bypass service layer for shared logic.
