# SPECS KNOWLEDGE BASE

## OVERVIEW

`specs/` stores legacy requirements/design/task documents used for planning context and historical
traceability.

## WHERE TO LOOK

| Task                          | Location                                | Notes                                         |
| ----------------------------- | --------------------------------------- | --------------------------------------------- |
| Gateway baseline requirements | `specs/gateway_service/requirements.md` | Product requirements and acceptance criteria  |
| Auth system design set        | `specs/unified_auth_system/`            | Requirements/design/tasks for auth initiative |
| Observability package         | `specs/enhanced_observability/`         | Design + task set for monitoring work         |
| Executor historical design    | `specs/executor_layer/design.md`        | Deep executor rationale and earlier decisions |

## CONVENTIONS

- Treat `specs/` as reference context, not implementation truth.
- Keep requirement, design, and tasks docs grouped by capability directory when possible.
- Use explicit status markers (draft, approved, archived) when adding new spec documents.
- For proposal/change workflow, follow `openspec/AGENTS.md` as authoritative process guidance.

## ANTI-PATTERNS

- Do not implement from `specs/` alone when `openspec` change workflow is required.
- Do not duplicate the same requirement text across multiple spec trees without linkage.
- Do not treat outdated paths in older docs as current architecture.
