//! AST parsing module for Rust code
//!
//! This module provides functionality to parse Rust source code into an AST
//! and extract meaningful structural information for comparison and merging.

use anyhow::{Context, Result};
use syn::{File, Item};

/// Parsed Rust file with AST representation
#[derive(Debug, Clone)]
pub struct ParsedFile {
    pub syntax_tree: File,
    pub items: Vec<Item>,
}

impl ParsedFile {
    /// Parse Rust source code into a structured AST
    pub fn parse(source: &str) -> Result<Self> {
        let syntax_tree = syn::parse_file(source).context("Failed to parse Rust source code")?;

        let items = syntax_tree.items.clone();

        Ok(ParsedFile { syntax_tree, items })
    }

    /// Get item by identifier (function name, struct name, etc.)
    pub fn find_item(&self, name: &str) -> Option<&Item> {
        self.items.iter().find(|item| match item {
            Item::Fn(func) => func.sig.ident == name,
            Item::Struct(s) => s.ident == name,
            Item::Enum(e) => e.ident == name,
            Item::Trait(t) => t.ident == name,
            Item::Type(t) => t.ident == name,
            Item::Const(c) => c.ident == name,
            Item::Static(s) => s.ident == name,
            Item::Impl(i) => {
                if let Some((_, path, _)) = &i.trait_ {
                    path.segments
                        .last()
                        .map(|s| s.ident == name)
                        .unwrap_or(false)
                } else {
                    false
                }
            }
            _ => false,
        })
    }

    /// Extract all item identifiers
    pub fn get_item_names(&self) -> Vec<String> {
        self.items
            .iter()
            .filter_map(|item| match item {
                Item::Fn(func) => Some(func.sig.ident.to_string()),
                Item::Struct(s) => Some(s.ident.to_string()),
                Item::Enum(e) => Some(e.ident.to_string()),
                Item::Trait(t) => Some(t.ident.to_string()),
                Item::Type(t) => Some(t.ident.to_string()),
                Item::Const(c) => Some(c.ident.to_string()),
                Item::Static(s) => Some(s.ident.to_string()),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let code = r#"
            fn hello() {
                println!("Hello, world!");
            }
        "#;

        let parsed = ParsedFile::parse(code).unwrap();
        assert_eq!(parsed.items.len(), 1);
        assert!(parsed.find_item("hello").is_some());
    }

    #[test]
    fn test_get_item_names() {
        let code = r#"
            fn foo() {}
            struct Bar {}
            enum Baz {}
        "#;

        let parsed = ParsedFile::parse(code).unwrap();
        let names = parsed.get_item_names();
        assert_eq!(names, vec!["foo", "Bar", "Baz"]);
    }
}
