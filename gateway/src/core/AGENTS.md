# CORE KNOWLEDGE BASE

## OVERVIEW

Cross-cutting primitives for config, errors, metrics, tracing, and correlation context.

## WHERE TO LOOK

| Task              | Location                          | Notes                             |
| ----------------- | --------------------------------- | --------------------------------- |
| Global config     | `gateway/src/core/config.rs`      | Env loading, defaults, validation |
| Correlation IDs   | `gateway/src/core/correlation.rs` | Request context storage           |
| Error aggregation | `gateway/src/core/error.rs`       | App-wide error modeling           |
| Metrics setup     | `gateway/src/core/metrics.rs`     | Metrics registry/config           |
| Tracing setup     | `gateway/src/core/tracing.rs`     | Subscriber + formatting           |

## CONVENTIONS

- Config lives in core; feature configs should use `get_config()` where possible.
- Correlation context is injected in middleware and consumed by services.

## ANTI-PATTERNS

- Do not add feature-specific config here; keep per-feature config in feature modules.
- Avoid logging secrets or full payloads in config validation.
