# Usage Guide

This guide demonstrates how to use the Rust-Patchs-Templates tool for code generation with intelligent merging.

## Quick Start

### 1. Generate Code from a Template

First, create a template file (Tera format):

**templates/user.tera**
```rust
pub struct {{ name }} {
    pub id: {{ id_type }},
    pub email: String,
}
```

Create a context file (JSON):

**contexts/user.json**
```json
{
  "name": "User",
  "id_type": "u64"
}
```

Generate the code:

```bash
rust-patchs-templates generate \
  --template templates/user.tera \
  --context contexts/user.json \
  --output src/user.rs \
  --strategy manual
```

### 2. Make Manual Edits

Edit the generated file to add custom functionality:

**src/user.rs**
```rust
pub struct User {
    pub id: u64,
    pub email: String,
}

// Manual addition
impl User {
    pub fn validate_email(&self) -> bool {
        self.email.contains('@')
    }
}
```

### 3. Update Template and Regenerate

Update your template to add a new field:

**templates/user.tera**
```rust
pub struct {{ name }} {
    pub id: {{ id_type }},
    pub email: String,
    pub username: String,  // New field
}
```

Before regenerating, check for conflicts:

```bash
rust-patchs-templates check \
  --template templates/user.tera \
  --context contexts/user.json \
  --existing src/user.rs
```

Show what will change:

```bash
rust-patchs-templates diff \
  --template templates/user.tera \
  --context contexts/user.json \
  --existing src/user.rs
```

Regenerate with manual strategy to preserve your edits:

```bash
rust-patchs-templates generate \
  --template templates/user.tera \
  --context contexts/user.json \
  --output src/user.rs \
  --strategy manual
```

## Merge Strategies

### Manual Strategy (Recommended)

Preserves manual edits when there are conflicts:

```bash
--strategy manual
```

- ✅ Keeps manual additions
- ✅ Skips template updates for manually modified items
- ✅ Reports conflicts as warnings

### Template Strategy

Prefers template-generated code:

```bash
--strategy template
```

- ⚠️ Overwrites manual edits with template code
- ✅ Ensures code matches template
- ⚠️ Reports conflicts as warnings

### Fail Strategy

Fails on conflicts, requiring explicit resolution:

```bash
--strategy fail
```

- 🛑 Exits with error on conflicts
- ✅ Forces you to resolve conflicts manually
- ✅ Safest option for critical code

## Advanced Usage

### Working with Multiple Templates

Organize templates by purpose:

```
templates/
  ├── models/
  │   ├── user.tera
  │   └── post.tera
  ├── handlers/
  │   ├── auth.tera
  │   └── api.tera
  └── tests/
      └── integration.tera
```

### Dynamic Context Data

Generate context from external sources:

```bash
# From database schema
./scripts/schema-to-json.sh > contexts/models.json

# Generate models
rust-patchs-templates generate \
  --template templates/models/entity.tera \
  --context contexts/models.json \
  --output src/models.rs
```

### Integration with Build Scripts

Add to `build.rs`:

```rust
use std::process::Command;

fn main() {
    Command::new("rust-patchs-templates")
        .args(&["generate", 
                "--template", "templates/config.tera",
                "--context", "config.json",
                "--output", "src/generated/config.rs",
                "--strategy", "template"])
        .status()
        .expect("Failed to generate code");
}
```

### CI/CD Integration

Check for conflicts in CI:

```yaml
# .github/workflows/check.yml
- name: Check for template conflicts
  run: |
    rust-patchs-templates check \
      --template templates/api.tera \
      --context contexts/api.json \
      --existing src/api.rs
```

## Best Practices

### 1. Version Control Templates

Commit both templates and context files:

```
git add templates/ contexts/
git commit -m "Add user model template"
```

### 2. Document Template Variables

Add comments in templates:

```rust
// Template variables:
// - name: struct name
// - fields: array of {name, type, doc}
pub struct {{ name }} {
    {% for field in fields %}
    /// {{ field.doc }}
    pub {{ field.name }}: {{ field.type }},
    {% endfor %}
}
```

### 3. Use Descriptive Context Files

Name context files to match their purpose:

```
contexts/
  ├── user-model.json
  ├── auth-handler.json
  └── test-data.json
```

### 4. Keep Templates Modular

Create small, focused templates:

```
templates/
  ├── struct.tera          # Just the struct
  ├── impl-new.tera        # Constructor
  ├── impl-getters.tera    # Getters
  └── tests.tera           # Tests
```

### 5. Test Generated Code

Always verify generated code compiles:

```bash
rust-patchs-templates generate \
  --template templates/model.tera \
  --context contexts/model.json \
  --output src/model.rs

cargo check
```

## Troubleshooting

### Parse Errors

If you get parse errors, ensure your template generates valid Rust:

```bash
# Test template output
rust-patchs-templates generate \
  --template templates/test.tera \
  --context contexts/test.json \
  --output /tmp/test.rs

# Verify it parses
rustc --crate-type lib /tmp/test.rs
```

### Merge Conflicts

When merge conflicts occur:

1. Check the diff:
   ```bash
   rust-patchs-templates diff --template ... --existing ... --verbose
   ```

2. Decide on strategy:
   - `manual`: Keep your changes
   - `template`: Use template version
   - `fail`: Resolve manually

3. Apply the appropriate strategy:
   ```bash
   rust-patchs-templates generate --strategy manual ...
   ```

### Template Errors

Debug template rendering:

1. Simplify the template
2. Check JSON context is valid
3. Use Tera playground for testing
4. Add debug output in template:
   ```
   {{ debug() }}
   ```

## Examples

See the `examples/` directory for:

- `simple_struct.tera`: Basic struct generation
- `api_handler.tera`: REST API handler template
- `person_with_validation.tera`: Struct with validation functions

Each example includes:
- Template file (`.tera`)
- Context data (`.json`)
- Sample output (`.rs`)

Run examples:

```bash
# Generate person struct
cargo run -- generate \
  --template examples/templates/simple_struct.tera \
  --context examples/contexts/simple_struct.json \
  --output examples/output/person.rs

# Generate API handler
cargo run -- generate \
  --template examples/templates/api_handler.tera \
  --context examples/contexts/api_handler.json \
  --output examples/output/auth_handler.rs
```
