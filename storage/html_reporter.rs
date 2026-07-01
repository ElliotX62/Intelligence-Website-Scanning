// storage/html_reporter.rs
// IWS v1.0 - HTML Reporter
// Menghasilkan laporan HTML interaktif dengan Chart.js dan DataTable

use std::collections::HashMap;
use handlebars::Handlebars;

#[derive(Debug, Clone)]
pub struct HtmlReporter {
    engine: Handlebars<'static>,
    template: String,
}

#[derive(Debug, Clone)]
pub struct ChartConfig {
    pub chart_type: String,
    pub title: String,
    pub labels: Vec<String>,
    pub datasets: Vec<ChartDataset>,
}

#[derive(Debug, Clone)]
pub struct ChartDataset {
    pub label: String,
    pub data: Vec<f64>,
    pub background_color: Vec<String>,
    pub border_color: String,
}

#[derive(Debug, Clone)]
pub struct TableData {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub sortable: bool,
    pub filterable: bool,
}

impl HtmlReporter {
    pub fn new() -> Self {
        let mut engine = Handlebars::new();
        engine.register_template_string("report", include_str!("../../templates/report.hbs"))
            .unwrap_or_else(|_| engine.register_template_string("report", "<html><body><h1>{{title}}</h1></body></html>").unwrap());
        HtmlReporter { engine, template: "report".to_string() }
    }

    pub fn set_template(&mut self, name: &str, content: &str) -> Result<(), String> {
        self.engine.register_template_string(name, content)
            .map_err(|e| format!("Template error: {}", e))?;
        self.template = name.to_string();
        Ok(())
    }

    pub fn generate(&self, title: &str, charts: &[ChartConfig], tables: &[TableData], summary: &str) -> Result<String, String> {
        let mut data = HashMap::new();
        data.insert("title".to_string(), serde_json::json!(title));
        data.insert("charts_json".to_string(), serde_json::json!(charts));
        data.insert("tables_json".to_string(), serde_json::json!(tables));
        data.insert("summary".to_string(), serde_json::json!(summary));
        data.insert("generated_at".to_string(), serde_json::json!(chrono::Utc::now().to_rfc3339()));

        self.engine.render(&self.template, &data)
            .map_err(|e| format!("Render error: {}", e))
    }

    pub fn export(&self, title: &str, charts: &[ChartConfig], tables: &[TableData], summary: &str, path: &str) -> Result<(), String> {
        let html = self.generate(title, charts, tables, summary)?;
        std::fs::write(path, html).map_err(|e| format!("Write error: {}", e))
    }

    // Predefined chart builders
    pub fn severity_chart(data: &HashMap<String, usize>) -> ChartConfig {
        let labels: Vec<String> = data.keys().cloned().collect();
        let values: Vec<f64> = data.values().map(|&v| v as f64).collect();
        let colors = vec![
            "#FF0000".into(), "#FF6600".into(), "#FFCC00".into(),
            "#00CC00".into(), "#999999".into(),
        ];

        ChartConfig {
            chart_type: "bar".into(),
            title: "Severity Distribution".into(),
            labels,
            datasets: vec![ChartDataset {
                label: "Count".into(), data: values,
                background_color: colors, border_color: "#333".into(),
            }],
        }
    }

    pub fn findings_table(findings: &[HashMap<String, String>]) -> TableData {
        let columns = vec!["Title".into(), "Severity".into(), "Type".into(), "Confidence".into()];
        let rows: Vec<Vec<String>> = findings.iter().map(|f| {
            vec![
                f.get("title").cloned().unwrap_or_default(),
                f.get("severity").cloned().unwrap_or_default(),
                f.get("type").cloned().unwrap_or_default(),
                f.get("confidence").cloned().unwrap_or_default(),
            ]
        }).collect();

        TableData { columns, rows, sortable: true, filterable: true }
    }
}

impl Default for HtmlReporter {
    fn default() -> Self { HtmlReporter::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_basic_html() {
        let reporter = HtmlReporter::new();
        let result = reporter.generate("Test", &[], &[], "No findings");
        assert!(result.is_ok());
        let html = result.unwrap();
        assert!(html.contains("Test"));
    }

    #[test]
    fn test_severity_chart() {
        let mut data = HashMap::new();
        data.insert("Critical".into(), 5);
        data.insert("High".into(), 10);
        let chart = HtmlReporter::severity_chart(&data);
        assert_eq!(chart.chart_type, "bar");
        assert_eq!(chart.labels.len(), 2);
    }

    #[test]
    fn test_findings_table() {
        let findings = vec![{
            let mut m = HashMap::new();
            m.insert("title".into(), "XSS Found".into());
            m.insert("severity".into(), "high".into());
            m.insert("type".into(), "vulnerability".into());
            m.insert("confidence".into(), "0.9".into());
            m
        }];
        let table = HtmlReporter::findings_table(&findings);
        assert_eq!(table.columns.len(), 4);
        assert_eq!(table.rows.len(), 1);
    }
}
