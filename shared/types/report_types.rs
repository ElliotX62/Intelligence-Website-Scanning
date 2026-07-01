// shared/types/report_types.rs
// IWS v1.0 - Report Types
// Mendefinisikan tipe data untuk report generation

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::common_types::{Severity, Confidence, Priority, GeoLocation, Timestamp};

// ============================================================
// REPORT FORMAT & TYPE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFormat {
    JSON,
    TXT,
    DOCS,
    CSV,
    HTML,
    PDF,
}

impl ReportFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(ReportFormat::JSON),
            "txt" | "text" => Some(ReportFormat::TXT),
            "docs" | "docx" => Some(ReportFormat::DOCS),
            "csv" => Some(ReportFormat::CSV),
            "html" => Some(ReportFormat::HTML),
            "pdf" => Some(ReportFormat::PDF),
            _ => None,
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            ReportFormat::JSON => "json",
            ReportFormat::TXT => "txt",
            ReportFormat::DOCS => "docx",
            ReportFormat::CSV => "csv",
            ReportFormat::HTML => "html",
            ReportFormat::PDF => "pdf",
        }
    }

    pub fn mime_type(&self) -> &str {
        match self {
            ReportFormat::JSON => "application/json",
            ReportFormat::TXT => "text/plain",
            ReportFormat::DOCS => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            ReportFormat::CSV => "text/csv",
            ReportFormat::HTML => "text/html",
            ReportFormat::PDF => "application/pdf",
        }
    }

    pub fn all() -> Vec<ReportFormat> {
        vec![
            ReportFormat::JSON,
            ReportFormat::TXT,
            ReportFormat::DOCS,
            ReportFormat::CSV,
            ReportFormat::HTML,
            ReportFormat::PDF,
        ]
    }
}

impl fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.extension())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportType {
    ExecutiveSummary,
    TechnicalDeepDive,
    VulnerabilityTracker,
    FullReport,
    Custom(String),
}

impl fmt::Display for ReportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportType::ExecutiveSummary => write!(f, "Executive Summary"),
            ReportType::TechnicalDeepDive => write!(f, "Technical Deep Dive"),
            ReportType::VulnerabilityTracker => write!(f, "Vulnerability Tracker"),
            ReportType::FullReport => write!(f, "Full Report"),
            ReportType::Custom(s) => write!(f, "Custom: {}", s),
        }
    }
}

// ============================================================
// REPORT DATA (INPUT UNTUK GENERATOR)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub report_id: Uuid,
    pub scan_id: Uuid,
    pub report_type: ReportType,
    pub title: String,
    pub target_url: String,
    pub scan_date: DateTime<Utc>,
    pub generated_by: String,
    pub summary: Option<String>,
    pub executive_summary: Option<ExecutiveSummary>,
    pub technical_details: Option<TechnicalDetails>,
    pub vulnerability_tracker: Option<VulnerabilityTrackerData>,
    pub timeline: Option<ReportTimeline>,
    pub statistics: Option<ReportStatistics>,
    pub findings: Vec<ReportFinding>,
    pub recommendations: Vec<Recommendation>,
    pub metadata: ReportMetadata,
    pub created_at: DateTime<Utc>,
}

impl ReportData {
    pub fn new(report_type: ReportType, scan_id: Uuid, target_url: &str, title: &str) -> Self {
        ReportData {
            report_id: Uuid::new_v4(),
            scan_id,
            report_type,
            title: title.to_string(),
            target_url: target_url.to_string(),
            scan_date: Utc::now(),
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
            created_at: Utc::now(),
        }
    }

    pub fn total_findings(&self) -> usize {
        self.findings.len()
    }

    pub fn critical_findings(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == Severity::Critical).count()
    }

    pub fn high_findings(&self) -> usize {
        self.findings.iter().filter(|f| f.severity == Severity::High).count()
    }

    pub fn findings_by_severity(&self) -> HashMap<Severity, usize> {
        let mut map = HashMap::new();
        for finding in &self.findings {
            *map.entry(finding.severity).or_insert(0) += 1;
        }
        map
    }

    pub fn overall_risk_score(&self) -> f32 {
        if self.findings.is_empty() {
            return 0.0;
        }
        let total: f32 = self.findings.iter().map(|f| f.severity.to_score()).sum();
        total / self.findings.len() as f32
    }
}

// ============================================================
// EXECUTIVE SUMMARY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    pub overview: String,
    pub key_findings: Vec<KeyFinding>,
    pub risk_summary: RiskSummary,
    pub security_grade: String,
    pub actions: Vec<ActionItem>,
    pub generated_by_ai: bool,
    pub ai_model: Option<String>,
}

impl ExecutiveSummary {
    pub fn new() -> Self {
        ExecutiveSummary {
            overview: String::new(),
            key_findings: vec![],
            risk_summary: RiskSummary::new(),
            security_grade: String::new(),
            actions: vec![],
            generated_by_ai: false,
            ai_model: None,
        }
    }

    pub fn action_count(&self) -> usize {
        self.actions.len()
    }

    pub fn critical_actions(&self) -> Vec<&ActionItem> {
        self.actions.iter().filter(|a| a.priority == Priority::P0).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyFinding {
    pub title: String,
    pub severity: Severity,
    pub impact: String,
    pub description: String,
    pub affected_systems: Vec<String>,
}

impl KeyFinding {
    pub fn new(title: &str, severity: Severity, impact: &str, description: &str) -> Self {
        KeyFinding {
            title: title.to_string(),
            severity,
            impact: impact.to_string(),
            description: description.to_string(),
            affected_systems: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub total_vulnerabilities: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
    pub info_count: usize,
    pub overall_risk_level: Severity,
    pub risk_trend: Option<String>,
}

impl RiskSummary {
    pub fn new() -> Self {
        RiskSummary {
            total_vulnerabilities: 0,
            critical_count: 0,
            high_count: 0,
            medium_count: 0,
            low_count: 0,
            info_count: 0,
            overall_risk_level: Severity::Info,
            risk_trend: None,
        }
    }

    pub fn critical_percentage(&self) -> f32 {
        if self.total_vulnerabilities == 0 { 0.0 }
        else { (self.critical_count as f32 / self.total_vulnerabilities as f32) * 100.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionItem {
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub estimated_effort: String,
    pub estimated_duration: String,
    pub responsible_team: Option<String>,
    pub deadline_recommendation: Option<String>,
}

impl ActionItem {
    pub fn new(priority: Priority, title: &str, description: &str) -> Self {
        ActionItem {
            priority,
            title: title.to_string(),
            description: description.to_string(),
            estimated_effort: "medium".to_string(),
            estimated_duration: "unknown".to_string(),
            responsible_team: None,
            deadline_recommendation: None,
        }
    }
}

// ============================================================
// TECHNICAL DETAILS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnicalDetails {
    pub detailed_findings: Vec<DetailedFinding>,
    pub root_cause_analysis: Option<String>,
    pub affected_systems: Vec<String>,
    pub attack_surface_summary: String,
    pub methodology: String,
}

impl TechnicalDetails {
    pub fn new() -> Self {
        TechnicalDetails {
            detailed_findings: vec![],
            root_cause_analysis: None,
            affected_systems: vec![],
            attack_surface_summary: String::new(),
            methodology: "Automated scanning with IWS".to_string(),
        }
    }

    pub fn total_evidence_items(&self) -> usize {
        self.detailed_findings.iter().map(|f| f.evidence.len()).sum()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedFinding {
    pub id: Uuid,
    pub title: String,
    pub severity: Severity,
    pub cvss_score: f32,
    pub cve_id: Option<String>,
    pub description: String,
    pub technical_details: String,
    pub evidence: Vec<Evidence>,
    pub proof_of_concept: Option<String>,
    pub impact: String,
    pub remediation: String,
    pub references: Vec<String>,
    pub discovered_at: DateTime<Utc>,
}

impl DetailedFinding {
    pub fn new(title: &str, severity: Severity, description: &str) -> Self {
        DetailedFinding {
            id: Uuid::new_v4(),
            title: title.to_string(),
            severity,
            cvss_score: severity.to_score(),
            cve_id: None,
            description: description.to_string(),
            technical_details: String::new(),
            evidence: vec![],
            proof_of_concept: None,
            impact: String::new(),
            remediation: String::new(),
            references: vec![],
            discovered_at: Utc::now(),
        }
    }

    pub fn with_cve(mut self, cve: &str) -> Self {
        self.cve_id = Some(cve.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub description: String,
    pub data: String,
    pub timestamp: DateTime<Utc>,
}

impl Evidence {
    pub fn new(evidence_type: EvidenceType, description: &str, data: &str) -> Self {
        Evidence {
            evidence_type,
            description: description.to_string(),
            data: data.to_string(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceType {
    Screenshot,
    RequestResponse,
    LogEntry,
    CodeSnippet,
    Configuration,
    NetworkCapture,
    Other(String),
}

impl fmt::Display for EvidenceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvidenceType::Screenshot => write!(f, "screenshot"),
            EvidenceType::RequestResponse => write!(f, "request_response"),
            EvidenceType::LogEntry => write!(f, "log_entry"),
            EvidenceType::CodeSnippet => write!(f, "code_snippet"),
            EvidenceType::Configuration => write!(f, "configuration"),
            EvidenceType::NetworkCapture => write!(f, "network_capture"),
            EvidenceType::Other(s) => write!(f, "{}", s),
        }
    }
}

// ============================================================
// VULNERABILITY TRACKER
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityTrackerData {
    pub tracked_vulnerabilities: Vec<TrackedVulnerability>,
    pub status_history: Vec<StatusChange>,
    pub sla_compliance: Option<SlaCompliance>,
}

impl VulnerabilityTrackerData {
    pub fn new() -> Self {
        VulnerabilityTrackerData {
            tracked_vulnerabilities: vec![],
            status_history: vec![],
            sla_compliance: None,
        }
    }

    pub fn open_count(&self) -> usize {
        self.tracked_vulnerabilities.iter().filter(|v| v.status == VulnStatus::Open).count()
    }

    pub fn fixed_count(&self) -> usize {
        self.tracked_vulnerabilities.iter().filter(|v| v.status == VulnStatus::Fixed).count()
    }

    pub fn overdue_count(&self) -> usize {
        self.tracked_vulnerabilities.iter().filter(|v| v.is_overdue()).count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedVulnerability {
    pub id: Uuid,
    pub title: String,
    pub severity: Severity,
    pub status: VulnStatus,
    pub assigned_to: Option<String>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub sla_deadline: Option<DateTime<Utc>>,
    pub notes: Vec<String>,
}

impl TrackedVulnerability {
    pub fn new(title: &str, severity: Severity) -> Self {
        let now = Utc::now();
        let sla = match severity {
            Severity::Critical => Some(now + chrono::Duration::hours(24)),
            Severity::High => Some(now + chrono::Duration::days(7)),
            Severity::Medium => Some(now + chrono::Duration::days(30)),
            Severity::Low => Some(now + chrono::Duration::days(90)),
            Severity::Info => None,
        };

        TrackedVulnerability {
            id: Uuid::new_v4(),
            title: title.to_string(),
            severity,
            status: VulnStatus::Open,
            assigned_to: None,
            opened_at: now,
            closed_at: None,
            updated_at: now,
            sla_deadline: sla,
            notes: vec![],
        }
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(deadline) = self.sla_deadline {
            self.status == VulnStatus::Open && Utc::now() > deadline
        } else {
            false
        }
    }

    pub fn days_open(&self) -> i64 {
        let end = self.closed_at.unwrap_or_else(Utc::now);
        (end - self.opened_at).num_days()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnStatus {
    Open,
    InProgress,
    Fixed,
    WontFix,
    FalsePositive,
    Duplicate,
}

impl fmt::Display for VulnStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VulnStatus::Open => write!(f, "open"),
            VulnStatus::InProgress => write!(f, "in_progress"),
            VulnStatus::Fixed => write!(f, "fixed"),
            VulnStatus::WontFix => write!(f, "wont_fix"),
            VulnStatus::FalsePositive => write!(f, "false_positive"),
            VulnStatus::Duplicate => write!(f, "duplicate"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusChange {
    pub vulnerability_id: Uuid,
    pub from_status: VulnStatus,
    pub to_status: VulnStatus,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
    pub comment: Option<String>,
}

impl StatusChange {
    pub fn new(
        vulnerability_id: Uuid,
        from_status: VulnStatus,
        to_status: VulnStatus,
        changed_by: &str,
    ) -> Self {
        StatusChange {
            vulnerability_id,
            from_status,
            to_status,
            changed_by: changed_by.to_string(),
            changed_at: Utc::now(),
            comment: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaCompliance {
    pub critical_sla_hours: u32,
    pub high_sla_days: u32,
    pub medium_sla_days: u32,
    pub low_sla_days: u32,
    pub critical_compliance_rate: f32,
    pub high_compliance_rate: f32,
    pub medium_compliance_rate: f32,
    pub low_compliance_rate: f32,
    pub overall_compliance_rate: f32,
}

impl SlaCompliance {
    pub fn new() -> Self {
        SlaCompliance {
            critical_sla_hours: 24,
            high_sla_days: 7,
            medium_sla_days: 30,
            low_sla_days: 90,
            critical_compliance_rate: 100.0,
            high_compliance_rate: 100.0,
            medium_compliance_rate: 100.0,
            low_compliance_rate: 100.0,
            overall_compliance_rate: 100.0,
        }
    }

    pub fn is_compliant(&self) -> bool {
        self.overall_compliance_rate >= 80.0
    }
}

// ============================================================
// REPORT TIMELINE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTimeline {
    pub events: Vec<TimelineEvent>,
}

impl ReportTimeline {
    pub fn new() -> Self {
        ReportTimeline { events: vec![] }
    }

    pub fn add_event(&mut self, event: TimelineEvent) {
        self.events.push(event);
        self.events.sort_by_key(|e| e.timestamp.to_string());
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn events_by_type(&self, event_type: &TimelineEventType) -> Vec<&TimelineEvent> {
        self.events.iter().filter(|e| e.event_type == *event_type).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: TimelineEventType,
    pub description: String,
    pub details: Option<serde_json::Value>,
    pub severity: Severity,
}

impl TimelineEvent {
    pub fn new(event_type: TimelineEventType, description: &str) -> Self {
        TimelineEvent {
            timestamp: Utc::now(),
            event_type,
            description: description.to_string(),
            details: None,
            severity: Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TimelineEventType {
    ScanStarted,
    ScanCompleted,
    ScanFailed,
    FindingFound,
    VulnerabilityDetected,
    StatusChanged,
    AlertTriggered,
    ReportGenerated,
    AnalysisStarted,
    AnalysisCompleted,
    MonitoringEvent,
    Custom(String),
}

impl fmt::Display for TimelineEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimelineEventType::ScanStarted => write!(f, "Scan Started"),
            TimelineEventType::ScanCompleted => write!(f, "Scan Completed"),
            TimelineEventType::ScanFailed => write!(f, "Scan Failed"),
            TimelineEventType::FindingFound => write!(f, "Finding Found"),
            TimelineEventType::VulnerabilityDetected => write!(f, "Vulnerability Detected"),
            TimelineEventType::StatusChanged => write!(f, "Status Changed"),
            TimelineEventType::AlertTriggered => write!(f, "Alert Triggered"),
            TimelineEventType::ReportGenerated => write!(f, "Report Generated"),
            TimelineEventType::AnalysisStarted => write!(f, "Analysis Started"),
            TimelineEventType::AnalysisCompleted => write!(f, "Analysis Completed"),
            TimelineEventType::MonitoringEvent => write!(f, "Monitoring Event"),
            TimelineEventType::Custom(s) => write!(f, "{}", s),
        }
    }
}

// ============================================================
// REPORT STATISTICS & FINDING
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportStatistics {
    pub scan_duration_secs: u64,
    pub pages_crawled: usize,
    pub links_found: usize,
    pub forms_analyzed: usize,
    pub scripts_analyzed: usize,
    pub ports_scanned: usize,
    pub vulnerabilities_found: usize,
    pub critical_findings: usize,
    pub high_findings: usize,
    pub medium_findings: usize,
    pub low_findings: usize,
    pub modules_executed: usize,
    pub api_calls_made: usize,
    pub data_transferred_bytes: u64,
    pub average_response_time_ms: f64,
}

impl ReportStatistics {
    pub fn new() -> Self {
        ReportStatistics {
            scan_duration_secs: 0,
            pages_crawled: 0,
            links_found: 0,
            forms_analyzed: 0,
            scripts_analyzed: 0,
            ports_scanned: 0,
            vulnerabilities_found: 0,
            critical_findings: 0,
            high_findings: 0,
            medium_findings: 0,
            low_findings: 0,
            modules_executed: 0,
            api_calls_made: 0,
            data_transferred_bytes: 0,
            average_response_time_ms: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportFinding {
    pub id: Uuid,
    pub title: String,
    pub finding_type: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub description: String,
    pub location: Option<String>,
    pub evidence: Option<String>,
    pub remediation: Option<String>,
    pub cve_id: Option<String>,
    pub cvss_score: Option<f32>,
    pub module_source: String,
    pub discovered_at: DateTime<Utc>,
}

impl ReportFinding {
    pub fn new(title: &str, finding_type: &str, severity: Severity, module: &str) -> Self {
        ReportFinding {
            id: Uuid::new_v4(),
            title: title.to_string(),
            finding_type: finding_type.to_string(),
            severity,
            confidence: Confidence::Medium,
            description: String::new(),
            location: None,
            evidence: None,
            remediation: None,
            cve_id: None,
            cvss_score: Some(severity.to_score()),
            module_source: module.to_string(),
            discovered_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub priority: Priority,
    pub title: String,
    pub description: String,
    pub category: String,
    pub effort: String,
    pub impact: String,
    pub related_findings: Vec<Uuid>,
}

impl Recommendation {
    pub fn new(priority: Priority, title: &str, description: &str) -> Self {
        Recommendation {
            priority,
            title: title.to_string(),
            description: description.to_string(),
            category: "security".to_string(),
            effort: "medium".to_string(),
            impact: "high".to_string(),
            related_findings: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub version: String,
    pub language: String,
    pub template: String,
    pub include_charts: bool,
    pub include_raw_data: bool,
    pub watermark: Option<String>,
    pub classification: String,
    pub custom_fields: HashMap<String, String>,
}

impl Default for ReportMetadata {
    fn default() -> Self {
        ReportMetadata {
            version: "1.0.0".to_string(),
            language: "en".to_string(),
            template: "default".to_string(),
            include_charts: true,
            include_raw_data: false,
            watermark: None,
            classification: "confidential".to_string(),
            custom_fields: HashMap::new(),
        }
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_format_from_str() {
        assert_eq!(ReportFormat::from_str("json"), Some(ReportFormat::JSON));
        assert_eq!(ReportFormat::from_str("PDF"), Some(ReportFormat::PDF));
        assert_eq!(ReportFormat::from_str("unknown"), None);
    }

    #[test]
    fn test_report_format_extension() {
        assert_eq!(ReportFormat::PDF.extension(), "pdf");
        assert_eq!(ReportFormat::HTML.mime_type(), "text/html");
    }

    #[test]
    fn test_report_data_findings_by_severity() {
        let mut data = ReportData::new(
            ReportType::FullReport,
            Uuid::new_v4(),
            "https://example.com",
            "Test Report",
        );
        data.findings.push(ReportFinding::new("XSS", "vulnerability", Severity::High, "xss_detector"));
        data.findings.push(ReportFinding::new("SQLi", "vulnerability", Severity::Critical, "sql_detector"));
        data.findings.push(ReportFinding::new("Info", "info", Severity::Info, "info_module"));

        let by_sev = data.findings_by_severity();
        assert_eq!(by_sev.get(&Severity::Critical).unwrap(), &1);
        assert_eq!(by_sev.get(&Severity::High).unwrap(), &1);
        assert!(data.overall_risk_score() > 0.0);
    }

    #[test]
    fn test_executive_summary_actions() {
        let mut summary = ExecutiveSummary::new();
        summary.actions.push(ActionItem::new(Priority::P0, "Fix RCE", "Critical RCE vulnerability"));
        summary.actions.push(ActionItem::new(Priority::P2, "Update headers", "Missing security headers"));

        assert_eq!(summary.action_count(), 2);
        assert_eq!(summary.critical_actions().len(), 1);
    }

    #[test]
    fn test_risk_summary_percentage() {
        let mut summary = RiskSummary::new();
        summary.total_vulnerabilities = 10;
        summary.critical_count = 2;
        assert!((summary.critical_percentage() - 20.0).abs() < 0.1);
    }

    #[test]
    fn test_tracked_vulnerability_sla() {
        let vuln = TrackedVulnerability::new("RCE", Severity::Critical);
        assert!(vuln.sla_deadline.is_some());
        assert!(!vuln.is_overdue());

        let mut old_vuln = TrackedVulnerability::new("Old bug", Severity::High);
        old_vuln.sla_deadline = Some(Utc::now() - chrono::Duration::days(10));
        assert!(old_vuln.is_overdue());
    }

    #[test]
    fn test_vulnerability_tracker_counts() {
        let mut tracker = VulnerabilityTrackerData::new();
        tracker.tracked_vulnerabilities.push(TrackedVulnerability::new("V1", Severity::Critical));
        tracker.tracked_vulnerabilities.push(TrackedVulnerability::new("V2", Severity::Medium));
        let mut fixed = TrackedVulnerability::new("V3", Severity::Low);
        fixed.status = VulnStatus::Fixed;
        tracker.tracked_vulnerabilities.push(fixed);

        assert_eq!(tracker.open_count(), 2);
        assert_eq!(tracker.fixed_count(), 1);
    }

    #[test]
    fn test_report_timeline_events() {
        let mut timeline = ReportTimeline::new();
        timeline.add_event(TimelineEvent::new(TimelineEventType::ScanStarted, "Scan began"));
        timeline.add_event(TimelineEvent::new(TimelineEventType::FindingFound, "XSS found"));

        assert_eq!(timeline.event_count(), 2);
        assert_eq!(timeline.events_by_type(&TimelineEventType::FindingFound).len(), 1);
    }

    #[test]
    fn test_sla_compliance() {
        let sla = SlaCompliance::new();
        assert!(sla.is_compliant());

        let mut bad_sla = SlaCompliance::new();
        bad_sla.overall_compliance_rate = 50.0;
        assert!(!bad_sla.is_compliant());
    }
}
