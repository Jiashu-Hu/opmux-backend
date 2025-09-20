# Repository Guidelines

## Project Structure & Module Organization

- `common/` — Shared Rust crate for utilities.
- `gateway/` — Main Axum HTTP service (entrypoint `src/main.rs`).
- `scripts/` — Security and git hook helpers (e.g., `check-security.sh`).
- `specs/`, `docs/`, `.augment/` — Design, docs, and development rules (read before major changes).
- `.github/workflows/` — CI for tests, fmt, clippy, prettier, audit.

## Build, Test, and Development Commands

- Build: `cargo build` (release: `cargo build --release`).
- Run gateway: `cargo run -p gateway` → serves on `0.0.0.0:3000`.
  - Example: `curl http://localhost:3000/health`
- Test: `cargo test` (verbose: `cargo test --verbose`).
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`.
- Format (Rust): `cargo fmt` (check: `cargo fmt -- --check`).
- Format (other files): `npm run format` (check: `npm run format:check`).
- Security checks: `./scripts/check-security.sh` (run before PRs).

## Coding Style & Naming Conventions

- Rustfmt enforced (`rustfmt.toml`, `edition=2021`, `max_width=90`).
- Prettier for Markdown/YAML/JSON/TOML.
- Naming: crates/modules/functions `snake_case`; types `CamelCase`; constants
  `SCREAMING_SNAKE_CASE`.
- Gateway features follow 3-layer pattern: `handler.rs` (HTTP), `service.rs` (business),
  `repository.rs` (data).

## Testing Guidelines

- Use Rust unit tests with `#[test]` inside `mod tests` next to code.
- Integration tests may live under `gateway/tests/` when needed.
- Keep tests deterministic and isolated; prefer feature-level service tests over handler-only tests.
- Run `cargo test` locally; CI must be green.

## Commit & Pull Request Guidelines

- Use Conventional Commits: `feat:`, `fix:`, `refactor:`, `docs:`, `style:`.
- PRs must include: clear description, linked issues, reproduction steps, and relevant
  screenshots/logs (e.g., curl output).
- Requirements: CI green (tests, fmt, clippy, prettier, audit) and `./scripts/check-security.sh`
  passes.

## Security & Configuration Tips

- Copy `.env.example` → `.env` for local dev; never commit `.env`.
- Do not enable `AUTH_DEVELOPMENT_MODE` in committed code or CI. Local-only usage is allowed but
  must not be pushed.
- Production requires `X-API-Key` header; dev mode bypass is for local testing only.
- Install hooks (optional): `./scripts/install-hooks.sh` to enforce local checks.
