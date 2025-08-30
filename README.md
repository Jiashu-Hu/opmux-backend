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
