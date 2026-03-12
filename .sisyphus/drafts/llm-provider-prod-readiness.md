# Draft: LLM Provider Production Readiness

## Requirements (confirmed)

- Fix production readiness issues for LLM provider integration in opmux-backend.
- Align ingress payload schema with executor (messages vs prompt).
- Apply OpenAI request timeouts from config.
- Improve error mapping for HTTP statuses (401/403/429/4xx/5xx).
- Add retry policy with jitter and respect rate-limit headers.
- Decide how to handle stream parameter (implement or reject).
- Consider Anthropic config wiring decision.
- Include risks, dependencies, and a phased plan.
- Wait for user confirmation before any code.

## Technical Decisions

- (pending) Stream parameter handling: implement vs reject.
- (pending) Anthropic config wiring approach.
- (pending) Retry policy specifics (max attempts, backoff algorithm, rate-limit header precedence).

## Research Findings

- (pending)

## Open Questions

- Desired behavior for stream parameter? (implement streaming vs explicit rejection)
- Anthropic config wiring: enable now with flags or defer?
- Test strategy: TDD vs tests-after vs manual-only?
- Any specific SLOs (timeouts, retry limits) to target?

## Scope Boundaries

- INCLUDE: LLM provider integration readiness, ingress-executor schema alignment, timeouts, error
  mapping, retry with jitter, stream handling, Anthropic config decision.
- EXCLUDE: Unrelated refactors or feature work outside LLM provider integration.
