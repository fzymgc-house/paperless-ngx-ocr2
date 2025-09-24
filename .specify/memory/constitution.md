<!--
Sync Impact Report
- Version change: N/A → 1.0.0
- Modified principles: (template placeholders) → Concrete Rust CLI principles
- Added sections: Security & Performance Baselines; Development Workflow
- Removed sections: None
- Templates requiring updates:
  ✅ .specify/templates/plan-template.md (version reference and path)
  ✅ .specify/templates/tasks-template.md (Rust paths and tasks)
  ✅ .specify/templates/agent-file-template.md (no changes required)
  ✅ .specify/templates/spec-template.md (no changes required)
- Follow-up TODOs: None
-->

# Paperless-NGX-OCR2 Constitution

## Core Principles

### I. CLI Contract (Non-Negotiable)
All functionality MUST be invocable via a stable CLI with text I/O contracts:
- Inputs: command-line flags/args and optional stdin
- Outputs: human-readable stdout by default; JSON via `--json` flag
- Errors: strictly to stderr with non-zero exit codes
- Exit codes: 0 success; 2 validation error; 3 I/O error; 4 config error; 5 internal error
Rationale: Ensures composability in shells and predictable behavior in automation.

### II. Safety & Reliability
The codebase MUST compile warning-free with `rustc` stable and enforce:
- `rustfmt` formatting; `clippy` with `-D warnings` in CI
- No `unwrap`/`expect` in user-facing CLI paths (use `anyhow`/`thiserror`)
- Prefer `Result`-based flows; avoid `unsafe` unless justified in review
Rationale: Rust’s guarantees only hold when we treat warnings as errors and model failures explicitly.

### III. Test-First Discipline
Tests MUST precede implementation for new features and bug fixes:
- Unit tests in `src/` modules; integration tests in `tests/`
- CLI behavior tests using `assert_cmd` and `predicates`
- Golden-output tests for `--json` schema and human output where stable
Rationale: Locks in behavior and prevents regressions on the CLI contract.

### IV. Observability & Logging
Provide structured, level-based logging:
- Default human-readable logs; JSON logs via `RUST_LOG_FORMAT=json` or `--log-json`
- Respect `RUST_LOG` levels; no logs on stdout (stdout reserved for program output)
- Include error contexts and actionable messages
Rationale: Debuggability without polluting program output.

### V. Versioning & Compatibility
Adopt Semantic Versioning for both crate and CLI surface:
- Breaking CLI flag/arg changes or JSON schema changes require MAJOR bump
- Additive flags/fields are MINOR; bugfixes and non-semantic copy changes are PATCH
- Deprecations announced one MINOR before removal when feasible
Rationale: Predictable upgrades for users and automation.

## Security & Performance Baselines

- No `unsafe` blocks unless a documented, reviewed justification exists
- Handle untrusted inputs defensively; validate file paths and external calls
- Cross-platform support: macOS and Linux must be kept green in CI
- Performance targets: avoid O(n^2) on large inputs; stream when practical
- Dependencies: prefer actively maintained crates (clap, serde, anyhow, thiserror, tracing)

## Development Workflow

- Format with `rustfmt`; lint with `clippy -D warnings`
- CI gates: build, tests, fmt, clippy, minimal MSRV, and release dry-run
- Code review: at least one reviewer; justify complexity trade-offs in PR
- Documentation: `--help` must be accurate; README quickstart stays up to date

## Governance

- This constitution supersedes ad-hoc practices. All PRs must include a Constitution Check
  in the plan to confirm compliance or explicitly justify exceptions.
- Amendments: propose via PR describing changes, migration impact, and version bump type.
- Versioning policy: use SemVer for the constitution document itself:
  MAJOR for removals/redefinitions, MINOR for new/expanded sections, PATCH for clarifications.
- Compliance review: CI enforces fmt, clippy, tests; reviewers enforce principles.

**Version**: 1.0.0 | **Ratified**: 2025-09-23 | **Last Amended**: 2025-09-23
# [PROJECT_NAME] Constitution
<!-- Example: Spec Constitution, TaskFlow Constitution, etc. -->

## Core Principles

### [PRINCIPLE_1_NAME]
<!-- Example: I. Library-First -->
[PRINCIPLE_1_DESCRIPTION]
<!-- Example: Every feature starts as a standalone library; Libraries must be self-contained, independently testable, documented; Clear purpose required - no organizational-only libraries -->

### [PRINCIPLE_2_NAME]
<!-- Example: II. CLI Interface -->
[PRINCIPLE_2_DESCRIPTION]
<!-- Example: Every library exposes functionality via CLI; Text in/out protocol: stdin/args → stdout, errors → stderr; Support JSON + human-readable formats -->

### [PRINCIPLE_3_NAME]
<!-- Example: III. Test-First (NON-NEGOTIABLE) -->
[PRINCIPLE_3_DESCRIPTION]
<!-- Example: TDD mandatory: Tests written → User approved → Tests fail → Then implement; Red-Green-Refactor cycle strictly enforced -->

### [PRINCIPLE_4_NAME]
<!-- Example: IV. Integration Testing -->
[PRINCIPLE_4_DESCRIPTION]
<!-- Example: Focus areas requiring integration tests: New library contract tests, Contract changes, Inter-service communication, Shared schemas -->

### [PRINCIPLE_5_NAME]
<!-- Example: V. Observability, VI. Versioning & Breaking Changes, VII. Simplicity -->
[PRINCIPLE_5_DESCRIPTION]
<!-- Example: Text I/O ensures debuggability; Structured logging required; Or: MAJOR.MINOR.BUILD format; Or: Start simple, YAGNI principles -->

## [SECTION_2_NAME]
<!-- Example: Additional Constraints, Security Requirements, Performance Standards, etc. -->

[SECTION_2_CONTENT]
<!-- Example: Technology stack requirements, compliance standards, deployment policies, etc. -->

## [SECTION_3_NAME]
<!-- Example: Development Workflow, Review Process, Quality Gates, etc. -->

[SECTION_3_CONTENT]
<!-- Example: Code review requirements, testing gates, deployment approval process, etc. -->

## Governance
<!-- Example: Constitution supersedes all other practices; Amendments require documentation, approval, migration plan -->

[GOVERNANCE_RULES]
<!-- Example: All PRs/reviews must verify compliance; Complexity must be justified; Use [GUIDANCE_FILE] for runtime development guidance -->

**Version**: [CONSTITUTION_VERSION] | **Ratified**: [RATIFICATION_DATE] | **Last Amended**: [LAST_AMENDED_DATE]
<!-- Example: Version: 2.1.1 | Ratified: 2025-06-13 | Last Amended: 2025-07-16 -->