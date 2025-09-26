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
   → Integration: API client, middleware, logging
   → Polish: unit tests, performance, docs, containerization
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
   → All API endpoints implemented?
9. Return: SUCCESS (tasks ready for execution)
```

## Format: `[ID] [P?] Description`
- **[P]**: Can run in parallel (different files, no dependencies)
- Include exact file paths in descriptions

## Path Conventions
- **Single project**: `src/`, `tests/` at repository root (Rust: `tests/` for integration tests)

## Phase 3.1: Setup
- [x] T001 Create Rust project structure with Cargo.toml and basic directories
- [x] T002 Initialize Cargo.toml with dependencies: clap, serde, anyhow, thiserror, tracing, reqwest, tokio, toml, dotenv, clap_complete
- [x] T003 [P] Configure rustfmt.toml and clippy configuration files
- [x] T004 [P] Create .gitignore for Rust project
- [x] T005 [P] Set up GitHub Actions workflow for CI (build, test, lint, clippy)
- [x] T006 [P] Create a renovate.json file for auto-updating dependencies

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE 3.3
**CRITICAL: These tests MUST be written and MUST FAIL before ANY implementation**

### Contract Tests
- [x] T007 [P] Contract test for file upload request in `tests/integration/test_file_upload_contract.rs`
- [x] T008 [P] Contract test for file upload response in `tests/integration/test_file_upload_contract.rs`
- [x] T009 [P] Contract test for OCR API request in `tests/integration/test_ocr_api_contract.rs`
- [x] T010 [P] Contract test for OCR API response in `tests/integration/test_ocr_api_contract.rs`
- [x] T011 [P] Contract test for CLI output format in `tests/integration/test_cli_output_contract.rs`
- [x] T012 [P] Contract test for API error handling in `tests/integration/test_api_error_contract.rs`

### CLI Behavior Tests
- [x] T013 [P] CLI smoke test using assert_cmd: `paperless-ngx-ocr2 --help` in `tests/integration/test_cli_basic.rs`
- [x] T014 [P] CLI file argument test with assert_cmd in `tests/integration/test_cli_basic.rs`
- [x] T015 [P] CLI JSON output test with assert_cmd + predicates in `tests/integration/test_cli_json.rs`
- [x] T016 [P] CLI error exit codes test in `tests/integration/test_cli_errors.rs`
- [x] T017 [P] CLI verbose logging test in `tests/integration/test_cli_logging.rs`

### Integration Tests
- [x] T018 [P] Create example test input files for happy and failure paths in `tests/fixtures/`
- [x] T019 [P] Happy path integration test: PDF upload and OCR in `tests/integration/test_ocr_workflow.rs`
- [x] T020 [P] Happy path integration test: Image upload and OCR in `tests/integration/test_ocr_workflow.rs`
- [x] T021 [P] Error handling test: Invalid file format in `tests/integration/test_error_handling.rs`
- [x] T022 [P] Error handling test: File too large in `tests/integration/test_error_handling.rs`
- [x] T023 [P] Error handling test: Invalid API key in `tests/integration/test_error_handling.rs`
- [x] T024 [P] Error handling test: Network timeout in `tests/integration/test_error_handling.rs`
- [x] T025 [P] Error handling test: Rate limit retry logic in `tests/integration/test_error_handling.rs`
- [x] T026 [P] Error handling test: Password-protected PDF detection in `tests/integration/test_error_handling.rs`
- [x] T027 [P] Error handling test: Empty OCR text handling in `tests/integration/test_error_handling.rs`
- [x] T028 [P] Configuration test: .env file loading in `tests/integration/test_config_loading.rs`
- [x] T029 [P] Configuration test: Config file search order in `tests/integration/test_config_loading.rs`
- [x] T030 [P] Configuration test: --config flag handling in `tests/integration/test_config_loading.rs`
- [x] T031 [P] Shell completion test: --completions flag for bash in `tests/integration/test_shell_completion.rs`
- [x] T032 [P] Shell completion test: --completions flag for zsh in `tests/integration/test_shell_completion.rs`
- [x] T033 [P] Shell completion test: --completions flag for fish in `tests/integration/test_shell_completion.rs`
- [x] T034 [P] Shell completion test: --completions flag for powershell in `tests/integration/test_shell_completion.rs`
- [x] T035 [P] Shell completion test: unsupported shell error handling in `tests/integration/test_shell_completion.rs`

## Phase 3.3: Core Implementation (ONLY after tests are failing)

### Entity Models
- [x] T036 [P] Implement Configuration entity in `src/config.rs`
- [x] T037 [P] Implement FileUpload entity in `src/file.rs`
- [x] T038 [P] Implement OCRResult entity in `src/ocr.rs`
- [x] T039 [P] Implement APICredentials entity in `src/credentials.rs`
- [x] T040 [P] Implement Error entity with thiserror in `src/error.rs`

### Core Library
- [x] T041 [P] Implement main library entry point in `src/lib.rs`
- [x] T042 [P] Implement CLI argument parsing with clap in `src/cli/mod.rs`
- [x] T043 [P] Implement CLI commands in `src/cli/commands.rs`

### API Client
- [x] T044 [P] Implement Mistral AI Files API client in `src/api/files.rs`
- [x] T045 [P] Implement Mistral AI OCR API client in `src/api/ocr.rs`
- [x] T046 [P] Implement API client base with reqwest in `src/api/mod.rs`
- [x] T047 [P] Implement authentication handling in `src/api/auth.rs`

### Main Application
- [x] T048 Implement main.rs with CLI entry point and error handling
- [x] T049 Wire configuration loading into main flow
- [x] T050 Wire file validation into main flow
- [x] T051 Wire API client into main flow

## Phase 3.4: Integration

### Configuration and Logging
- [x] T052 Implement TOML configuration file loading with 12-factor support
- [x] T053 Implement environment variable override handling
- [x] T054 Implement .env file loading with dotenv
- [x] T055 Implement configuration file search order (custom → current → ~/.config/)
- [x] T056 Implement --config flag for custom config file path
- [x] T057 Add structured logging with tracing (stderr only)
- [x] T058 Add verbose logging support with --verbose flag

### File Processing
- [x] T059 Implement file validation (size, format, readability)
- [x] T060 Implement password-protected PDF detection and rejection
- [x] T061 Implement file upload to Mistral AI Files API
- [x] T062 Implement OCR processing via Mistral AI OCR API
- [x] T063 Implement response parsing and text extraction
- [x] T064 Implement empty text warning handling (exit code 0)

### Error Handling and Resilience
- [x] T065 Implement retry logic with exponential backoff for rate limits (3 attempts max)
- [x] T066 Implement API error response parsing and handling
- [x] T067 Implement network timeout handling

### Output Formatting
- [x] T068 Implement human-readable output formatting
- [x] T069 Implement JSON output formatting with --json flag
- [x] T070 Implement proper exit codes per constitution

### Shell Completion
- [x] T071 Implement --completions flag for shell completion generation
- [x] T072 Implement bash completion script generation
- [x] T073 Implement zsh completion script generation
- [x] T074 Implement fish completion script generation
- [x] T075 Implement powershell completion script generation
- [x] T076 Make --file argument optional when using --completions

## Phase 3.5: Polish

### Unit Tests
- [x] T077 [P] Unit tests for configuration validation in `tests/unit/test_config.rs`
- [x] T078 [P] Unit tests for file validation in `tests/unit/test_file.rs`
- [x] T079 [P] Unit tests for API client methods in `tests/unit/test_api.rs`
- [x] T080 [P] Unit tests for error handling in `tests/unit/test_error.rs`
- [x] T081 [P] Unit tests for CLI argument parsing in `tests/unit/test_cli.rs`

### Performance and Edge Cases
- [x] T082 [P] Performance tests for large files (up to 100MB) in `tests/performance/test_large_files.rs`
- [x] T083 [P] Memory usage tests with streaming in `tests/performance/test_memory.rs`
- [x] T084 [P] Network timeout and retry logic tests in `tests/integration/test_network.rs`

### Documentation and Examples
- [x] T085 [P] Update README.md with installation and usage instructions
- [x] T086 [P] Create examples directory with sample files and configurations
- [x] T087 [P] Generate man page from clap help text
- [x] T088 [P] Update shell completion documentation in README.md

### Containerization
- [x] T089 [P] Create Dockerfile for multi-stage Rust build
- [x] T090 [P] Create Docker Compose file for local testing
- [x] T091 [P] Set up GitHub Actions for multi-arch container builds (AMD64, ARM64)
- [x] T092 [P] Add container integration tests in `tests/container/test_docker.rs`

### Final Polish
- [x] T093 [P] Remove code duplication and refactor
- [x] T094 [P] Run cargo clippy and fix all warnings
- [x] T095 [P] Run cargo fmt and ensure consistent formatting
- [x] T096 [P] Manual testing following quickstart.md scenarios
- [x] T097 [P] Performance validation and optimization

## Dependencies
- Setup (T001-T006) before everything else
- Contract tests (T007-T012) before CLI tests (T013-T017)
- Test fixtures (T018) before integration tests (T019-T035)
- All tests (T007-T035) before implementation (T036-T076)
- Entity models (T036-T040) before core library (T041-T043)
- API client (T044-T047) before main application (T048-T051)
- Core implementation (T036-T051) before integration (T052-T076)
- Integration (T052-T076) before polish (T077-T097)

## Parallel Example
```
# Launch contract tests together (T007-T012):
Task: "Contract test for file upload request in tests/integration/test_file_upload_contract.rs"
Task: "Contract test for file upload response in tests/integration/test_file_upload_contract.rs"
Task: "Contract test for OCR API request in tests/integration/test_ocr_api_contract.rs"
Task: "Contract test for OCR API response in tests/integration/test_ocr_api_contract.rs"
Task: "Contract test for CLI output format in tests/integration/test_cli_output_contract.rs"
Task: "Contract test for API error handling in tests/integration/test_api_error_contract.rs"

# Launch entity models together (T031-T035):
Task: "Implement Configuration entity in src/config.rs"
Task: "Implement FileUpload entity in src/file.rs"
Task: "Implement OCRResult entity in src/ocr.rs"
Task: "Implement APICredentials entity in src/credentials.rs"
Task: "Implement Error entity with thiserror in src/error.rs"

# Launch new error handling and completion tests together (T025-T035):
Task: "Error handling test: Rate limit retry logic in tests/integration/test_error_handling.rs"
Task: "Error handling test: Password-protected PDF detection in tests/integration/test_error_handling.rs"
Task: "Error handling test: Empty OCR text handling in tests/integration/test_error_handling.rs"
Task: "Configuration test: .env file loading in tests/integration/test_config_loading.rs"
Task: "Configuration test: Config file search order in tests/integration/test_config_loading.rs"
Task: "Configuration test: --config flag handling in tests/integration/test_config_loading.rs"
Task: "Shell completion test: --completions flag for bash in tests/integration/test_shell_completion.rs"
Task: "Shell completion test: --completions flag for zsh in tests/integration/test_shell_completion.rs"
Task: "Shell completion test: --completions flag for fish in tests/integration/test_shell_completion.rs"
Task: "Shell completion test: --completions flag for powershell in tests/integration/test_shell_completion.rs"
Task: "Shell completion test: unsupported shell error handling in tests/integration/test_shell_completion.rs"
```

## Notes
- [P] tasks = different files, no dependencies between them
- Verify all tests fail before implementing
- Commit after completing each phase
- Follow constitutional principles: TDD, proper error handling, structured logging
- Use anyhow/thiserror for error handling (no unwrap in CLI paths)
- Ensure cross-platform compatibility (macOS, Linux)
- Multi-architecture container support required

## Task Generation Rules
*Applied during main() execution*

1. **From Contracts** (6 contract files):
   - Each contract file → contract test task [P]
   - API integration → implementation tasks
   
2. **From Data Model** (5 entities):
   - Each entity → model creation task [P]
   - Relationships → integration tasks
   
3. **From Quickstart Scenarios**:
   - Each usage example → integration test [P]
   - Installation methods → setup and containerization tasks

4. **From Research Decisions**:
   - Containerization → Docker and CI/CD tasks
   - API workflow → file upload + OCR processing tasks

## Validation Checklist
*GATE: Checked by main() before returning*

- [x] All contracts have corresponding tests (T007-T012)
- [x] All entities have model tasks (T036-T040)
- [x] All tests come before implementation (T007-T035 before T036+)
- [x] Test fixtures created before integration tests (T018 before T019-T035)
- [x] New clarification requirements covered (T025-T030, T052-T056, T060, T064-T067)
- [x] Shell completion requirements covered (T031-T035, T071-T076, T088)
- [x] Parallel tasks truly independent (different files)
- [x] Each task specifies exact file path
- [x] No task modifies same file as another [P] task
- [x] Containerization requirements included (T089-T092)
- [x] Constitutional compliance verified (TDD, error handling, logging)
- [x] dotenv dependency added (T002)
- [x] clap_complete dependency added (T002)
- [x] Configuration priority order implemented (T052-T056)
- [x] Retry logic and error handling implemented (T065-T067)
- [x] Shell completion generation implemented (T071-T076)