# Prometheus Integration Guide

## Metrics endpoint

By default, metrics are exposed at `GET /metrics` when `METRICS_ENABLED=true`.

## Environment configuration

- `METRICS_ENABLED=true`
- `METRICS_PATH=/metrics`

## Prometheus scrape config example

```yaml
scrape_configs:
  - job_name: gateway
    metrics_path: /metrics
    static_configs:
      - targets: ['localhost:3000']
```

## Useful metric names

- `gateway_http_requests_total`
- `gateway_http_requests_pending`
- `gateway_http_requests_duration_seconds`

## Basic alert examples

- High 5xx ratio on `/ready`
- No scrape data for gateway target
- Elevated request duration on critical endpoints
