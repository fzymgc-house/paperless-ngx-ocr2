
# Implementation Plan: OCR CLI Tool

**Branch**: `001-ocr-cli` | **Date**: 2025-01-23 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-ocr-cli/spec.md`

## Execution Flow (/plan command scope)
```
1. Load feature spec from Input path
   → If not found: ERROR "No feature spec at {path}"
2. Fill Technical Context (scan for NEEDS CLARIFICATION)
   → Detect Project Type from context (web=frontend+backend, mobile=app+api)
   → Set Structure Decision based on project type
3. Fill the Constitution Check section based on the content of the constitution document.
4. Evaluate Constitution Check section below
   → If violations exist: Document in Complexity Tracking
   → If no justification possible: ERROR "Simplify approach first"
   → Update Progress Tracking: Initial Constitution Check
5. Execute Phase 0 → research.md
   → If NEEDS CLARIFICATION remain: ERROR "Resolve unknowns"
6. Execute Phase 1 → contracts, data-model.md, quickstart.md, agent-specific template file (e.g., `CLAUDE.md` for Claude Code, `.github/copilot-instructions.md` for GitHub Copilot, `GEMINI.md` for Gemini CLI, `QWEN.md` for Qwen Code or `AGENTS.md` for opencode).
7. Re-evaluate Constitution Check section
   → If new violations: Refactor design, return to Phase 1
   → Update Progress Tracking: Post-Design Constitution Check
8. Plan Phase 2 → Describe task generation approach (DO NOT create tasks.md)
9. STOP - Ready for /tasks command
```

**IMPORTANT**: The /plan command STOPS at step 7. Phases 2-4 are executed by other commands:
- Phase 2: /tasks command creates tasks.md
- Phase 3-4: Implementation execution (manual or via tools)

## Summary
OCR CLI tool that uploads PDF/image files to Mistral AI APIs for text extraction. Supports TOML configuration, 12-factor app principles, and provides both human-readable and JSON output formats. Uses Rust for cross-platform CLI with official Mistral AI Files and OCR APIs. Includes multi-architecture container image for deployment.

## Updated Functional Requirements (Post-Clarification)
Based on clarification session, the following additional requirements have been added:

### Configuration Management
- **FR-021**: System MUST support --config flag for custom configuration file path
- **FR-022**: System MUST search for config files in: custom path (if specified), current directory, then ~/.config/paperless-ngx-ocr2/
- **Configuration Priority**: CLI args → Environment variables → .env file → TOML config → defaults

### Error Handling & Resilience
- **FR-018**: System MUST implement retry logic with exponential backoff for rate limit errors (3 attempts max)
- **FR-019**: System MUST detect and reject password-protected PDFs with validation error
- **FR-020**: System MUST return warning message for files with no extractable text (exit code 0)

### Shell Completion Support
- **FR-023**: System MUST support generating shell completion scripts via --completions flag
- **FR-024**: System MUST support completion generation for bash, zsh, fish, and PowerShell shells
- **FR-025**: System MUST make --file argument optional when generating completions
- **FR-026**: System MUST output completion scripts to stdout for easy redirection to files

### Dependencies
- **dotenv**: Added for .env file support in configuration loading
- **clap_complete**: Added for shell completion generation support

## Technical Context
**Language/Version**: Rust 1.80 (stable)  
**Primary Dependencies**: clap, serde, anyhow, thiserror, tracing, reqwest, tokio, toml, dotenv, clap_complete  
**Storage**: N/A (stateless CLI tool)  
**Testing**: cargo test with assert_cmd, predicates  
**Target Platform**: macOS, Linux (cross-platform CLI)
**Project Type**: single (CLI application)  
**Performance Goals**: <5s for typical PDF/image processing  
**Constraints**: <50MB file size limit, network timeout handling, memory efficient streaming, retry logic for rate limits, password-protected PDF detection  
**Scale/Scope**: Single-user CLI tool, batch processing capability
**Containerization**: Must have a multi-architecture container image

## Constitution Check
*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### CLI Contract Compliance
- ✅ Text I/O: stdin for config, stdout for results, stderr for errors
- ✅ JSON output via --json flag
- ✅ Proper exit codes (0 success, 2 validation, 3 I/O, 4 config, 5 internal)
- ✅ Human-readable output by default

### Safety & Reliability
- ✅ Error handling with anyhow/thiserror (no unwrap in CLI paths)
- ✅ Input validation for file paths and API credentials
- ✅ Network error handling with timeouts
- ✅ Retry logic with exponential backoff for rate limits
- ✅ Password-protected PDF detection and rejection

### Test-First Discipline
- ✅ CLI behavior tests with assert_cmd
- ✅ Unit tests for core parsing/validation
- ✅ Integration tests for API interactions

### Observability & Logging
- ✅ Structured logging with tracing
- ✅ --verbose flag for debug output
- ✅ No logs on stdout (reserved for program output)

### Versioning & Compatibility
- ✅ Semantic versioning for CLI interface
- ✅ Backward compatible configuration format
- ✅ Configuration file search order with fallback paths
- ✅ Shell completion generation for multiple shells (bash, zsh, fish, PowerShell)

### Containerization Compliance
- ✅ Cross-platform support (macOS, Linux) enables container deployment
- ✅ Stateless design suitable for container environments
- ✅ 12-factor app principles support containerization
- ✅ Multi-architecture image requirement noted for implementation

## Project Structure

### Documentation (this feature)
```
specs/[###-feature]/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure]
```

**Structure Decision**: Option 1 (Single CLI application) with containerization support

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - For each NEEDS CLARIFICATION → research task
   - For each dependency → best practices task
   - For each integration → patterns task

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research {unknown} for {feature context}"
   For each technology choice:
     Task: "Find best practices for {tech} in {domain}"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Entity name, fields, relationships
   - Validation rules from requirements
   - State transitions if applicable
   - Configuration loading priority and file search paths
   - Retry logic configuration and error handling patterns

2. **Generate API contracts** from functional requirements:
   - For each user action → endpoint
   - Use standard REST/GraphQL patterns
   - Output OpenAPI/GraphQL schema to `/contracts/`

3. **Generate contract tests** from contracts:
   - One test file per endpoint
   - Assert request/response schemas
   - Tests must fail (no implementation yet)

4. **Extract test scenarios** from user stories:
   - Each story → integration test scenario
   - Quickstart test = story validation steps
   - Error handling scenarios for rate limits, password-protected PDFs, empty text
   - Configuration loading priority test scenarios

5. **Update agent file incrementally** (O(1) operation):
   - Run `.specify/scripts/bash/update-agent-context.sh cursor`
     **IMPORTANT**: Execute it exactly as specified above. Do not add or remove any arguments.
   - If exists: Add only NEW tech from current plan
   - Preserve manual additions between markers
   - Update recent changes (keep last 3)
   - Keep under 150 lines for token efficiency
   - Output to repository root

**Output**: data-model.md, /contracts/*, failing tests, quickstart.md, agent-specific file

## Phase 2: Task Planning Approach
*This section describes what the /tasks command will do - DO NOT execute during /plan*

**Task Generation Strategy**:
- Load `.specify/templates/tasks-template.md` as base
- Generate tasks from Phase 1 design docs (contracts, data model, quickstart)
- Each contract → contract test task [P]
- Each entity → model creation task [P] 
- Each user story → integration test task
- Implementation tasks to make tests pass
- Configuration management tasks (dotenv, file search, priority order)
- Error handling tasks (retry logic, PDF validation, empty text handling)
- Shell completion generation tasks (--completions flag, multi-shell support)

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before UI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 35-40 numbered, ordered tasks in tasks.md (increased due to additional requirements including shell completion)

**IMPORTANT**: This phase is executed by the /tasks command, NOT by /plan

## Phase 3+: Future Implementation
*These phases are beyond the scope of the /plan command*

**Phase 3**: Task execution (/tasks command creates tasks.md)  
**Phase 4**: Implementation (execute tasks.md following constitutional principles)  
**Phase 5**: Validation (run tests, execute quickstart.md, performance validation)

## Complexity Tracking
*Fill ONLY if Constitution Check has violations that must be justified*

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |


## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [x] Phase 0: Research complete (/plan command) - Updated with containerization
- [x] Phase 1: Design complete (/plan command) - Updated with container deployment
- [x] Phase 2: Task planning complete (/plan command - describe approach only)
- [x] Plan Update: Updated with clarification session requirements (2025-01-23)
- [x] Plan Update: Updated with shell completion feature requirements (2025-01-23)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [x] Initial Constitution Check: PASS
- [x] Post-Design Constitution Check: PASS - Including containerization compliance
- [x] All NEEDS CLARIFICATION resolved
- [x] Complexity deviations documented

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*
