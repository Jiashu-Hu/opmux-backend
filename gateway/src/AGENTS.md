# GATEWAY SRC KNOWLEDGE BASE

## OVERVIEW

`gateway/src` contains runtime service code only: startup wiring, core primitives, middleware, and
feature modules.

## STRUCTURE

```
gateway/src/
├── main.rs              # Process boot, router composition, middleware ordering
├── lib.rs               # AppState and top-level module exports
├── core/                # Config, errors, metrics, tracing, correlation context
├── middleware/          # HTTP middleware concerns only
└── features/            # Handler/service/repository feature modules
```

## WHERE TO LOOK

| Task                  | Location                           | Notes                                                    |
| --------------------- | ---------------------------------- | -------------------------------------------------------- |
| Startup behavior      | `gateway/src/main.rs`              | Fail-fast initialization and route mounting              |
| Shared app state      | `gateway/src/lib.rs`               | `AppState` lives at crate root to avoid feature coupling |
| Feature contracts     | `gateway/src/features/AGENTS.md`   | Cross-feature layer conventions                          |
| Core invariants       | `gateway/src/core/AGENTS.md`       | Cross-cutting config/error/observability constraints     |
| Middleware invariants | `gateway/src/middleware/AGENTS.md` | Request lifecycle constraints                            |

## CONVENTIONS

- Keep startup and router wiring in `main.rs`; keep reusable types and module exports in `lib.rs`.
- Keep feature logic in `features/*`; keep cross-cutting concerns in `core`/`middleware`.
- Child AGENTS files are delta-only and override this file for narrower paths.

## ANTI-PATTERNS

- Do not add planning/docs content under `gateway/src`.
- Do not move business logic into middleware or startup wiring.
- Do not couple features directly through feature-to-feature imports.
