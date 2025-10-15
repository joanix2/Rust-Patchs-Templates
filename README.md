# Rust-Patchs-Templates

A Rust code generator with AST-based intelligent merging that uses Patch Theory to preserve manual edits while applying template updates.

## Features

- 🎯 **Template-based code generation** using Tera templates
- 🌳 **AST-based intelligent merging** using `syn` for parsing
- 🔀 **Patch Theory** for computing minimal structural diffs
- ✅ **Preserves manual edits** when regenerating code
- 📝 **Fully compilable output** formatted with `prettyplease`
- 🚫 **No text markers** - pure AST-based approach
- 🛠️ **CLI with three commands**: generate, diff, check

## Installation

```bash
cargo install --path .
```

Or build from source:

```bash
cargo build --release
```

## Usage

### Generate Command

Generate code from a template and intelligently merge with existing files:

```bash
rust-patchs-templates generate \
  --template examples/templates/simple_struct.tera \
  --context examples/contexts/simple_struct.json \
  --output examples/output/person.rs \
  --strategy manual
```

**Merge Strategies:**
- `manual` (default): Prefer manual edits, skip template updates for modified items
- `template`: Prefer template updates, override manual edits
- `fail`: Fail on conflicts, forcing explicit resolution

### Diff Command

Show differences between generated code and existing file:

```bash
rust-patchs-templates diff \
  --template examples/templates/simple_struct.tera \
  --context examples/contexts/simple_struct.json \
  --existing examples/output/person.rs
```

Add `--verbose` for detailed AST-level diff:

```bash
rust-patchs-templates diff \
  --template examples/templates/simple_struct.tera \
  --context examples/contexts/simple_struct.json \
  --existing examples/output/person.rs \
  --verbose
```

### Check Command

Check for conflicts without applying changes:

```bash
rust-patchs-templates check \
  --template examples/templates/simple_struct.tera \
  --context examples/contexts/simple_struct.json \
  --existing examples/output/person.rs
```

Exit code 0 indicates no conflicts, exit code 1 indicates conflicts detected.

## How It Works

### 1. Template Rendering

Templates are written in Tera format with variables and control structures:

```rust
pub struct {{ struct_name }} {
    {% for field in fields %}
    pub {{ field.name }}: {{ field.type }},
    {% endfor %}
}
```

Context data is provided as JSON:

```json
{
  "struct_name": "Person",
  "fields": [
    {"name": "name", "type": "String"},
    {"name": "age", "type": "u32"}
  ]
}
```

### 2. AST Parsing

Both generated and existing code are parsed into Abstract Syntax Trees using `syn`:

```rust
let generated_ast = syn::parse_file(&generated_code)?;
let existing_ast = syn::parse_file(&existing_code)?;
```

### 3. Structural Diffing

The tool computes a structural diff at the item level (functions, structs, etc.):

- **Insert**: Item exists in template but not in existing file
- **Delete**: Item exists in existing file but not in template
- **Modify**: Item exists in both but has changed
- **Keep**: Item is unchanged

### 4. Intelligent Merging

The merge algorithm preserves manual edits:

- Manual additions are kept
- Template updates apply only to unmodified items (with `manual` strategy)
- Conflicts are detected and reported

### 5. Code Formatting

Merged code is formatted with `prettyplease` for consistent, readable output.

## Examples

### Example 1: Simple Struct Generation

Template (`simple_struct.tera`):
```rust
pub struct {{ struct_name }} {
    {% for field in fields %}
    pub {{ field.name }}: {{ field.type }},
    {% endfor %}
}
```

Context (`simple_struct.json`):
```json
{
  "struct_name": "User",
  "fields": [
    {"name": "id", "type": "u64"},
    {"name": "email", "type": "String"}
  ]
}
```

Generated code:
```rust
pub struct User {
    pub id: u64,
    pub email: String,
}
```

### Example 2: Preserving Manual Edits

Initial generation creates:
```rust
pub struct User {
    pub id: u64,
    pub email: String,
}
```

Manual edits add a method:
```rust
pub struct User {
    pub id: u64,
    pub email: String,
}

impl User {
    pub fn validate_email(&self) -> bool {
        self.email.contains('@')
    }
}
```

Template updated to add a field. Regeneration preserves manual `impl`:
```rust
pub struct User {
    pub id: u64,
    pub email: String,
    pub name: String,  // New field from template
}

impl User {
    pub fn validate_email(&self) -> bool {
        self.email.contains('@')
    }  // Manual edit preserved!
}
```

## Architecture

```
┌─────────────┐
│  Template   │
│   (.tera)   │
└──────┬──────┘
       │
       ▼
┌─────────────┐     ┌─────────────┐
│   Context   │────▶│   Render    │
│   (.json)   │     │   (Tera)    │
└─────────────┘     └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  Generated  │
                    │    Code     │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │  AST Parse  │
                    │   (syn)     │
                    └──────┬──────┘
                           │
       ┌───────────────────┼───────────────────┐
       ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Existing   │    │    Diff     │    │    Merge    │
│    Code     │───▶│  (similar)  │───▶│  (Patch)    │
└─────────────┘    └─────────────┘    └──────┬──────┘
                                              │
                                              ▼
                                       ┌─────────────┐
                                       │   Format    │
                                       │(prettyplease)│
                                       └──────┬──────┘
                                              │
                                              ▼
                                       ┌─────────────┐
                                       │   Output    │
                                       │    File     │
                                       └─────────────┘
```

## Dependencies

- **syn**: Rust AST parsing with full feature support
- **quote**: Token stream manipulation
- **prettyplease**: Code formatting
- **tera**: Template engine
- **similar**: Text diffing for display
- **clap**: Command-line interface
- **anyhow**: Error handling
- **serde/serde_json**: Context data parsing

## Testing

Run all tests:

```bash
cargo test
```

Run specific test module:

```bash
cargo test ast_parser
cargo test diff
cargo test merger
```

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Code is formatted: `cargo fmt`
3. No clippy warnings: `cargo clippy`

## License

This project is licensed under the MIT License.

## Use Cases

- **Code scaffolding**: Generate boilerplate code from templates
- **API generation**: Generate REST API handlers from specifications
- **Data models**: Generate struct definitions with derived traits
- **Test generation**: Generate test stubs from templates
- **Documentation**: Generate documented code with consistent style
- **Refactoring**: Update code structure across multiple files while preserving manual edits