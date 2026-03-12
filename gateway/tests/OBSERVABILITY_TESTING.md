# Observability Manual Testing Guide

This guide verifies correlation IDs, health/readiness, and Prometheus metrics.

## Prerequisites

- Run from repo root
- Use a local API key value to initialize executor

```bash
export OPENAI_API_KEY=dummy-key
export OPENAI_BASE_URL=http://127.0.0.1:9/v1
cargo run -p gateway
```

## 1) Correlation ID propagation

```bash
curl -i "http://127.0.0.1:3000/health" \
  -H "X-Correlation-ID: manual-corr-001"
```

Expected:
- `X-Request-ID` exists in response headers
- `X-Correlation-ID: manual-corr-001` echoed in response headers

## 2) Health endpoint response

```bash
curl -i "http://127.0.0.1:3000/health"
```

Expected:
- `HTTP/1.1 200 OK`
- JSON fields: `status`, `timestamp`, `version`, `uptime_seconds`

## 3) Readiness endpoint response

```bash
curl -i "http://127.0.0.1:3000/ready"
```

Expected with dummy upstream config:
- `HTTP/1.1 503 Service Unavailable`
- JSON fields: `status: not_ready`, dependency details under `dependencies`

## 4) Metrics endpoint accessibility

```bash
curl -i "http://127.0.0.1:3000/metrics"
```

Expected:
- `HTTP/1.1 200 OK`
- Prometheus payload includes `gateway_http_requests_total`

## 5) Protected ingress route behavior

```bash
curl -i -X POST "http://127.0.0.1:3000/api/v1/route" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-api-key-123" \
  -H "X-Correlation-ID: manual-corr-002" \
  -d '{"prompt":"hello","metadata":{}}'
```

Expected with dummy upstream config:
- `HTTP/1.1 500 Internal Server Error`
- `X-Correlation-ID: manual-corr-002` preserved in response
