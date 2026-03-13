# Opmux Backend

A Rust backend project for Opmux.

## Development

### Prerequisites

- Rust (latest stable version)
- Node.js (for Prettier formatting)

### Setup

1. Clone the repository
2. Install Rust dependencies:
   ```bash
   cargo build
   ```
3. Install Node.js dependencies for formatting:
   ```bash
   npm install
   ```

## How to Run Locally

Start the gateway service and hit the health endpoint:

```bash
cargo run -p gateway
# in another terminal
curl http://localhost:3000/health
```

Optional: copy `.env.example` to `.env` for local overrides. For more commands and contributor
rules, see AGENTS.md.

## Observability

The gateway includes production-oriented observability primitives:

- Correlation IDs (`X-Request-ID`, optional `X-Correlation-ID` echo)
- Health and readiness endpoints (`/health`, `/ready`)
- Prometheus metrics endpoint (`/metrics` when enabled)

Quick check:

```bash
export OPENAI_API_KEY=dummy-key
export OPENAI_BASE_URL=http://127.0.0.1:9/v1
cargo run -p gateway
```

Then in another terminal:

```bash
curl -i http://127.0.0.1:3000/health
curl -i http://127.0.0.1:3000/ready
curl -i http://127.0.0.1:3000/metrics
```

Additional guides:

- `docs/OBSERVABILITY.md`
- `docs/PROMETHEUS.md`
- `gateway/tests/OBSERVABILITY_TESTING.md`

## API and Operations Docs

- API reference: `docs/API_REFERENCE.md`
- Operations runbook: `docs/OPERATIONS_RUNBOOK.md`
- Configuration troubleshooting: `docs/CONFIGURATION_TROUBLESHOOTING.md`
- Performance/load testing: `gateway/tests/PERFORMANCE_LOAD_TESTING.md`

## Docker Deployment

Build and run with Docker Compose:

```bash
docker compose up --build
```

Service is exposed on `http://127.0.0.1:3000`.

### Code Formatting

This project uses both Rust's built-in formatter and Prettier for different file types:

- **Rust code**: Formatted with `rustfmt`

  ```bash
  cargo fmt
  ```

- **Other files** (Markdown, YAML, JSON, TOML): Formatted with Prettier

  ```bash
  npm run format
  ```

- **Check formatting** without making changes:
  ```bash
  cargo fmt -- --check
  npm run format:check
  ```

### Testing

Run tests with:

```bash
cargo test
```

### Linting

Run Clippy for Rust code analysis:

```bash
cargo clippy
```

### CI/CD

The project includes a comprehensive GitHub Actions workflow that:

- Tests on multiple Rust versions (stable, beta, nightly)
- Checks code formatting (Rust and other files)
- Runs linting and security audits
- Generates code coverage reports
- Builds release artifacts

## Project Structure

```
opmux-backend/
├── src/           # Rust source code
├── .github/       # GitHub Actions workflows
├── Cargo.toml     # Rust dependencies and metadata
├── package.json   # Node.js dependencies for tooling
└── README.md      # This file
```
