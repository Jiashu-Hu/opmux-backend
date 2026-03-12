# SCRIPTS KNOWLEDGE BASE

## OVERVIEW

`scripts/` contains developer and CI helper scripts for security checks, hook installation, and
integration test execution.

## WHERE TO LOOK

| Task                      | Location                           | Notes                                         |
| ------------------------- | ---------------------------------- | --------------------------------------------- |
| Security guardrail checks | `scripts/check-security.sh`        | Blocks unsafe auth development mode commits   |
| Integration test helper   | `scripts/run-integration-tests.sh` | Runs external OpenAI-backed integration tests |
| Hook setup                | `scripts/install-hooks.sh`         | Installs local pre-commit security hook       |

## CONVENTIONS

- Keep scripts idempotent and safe for local/CI repeated execution.
- Fail fast with non-zero exit codes on policy violations.
- Keep script messaging explicit about required env vars and failure causes.

## ANTI-PATTERNS

- Do not bypass security checks in committed scripts.
- Do not add destructive filesystem or git operations to shared helper scripts.
- Do not assume interactive shells in CI-executed scripts.
