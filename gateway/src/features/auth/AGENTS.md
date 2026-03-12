# AUTH FEATURE KNOWLEDGE BASE

## OVERVIEW

Authentication feature implementing API key validation and auth error modeling.

## WHERE TO LOOK

| Task                | Location                                  | Notes                   |
| ------------------- | ----------------------------------------- | ----------------------- |
| Auth handlers       | `gateway/src/features/auth/handler.rs`    | HTTP auth endpoints     |
| Auth service        | `gateway/src/features/auth/service.rs`    | Auth orchestration      |
| Auth repository     | `gateway/src/features/auth/repository.rs` | Data access and mocks   |
| Errors              | `gateway/src/features/auth/error.rs`      | Auth error mapping      |
| Config (deprecated) | `gateway/src/features/auth/config.rs`     | Use core config instead |

## CONVENTIONS

- Use core config (`gateway::core::config::get_config().auth`) for settings.
- Keep auth logic in service; repository stays data/mocks.

## ANTI-PATTERNS

- Do not add new feature config here; prefer core config.
