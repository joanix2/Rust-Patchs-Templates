# Contributing to Rust-Patchs-Templates

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo
- Git

### Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Rust-Patchs-Templates.git
   cd Rust-Patchs-Templates
   ```

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/your-feature-name
```

Use prefixes:
- `feature/` for new features
- `fix/` for bug fixes
- `docs/` for documentation
- `refactor/` for refactoring

### 2. Make Changes

Follow these guidelines:

#### Code Style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add documentation for public APIs
- Write clear, descriptive commit messages

#### Testing

- Add tests for new features
- Ensure all tests pass: `cargo test`
- Test examples work: `./demo.sh`

#### Documentation

- Update README.md for user-facing changes
- Update USAGE.md for new features
- Add inline documentation for code
- Include examples in doc comments

### 3. Commit Changes

Write clear commit messages:

```
feat: add support for multiple template directories

- Allow users to specify multiple template paths
- Add --template-dir flag to CLI
- Update documentation with examples
```

Format: `type: description`

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance

### 4. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Create a Pull Request with:
- Clear title and description
- Reference any related issues
- Include test results
- Add screenshots for UI changes

## Code Organization

```
src/
├── main.rs          # Entry point and CLI handling
├── cli.rs           # CLI argument parsing
├── ast_parser.rs    # Rust AST parsing
├── diff.rs          # Patch computation
├── merger.rs        # AST merging logic
├── template.rs      # Template engine
└── generator.rs     # Code generation orchestration
```

## Testing Guidelines

### Unit Tests

Add tests in the same file as the code:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

### Integration Tests

For end-to-end tests, use the examples directory:

```bash
cargo run -- generate \
  --template examples/templates/test.tera \
  --context examples/contexts/test.json \
  --output /tmp/test.rs
```

### Test Coverage

Aim for:
- 80%+ line coverage
- All public APIs tested
- Edge cases covered
- Error paths tested

## Adding New Features

### 1. AST Parser Features

When adding new AST parsing capabilities:

1. Add to `ast_parser.rs`
2. Update `ParsedFile` struct if needed
3. Add tests for parsing
4. Update documentation

Example:
```rust
impl ParsedFile {
    pub fn find_trait(&self, name: &str) -> Option<&ItemTrait> {
        // Implementation
    }
}
```

### 2. Diff Algorithm Improvements

When enhancing the diff algorithm:

1. Update `diff.rs`
2. Add new `PatchOp` variants if needed
3. Test with various code scenarios
4. Document the change

### 3. Merge Strategy Extensions

To add new merge strategies:

1. Add to `MergeStrategy` enum in `merger.rs`
2. Implement handling in `merge_patch`
3. Add CLI flag in `cli.rs`
4. Update documentation

### 4. Template Features

For template engine enhancements:

1. Update `template.rs`
2. Add example templates
3. Document new features
4. Add context examples

## Bug Reports

When reporting bugs, include:

1. **Description**: Clear description of the issue
2. **Steps to Reproduce**: Minimal example
3. **Expected Behavior**: What should happen
4. **Actual Behavior**: What actually happens
5. **Environment**: OS, Rust version, etc.

Example:

```markdown
## Description
Merge fails when struct has generic parameters

## Steps to Reproduce
1. Create template with generic struct
2. Generate code
3. Run merge

## Expected
Code merges successfully

## Actual
Error: "Failed to parse generic parameters"

## Environment
- OS: Ubuntu 22.04
- Rust: 1.75.0
- rust-patchs-templates: 0.1.0
```

## Feature Requests

When requesting features:

1. **Use Case**: Describe the problem
2. **Proposed Solution**: Your idea
3. **Alternatives**: Other approaches considered
4. **Examples**: Show how it would work

## Code Review Process

All contributions require code review:

1. Automated checks must pass:
   - Tests
   - Formatting
   - Clippy

2. At least one maintainer approval

3. Address review feedback promptly

4. Squash commits if requested

## Performance Considerations

- Profile code for performance-critical paths
- Avoid unnecessary allocations
- Use appropriate data structures
- Consider memory usage

## Security

- Never commit secrets or credentials
- Validate all user inputs
- Handle errors properly
- Follow Rust security best practices

## Documentation

### Code Documentation

Use rustdoc format:

```rust
/// Parse Rust source code into an AST
///
/// # Arguments
///
/// * `source` - The Rust source code to parse
///
/// # Returns
///
/// Returns a `ParsedFile` containing the AST
///
/// # Errors
///
/// Returns an error if the source code is invalid
///
/// # Example
///
/// ```
/// let code = "fn main() {}";
/// let parsed = ParsedFile::parse(code)?;
/// ```
pub fn parse(source: &str) -> Result<ParsedFile> {
    // Implementation
}
```

### Example Templates

When adding examples:

1. Add template file to `examples/templates/`
2. Add context file to `examples/contexts/`
3. Document in `USAGE.md`
4. Test the example

## Release Process

Maintainers follow this process:

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create git tag
4. Publish to crates.io (when ready)
5. Create GitHub release

## Questions?

- Open an issue for questions
- Check existing issues and PRs
- Read the documentation

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be:
- Listed in release notes
- Acknowledged in README
- Added to AUTHORS file

Thank you for contributing! 🎉
