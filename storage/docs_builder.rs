// storage/docs_builder.rs
// IWS v1.0 - Docs Builder
// Membangun dokumentasi lengkap dengan template engine dan formatting

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use handlebars::Handlebars;

#[derive(Debug, Clone)]
pub struct DocsBuilder {
    engine: Handlebars<'static>,
    template: String,
    variables: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSection {
    pub title: String,
    pub content: String,
    pub level: u8,
    pub subsections: Vec<DocSection>,
}

impl DocsBuilder {
    pub fn new(template: &str) -> Self {
        DocsBuilder {
            engine: Handlebars::new(),
            template: template.to_string(),
            variables: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, key: &str, value: serde_json::Value) {
        self.variables.insert(key.to_string(), value);
    }

    pub fn set_template(&mut self, template: &str) {
        self.template = template.to_string();
    }

    pub fn build(&self) -> Result<String, String> {
        self.engine
            .render_template(&self.template, &self.variables)
            .map_err(|e| format!("Template render error: {}", e))
    }

    pub fn table_of_contents(sections: &[DocSection]) -> String {
        let mut toc = String::from("## Table of Contents\n\n");
        for section in sections {
            toc.push_str(&format!("- {}\n", section.title));
            for sub in &section.subsections {
                toc.push_str(&format!("  - {}\n", sub.title));
            }
        }
        toc
    }

    pub fn cover_page(title: &str, author: &str, date: &str, version: &str) -> String {
        format!(
            "{}\n\nTitle: {}\nAuthor: {}\nDate: {}\nVersion: {}\n\n{}\n",
            "=".repeat(60), title, author, date, version, "=".repeat(60)
        )
    }

    pub fn section(title: &str, level: u8) -> String {
        let prefix = match level {
            1 => "# ", 2 => "## ", 3 => "### ", _ => "#### ",
        };
        format!("{}{}\n", prefix, title)
    }

    pub fn reference_list(refs: &[String]) -> String {
        let mut list = String::from("## References\n\n");
        for (i, r) in refs.iter().enumerate() {
            list.push_str(&format!("{}. {}\n", i + 1, r));
        }
        list
    }

    pub fn export_markdown(content: &str, path: &str) -> Result<(), String> {
        std::fs::write(path, content).map_err(|e| format!("Write error: {}", e))
    }
}

impl Default for DocsBuilder {
    fn default() -> Self {
        DocsBuilder::new("default")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_substitution() {
        let mut builder = DocsBuilder::new("Hello {{name}}!");
        builder.set_variable("name", serde_json::json!("World"));
        let result = builder.build().unwrap();
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_table_of_contents() {
        let sections = vec![DocSection {
            title: "Overview".into(), content: "".into(), level: 1,
            subsections: vec![DocSection {
                title: "Setup".into(), content: "".into(), level: 2, subsections: vec![],
            }],
        }];
        let toc = DocsBuilder::table_of_contents(&sections);
        assert!(toc.contains("Overview"));
        assert!(toc.contains("Setup"));
    }

    #[test]
    fn test_cover_page() {
        let cover = DocsBuilder::cover_page("IWS Report", "Admin", "2024-01-01", "1.0");
        assert!(cover.contains("IWS Report"));
        assert!(cover.contains("Admin"));
    }

    #[test]
    fn test_section_header() {
        assert_eq!(DocsBuilder::section("Intro", 1), "# Intro\n");
        assert_eq!(DocsBuilder::section("Details", 2), "## Details\n");
    }
}
