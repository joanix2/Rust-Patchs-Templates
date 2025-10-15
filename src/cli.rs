//! CLI module using clap
//!
//! This module defines the command-line interface with generate, diff, and check commands.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Rust code generator with AST-based intelligent merging
#[derive(Parser, Debug)]
#[command(name = "rust-patchs-templates")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate code from templates
    Generate {
        /// Path to template file or directory
        #[arg(short, long)]
        template: PathBuf,
        
        /// Path to context data file (JSON)
        #[arg(short, long)]
        context: PathBuf,
        
        /// Output file path
        #[arg(short, long)]
        output: PathBuf,
        
        /// Merge strategy: template, manual, or fail
        #[arg(short, long, default_value = "manual")]
        strategy: String,
    },
    
    /// Show diff between generated code and existing file
    Diff {
        /// Path to template file or directory
        #[arg(short, long)]
        template: PathBuf,
        
        /// Path to context data file (JSON)
        #[arg(short, long)]
        context: PathBuf,
        
        /// Existing file to compare against
        #[arg(short, long)]
        existing: PathBuf,
        
        /// Show detailed AST-level diff
        #[arg(short, long, default_value = "false")]
        verbose: bool,
    },
    
    /// Check for conflicts without applying changes
    Check {
        /// Path to template file or directory
        #[arg(short, long)]
        template: PathBuf,
        
        /// Path to context data file (JSON)
        #[arg(short, long)]
        context: PathBuf,
        
        /// Existing file to check
        #[arg(short, long)]
        existing: PathBuf,
    },
}

impl Commands {
    /// Get the merge strategy from string
    pub fn parse_strategy(strategy: &str) -> crate::merger::MergeStrategy {
        match strategy.to_lowercase().as_str() {
            "template" => crate::merger::MergeStrategy::PreferTemplate,
            "manual" => crate::merger::MergeStrategy::PreferManual,
            "fail" => crate::merger::MergeStrategy::FailOnConflict,
            _ => crate::merger::MergeStrategy::PreferManual,
        }
    }
}
