# Observability Guide

This service provides correlation IDs, structured logs, health/readiness checks, and Prometheus metrics.

## Correlation IDs

- Incoming `X-Correlation-ID` is validated and echoed back when present.
- Service always generates `X-Request-ID`.
- Correlation context is injected by middleware and available to handlers/services.

## Logging

- Configure level with `LOG_LEVEL`.
- Enable structured JSON with `LOG_JSON=true`.
- `RUST_LOG` can be used to tune module-level filters.

## Endpoints

- `GET /health` - liveness/status payload
- `GET /ready` - readiness with dependency health
- `GET /metrics` - Prometheus metrics payload (when enabled)

## Local verification

Use the manual guide in `gateway/tests/OBSERVABILITY_TESTING.md`.
