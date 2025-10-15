//! Template engine module
//!
//! This module provides integration with Tera templates for code generation.

use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tera::{Context as TeraContext, Tera};

/// Template engine for generating code
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Create a new template engine from a templates directory
    pub fn new(template_dir: &Path) -> Result<Self> {
        let pattern = template_dir
            .join("**/*.tera")
            .to_str()
            .context("Invalid template directory path")?
            .to_string();

        let tera = Tera::new(&pattern).context("Failed to initialize Tera template engine")?;

        Ok(TemplateEngine { tera })
    }

    /// Create a template engine from a single template string
    pub fn from_string(name: &str, template: &str) -> Result<Self> {
        let mut tera = Tera::default();
        tera.add_raw_template(name, template)
            .context("Failed to add template")?;

        Ok(TemplateEngine { tera })
    }

    /// Render a template with the given context data
    pub fn render(&self, template_name: &str, context: &HashMap<String, Value>) -> Result<String> {
        let mut tera_context = TeraContext::new();

        for (key, value) in context {
            tera_context.insert(key, value);
        }

        self.tera
            .render(template_name, &tera_context)
            .context("Failed to render template")
    }

    /// Get list of available templates
    pub fn get_template_names(&self) -> Vec<&str> {
        self.tera.get_template_names().collect()
    }
}

/// Create a simple context from key-value pairs
pub fn create_context(pairs: Vec<(&str, Value)>) -> HashMap<String, Value> {
    pairs.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_render_simple_template() {
        let template = r#"
fn {{ function_name }}() {
    println!("Hello, {{ name }}!");
}
"#;

        let engine = TemplateEngine::from_string("test", template).unwrap();
        let context = create_context(vec![
            ("function_name", json!("greet")),
            ("name", json!("World")),
        ]);

        let result = engine.render("test", &context).unwrap();
        assert!(result.contains("fn greet()"));
        assert!(result.contains("Hello, World!"));
    }

    #[test]
    fn test_render_struct_template() {
        let template = r#"
/// {{ doc }}
pub struct {{ struct_name }} {
    {% for field in fields %}
    pub {{ field.name }}: {{ field.type }},
    {% endfor %}
}
"#;

        let engine = TemplateEngine::from_string("struct", template).unwrap();
        let context = create_context(vec![
            ("struct_name", json!("Person")),
            ("doc", json!("A person struct")),
            (
                "fields",
                json!([
                    {"name": "name", "type": "String"},
                    {"name": "age", "type": "u32"},
                ]),
            ),
        ]);

        let result = engine.render("struct", &context).unwrap();
        assert!(result.contains("pub struct Person"));
        assert!(result.contains("pub name: String"));
        assert!(result.contains("pub age: u32"));
    }
}
