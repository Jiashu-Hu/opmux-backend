# GATEWAY KNOWLEDGE BASE

## OVERVIEW

Axum HTTP service entrypoint and wiring for middleware, features, and shared core utilities.

## STRUCTURE

```
gateway/
├── src/
│   ├── main.rs           # Router and middleware ordering
│   ├── core/             # Config, error, metrics, tracing, correlation
│   ├── middleware/       # Auth, correlation, metrics
│   └── features/         # Feature modules (handler/service/repository)
└── tests/                # Integration tests
```

## WHERE TO LOOK

| Task                | Location                         | Notes                               |
| ------------------- | -------------------------------- | ----------------------------------- |
| Startup wiring      | `gateway/src/main.rs`            | Router setup and middleware order   |
| Config + validation | `gateway/src/core/config.rs`     | Global env loading                  |
| Feature modules     | `gateway/src/features/*`         | Per-feature handlers/services/repos |
| Auth middleware     | `gateway/src/middleware/auth.rs` | API key enforcement                 |
| Integration tests   | `gateway/tests/AGENTS.md`        | Scope-local integration test policy |

## CONVENTIONS

- Repo-wide rules live in `AGENTS.md`; this file contains gateway-only deltas.
- Child files under `gateway/src/**/AGENTS.md` override this file for narrower scope.
- Middleware order is explicit and documented in `gateway/src/main.rs`.
- Feature modules follow `handler.rs`, `service.rs`, `repository.rs` layout.
- Startup is fail-fast: if `ExecutorService::from_config` has zero usable vendors, process exits.
- Integration tests under `gateway/tests` are external-call tests and should stay out of default CI.

## ANTI-PATTERNS

- Do not reorder middleware in `main.rs` without validating auth/correlation behavior.
- Do not assume `/metrics` exists when metrics are disabled in configuration.

## NOTES

- gRPC clients are currently mocked in ingress repository.
- Metrics endpoint mounting is runtime-configured; `/metrics` may be absent when disabled.
