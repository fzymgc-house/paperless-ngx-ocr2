# Implementation Plan: OCR CLI Tool

**Branch**: `001-ocr-cli` | **Date**: 2025-01-23 | **Spec**: [link]
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
OCR CLI tool that uploads PDF/image files to Mistral AI APIs for text extraction. Supports TOML configuration, 12-factor app principles, and provides both human-readable and JSON output formats.

## Technical Context
**Language/Version**: Rust 1.80 (stable)  
**Primary Dependencies**: clap, serde, anyhow, thiserror, tracing, reqwest, tokio, toml  
**Storage**: N/A (stateless CLI tool)  
**Testing**: cargo test with assert_cmd, predicates  
**Target Platform**: macOS, Linux (cross-platform CLI)  
**Project Type**: single (CLI application)  
**Performance Goals**: <5s for typical PDF/image processing  
**Constraints**: <50MB file size limit, network timeout handling, memory efficient streaming  
**Scale/Scope**: Single-user CLI tool, batch processing capability

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

## Project Structure

### Documentation (this feature)
```
specs/001-ocr-cli/
├── plan.md              # This file (/plan command output)
├── research.md          # Phase 0 output (/plan command)
├── data-model.md        # Phase 1 output (/plan command)
├── quickstart.md        # Phase 1 output (/plan command)
├── contracts/           # Phase 1 output (/plan command)
└── tasks.md             # Phase 2 output (/tasks command - NOT created by /plan)
```

### Source Code (repository root)
```
# Single project (CLI application)
src/
├── main.rs              # CLI entry point with clap
├── lib.rs               # Core library functionality
├── config.rs            # TOML configuration handling
├── api/                 # Mistral AI API client
│   ├── mod.rs
│   ├── file.rs          # File upload API
│   └── ocr.rs           # OCR processing API
├── cli/                 # CLI command definitions
│   ├── mod.rs
│   └── commands.rs
└── error.rs             # Error types and handling

tests/
├── integration/         # CLI integration tests
│   └── cli_tests.rs
└── unit/               # Unit tests for core modules
    ├── config_tests.rs
    └── api_tests.rs
```

**Structure Decision**: Single project (CLI application)

## Phase 0: Outline & Research
1. **Extract unknowns from Technical Context** above:
   - Mistral AI file upload API specifications and limits
   - Mistral AI OCR API response format and processing time
   - Supported file formats and size limits
   - Authentication method and rate limiting

2. **Generate and dispatch research agents**:
   ```
   For each unknown in Technical Context:
     Task: "Research Mistral AI file upload API specifications and limits"
     Task: "Research Mistral AI OCR API response format and processing time"
     Task: "Research supported file formats and size limits for Mistral AI"
     Task: "Research Mistral AI authentication and rate limiting"
   ```

3. **Consolidate findings** in `research.md` using format:
   - Decision: [what was chosen]
   - Rationale: [why chosen]
   - Alternatives considered: [what else evaluated]

**Output**: research.md with all NEEDS CLARIFICATION resolved

## Phase 1: Design & Contracts
*Prerequisites: research.md complete*

1. **Extract entities from feature spec** → `data-model.md`:
   - Configuration entity (TOML structure)
   - File upload entity (metadata, validation)
   - OCR result entity (text content, metadata)
   - API credentials entity (key, endpoint, validation)

2. **Generate API contracts** from functional requirements:
   - File upload endpoint contract
   - OCR processing endpoint contract
   - Error response schemas
   - Output format schemas (human-readable, JSON)

3. **Generate contract tests** from contracts:
   - API response validation tests
   - Error handling tests
   - Output format tests

4. **Extract test scenarios** from user stories:
   - Happy path: valid file upload and OCR
   - Error paths: invalid file, API errors, network issues
   - Edge cases: large files, unsupported formats

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

**Ordering Strategy**:
- TDD order: Tests before implementation 
- Dependency order: Models before services before CLI
- Mark [P] for parallel execution (independent files)

**Estimated Output**: 25-30 numbered, ordered tasks in tasks.md

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
| None | N/A | N/A |

## Progress Tracking
*This checklist is updated during execution flow*

**Phase Status**:
- [ ] Phase 0: Research complete (/plan command)
- [ ] Phase 1: Design complete (/plan command)
- [ ] Phase 2: Task planning complete (/plan command - describe approach only)
- [ ] Phase 3: Tasks generated (/tasks command)
- [ ] Phase 4: Implementation complete
- [ ] Phase 5: Validation passed

**Gate Status**:
- [ ] Initial Constitution Check: PASS
- [ ] Post-Design Constitution Check: PASS
- [ ] All NEEDS CLARIFICATION resolved
- [ ] Complexity deviations documented

---
*Based on Constitution v1.0.0 - See `.specify/memory/constitution.md`*
