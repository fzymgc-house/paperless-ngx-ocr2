# Contributing to paperless-ngx-ocr2

Thank you for your interest in contributing to paperless-ngx-ocr2! This
document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites

- Rust 1.80 or later
- Git
- Python 3.9+ (for pre-commit hooks)

### Getting Started

1. **Fork and clone the repository**:

   ```bash
   git clone https://github.com/your-username/paperless-ngx-ocr2.git
   cd paperless-ngx-ocr2
   ```

2. **Setup pre-commit hooks** (recommended):

   ```bash
   ./scripts/setup-pre-commit.sh
   ```

3. **Build the project**:

   ```bash
   cargo build
   ```

4. **Run tests**:

   ```bash
   cargo test
   ```

## Development Workflow

### Code Quality Standards

This project enforces code quality through automated checks:

- **Rust formatting**: Code must be formatted with `rustfmt`
- **Rust linting**: Code must pass `clippy` checks
- **Tests**: All tests must pass
- **Security**: Code must pass `cargo audit` security checks
- **File quality**: No trailing whitespace, proper line endings, etc.

### Pre-commit Hooks

Pre-commit hooks automatically run quality checks before each commit:

```bash
# Install hooks (run once)
pre-commit install

# Run hooks manually on all files
pre-commit run --all-files

# Run hooks on staged files only
pre-commit run
```bash

### Commit Messages

Follow conventional commit format:

```text
type(scope): description

[optional body]

[optional footer]
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:

```text
feat(cli): add support for batch processing
fix(api): handle rate limit errors gracefully
docs(readme): update installation instructions
test(memory): add large file handling tests
```

### Pull Request Process

1. **Create a feature branch**:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and ensure:
   - All tests pass (`cargo test`)
   - Code is properly formatted (`cargo fmt`)
   - No clippy warnings (`cargo clippy`)
   - Pre-commit hooks pass

3. **Commit your changes**:

   ```bash
   git add .
   git commit -m "feat: add your feature description"
   ```

4. **Push and create a pull request**:

   ```bash
   git push origin feature/your-feature-name
   ```

5. **Create a pull request** on GitHub with:
   - Clear title and description
   - Reference any related issues
   - Include test coverage for new features

### Testing

#### Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test test_cli_basic

# Run with output
cargo test -- --nocapture

# Run tests with coverage (if tarpaulin is installed)
cargo tarpaulin --out html
```bash

#### Test Categories

- **Unit tests**: Test individual functions and modules
- **Integration tests**: Test CLI functionality and API
  contracts
- **Performance tests**: Test memory usage and large file
  handling
- **Container tests**: Test Docker functionality

#### Writing Tests

- Use the common test utilities in `tests/common/`
- Follow the existing test patterns
- Include both positive and negative test cases
- Test error conditions and edge cases
- Use descriptive test names

### Code Style

#### Rust Code

- Follow standard Rust formatting (`cargo fmt`)
- Use `clippy` recommendations
- Prefer `?` operator over `unwrap()` in production code
- Use meaningful variable and function names
- Add documentation for public APIs

#### Documentation

- Update README.md for user-facing changes
- Add inline documentation for complex logic
- Update CHANGELOG.md for significant changes
- Follow markdown linting rules

### Security

- Never commit API keys or secrets
- Use environment variables for sensitive configuration
- Follow security best practices in code
- Run `cargo audit` to check for vulnerabilities

### Performance

- Test with large files (up to 100MB limit)
- Monitor memory usage in tests
- Use streaming where appropriate
- Optimize for common use cases

## Getting Help

- **Issues**: Use GitHub issues for bug reports and feature requests
- **Discussions**: Use GitHub discussions for questions and ideas
- **Documentation**: Check the README.md and inline documentation

## Release Process

Releases are managed through GitHub Actions and follow semantic versioning:

- **Major** (x.0.0): Breaking changes
- **Minor** (x.y.0): New features, backward compatible
- **Patch** (x.y.z): Bug fixes, backward compatible

## Code of Conduct

Please be respectful and inclusive in all interactions. This project follows the Contributor Covenant Code of Conduct.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).
