//! Code generator module
//!
//! This module orchestrates the entire code generation workflow,
//! from template rendering to AST merging and formatting.

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::ast_parser::ParsedFile;
use crate::diff::compute_patch;
use crate::merger::{format_merged_code, merge_patch, MergeStrategy};
use crate::template::TemplateEngine;

/// Generate code from template and merge with existing file if present
pub fn generate(
    template_path: &Path,
    context_data: HashMap<String, Value>,
    output_path: &Path,
    strategy: MergeStrategy,
) -> Result<String> {
    // Load and render template
    let template_name = template_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid template file name")?;

    let template_content =
        fs::read_to_string(template_path).context("Failed to read template file")?;

    let engine = TemplateEngine::from_string(template_name, &template_content)?;
    let generated_code = engine.render(template_name, &context_data)?;

    // Parse generated code
    let generated_ast =
        ParsedFile::parse(&generated_code).context("Failed to parse generated code")?;

    // Check if output file exists
    let merged_code = if output_path.exists() {
        // Read and parse existing file
        let existing_code =
            fs::read_to_string(output_path).context("Failed to read existing file")?;

        let existing_ast =
            ParsedFile::parse(&existing_code).context("Failed to parse existing file")?;

        // Compute patch
        let patch = compute_patch(&existing_ast.items, &generated_ast.items)?;

        // Merge changes
        let merge_result = merge_patch(&existing_ast.items, &patch, strategy)?;

        if !merge_result.conflicts.is_empty() && strategy == MergeStrategy::FailOnConflict {
            anyhow::bail!(
                "Merge conflicts detected:\n{}",
                merge_result.conflicts.join("\n")
            );
        }

        // Print warnings for conflicts
        for conflict in &merge_result.conflicts {
            eprintln!("Warning: {}", conflict);
        }

        format_merged_code(merge_result.merged_items)?
    } else {
        // No existing file, use generated code as-is
        generated_code
    };

    Ok(merged_code)
}

/// Show diff between generated code and existing file
pub fn show_diff(
    template_path: &Path,
    context_data: HashMap<String, Value>,
    existing_path: &Path,
    verbose: bool,
) -> Result<String> {
    // Load and render template
    let template_name = template_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid template file name")?;

    let template_content =
        fs::read_to_string(template_path).context("Failed to read template file")?;

    let engine = TemplateEngine::from_string(template_name, &template_content)?;
    let generated_code = engine.render(template_name, &context_data)?;

    // Parse generated code
    let generated_ast = ParsedFile::parse(&generated_code)?;

    // Read and parse existing file
    let existing_code =
        fs::read_to_string(existing_path).context("Failed to read existing file")?;

    let existing_ast = ParsedFile::parse(&existing_code)?;

    // Compute patch
    let patch = compute_patch(&existing_ast.items, &generated_ast.items)?;

    if patch.is_empty() {
        return Ok("No differences found.".to_string());
    }

    // Format diff output
    let mut diff_output = String::new();

    if verbose {
        // Show detailed AST-level diff
        for op in &patch.operations {
            diff_output.push_str(&format!("{:?}\n", op));
        }
    } else {
        // Show text-based diff
        diff_output = crate::diff::compute_text_diff(&existing_code, &generated_code);
    }

    Ok(diff_output)
}

/// Check for conflicts without applying changes
pub fn check_conflicts(
    template_path: &Path,
    context_data: HashMap<String, Value>,
    existing_path: &Path,
) -> Result<Vec<String>> {
    // Load and render template
    let template_name = template_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid template file name")?;

    let template_content =
        fs::read_to_string(template_path).context("Failed to read template file")?;

    let engine = TemplateEngine::from_string(template_name, &template_content)?;
    let generated_code = engine.render(template_name, &context_data)?;

    // Parse generated code
    let generated_ast = ParsedFile::parse(&generated_code)?;

    // Read and parse existing file
    let existing_code =
        fs::read_to_string(existing_path).context("Failed to read existing file")?;

    let existing_ast = ParsedFile::parse(&existing_code)?;

    // Compute patch
    let patch = compute_patch(&existing_ast.items, &generated_ast.items)?;

    // Try merge with FailOnConflict strategy
    let merge_result = merge_patch(&existing_ast.items, &patch, MergeStrategy::FailOnConflict)?;

    Ok(merge_result.conflicts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_new_file() -> Result<()> {
        let mut template_file = NamedTempFile::new()?;
        writeln!(template_file, "fn {{{{ name }}}}() {{}}")?;

        let context = [("name".to_string(), json!("test_fn"))]
            .iter()
            .cloned()
            .collect();

        let output_path = Path::new("/tmp/test_output.rs");
        let result = generate(
            template_file.path(),
            context,
            output_path,
            MergeStrategy::PreferManual,
        )?;

        assert!(result.contains("fn test_fn"));
        Ok(())
    }
}
