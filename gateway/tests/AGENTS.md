# GATEWAY TESTS KNOWLEDGE BASE

## OVERVIEW

`gateway/tests` holds integration tests that may call real external APIs and are intentionally
separate from feature-local unit tests.

## WHERE TO LOOK

| Task                                 | Location                                     | Notes                                      |
| ------------------------------------ | -------------------------------------------- | ------------------------------------------ |
| Integration test implementation      | `gateway/tests/executor_integration_test.rs` | Real API-call executor coverage            |
| Test run policy and env requirements | `gateway/tests/README.md`                    | Cost, credentials, and invocation guidance |

## CONVENTIONS

- Keep external-call tests in this directory; keep fast unit tests in `gateway/src/features/*`.
- Guard external tests behind env checks and clear skip messaging.
- Use low-cost models/limits and `--nocapture` when diagnosing external failures.

## ANTI-PATTERNS

- Do not add flaky time-sensitive assertions tied to external latency.
- Do not run credentialed integration tests as mandatory default CI checks.
- Do not commit secrets or key-like values in test fixtures/logging.
