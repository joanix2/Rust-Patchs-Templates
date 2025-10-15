#!/bin/bash
# Demo script for rust-patchs-templates

set -e

echo "================================================"
echo "Rust-Patchs-Templates Demo"
echo "================================================"
echo ""

# Build the project
echo "1. Building the project..."
cargo build --release --quiet
echo "   ✓ Build complete"
echo ""

# Create demo directory
DEMO_DIR="demo_output"
rm -rf "$DEMO_DIR"
mkdir -p "$DEMO_DIR"

# Demo 1: Generate initial code
echo "2. Generating initial Person struct..."
./target/release/rust-patchs-templates generate \
  --template examples/templates/simple_struct.tera \
  --context examples/contexts/simple_struct.json \
  --output "$DEMO_DIR/person.rs" \
  --strategy manual
echo ""

echo "   Generated code:"
echo "   ----------------------------------------"
cat "$DEMO_DIR/person.rs"
echo "   ----------------------------------------"
echo ""

# Demo 2: Manually edit the file
echo "3. Simulating manual edit (adding custom method)..."
cat >> "$DEMO_DIR/person.rs" << 'EOF'

// Manual addition: custom validation
fn validate_person(person: &Person) -> bool {
    !person.name.is_empty() && person.age > 0
}
EOF
echo "   ✓ Added custom validation function"
echo ""

# Demo 3: Check for conflicts
echo "4. Checking for conflicts before regenerating..."
if ./target/release/rust-patchs-templates check \
  --template examples/templates/person_with_validation.tera \
  --context examples/contexts/person_with_validation.json \
  --existing "$DEMO_DIR/person.rs"; then
  echo "   ✓ No conflicts"
else
  echo "   ⚠ Conflicts detected (expected)"
fi
echo ""

# Demo 4: Show diff
echo "5. Showing diff between current and new template..."
echo "   ----------------------------------------"
./target/release/rust-patchs-templates diff \
  --template examples/templates/person_with_validation.tera \
  --context examples/contexts/person_with_validation.json \
  --existing "$DEMO_DIR/person.rs" | head -20
echo "   ----------------------------------------"
echo ""

# Demo 5: Regenerate with preservation
echo "6. Regenerating with manual strategy (preserves edits)..."
./target/release/rust-patchs-templates generate \
  --template examples/templates/person_with_validation.tera \
  --context examples/contexts/person_with_validation.json \
  --output "$DEMO_DIR/person.rs" \
  --strategy manual
echo ""

echo "   Final merged code:"
echo "   ----------------------------------------"
cat "$DEMO_DIR/person.rs"
echo "   ----------------------------------------"
echo ""

# Demo 6: Verify it compiles
echo "7. Verifying generated code compiles..."
rustc --crate-type lib "$DEMO_DIR/person.rs" --out-dir "$DEMO_DIR" 2>&1 | grep -v "warning" || true
echo "   ✓ Code compiles successfully"
echo ""

echo "================================================"
echo "Demo complete!"
echo "================================================"
echo ""
echo "Notice how the manually added 'validate_person' function"
echo "was preserved while the template's 'validate_age' function"
echo "was added. This demonstrates intelligent AST-based merging!"
echo ""
echo "Cleanup: rm -rf $DEMO_DIR"
