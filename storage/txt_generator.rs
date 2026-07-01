// storage/txt_generator.rs
// IWS v1.0 - Text Generator
// Menghasilkan file teks dengan formatting untuk berbagai keperluan

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TxtGenerator;

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub alignments: Vec<Align>,
}

#[derive(Debug, Clone, Copy)]
pub enum Align { Left, Center, Right }

impl TxtGenerator {
    pub fn heading(text: &str, level: u8) -> String {
        let prefix = match level {
            1 => "=", 2 => "-", _ => "~",
        };
        let line = prefix.repeat(text.len());
        format!("{}\n{}\n", text, line)
    }

    pub fn bullet_list(items: &[String]) -> String {
        items.iter().map(|i| format!("• {}\n", i)).collect()
    }

    pub fn numbered_list(items: &[String]) -> String {
        items.iter().enumerate().map(|(i, item)| format!("{}. {}\n", i + 1, item)).collect()
    }

    pub fn table(config: &TableConfig) -> String {
        let mut widths = vec![0; config.columns.len()];
        for (i, col) in config.columns.iter().enumerate() {
            widths[i] = col.len();
        }
        for row in &config.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() && cell.len() > widths[i] {
                    widths[i] = cell.len();
                }
            }
        }

        let mut output = String::new();

        // Header
        output.push_str(&TxtGenerator::table_row(&config.columns, &widths, &config.alignments));
        output.push_str(&TxtGenerator::table_separator(&widths));

        // Rows
        for row in &config.rows {
            output.push_str(&TxtGenerator::table_row(row, &widths, &config.alignments));
        }

        output
    }

    fn table_row(cells: &[String], widths: &[usize], alignments: &[Align]) -> String {
        let mut row = String::from("| ");
        for (i, cell) in cells.iter().enumerate() {
            let width = widths.get(i).copied().unwrap_or(10);
            let align = alignments.get(i).copied().unwrap_or(Align::Left);
            let padded = match align {
                Align::Left => format!("{:<width$}", cell, width = width),
                Align::Center => format!("{:^width$}", cell, width = width),
                Align::Right => format!("{:>width$}", cell, width = width),
            };
            row.push_str(&padded);
            row.push_str(" | ");
        }
        row.push('\n');
        row
    }

    fn table_separator(widths: &[usize]) -> String {
        let mut sep = String::from("|");
        for w in widths {
            sep.push_str(&format!("{:-<width$}|", "", width = w + 1));
        }
        sep.push('\n');
        sep
    }

    pub fn key_value(data: &HashMap<String, String>) -> String {
        let max_key = data.keys().map(|k| k.len()).max().unwrap_or(0);
        data.iter()
            .map(|(k, v)| format!("{:<width$}: {}\n", k, v, width = max_key))
            .collect()
    }

    pub fn section(title: &str, content: &str) -> String {
        format!("--- {} ---\n{}\n", title, content)
    }

    pub fn code_block(code: &str, language: &str) -> String {
        format!("```{}\n{}\n```\n", language, code)
    }

    pub fn summary(title: &str, items: &[(&str, &str)]) -> String {
        let mut output = TxtGenerator::heading(title, 2);
        for (label, value) in items {
            output.push_str(&format!("  {}: {}\n", label, value));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading() {
        let h = TxtGenerator::heading("Test", 1);
        assert!(h.contains("Test"));
        assert!(h.contains("===="));
    }

    #[test]
    fn test_bullet_list() {
        let items = vec!["one".into(), "two".into()];
        let result = TxtGenerator::bullet_list(&items);
        assert!(result.contains("• one"));
        assert!(result.contains("• two"));
    }

    #[test]
    fn test_numbered_list() {
        let items = vec!["first".into(), "second".into()];
        let result = TxtGenerator::numbered_list(&items);
        assert!(result.contains("1. first"));
        assert!(result.contains("2. second"));
    }

    #[test]
    fn test_table() {
        let config = TableConfig {
            columns: vec!["Name".into(), "Value".into()],
            rows: vec![vec!["Alice".into(), "100".into()]],
            alignments: vec![Align::Left, Align::Right],
        };
        let result = TxtGenerator::table(&config);
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
    }

    #[test]
    fn test_key_value() {
        let mut data = HashMap::new();
        data.insert("host".into(), "localhost".into());
        data.insert("port".into(), "8080".into());
        let result = TxtGenerator::key_value(&data);
        assert!(result.contains("host"));
        assert!(result.contains("8080"));
    }
}
