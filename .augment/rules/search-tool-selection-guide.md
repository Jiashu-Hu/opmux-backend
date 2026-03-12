---
type: "manual"
---

# Search Tool Selection Guide

**Use Claude Context MCP for conceptual understanding** (80% of development): When you need to understand "how does X work", learn architectural patterns, find related implementations across multiple files, or explore business logic—MCP excels at providing high-context, ranked results from both specs and code. However, **NEVER use MCP for refactoring tasks requiring 100% coverage**: in our tests, MCP found only 46% (6/13) of function call sites, completely missing critical production code. Always use Grep (`grep -rn "function_name"`) or LSP "Find All References" for refactoring scenarios like changing function signatures, renaming variables, or verifying unused code before deletion—false negatives here cause compilation errors or runtime bugs.

**Optimal workflows**: For learning, start with MCP ("how does feature X work") then Read specific files. For refactoring, use Grep to find ALL occurrences first, verify completeness, then make changes with compiler errors as safety net. For debugging, use MCP to understand conceptual flow then Grep for exact error messages. Use Glob (`**/auth/**/*.rs`) when you need complete file lists by pattern.
