// shared/interfaces/reporter_interface.rs
// IWS v1.0 - Reporter Interface
// Mendefinisikan trait Reporter untuk report generation

use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::shared::contracts::api_contract::ApiContractError;
use crate::shared::types::report_types::{
    ReportFormat, ReportType, ReportData, ReportMetadata,
    ExecutiveSummary, TechnicalDetails, VulnerabilityTrackerData,
    ReportTimeline, ReportFinding, ReportStatistics,
};

// ============================================================
// REPORTER TRAIT
// ============================================================

#[async_trait]
pub trait Reporter: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    /// Generate report dalam format tertentu
    async fn generate(
        &self,
        data: &ReportData,
        format: ReportFormat,
    ) -> Result<Vec<u8>, Self::Error>;

    /// Export report ke path
    async fn export(
        &self,
        data: &ReportData,
        format: ReportFormat,
        output_path: &str,
    ) -> Result<(), Self::Error>;

    /// Validasi template
    fn validate_template(&self, template: &str) -> bool;

    /// Dapatkan supported formats
    fn supported_formats(&self) -> Vec<ReportFormat>;

    /// Set template
    fn set_template(&mut self, template: &str) -> Result<(), Self::Error>;

    /// Dapatkan template saat ini
    fn template(&self) -> &str;

    /// Dapatkan variabel yang tersedia di template
    fn template_variables(&self) -> Vec<String>;

    /// Generate multiple formats sekaligus
    async fn generate_all(
        &self,
        data: &ReportData,
    ) -> Result<HashMap<ReportFormat, Vec<u8>>, Self::Error> {
        let mut results = HashMap::new();
        for format in self.supported_formats() {
            let output = self.generate(data, format.clone()).await?;
            results.insert(format, output);
        }
        Ok(results)
    }

    /// Generate dan export semua format
    async fn export_all(
        &self,
        data: &ReportData,
        output_dir: &str,
    ) -> Result<HashMap<ReportFormat, String>, Self::Error> {
        let mut paths = HashMap::new();
        for format in self.supported_formats() {
            let output = self.generate(data, format.clone()).await?;
            let file_path = format!("{}/report_{}.{}", output_dir, data.report_id, format.extension());
            std::fs::create_dir_all(output_dir).map_err(|e| {
                ApiContractError::InternalError(format!("Failed to create dir: {}", e))
            })?;
            std::fs::write(&file_path, output).map_err(|e| {
                ApiContractError::InternalError(format!("Failed to write: {}", e))
            })?;
            paths.insert(format, file_path);
        }
        Ok(paths)
    }

    /// Generate report dengan custom options
    async fn generate_with_options(
        &self,
        data: &ReportData,
        format: ReportFormat,
        options: &ReportOptions,
    ) -> Result<Vec<u8>, Self::Error> {
        let mut modified_data = data.clone();

        if let Some(ref title) = options.title_override {
            modified_data.title = title.clone();
        }
        if !options.include_charts {
            modified_data.metadata.include_charts = false;
        }
        if !options.include_raw_data {
            modified_data.metadata.include_raw_data = false;
        }
        if let Some(ref watermark) = options.watermark {
            modified_data.metadata.watermark = Some(watermark.clone());
        }
        if let Some(ref language) = options.language {
            modified_data.metadata.language = language.clone();
        }

        self.generate(&modified_data, format).await
    }
}

// ============================================================
// REPORT OPTIONS
// ============================================================

#[derive(Debug, Clone)]
pub struct ReportOptions {
    pub title_override: Option<String>,
    pub include_charts: bool,
    pub include_raw_data: bool,
    pub include_executive_summary: bool,
    pub include_technical_details: bool,
    pub include_vulnerability_tracker: bool,
    pub include_timeline: bool,
    pub watermark: Option<String>,
    pub language: Option<String>,
    pub template: Option<String>,
    pub custom: HashMap<String, String>,
}

impl Default for ReportOptions {
    fn default() -> Self {
        ReportOptions {
            title_override: None,
            include_charts: true,
            include_raw_data: false,
            include_executive_summary: true,
            include_technical_details: true,
            include_vulnerability_tracker: true,
            include_timeline: true,
            watermark: None,
            language: None,
            template: None,
            custom: HashMap::new(),
        }
    }
}

impl ReportOptions {
    pub fn new() -> Self {
        ReportOptions::default()
    }

    pub fn executive_only() -> Self {
        ReportOptions {
            include_technical_details: false,
            include_vulnerability_tracker: false,
            include_timeline: false,
            ..Default::default()
        }
    }

    pub fn technical_only() -> Self {
        ReportOptions {
            include_executive_summary: false,
            include_charts: false,
            ..Default::default()
        }
    }

    pub fn full() -> Self {
        ReportOptions::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title_override = Some(title.to_string());
        self
    }

    pub fn with_watermark(mut self, watermark: &str) -> Self {
        self.watermark = Some(watermark.to_string());
        self
    }

    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_string());
        self
    }

    pub fn with_template(mut self, template: &str) -> Self {
        self.template = Some(template.to_string());
        self
    }

    pub fn without_charts(mut self) -> Self {
        self.include_charts = false;
        self
    }

    pub fn with_raw_data(mut self) -> Self {
        self.include_raw_data = true;
        self
    }
}

// ============================================================
// REPORT BUILDER
// ============================================================

pub struct ReportBuilder {
    report_type: ReportType,
    scan_id: Option<Uuid>,
    target_url: Option<String>,
    title: Option<String>,
    summary: Option<String>,
    executive_summary: Option<ExecutiveSummary>,
    technical_details: Option<TechnicalDetails>,
    vulnerability_tracker: Option<VulnerabilityTrackerData>,
    timeline: Option<ReportTimeline>,
    statistics: Option<ReportStatistics>,
    findings: Vec<ReportFinding>,
    metadata: ReportMetadata,
}

impl ReportBuilder {
    pub fn new(report_type: ReportType) -> Self {
        ReportBuilder {
            report_type,
            scan_id: None,
            target_url: None,
            title: None,
            summary: None,
            executive_summary: None,
            technical_details: None,
            vulnerability_tracker: None,
            timeline: None,
            statistics: None,
            findings: Vec::new(),
            metadata: ReportMetadata::default(),
        }
    }

    pub fn scan_id(mut self, id: Uuid) -> Self {
        self.scan_id = Some(id);
        self
    }

    pub fn target_url(mut self, url: &str) -> Self {
        self.target_url = Some(url.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_string());
        self
    }

    pub fn executive_summary(mut self, summary: ExecutiveSummary) -> Self {
        self.executive_summary = Some(summary);
        self
    }

    pub fn technical_details(mut self, details: TechnicalDetails) -> Self {
        self.technical_details = Some(details);
        self
    }

    pub fn vulnerability_tracker(mut self, tracker: VulnerabilityTrackerData) -> Self {
        self.vulnerability_tracker = Some(tracker);
        self
    }

    pub fn timeline(mut self, timeline: ReportTimeline) -> Self {
        self.timeline = Some(timeline);
        self
    }

    pub fn statistics(mut self, stats: ReportStatistics) -> Self {
        self.statistics = Some(stats);
        self
    }

    pub fn add_finding(mut self, finding: ReportFinding) -> Self {
        self.findings.push(finding);
        self
    }

    pub fn add_findings(mut self, findings: Vec<ReportFinding>) -> Self {
        self.findings.extend(findings);
        self
    }

    pub fn metadata(mut self, metadata: ReportMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn classification(mut self, classification: &str) -> Self {
        self.metadata.classification = classification.to_string();
        self
    }

    pub fn template(mut self, template: &str) -> Self {
        self.metadata.template = template.to_string();
        self
    }

    pub fn build(self) -> ReportData {
        let scan_id = self.scan_id.unwrap_or_else(Uuid::new_v4);
        let target_url = self.target_url.unwrap_or_else(|| "unknown".to_string());
        let title = self.title.unwrap_or_else(|| format!("Security Report - {}", target_url));

        ReportData {
            report_id: Uuid::new_v4(),
            scan_id,
            report_type: self.report_type,
            title,
            target_url,
            scan_date: chrono::Utc::now(),
            generated_by: "IWS".to_string(),
            summary: self.summary,
            executive_summary: self.executive_summary,
            technical_details: self.technical_details,
            vulnerability_tracker: self.vulnerability_tracker,
            timeline: self.timeline,
            statistics: self.statistics,
            findings: self.findings,
            recommendations: vec![],
            metadata: self.metadata,
            created_at: chrono::Utc::now(),
        }
    }
}

// ============================================================
// REPORT TEMPLATE ENGINE
// ============================================================

pub trait TemplateEngine: Send + Sync {
    /// Render template dengan data
    fn render(&self, template: &str, data: &serde_json::Value) -> Result<String, ApiContractError>;

    /// Validasi template syntax
    fn validate_syntax(&self, template: &str) -> Result<(), ApiContractError>;

    /// Dapatkan variabel dari template
    fn extract_variables(&self, template: &str) -> Vec<String>;

    /// Dapatkan nama engine
    fn engine_name(&self) -> &str;

    /// Dapatkan supported template extensions
    fn supported_extensions(&self) -> Vec<String>;
}

// ============================================================
// REPORT SCHEDULER
// ============================================================

#[async_trait]
pub trait ReportScheduler: Send + Sync {
    /// Schedule report generation
    async fn schedule(
        &self,
        scan_id: Uuid,
        format: ReportFormat,
        cron_expression: &str,
        options: ReportOptions,
    ) -> Result<Uuid, ApiContractError>;

    /// Cancel scheduled report
    async fn cancel_schedule(&self, schedule_id: Uuid) -> Result<(), ApiContractError>;

    /// List scheduled reports
    async fn list_schedules(&self) -> Result<Vec<ScheduleEntry>, ApiContractError>;

    /// Dapatkan schedule entry
    async fn get_schedule(&self, schedule_id: Uuid) -> Result<ScheduleEntry, ApiContractError>;
}

#[derive(Debug, Clone)]
pub struct ScheduleEntry {
    pub schedule_id: Uuid,
    pub scan_id: Uuid,
    pub format: ReportFormat,
    pub cron_expression: String,
    pub options: ReportOptions,
    pub enabled: bool,
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
    pub next_run: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub created_by: String,
}

impl ScheduleEntry {
    pub fn new(
        schedule_id: Uuid,
        scan_id: Uuid,
        format: ReportFormat,
        cron_expression: &str,
        created_by: &str,
    ) -> Self {
        ScheduleEntry {
            schedule_id,
            scan_id,
            format,
            cron_expression: cron_expression.to_string(),
            options: ReportOptions::default(),
            enabled: true,
            last_run: None,
            next_run: None,
            created_at: chrono::Utc::now(),
            created_by: created_by.to_string(),
        }
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestReporter {
        template: String,
        formats: Vec<ReportFormat>,
    }

    impl TestReporter {
        fn new() -> Self {
            TestReporter {
                template: "default".to_string(),
                formats: vec![ReportFormat::JSON, ReportFormat::HTML, ReportFormat::PDF],
            }
        }
    }

    #[async_trait]
    impl Reporter for TestReporter {
        type Error = ApiContractError;

        async fn generate(&self, _data: &ReportData, format: ReportFormat) -> Result<Vec<u8>, Self::Error> {
            match format {
                ReportFormat::JSON => Ok(b"{\"report\": true}".to_vec()),
                ReportFormat::HTML => Ok(b"<html>report</html>".to_vec()),
                ReportFormat::PDF => Ok(b"%PDF-report".to_vec()),
                _ => Ok(format!("report.{}", format.extension()).into_bytes()),
            }
        }

        async fn export(&self, data: &ReportData, format: ReportFormat, output_path: &str) -> Result<(), Self::Error> {
            let content = self.generate(data, format).await?;
            std::fs::write(output_path, content).map_err(|e| {
                ApiContractError::InternalError(format!("Write failed: {}", e))
            })?;
            Ok(())
        }

        fn validate_template(&self, template: &str) -> bool {
            !template.is_empty()
        }

        fn supported_formats(&self) -> Vec<ReportFormat> {
            self.formats.clone()
        }

        fn set_template(&mut self, template: &str) -> Result<(), Self::Error> {
            if !self.validate_template(template) {
                return Err(ApiContractError::InvalidRequest("Invalid template".to_string()));
            }
            self.template = template.to_string();
            Ok(())
        }

        fn template(&self) -> &str {
            &self.template
        }

        fn template_variables(&self) -> Vec<String> {
            vec!["title".to_string(), "findings".to_string(), "severity".to_string()]
        }
    }

    fn make_report_data() -> ReportData {
        ReportData {
            report_id: Uuid::new_v4(),
            scan_id: Uuid::new_v4(),
            report_type: ReportType::FullReport,
            title: "Test Report".to_string(),
            target_url: "https://example.com".to_string(),
            scan_date: chrono::Utc::now(),
            generated_by: "IWS".to_string(),
            summary: None,
            executive_summary: None,
            technical_details: None,
            vulnerability_tracker: None,
            timeline: None,
            statistics: None,
            findings: vec![],
            recommendations: vec![],
            metadata: ReportMetadata::default(),
            created_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_reporter_generate_json() {
        let reporter = TestReporter::new();
        let data = make_report_data();
        let result = reporter.generate(&data, ReportFormat::JSON).await.unwrap();
        assert!(result.starts_with(b"{"));
    }

    #[tokio::test]
    async fn test_reporter_generate_html() {
        let reporter = TestReporter::new();
        let data = make_report_data();
        let result = reporter.generate(&data, ReportFormat::HTML).await.unwrap();
        assert!(result.starts_with(b"<html>"));
    }

    #[tokio::test]
    async fn test_reporter_generate_all() {
        let reporter = TestReporter::new();
        let data = make_report_data();
        let results = reporter.generate_all(&data).await.unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.contains_key(&ReportFormat::JSON));
        assert!(results.contains_key(&ReportFormat::HTML));
        assert!(results.contains_key(&ReportFormat::PDF));
    }

    #[tokio::test]
    async fn test_reporter_export() {
        let reporter = TestReporter::new();
        let data = make_report_data();
        let tmp = std::env::temp_dir().join("test_report.json");
        let path = tmp.to_str().unwrap();

        reporter.export(&data, ReportFormat::JSON, path).await.unwrap();
        assert!(tmp.exists());

        let content = std::fs::read_to_string(&tmp).unwrap();
        assert!(content.contains("report"));
        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    fn test_reporter_template_validation() {
        let reporter = TestReporter::new();
        assert!(reporter.validate_template("valid"));
        assert!(!reporter.validate_template(""));
    }

    #[test]
    fn test_reporter_template_variables() {
        let reporter = TestReporter::new();
        let vars = reporter.template_variables();
        assert!(vars.contains(&"title".to_string()));
        assert!(vars.contains(&"findings".to_string()));
    }

    #[test]
    fn test_report_builder() {
        let report = ReportBuilder::new(ReportType::FullReport)
            .scan_id(Uuid::new_v4())
            .target_url("https://example.com")
            .title("Custom Report Title")
            .summary("Executive summary here")
            .classification("confidential")
            .template("professional")
            .add_finding(ReportFinding::new(
                "XSS Vulnerability",
                "vulnerability",
                crate::shared::types::common_types::Severity::High,
                "xss_detector",
            ))
            .build();

        assert_eq!(report.title, "Custom Report Title");
        assert_eq!(report.target_url, "https://example.com");
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.metadata.classification, "confidential");
        assert_eq!(report.metadata.template, "professional");
        assert!(report.summary.is_some());
    }

    #[test]
    fn test_report_options_presets() {
        let exec_only = ReportOptions::executive_only();
        assert!(!exec_only.include_technical_details);
        assert!(exec_only.include_executive_summary);

        let tech_only = ReportOptions::technical_only();
        assert!(tech_only.include_technical_details);
        assert!(!tech_only.include_executive_summary);

        let full = ReportOptions::full();
        assert!(full.include_executive_summary);
        assert!(full.include_technical_details);
    }

    #[test]
    fn test_report_options_builder() {
        let options = ReportOptions::new()
            .with_title("Custom Title")
            .with_watermark("DRAFT")
            .with_language("id")
            .without_charts()
            .with_raw_data();

        assert_eq!(options.title_override, Some("Custom Title".to_string()));
        assert_eq!(options.watermark, Some("DRAFT".to_string()));
        assert_eq!(options.language, Some("id".to_string()));
        assert!(!options.include_charts);
        assert!(options.include_raw_data);
    }

    #[test]
    fn test_schedule_entry() {
        let entry = ScheduleEntry::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ReportFormat::PDF,
            "0 9 * * MON",
            "admin",
        );

        assert_eq!(entry.format, ReportFormat::PDF);
        assert!(entry.enabled);
        assert_eq!(entry.created_by, "admin");
    }
}
