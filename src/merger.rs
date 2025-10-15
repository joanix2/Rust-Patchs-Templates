//! AST merging module
//!
//! This module implements intelligent merging of AST changes, preserving
//! manual edits while applying template-generated updates.

use crate::diff::{Patch, PatchOp};
use anyhow::Result;
use syn::{File, Item};

/// Merge strategy for handling conflicts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MergeStrategy {
    /// Prefer template-generated code
    PreferTemplate,
    /// Prefer manual edits
    PreferManual,
    /// Fail on conflicts
    FailOnConflict,
}

/// Result of a merge operation
#[derive(Debug)]
pub struct MergeResult {
    pub merged_items: Vec<Item>,
    pub conflicts: Vec<String>,
}

/// Merge changes from a patch into existing items
pub fn merge_patch(
    base_items: &[Item],
    patch: &Patch,
    strategy: MergeStrategy,
) -> Result<MergeResult> {
    let mut merged_items = Vec::new();
    let mut conflicts = Vec::new();
    let mut base_map: std::collections::HashMap<String, Item> = base_items
        .iter()
        .filter_map(|item| extract_item_name(item).map(|name| (name, item.clone())))
        .collect();

    // Apply patch operations
    for op in &patch.operations {
        match op {
            PatchOp::Insert { name, item } => {
                // Check if item already exists (manual addition)
                if base_map.contains_key(name) {
                    match strategy {
                        MergeStrategy::PreferTemplate => {
                            merged_items.push(item.clone());
                            base_map.remove(name);
                        }
                        MergeStrategy::PreferManual => {
                            if let Some(base_item) = base_map.remove(name) {
                                merged_items.push(base_item);
                            }
                            conflicts
                                .push(format!("Item '{}' exists in both base and patch", name));
                        }
                        MergeStrategy::FailOnConflict => {
                            conflicts.push(format!(
                                "Conflict: Item '{}' exists in both base and patch",
                                name
                            ));
                        }
                    }
                } else {
                    merged_items.push(item.clone());
                }
            }

            PatchOp::Delete { name } => {
                // Check if item still exists and has been modified
                if let Some(base_item) = base_map.get(name) {
                    // Compare with what patch expects to delete
                    match strategy {
                        MergeStrategy::PreferTemplate => {
                            // Remove the item
                            base_map.remove(name);
                        }
                        MergeStrategy::PreferManual => {
                            // Keep the item
                            merged_items.push(base_item.clone());
                            base_map.remove(name);
                            conflicts.push(format!(
                                "Item '{}' was deleted in template but exists in base",
                                name
                            ));
                        }
                        MergeStrategy::FailOnConflict => {
                            conflicts.push(format!(
                                "Conflict: Item '{}' was deleted in template but modified in base",
                                name
                            ));
                        }
                    }
                } else {
                    // Item already deleted - no action needed
                }
            }

            PatchOp::Modify {
                name,
                old_item: _,
                new_item,
            } => {
                // Check if base item differs from old_item (manual modification)
                if let Some(base_item) = base_map.remove(name) {
                    let base_code = quote::quote!(#base_item).to_string();
                    let new_code = quote::quote!(#new_item).to_string();

                    if base_code == new_code {
                        // No manual changes, apply template update
                        merged_items.push(new_item.clone());
                    } else {
                        // Manual changes detected
                        match strategy {
                            MergeStrategy::PreferTemplate => {
                                merged_items.push(new_item.clone());
                                conflicts.push(format!(
                                    "Item '{}' has manual changes, overridden by template",
                                    name
                                ));
                            }
                            MergeStrategy::PreferManual => {
                                merged_items.push(base_item);
                                conflicts.push(format!(
                                    "Item '{}' has manual changes, template update skipped",
                                    name
                                ));
                            }
                            MergeStrategy::FailOnConflict => {
                                conflicts.push(format!("Conflict: Item '{}' has manual changes conflicting with template", name));
                            }
                        }
                    }
                } else {
                    // Item doesn't exist in base - treat as insert
                    merged_items.push(new_item.clone());
                }
            }

            PatchOp::Keep { name } => {
                // Keep existing item if it exists
                if let Some(base_item) = base_map.remove(name) {
                    merged_items.push(base_item);
                }
            }
        }
    }

    // Add any remaining items from base (manual additions)
    for (_name, item) in base_map {
        merged_items.push(item);
    }

    Ok(MergeResult {
        merged_items,
        conflicts,
    })
}

/// Extract the name/identifier from an AST item
fn extract_item_name(item: &Item) -> Option<String> {
    match item {
        Item::Fn(func) => Some(func.sig.ident.to_string()),
        Item::Struct(s) => Some(s.ident.to_string()),
        Item::Enum(e) => Some(e.ident.to_string()),
        Item::Trait(t) => Some(t.ident.to_string()),
        Item::Type(t) => Some(t.ident.to_string()),
        Item::Const(c) => Some(c.ident.to_string()),
        Item::Static(s) => Some(s.ident.to_string()),
        Item::Mod(m) => Some(m.ident.to_string()),
        _ => None,
    }
}

/// Format merged items back into a complete Rust file
pub fn format_merged_code(merged_items: Vec<Item>) -> Result<String> {
    let file = File {
        shebang: None,
        attrs: Vec::new(),
        items: merged_items,
    };

    let syntax_tree = syn::parse_quote!(#file);
    let formatted = prettyplease::unparse(&syntax_tree);

    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff::compute_patch;
    use syn::parse_quote;

    #[test]
    fn test_merge_insert() {
        let base_items: Vec<Item> = vec![];
        let new_items: Vec<Item> = vec![parse_quote! { fn hello() {} }];

        let patch = compute_patch(&base_items, &new_items).unwrap();
        let result = merge_patch(&base_items, &patch, MergeStrategy::PreferTemplate).unwrap();

        assert_eq!(result.merged_items.len(), 1);
        assert_eq!(result.conflicts.len(), 0);
    }

    #[test]
    fn test_merge_keep_manual_additions() {
        let base_items: Vec<Item> = vec![parse_quote! { fn manual_fn() {} }];
        let new_items: Vec<Item> = vec![parse_quote! { fn template_fn() {} }];

        let patch = compute_patch(&[], &new_items).unwrap();
        let result = merge_patch(&base_items, &patch, MergeStrategy::PreferManual).unwrap();

        // Should have both manual and template functions
        assert_eq!(result.merged_items.len(), 2);
    }
}
