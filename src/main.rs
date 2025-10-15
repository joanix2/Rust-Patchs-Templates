//! Rust code generator with AST-based intelligent merging
//!
//! This tool generates Rust code from Tera templates and intelligently merges
//! changes with existing files using AST-based patch theory, preserving manual
//! edits while applying template updates.

mod ast_parser;
mod cli;
mod diff;
mod generator;
mod merger;
mod template;

use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::Path;

use cli::{Cli, Commands};
use generator::{check_conflicts, generate, show_diff};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            template,
            context,
            output,
            strategy,
        } => {
            // Load context data
            let context_data = load_context_data(&context)?;

            // Parse strategy
            let merge_strategy = Commands::parse_strategy(&strategy);

            // Generate code
            let result = generate(&template, context_data, &output, merge_strategy)?;

            // Write output
            fs::write(&output, result)?;

            println!("✓ Generated code written to: {}", output.display());
        }

        Commands::Diff {
            template,
            context,
            existing,
            verbose,
        } => {
            // Load context data
            let context_data = load_context_data(&context)?;

            // Show diff
            let diff = show_diff(&template, context_data, &existing, verbose)?;

            println!("{}", diff);
        }

        Commands::Check {
            template,
            context,
            existing,
        } => {
            // Load context data
            let context_data = load_context_data(&context)?;

            // Check for conflicts
            let conflicts = check_conflicts(&template, context_data, &existing)?;

            if conflicts.is_empty() {
                println!("✓ No conflicts detected. Safe to merge.");
            } else {
                println!("⚠ Conflicts detected:");
                for conflict in conflicts {
                    println!("  - {}", conflict);
                }
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

/// Load context data from JSON file
fn load_context_data(path: &Path) -> Result<std::collections::HashMap<String, serde_json::Value>> {
    let content = fs::read_to_string(path)?;
    let value: serde_json::Value = serde_json::from_str(&content)?;

    if let serde_json::Value::Object(map) = value {
        Ok(map.into_iter().collect())
    } else {
        anyhow::bail!("Context file must contain a JSON object")
    }
}
