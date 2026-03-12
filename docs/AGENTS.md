# DOCS KNOWLEDGE BASE

## OVERVIEW

`docs/` contains engineering-facing documentation and changelog-style references for implemented
work.

## WHERE TO LOOK

| Task                                      | Location                       | Notes                                  |
| ----------------------------------------- | ------------------------------ | -------------------------------------- |
| Architecture and coding practice guidance | `docs/engineering-practice.md` | Layering and implementation discipline |
| Executor implementation history           | `docs/CHANGELOG_EXECUTOR.md`   | Historical implementation log          |

## CONVENTIONS

- Keep documents scoped to engineering operations, implementation history, or contributor guidance.
- Prefer path-anchored, verifiable statements over aspirational or speculative prose.
- When docs reference commands, ensure they match current root `AGENTS.md` command set.

## ANTI-PATTERNS

- Do not place normative spec workflow rules here; keep those in `openspec/AGENTS.md`.
- Do not duplicate large sections from `AGENTS.md`; add only doc-specific guidance.
- Do not leave outdated path references unflagged after directory changes.
