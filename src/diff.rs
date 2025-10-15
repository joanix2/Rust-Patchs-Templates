//! Diff computation module using Patch Theory
//!
//! This module implements structural diffing for Rust ASTs, computing minimal
//! changes between two versions of code without relying on text markers.

use anyhow::Result;
use similar::{ChangeTag, TextDiff};
use syn::Item;

/// Represents a change operation in the patch
#[derive(Debug, Clone, PartialEq)]
pub enum PatchOp {
    /// Insert a new item
    Insert { name: String, item: Item },
    /// Delete an existing item
    Delete { name: String },
    /// Modify an existing item
    Modify { name: String, old_item: Item, new_item: Item },
    /// Keep an item unchanged
    Keep { name: String },
}

/// A patch is a sequence of operations
#[derive(Debug, Clone)]
pub struct Patch {
    pub operations: Vec<PatchOp>,
}

impl Patch {
    /// Create a new empty patch
    pub fn new() -> Self {
        Patch {
            operations: Vec::new(),
        }
    }
    
    /// Add an operation to the patch
    pub fn add_operation(&mut self, op: PatchOp) {
        self.operations.push(op);
    }
    
    /// Check if the patch is empty (no changes)
    pub fn is_empty(&self) -> bool {
        self.operations.iter().all(|op| matches!(op, PatchOp::Keep { .. }))
    }
}

impl Default for Patch {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute a structural diff between two ASTs
pub fn compute_patch(old_items: &[Item], new_items: &[Item]) -> Result<Patch> {
    let mut patch = Patch::new();
    
    // Extract item names for comparison
    let old_names: Vec<String> = old_items.iter()
        .filter_map(|item| extract_item_name(item))
        .collect();
    
    let _new_names: Vec<String> = new_items.iter()
        .filter_map(|item| extract_item_name(item))
        .collect();
    
    // Track which old items have been processed
    let mut processed_old = vec![false; old_items.len()];
    
    // Process new items
    for (_new_idx, new_item) in new_items.iter().enumerate() {
        let new_name = extract_item_name(new_item);
        
        if let Some(name) = new_name {
            // Find matching item in old items
            if let Some(old_idx) = old_names.iter().position(|n| n == &name) {
                processed_old[old_idx] = true;
                
                // Compare items to see if they've changed
                let old_item = &old_items[old_idx];
                let old_code = quote::quote!(#old_item).to_string();
                let new_code = quote::quote!(#new_item).to_string();
                
                if old_code != new_code {
                    patch.add_operation(PatchOp::Modify {
                        name: name.clone(),
                        old_item: old_item.clone(),
                        new_item: new_item.clone(),
                    });
                } else {
                    patch.add_operation(PatchOp::Keep { name: name.clone() });
                }
            } else {
                // New item - insert
                patch.add_operation(PatchOp::Insert {
                    name: name.clone(),
                    item: new_item.clone(),
                });
            }
        }
    }
    
    // Process deleted items (old items not found in new items)
    for (old_idx, old_item) in old_items.iter().enumerate() {
        if !processed_old[old_idx] {
            if let Some(name) = extract_item_name(old_item) {
                patch.add_operation(PatchOp::Delete { name });
            }
        }
    }
    
    Ok(patch)
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

/// Compute line-based text diff for display purposes
pub fn compute_text_diff(old_text: &str, new_text: &str) -> String {
    let diff = TextDiff::from_lines(old_text, new_text);
    let mut result = String::new();
    
    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        result.push_str(&format!("{}{}", sign, change));
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_compute_patch_insert() {
        let old_items: Vec<Item> = vec![];
        let new_items: Vec<Item> = vec![
            parse_quote! { fn hello() {} }
        ];
        
        let patch = compute_patch(&old_items, &new_items).unwrap();
        assert_eq!(patch.operations.len(), 1);
        assert!(matches!(patch.operations[0], PatchOp::Insert { .. }));
    }
    
    #[test]
    fn test_compute_patch_delete() {
        let old_items: Vec<Item> = vec![
            parse_quote! { fn hello() {} }
        ];
        let new_items: Vec<Item> = vec![];
        
        let patch = compute_patch(&old_items, &new_items).unwrap();
        assert_eq!(patch.operations.len(), 1);
        assert!(matches!(patch.operations[0], PatchOp::Delete { .. }));
    }
    
    #[test]
    fn test_compute_patch_keep() {
        let old_items: Vec<Item> = vec![
            parse_quote! { fn hello() {} }
        ];
        let new_items: Vec<Item> = vec![
            parse_quote! { fn hello() {} }
        ];
        
        let patch = compute_patch(&old_items, &new_items).unwrap();
        assert_eq!(patch.operations.len(), 1);
        assert!(matches!(patch.operations[0], PatchOp::Keep { .. }));
    }
}
