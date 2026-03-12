<!-- OPENSPEC:START -->

# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:

- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security
  work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:

- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->

# PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-12 **Commit:** 8b453e0 **Branch:** main

## OVERVIEW

Rust workspace centered on the `gateway` Axum service. Runtime code follows strict
handler/service/repository boundaries, while planning/process knowledge is split across `openspec`,
`specs`, `docs`, and `.augment/rules`.

## STRUCTURE

```
./
├── common/                  # Shared crate boundary (small today)
├── gateway/                 # Axum service crate
│   ├── src/                 # Runtime code (core, middleware, features)
│   └── tests/               # Integration tests (real external calls)
├── specs/                   # Legacy design and requirement documents
├── openspec/                # Spec workflow and proposal process rules
├── docs/                    # Engineering/changelog documentation
├── scripts/                 # CI and developer safety scripts
└── .augment/rules/          # Architecture and coding rules
```

## WHERE TO LOOK

| Task                       | Location                                 | Notes                                                 |
| -------------------------- | ---------------------------------------- | ----------------------------------------------------- |
| Service startup and routes | `gateway/src/main.rs`                    | Router composition, middleware order, server boot     |
| Cross-cutting config       | `gateway/src/core/config.rs`             | Env loading, defaults, validation                     |
| Feature layering pattern   | `gateway/src/features/*`                 | `handler.rs`, `service.rs`, `repository.rs`           |
| LLM vendor integration     | `gateway/src/features/executor/vendors/` | Vendor trait + OpenAI implementation                  |
| Health/readiness behavior  | `gateway/src/features/health/`           | Success-only cache and dependency checks              |
| Integration test rules     | `gateway/tests/AGENTS.md`                | Local test policy, env requirements, cost notes       |
| Spec/proposal process      | `openspec/AGENTS.md`                     | Authoritative rules for proposal/spec change workflow |
| Architecture guardrails    | `.augment/rules/`                        | Layering, error design, migration constraints         |

## CODE MAP

| Symbol               | Type   | Location                                          | Refs | Role                           |
| -------------------- | ------ | ------------------------------------------------- | ---- | ------------------------------ |
| `ExecutorService`    | struct | `gateway/src/features/executor/service.rs`        | high | Retry/fallback orchestration   |
| `ExecutorRepository` | struct | `gateway/src/features/executor/repository.rs`     | high | Vendor registry + direct calls |
| `OpenAIVendor`       | struct | `gateway/src/features/executor/vendors/openai.rs` | high | OpenAI API client              |
| `IngressService`     | struct | `gateway/src/features/ingress/service.rs`         | high | Request orchestration          |
| `Config`             | struct | `gateway/src/core/config.rs`                      | high | Global config + env loading    |

## CONVENTIONS

- Gateway feature logic stays in service/repository; handlers remain HTTP translation only.
- Feature config should route through core config when possible.
- Error types are modeled per feature and aggregated at app boundary.
- Use child `AGENTS.md` files as scope-local deltas; avoid copying root sections verbatim.

## ANTI-PATTERNS (THIS PROJECT)

- Do not bypass handler/service/repository boundaries or call external systems from handlers.
- Do not enable `AUTH_DEVELOPMENT_MODE` in committed code or CI.
- Do not implement retry logic in vendor clients; keep retry/fallback in service layer.
- Do not start implementation for spec/proposal requests before applying `openspec` workflow.

## UNIQUE STYLES

- Executor uses a vendor plugin pattern (`vendors/traits.rs`) with centralized retry/fallback.
- Health checks cache only successful dependency states; failures are intentionally uncached.
- Middleware ordering is explicit in startup wiring and should be treated as behavior-critical.

## COMMANDS

```bash
cargo build
cargo run -p gateway
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt
npm run format
./scripts/check-security.sh
```

## NOTES

- Integration tests under `gateway/tests` are external-call tests and skip when `OPENAI_API_KEY` is
  unset.
- `README.md` still shows an older top-level `src/` shape; prefer this AGENTS structure map for
  navigation.
