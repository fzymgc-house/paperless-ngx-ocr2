# Tasks: OCR CLI Tool

**Input**: Design documents from `/specs/001-ocr-cli/`
**Prerequisites**: plan.md (required), research.md, data-model.md, contracts/

## Execution Flow (main)
```
1. Load plan.md from feature directory
   → If not found: ERROR "No implementation plan found"
   → Extract: tech stack, libraries, structure
2. Load optional design documents:
   → data-model.md: Extract entities → model tasks
   → contracts/: Each file → contract test task
   → research.md: Extract decisions → setup tasks
3. Generate tasks by category:
   → Setup: project init, dependencies, linting
   → Tests: contract tests, integration tests
   → Core: models, services, CLI commands
   → Integration: DB, middleware, logging
   → Polish: unit tests, performance, docs
4. Apply task rules:
   → Different files = mark [P] for parallel
   → Same file = sequential (no [P])
   → Tests before implementation (TDD)
5. Number tasks sequentially (T001, T002...)
6. Generate dependency graph
7. Create parallel execution examples
8. Validate task completeness:
   → All contracts have tests?
   → All entities have models?
   → All endpoints implemented?
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root (Rust: `tests/` for integration tests)

## Phase 3.1: Setup
- [ ] T001 Create project structure per implementation plan
- [ ] T002 Initialize Rust project with Cargo and core deps (clap, serde, anyhow, thiserror, tracing, reqwest, tokio, toml)
- [ ] T003 [P] Configure rustfmt and clippy (CI `-D warnings`)

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**
- [ ] T004 [P] CLI smoke test using assert_cmd: `paperless-ngx-ocr2 --help`
- [ ] T005 [P] CLI JSON output contract test with assert_cmd + predicates
- [ ] T006 [P] Unit tests for configuration parsing in `tests/unit/config_tests.rs`
- [ ] T007 [P] Unit tests for file validation in `tests/unit/file_validation_tests.rs`
- [ ] T008 [P] Unit tests for API client in `tests/unit/api_tests.rs`
- [ ] T009 [P] Integration test for happy path in `tests/integration/cli_tests.rs`
- [ ] T010 [P] Integration test for error cases in `tests/integration/error_tests.rs`

## Phase 3.3: Core Implementation (ONLY after tests are failing)
- [ ] T011 [P] Implement CLI with clap in `src/main.rs`
- [ ] T012 [P] Implement core library in `src/lib.rs`
- [ ] T013 [P] Add configuration handling in `src/config.rs`
- [ ] T014 [P] Add error types in `src/error.rs`
- [ ] T015 [P] Add file validation in `src/file_validation.rs`
- [ ] T016 [P] Add API client in `src/api/mod.rs`
- [ ] T017 [P] Add file upload API in `src/api/file.rs`
- [ ] T018 [P] Add OCR API in `src/api/ocr.rs`
- [ ] T019 [P] Add CLI commands in `src/cli/mod.rs`
- [ ] T020 [P] Add CLI commands implementation in `src/cli/commands.rs`

## Phase 3.4: Integration
- [ ] T021 Wire configuration loading into CLI
- [ ] T022 Wire file validation into CLI flow
- [ ] T023 Wire API client into CLI flow
- [ ] T024 Add logging with tracing (stderr only)
- [ ] T025 Add proper exit codes per constitution
- [ ] T026 Add JSON output format support
- [ ] T027 Add verbose logging support

## Phase 3.5: Polish
- [ ] T028 [P] Unit tests for error handling edge cases
- [ ] T029 [P] Unit tests for configuration validation
- [ ] T030 [P] Unit tests for API response parsing
- [ ] T031 [P] Integration tests for all CLI scenarios
- [ ] T032 [P] Performance tests for large files
- [ ] T033 [P] Update README with quickstart guide
- [ ] T034 [P] Add man page generation
- [ ] T035 [P] Add shell completion generation
- [ ] T036 [P] Manual test script for CLI scenarios
- [ ] T037 [P] Remove duplication and refactor
- [ ] T038 [P] Add examples directory with sample configs

## Dependencies
- Tests (T004-T010) before implementation (T011-T020)
- T011 blocks T021, T022, T023
- T012 blocks T021, T022, T023
- T013 blocks T021
- T014 blocks T020, T021, T022, T023
- T015 blocks T022
- T016, T017, T018 blocks T023
- T019, T020 blocks T021, T022, T023
- Implementation before polish (T028-T038)

## Parallel Example
```
# Launch T004-T010 together:
Task: "CLI smoke test using assert_cmd: paperless-ngx-ocr2 --help"
Task: "CLI JSON output contract test with assert_cmd + predicates"
Task: "Unit tests for configuration parsing in tests/unit/config_tests.rs"
Task: "Unit tests for file validation in tests/unit/file_validation_tests.rs"
Task: "Unit tests for API client in tests/unit/api_tests.rs"
Task: "Integration test for happy path in tests/integration/cli_tests.rs"
Task: "Integration test for error cases in tests/integration/error_tests.rs"
```

## Notes
- [P] tasks = different files, no dependencies
- Verify tests fail before implementing
- Commit after each task
- Avoid: vague tasks, same file conflicts
- Follow Rust CLI constitution principles
- Use anyhow/thiserror for error handling
- Use tracing for logging
- Use clap for CLI parsing
- Use serde for JSON serialization
- Use reqwest for HTTP requests
- Use toml for configuration

## Task Generation Rules
*Applied during main() execution*

1. **From Contracts**:
   - Each contract file → contract test task [P]
   - Each endpoint → implementation task
   
2. **From Data Model**:
   - Each entity → model creation task [P]
   - Relationships → service layer tasks
   
3. **From User Stories**:
   - Each story → integration test [P]
   - Quickstart scenarios → validation tasks

4. **Ordering**:
   - Setup → Tests → Models → Services → Endpoints → Polish
   - Dependencies block parallel execution

## Validation Checklist
*GATE: Checked by main() before returning*

- [ ] All contracts have corresponding tests
- [ ] All entities have model tasks
- [ ] All tests come before implementation
- [ ] Parallel tasks truly independent
- [ ] Each task specifies exact file path
- [ ] No task modifies same file as another [P] task
