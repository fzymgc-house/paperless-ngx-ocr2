# Feature Specification: OCR CLI Tool

**Feature Branch**: `001-ocr-cli`  
**Created**: 2025-01-23  
**Status**: Draft  
**Input**: User description: "I am building a cli tool that will use rest API to upload a pdf or image file and have that file OCRd. This tool should be configured in a 12factor fashion that is fully compatible with a toml based configuration. It should take a few arguments: the file to upload, the api key to use, and the endpoint to connect to. The tool should work against the mistral AI apis, and most specifically against it's file and ocr apis"

## Execution Flow (main)
```
1. Parse user description from Input
   → If empty: ERROR "No feature description provided"
2. Extract key concepts from description
   → Identify: actors, actions, data, constraints
3. For each unclear aspect:
   → Mark with [NEEDS CLARIFICATION: specific question]
4. Fill User Scenarios & Testing section
   → If no clear user flow: ERROR "Cannot determine user scenarios"
5. Generate Functional Requirements
   → Each requirement must be testable
   → Mark ambiguous requirements
6. Identify Key Entities (if data involved)
7. Run Review Checklist
   → If any [NEEDS CLARIFICATION]: WARN "Spec has uncertainties"
   → If implementation details found: ERROR "Remove tech details"
8. Return: SUCCESS (spec ready for planning)
```

---

## ⚡ Quick Guidelines
- ✅ Focus on WHAT users need and WHY
- ❌ Avoid HOW to implement (no tech stack, APIs, code structure)
- 👥 Written for business stakeholders, not developers

### Section Requirements
- **Mandatory sections**: Must be completed for every feature
- **Optional sections**: Include only when relevant to the feature
- When a section doesn't apply, remove it entirely (don't leave as "N/A")

### For AI Generation
When creating this spec from a user prompt:
1. **Mark all ambiguities**: Use [NEEDS CLARIFICATION: specific question] for any assumption you'd need to make
2. **Don't guess**: If the prompt doesn't specify something (e.g., "login system" without auth method), mark it
3. **Think like a tester**: Every vague requirement should fail the "testable and unambiguous" checklist item
4. **Common underspecified areas**:
   - User types and permissions
   - Data retention/deletion policies  
   - Performance targets and scale
   - Error handling behaviors
   - Integration requirements
   - Security/compliance needs

---

## User Scenarios & Testing *(mandatory)*

### Primary User Story
A user wants to extract text from a PDF or image file using OCR technology. They have a Mistral AI API key and want to upload their file to get the extracted text back through a simple command-line interface.

### Acceptance Scenarios
1. **Given** a valid PDF file and API credentials, **When** the user runs the CLI with the file path, **Then** the tool uploads the file and returns the extracted text
2. **Given** a valid image file (PNG/JPG) and API credentials, **When** the user runs the CLI with the file path, **Then** the tool uploads the file and returns the extracted text
3. **Given** invalid file format, **When** the user runs the CLI, **Then** the tool shows a clear error message and exits with non-zero code
4. **Given** invalid API credentials, **When** the user runs the CLI, **Then** the tool shows authentication error and exits with non-zero code
5. **Given** network connectivity issues, **When** the user runs the CLI, **Then** the tool shows network error and exits with non-zero code
6. **Given** a user wants shell completion support, **When** they run the CLI with --completions <shell>, **Then** the tool outputs a completion script for the specified shell
7. **Given** a user specifies an unsupported shell, **When** they run the CLI with --completions <unsupported>, **Then** the tool shows an error message listing supported shells

### Edge Cases
- What happens when the file is too large for the API? (Files up to 100MB are supported)
- How does the tool handle files with no extractable text? (Returns warning message with success exit code)
- What happens when the API rate limit is exceeded? (Retry with exponential backoff, 3 attempts max, then fail)
- How does the tool handle corrupted or password-protected PDFs? (Detect and reject with validation error)
- What happens when generating completions without specifying a file? (File argument is optional for completion generation)
- How does the tool handle unsupported shell types for completion generation? (Show error with list of supported shells)

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: System MUST accept file path as command-line argument
- **FR-002**: System MUST accept API key as command-line argument or environment variable
- **FR-003**: System MUST accept API endpoint as command-line argument or configuration
- **FR-004**: System MUST support PDF and image file formats (PNG, JPG, JPEG) up to 100MB
- **FR-005**: System MUST upload file to Mistral AI file API
- **FR-006**: System MUST trigger OCR processing using Mistral AI OCR API
- **FR-007**: System MUST return extracted text to stdout
- **FR-008**: System MUST support TOML-based configuration file
- **FR-009**: System MUST follow 12-factor app principles for configuration
- **FR-010**: System MUST support .env file loading for environment variables
- **FR-011**: System MUST provide clear error messages for all failure scenarios
- **FR-012**: System MUST support JSON output format via --json flag
- **FR-013**: System MUST validate file exists and is readable before processing
- **FR-014**: System MUST validate file size does not exceed 100MB limit
- **FR-015**: System MUST validate API key format before making requests
- **FR-016**: System MUST handle API response errors gracefully
- **FR-017**: System MUST support verbose logging via --verbose flag
- **FR-018**: System MUST implement retry logic with exponential backoff for rate limit errors (3 attempts max)
- **FR-019**: System MUST detect and reject password-protected PDFs with validation error
- **FR-020**: System MUST return warning message for files with no extractable text (exit code 0)
- **FR-021**: System MUST support --config flag for custom configuration file path
- **FR-022**: System MUST search for config files in: custom path (if specified), current directory, then ~/.config/paperless-ngx-ocr2/
- **FR-023**: System MUST support generating shell completion scripts via --completions flag
- **FR-024**: System MUST support completion generation for bash, zsh, fish, and PowerShell shells
- **FR-025**: System MUST make --file argument optional when generating completions
- **FR-026**: System MUST output completion scripts to stdout for easy redirection to files

### Key Entities *(include if feature involves data)*
- **Configuration**: TOML file containing default API endpoint, optional API key, and other settings
- **File Upload**: The PDF or image file to be processed for OCR
- **OCR Result**: Extracted text content from the uploaded file
- **API Credentials**: API key and endpoint for Mistral AI services

## Clarifications

### Session 2025-01-23
- Q: Configuration loading priority order for CLI args, environment variables, .env file, TOML config, and defaults? → A: CLI args → Environment variables → .env file → TOML config → defaults
- Q: How should the tool behave when OCR returns empty or no text? → A: Return warning message with success exit code (0)
- Q: How should the tool handle Mistral AI rate limit responses (HTTP 429)? → A: Retry with exponential backoff (3 attempts max) then fail
- Q: How should the tool handle password-protected or encrypted PDF files? → A: Detect and reject with clear error message (validation error)
- Q: Where should the tool look for configuration files? → A: Current directory config.toml, then ~/.config/paperless-ngx-ocr2/config.toml, plus custom config path via --config flag
- Q: Should the tool support shell completion generation? → A: Yes, via --completions flag supporting bash, zsh, fish, and PowerShell
- Q: Should file argument be required when generating completions? → A: No, file argument should be optional when using --completions

---

## Review & Acceptance Checklist
*GATE: Automated checks run during main() execution*

### Content Quality
- [ ] No implementation details (languages, frameworks, APIs)
- [ ] Focused on user value and business needs
- [ ] Written for non-technical stakeholders
- [ ] All mandatory sections completed

### Requirement Completeness
- [ ] No [NEEDS CLARIFICATION] markers remain
- [ ] Requirements are testable and unambiguous  
- [ ] Success criteria are measurable
- [ ] Scope is clearly bounded
- [ ] Dependencies and assumptions identified

---

## Execution Status
*Updated by main() during processing*

- [ ] User description parsed
- [ ] Key concepts extracted
- [ ] Ambiguities marked
- [ ] User scenarios defined
- [ ] Requirements generated
- [ ] Entities identified
- [ ] Review checklist passed

---
