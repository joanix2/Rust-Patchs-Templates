# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-15

### Added

- Initial release of rust-patchs-templates
- AST-based code generation from Tera templates
- Intelligent merging using Patch Theory
- Three merge strategies: manual, template, and fail
- CLI with three commands:
  - `generate`: Generate and merge code
  - `diff`: Show differences between template and existing code
  - `check`: Check for conflicts without applying changes
- Support for preserving manual edits during code regeneration
- Structural diffing at AST item level (functions, structs, enums, etc.)
- Pretty-printing of generated code using prettyplease
- Comprehensive documentation:
  - README with examples
  - USAGE guide with best practices
  - CONTRIBUTING guidelines
  - Example templates and contexts
- Test suite with 10+ tests covering core functionality
- Demo script showcasing the tool's capabilities

### Core Modules

- **ast_parser**: Parse Rust code into AST using syn
- **diff**: Compute structural patches between ASTs
- **merger**: Intelligently merge changes while preserving manual edits
- **template**: Tera template engine integration
- **generator**: Orchestrate code generation workflow
- **cli**: Command-line interface using clap

### Dependencies

- syn 2.0 - Rust AST parsing
- quote 1.0 - Token stream manipulation
- prettyplease 0.2 - Code formatting
- tera 1.19 - Template engine
- similar 2.3 - Text diffing
- clap 4.4 - CLI framework
- anyhow 1.0 - Error handling
- serde/serde_json 1.0 - Context data handling

### Examples

- Simple struct generation with derives
- API handler generation with request/response types
- Multi-item templates with validation functions

[0.1.0]: https://github.com/joanix2/Rust-Patchs-Templates/releases/tag/v0.1.0
