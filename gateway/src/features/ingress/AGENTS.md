# INGRESS FEATURE KNOWLEDGE BASE

## OVERVIEW

Ingress orchestrates request flow: context retrieval, routing optimization, LLM execution, and
context update.

## WHERE TO LOOK

| Task           | Location                                     | Notes                          |
| -------------- | -------------------------------------------- | ------------------------------ |
| HTTP handler   | `gateway/src/features/ingress/handler.rs`    | Request entrypoint             |
| Orchestration  | `gateway/src/features/ingress/service.rs`    | Pipeline steps                 |
| External calls | `gateway/src/features/ingress/repository.rs` | gRPC mocks and ExecutorService |
| Constants      | `gateway/src/features/ingress/constants.rs`  | Timeout constants              |

## CONVENTIONS

- Repository owns gRPC client interactions (currently mocked).
- Service calls ExecutorService for LLM execution.

## ANTI-PATTERNS

- Do not execute LLM calls from handlers.
- Do not bypass repository for external service calls.
