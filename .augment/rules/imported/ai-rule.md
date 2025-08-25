---
type: "always_apply"
---

Intelligent Judgment Criteria:

Use spec: New feature development, complex architecture design, multi-module integration, involving database/UI design
Skip spec: Simple fixes, document updates, configuration modifications, code refactoring

Workflow Command Control
Users can also make requests via commands.
Available Commands:

/spec - Force the use of the full spec process
/no_spec - Skip the spec process and execute directly
/help - Display command help

The following is the spec workflow:
<spec_workflow> 0. Please note! You must adhere to the following rules: after completing each stage, you need my confirmation before proceeding to the next stage;

1. If you determine that my input proposes a new requirement, you can independently carry out the work following standard software engineering practices below, inquiring with me only when necessary, and you can use the interactiveDialog tool to collect information.

2. Whenever I input a new requirement, to standardize the quality of the requirement and acceptance criteria, you must first clarify the problem and requirement, then proceed to the next stage.
3. Requirements Document and Acceptance Criteria Design: First complete the design of the requirements, describing them using the EARS simple requirements syntax method. If you determine that the requirement involves a front-end page, you need to predetermine the design style and color scheme in the requirements, must confirm the requirement details with me, and after final confirmation, finalize the requirements, then proceed to the next stage, saving it in specs/spec_name/requirements.md, with the reference format as follows:

```markdown
# Requirements Document

## Introduction

Requirement description

## Requirements

### Requirement 1 - Requirement Name

**User Story:** User story content

#### Acceptance Criteria

1. Use EARS-described clauses: While <optional precondition>, when <optional trigger>, the <system name> shall <system response>, for example: When selecting "mute", the laptop shall suppress all audio output.
2. ...
   ...
```

Technical Solution Design: After completing the requirements design, based on the current technical architecture and the previously confirmed requirements, design the technical solution for the requirements. Keep it concise but accurately describe the technical architecture (e.g., architecture, technology stack, technology selection, database/interface design, testing strategy, security). Use mermaid for diagrams if necessary, must confirm clearly with me, save it in specs/spec_name/design.md, then proceed to the next stage.
Task Breakdown: After completing the technical solution design, based on the requirements document and technical solution, refine the specific tasks to be done, must confirm clearly with me, save it in specs/spec_name/tasks.md, then proceed to the next stage, start formally executing the tasks, and update the task status in a timely manner. During execution, run as independently and autonomously as possible to ensure efficiency and quality.

Task reference format as follows:

```markdown
# Implementation Plan

- [ ] 1. Task Information
  - Specific things to do
  - ...
  - \_Requirement: Related requirement point number
```

</spec_workflow>
