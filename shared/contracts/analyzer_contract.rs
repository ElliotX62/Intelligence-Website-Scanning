// shared/contracts/analyzer_contract.rs
// IWS v1.0 - Analyzer Contract
// Mendefinisikan kontrak formal untuk semua analyzer components

use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use super::scanner_contract::{
    ScanResult,
    ScanStatus,
    ModuleResult,
    ScanContractError,
};

// ============================================================
// ANALYSIS CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub enable_cross_reference: bool,
    pub enable_cve_matching: bool,
    pub enable_risk_scoring: bool,
    pub enable_pattern_detection: bool,
    pub enable_ml_classification: bool,
    pub enable_nlp_processing: bool,
    pub enable_anomaly_detection: bool,
    pub confidence_threshold: f32,
    pub max_analysis_time_secs: u64,
    pub parallel_stages: bool,
    pub cvss_version: CvssVersion,
    pub risk_formula: RiskFormula,
    pub correlation_depth: u8,
    pub custom_rules_path: Option<String>,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        AnalysisConfig {
            enable_cross_reference: true,
            enable_cve_matching: true,
            enable_risk_scoring: true,
            enable_pattern_detection: true,
            enable_ml_classification: true,
            enable_nlp_processing: true,
            enable_anomaly_detection: true,
            confidence_threshold: 0.5,
            max_analysis_time_secs: 300,
            parallel_stages: true,
            cvss_version: CvssVersion::V3_1,
            risk_formula: RiskFormula::CvssWithBusinessContext,
            correlation_depth: 3,
            custom_rules_path: None,
        }
    }
}

impl AnalysisConfig {
    pub fn quick() -> Self {
        AnalysisConfig {
            enable_cross_reference: false,
            enable_cve_matching: true,
            enable_risk_scoring: true,
            enable_pattern_detection: true,
            enable_ml_classification: false,
            enable_nlp_processing: false,
            enable_anomaly_detection: false,
            confidence_threshold: 0.7,
            max_analysis_time_secs: 60,
            parallel_stages: true,
            cvss_version: CvssVersion::V3_1,
            risk_formula: RiskFormula::CvssOnly,
            correlation_depth: 1,
            custom_rules_path: None,
        }
    }

    pub fn deep() -> Self {
        AnalysisConfig {
            enable_cross_reference: true,
            enable_cve_matching: true,
            enable_risk_scoring: true,
            enable_pattern_detection: true,
            enable_ml_classification: true,
            enable_nlp_processing: true,
            enable_anomaly_detection: true,
            confidence_threshold: 0.3,
            max_analysis_time_secs: 600,
            parallel_stages: true,
            cvss_version: CvssVersion::V3_1,
            risk_formula: RiskFormula::FullContext,
            correlation_depth: 5,
            custom_rules_path: None,
        }
    }

    pub fn validate(&self) -> Result<(), AnalyzerContractError> {
        if self.confidence_threshold < 0.0 || self.confidence_threshold > 1.0 {
            return Err(AnalyzerContractError::InvalidConfiguration(
                format!("confidence_threshold must be 0.0-1.0, got {}", self.confidence_threshold)
            ));
        }
        if self.max_analysis_time_secs == 0 || self.max_analysis_time_secs > 3600 {
            return Err(AnalyzerContractError::InvalidConfiguration(
                format!("max_analysis_time_secs must be 1-3600, got {}", self.max_analysis_time_secs)
            ));
        }
        if self.correlation_depth == 0 || self.correlation_depth > 10 {
            return Err(AnalyzerContractError::InvalidConfiguration(
                format!("correlation_depth must be 1-10, got {}", self.correlation_depth)
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CvssVersion {
    V2_0,
    V3_0,
    V3_1,
    V4_0,
}

impl fmt::Display for CvssVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CvssVersion::V2_0 => write!(f, "CVSS v2.0"),
            CvssVersion::V3_0 => write!(f, "CVSS v3.0"),
            CvssVersion::V3_1 => write!(f, "CVSS v3.1"),
            CvssVersion::V4_0 => write!(f, "CVSS v4.0"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskFormula {
    CvssOnly,
    CvssWithBusinessContext,
    OwaspRiskRating,
    FullContext,
    Custom(String),
}

impl fmt::Display for RiskFormula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskFormula::CvssOnly => write!(f, "CVSS Only"),
            RiskFormula::CvssWithBusinessContext => write!(f, "CVSS + Business Context"),
            RiskFormula::OwaspRiskRating => write!(f, "OWASP Risk Rating"),
            RiskFormula::FullContext => write!(f, "Full Context (CVSS + Business + OWASP + EPSS)"),
            RiskFormula::Custom(name) => write!(f, "Custom: {}", name),
        }
    }
}

// ============================================================
// ANALYSIS RESULT TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub analysis_id: Uuid,
    pub scan_id: Uuid,
    pub target_url: String,
    pub status: AnalysisStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_secs: u64,
    pub findings: Vec<Finding>,
    pub vulnerabilities: Vec<Vulnerability>,
    pub risk_assessment: Option<RiskAssessment>,
    pub correlations: Vec<Correlation>,
    pub anomalies: Vec<Anomaly>,
    pub patterns: Vec<PatternMatch>,
    pub summary: Option<String>,
    pub confidence_score: f32,
    pub stages_completed: Vec<String>,
    pub stages_failed: Vec<String>,
    pub errors: Vec<AnalysisError>,
    pub metadata: AnalysisMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnalysisStatus {
    Pending,
    Preprocessing,
    PatternDetecting,
    VulnerabilityMatching,
    CrossReferencing,
    RiskScoring,
    Summarizing,
    Completed,
    Failed,
    TimedOut,
}

impl fmt::Display for AnalysisStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisStatus::Pending => write!(f, "pending"),
            AnalysisStatus::Preprocessing => write!(f, "preprocessing"),
            AnalysisStatus::PatternDetecting => write!(f, "pattern_detecting"),
            AnalysisStatus::VulnerabilityMatching => write!(f, "vulnerability_matching"),
            AnalysisStatus::CrossReferencing => write!(f, "cross_referencing"),
            AnalysisStatus::RiskScoring => write!(f, "risk_scoring"),
            AnalysisStatus::Summarizing => write!(f, "summarizing"),
            AnalysisStatus::Completed => write!(f, "completed"),
            AnalysisStatus::Failed => write!(f, "failed"),
            AnalysisStatus::TimedOut => write!(f, "timed_out"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: Uuid,
    pub finding_type: FindingType,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub source_module: String,
    pub evidence: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub related_findings: Vec<Uuid>,
    pub false_positive_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FindingType {
    Vulnerability,
    Misconfiguration,
    InformationLeak,
    OutdatedSoftware,
    WeakCipher,
    ExposedService,
    SuspiciousBehavior,
    ComplianceIssue,
    ThreatIndicator,
    Anomaly,
    Pattern,
    Other(String),
}

impl fmt::Display for FindingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FindingType::Vulnerability => write!(f, "vulnerability"),
            FindingType::Misconfiguration => write!(f, "misconfiguration"),
            FindingType::InformationLeak => write!(f, "information_leak"),
            FindingType::OutdatedSoftware => write!(f, "outdated_software"),
            FindingType::WeakCipher => write!(f, "weak_cipher"),
            FindingType::ExposedService => write!(f, "exposed_service"),
            FindingType::SuspiciousBehavior => write!(f, "suspicious_behavior"),
            FindingType::ComplianceIssue => write!(f, "compliance_issue"),
            FindingType::ThreatIndicator => write!(f, "threat_indicator"),
            FindingType::Anomaly => write!(f, "anomaly"),
            FindingType::Pattern => write!(f, "pattern"),
            FindingType::Other(s) => write!(f, "other:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "info"),
            Severity::Low => write!(f, "low"),
            Severity::Medium => write!(f, "medium"),
            Severity::High => write!(f, "high"),
            Severity::Critical => write!(f, "critical"),
        }
    }
}

impl Severity {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 9.0 => Severity::Critical,
            s if s >= 7.0 => Severity::High,
            s if s >= 4.0 => Severity::Medium,
            s if s >= 0.1 => Severity::Low,
            _ => Severity::Info,
        }
    }

    pub fn to_score(&self) -> f32 {
        match self {
            Severity::Critical => 9.0,
            Severity::High => 7.0,
            Severity::Medium => 4.0,
            Severity::Low => 1.0,
            Severity::Info => 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    Low,
    Medium,
    High,
    Verified,
}

impl fmt::Display for Confidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Confidence::Low => write!(f, "low"),
            Confidence::Medium => write!(f, "medium"),
            Confidence::High => write!(f, "high"),
            Confidence::Verified => write!(f, "verified"),
        }
    }
}

impl Confidence {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 0.85 => Confidence::Verified,
            s if s >= 0.7 => Confidence::High,
            s if s >= 0.4 => Confidence::Medium,
            _ => Confidence::Low,
        }
    }

    pub fn to_score(&self) -> f32 {
        match self {
            Confidence::Verified => 0.95,
            Confidence::High => 0.8,
            Confidence::Medium => 0.5,
            Confidence::Low => 0.2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: Uuid,
    pub cve_id: Option<String>,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub cvss_score: f32,
    pub cvss_vector: Option<String>,
    pub epss_score: Option<f32>,
    pub affected_component: String,
    pub affected_version: String,
    pub fixed_version: Option<String>,
    pub references: Vec<String>,
    pub remediation: Option<String>,
    pub exploit_available: bool,
    pub exploit_maturity: ExploitMaturity,
    pub discovered_at: DateTime<Utc>,
    pub status: VulnerabilityStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExploitMaturity {
    Unproven,
    ProofOfConcept,
    Functional,
    High,
    Weaponized,
    Unknown,
}

impl fmt::Display for ExploitMaturity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExploitMaturity::Unproven => write!(f, "unproven"),
            ExploitMaturity::ProofOfConcept => write!(f, "proof_of_concept"),
            ExploitMaturity::Functional => write!(f, "functional"),
            ExploitMaturity::High => write!(f, "high"),
            ExploitMaturity::Weaponized => write!(f, "weaponized"),
            ExploitMaturity::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnerabilityStatus {
    Open,
    InProgress,
    Fixed,
    WontFix,
    FalsePositive,
    Duplicate,
}

impl fmt::Display for VulnerabilityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VulnerabilityStatus::Open => write!(f, "open"),
            VulnerabilityStatus::InProgress => write!(f, "in_progress"),
            VulnerabilityStatus::Fixed => write!(f, "fixed"),
            VulnerabilityStatus::WontFix => write!(f, "wont_fix"),
            VulnerabilityStatus::FalsePositive => write!(f, "false_positive"),
            VulnerabilityStatus::Duplicate => write!(f, "duplicate"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub base_score: f32,
    pub temporal_score: f32,
    pub environmental_score: f32,
    pub business_score: f32,
    pub overall_score: f32,
    pub risk_level: Severity,
    pub priority: Priority,
    pub business_impact: BusinessImpact,
    pub cvss_vector: Option<String>,
    pub risk_factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    P0Critical,
    P1High,
    P2Medium,
    P3Low,
    P4Info,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::P0Critical => write!(f, "P0-Critical"),
            Priority::P1High => write!(f, "P1-High"),
            Priority::P2Medium => write!(f, "P2-Medium"),
            Priority::P3Low => write!(f, "P3-Low"),
            Priority::P4Info => write!(f, "P4-Info"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessImpact {
    pub financial: Severity,
    pub reputational: Severity,
    pub regulatory: Severity,
    pub operational: Severity,
    pub data_loss: Severity,
}

impl Default for BusinessImpact {
    fn default() -> Self {
        BusinessImpact {
            financial: Severity::Low,
            reputational: Severity::Low,
            regulatory: Severity::Info,
            operational: Severity::Low,
            data_loss: Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Correlation {
    pub id: Uuid,
    pub finding_ids: Vec<Uuid>,
    pub correlation_type: CorrelationType,
    pub description: String,
    pub strength: f32,
    pub evidence: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorrelationType {
    SameComponent,
    AttackChain,
    SharedVulnerability,
    InfrastructureDependency,
    ConfigurationDrift,
    Temporal,
    Causal,
    Other(String),
}

impl fmt::Display for CorrelationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CorrelationType::SameComponent => write!(f, "same_component"),
            CorrelationType::AttackChain => write!(f, "attack_chain"),
            CorrelationType::SharedVulnerability => write!(f, "shared_vulnerability"),
            CorrelationType::InfrastructureDependency => write!(f, "infrastructure_dependency"),
            CorrelationType::ConfigurationDrift => write!(f, "configuration_drift"),
            CorrelationType::Temporal => write!(f, "temporal"),
            CorrelationType::Causal => write!(f, "causal"),
            CorrelationType::Other(s) => write!(f, "other:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub id: Uuid,
    pub anomaly_type: AnomalyType,
    pub description: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub data_point: serde_json::Value,
    pub expected_value: Option<serde_json::Value>,
    pub deviation_score: f32,
    pub timestamp: DateTime<Utc>,
    pub module_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnomalyType {
    Outlier,
    TrendBreak,
    Spike,
    Dip,
    PatternBreak,
    SeasonalAnomaly,
    ContextualAnomaly,
    Other(String),
}

impl fmt::Display for AnomalyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnomalyType::Outlier => write!(f, "outlier"),
            AnomalyType::TrendBreak => write!(f, "trend_break"),
            AnomalyType::Spike => write!(f, "spike"),
            AnomalyType::Dip => write!(f, "dip"),
            AnomalyType::PatternBreak => write!(f, "pattern_break"),
            AnomalyType::SeasonalAnomaly => write!(f, "seasonal_anomaly"),
            AnomalyType::ContextualAnomaly => write!(f, "contextual_anomaly"),
            AnomalyType::Other(s) => write!(f, "other:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub id: Uuid,
    pub pattern_name: String,
    pub pattern_type: PatternType,
    pub matches: Vec<PatternOccurrence>,
    pub total_occurrences: usize,
    pub pattern_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PatternType {
    Regex,
    Substring,
    Structural,
    Behavioral,
    AhoCorasick,
    Sequence,
    Other(String),
}

impl fmt::Display for PatternType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternType::Regex => write!(f, "regex"),
            PatternType::Substring => write!(f, "substring"),
            PatternType::Structural => write!(f, "structural"),
            PatternType::Behavioral => write!(f, "behavioral"),
            PatternType::AhoCorasick => write!(f, "aho_corasick"),
            PatternType::Sequence => write!(f, "sequence"),
            PatternType::Other(s) => write!(f, "other:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternOccurrence {
    pub location: String,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub matched_text: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisError {
    pub stage: String,
    pub error_type: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub recoverable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    pub analyzer_version: String,
    pub contract_version: String,
    pub analysis_host: String,
    pub analysis_os: String,
    pub models_used: Vec<String>,
    pub rules_version: Option<String>,
    pub cve_database_version: Option<String>,
}

impl Default for AnalysisMetadata {
    fn default() -> Self {
        AnalysisMetadata {
            analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
            contract_version: "v1.0".to_string(),
            analysis_host: "localhost".to_string(),
            analysis_os: std::env::consts::OS.to_string(),
            models_used: vec![],
            rules_version: None,
            cve_database_version: None,
        }
    }
}

// ============================================================
// CROSS REFERENCE RESULT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossReferenceResult {
    pub correlations: Vec<Correlation>,
    pub correlation_graph: Option<CorrelationGraph>,
    pub chained_vulnerabilities: Vec<AttackChain>,
    pub false_positive_suggestions: Vec<FalsePositiveSuggestion>,
    pub new_findings: Vec<Finding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
    pub density: f32,
    pub clusters: Vec<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: Uuid,
    pub label: String,
    pub node_type: String,
    pub severity: Severity,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub source: Uuid,
    pub target: Uuid,
    pub edge_type: CorrelationType,
    pub weight: f32,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackChain {
    pub name: String,
    pub description: String,
    pub steps: Vec<AttackStep>,
    pub overall_severity: Severity,
    pub likelihood: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackStep {
    pub step_number: u32,
    pub finding_id: Uuid,
    pub description: String,
    pub prerequisites: Vec<String>,
    pub consequences: Vec<String>,
    pub difficulty: Severity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FalsePositiveSuggestion {
    pub finding_id: Uuid,
    pub reason: String,
    pub confidence: f32,
    pub recommendation: String,
}

// ============================================================
// ANALYSIS STAGE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisStage {
    pub name: String,
    pub status: AnalysisStatus,
    pub progress: f32,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_ms: u64,
    pub items_processed: usize,
    pub items_total: usize,
    pub error: Option<String>,
}

impl AnalysisStage {
    pub fn new(name: &str) -> Self {
        AnalysisStage {
            name: name.to_string(),
            status: AnalysisStatus::Pending,
            progress: 0.0,
            start_time: None,
            end_time: None,
            duration_ms: 0,
            items_processed: 0,
            items_total: 0,
            error: None,
        }
    }

    pub fn start(&mut self) {
        self.status = AnalysisStatus::PatternDetecting;
        self.start_time = Some(Utc::now());
        self.progress = 0.0;
    }

    pub fn update_progress(&mut self, processed: usize, total: usize) {
        self.items_processed = processed;
        self.items_total = total;
        if total > 0 {
            self.progress = (processed as f32 / total as f32) * 100.0;
        }
    }

    pub fn complete(&mut self) {
        self.status = AnalysisStatus::Completed;
        self.end_time = Some(Utc::now());
        if let Some(start) = self.start_time {
            self.duration_ms = (Utc::now() - start).num_milliseconds() as u64;
        }
        self.progress = 100.0;
    }

    pub fn fail(&mut self, error: String) {
        self.status = AnalysisStatus::Failed;
        self.end_time = Some(Utc::now());
        if let Some(start) = self.start_time {
            self.duration_ms = (Utc::now() - start).num_milliseconds() as u64;
        }
        self.error = Some(error);
    }
}

// ============================================================
// ANALYZER CONTRACT TRAIT
// ============================================================

#[async_trait]
pub trait AnalyzerContract: Send + Sync {
    /// Menganalisis data hasil scanning
    /// Preconditions: data.status == Completed, data.modules_results.len() > 0
    /// Postconditions: result.findings.len() > 0, result.risk_score in 0.0-10.0
    async fn analyze_scan_data(
        &self,
        data: ScanResult,
        config: AnalysisConfig,
    ) -> Result<AnalysisResult, AnalyzerContractError>;

    /// Melakukan cross-reference analysis dari findings
    async fn cross_reference_analysis(
        &self,
        result: AnalysisResult,
    ) -> Result<CrossReferenceResult, AnalyzerContractError>;

    /// Menghitung risk score
    /// Postconditions: score >= 0.0 && score <= 10.0
    async fn calculate_risk(
        &self,
        result: AnalysisResult,
    ) -> Result<RiskAssessment, AnalyzerContractError>;

    /// Mendapatkan progress analisis (0-100)
    async fn get_analysis_progress(
        &self,
        analysis_id: Uuid,
    ) -> Result<f32, AnalyzerContractError>;

    /// Mendapatkan daftar stage analisis dan statusnya
    async fn get_analysis_stages(
        &self,
        analysis_id: Uuid,
    ) -> Result<Vec<AnalysisStage>, AnalyzerContractError>;

    /// Membatalkan analisis yang sedang berjalan
    async fn cancel_analysis(
        &self,
        analysis_id: Uuid,
    ) -> Result<(), AnalyzerContractError>;

    /// Mendapatkan capabilities analyzer
    fn get_capabilities(&self) -> AnalyzerCapabilities;

    /// Validasi data sebelum analisis
    fn validate_input(&self, data: &ScanResult) -> Result<(), AnalyzerContractError> {
        if data.status != ScanStatus::Completed {
            return Err(AnalyzerContractError::InvalidData(
                format!("Scan must be completed, current status: {}", data.status)
            ));
        }
        if data.modules_results.is_empty() {
            return Err(AnalyzerContractError::InvalidData(
                "No module results to analyze".to_string()
            ));
        }
        if data.target_url.as_str().is_empty() {
            return Err(AnalyzerContractError::InvalidData(
                "Target URL is empty".to_string()
            ));
        }
        Ok(())
    }

    /// Deteksi duplikasi findings
    fn deduplicate_findings(&self, findings: &[Finding]) -> Vec<Finding> {
        let mut seen: HashMap<String, Finding> = HashMap::new();
        for finding in findings {
            let key = format!(
                "{}:{}:{}",
                finding.finding_type,
                finding.title,
                finding.source_module
            );
            seen.entry(key)
                .and_modify(|existing| {
                    if finding.confidence > existing.confidence {
                        *existing = finding.clone();
                    }
                })
                .or_insert_with(|| finding.clone());
        }
        seen.into_values().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerCapabilities {
    pub max_findings_per_scan: usize,
    pub supported_finding_types: Vec<FindingType>,
    pub supported_correlation_types: Vec<CorrelationType>,
    pub supported_anomaly_types: Vec<AnomalyType>,
    pub supported_pattern_types: Vec<PatternType>,
    pub max_correlation_depth: u8,
    pub supports_ml: bool,
    pub supports_nlp: bool,
    pub supports_anomaly_detection: bool,
    pub cve_database_size: usize,
    pub api_version: String,
}

impl Default for AnalyzerCapabilities {
    fn default() -> Self {
        AnalyzerCapabilities {
            max_findings_per_scan: 10000,
            supported_finding_types: vec![
                FindingType::Vulnerability,
                FindingType::Misconfiguration,
                FindingType::InformationLeak,
                FindingType::OutdatedSoftware,
                FindingType::WeakCipher,
                FindingType::ExposedService,
                FindingType::SuspiciousBehavior,
                FindingType::ComplianceIssue,
                FindingType::ThreatIndicator,
                FindingType::Anomaly,
                FindingType::Pattern,
            ],
            supported_correlation_types: vec![
                CorrelationType::SameComponent,
                CorrelationType::AttackChain,
                CorrelationType::SharedVulnerability,
                CorrelationType::InfrastructureDependency,
                CorrelationType::ConfigurationDrift,
                CorrelationType::Temporal,
                CorrelationType::Causal,
            ],
            supported_anomaly_types: vec![
                AnomalyType::Outlier,
                AnomalyType::TrendBreak,
                AnomalyType::Spike,
                AnomalyType::Dip,
                AnomalyType::PatternBreak,
                AnomalyType::SeasonalAnomaly,
                AnomalyType::ContextualAnomaly,
            ],
            supported_pattern_types: vec![
                PatternType::Regex,
                PatternType::Substring,
                PatternType::Structural,
                PatternType::Behavioral,
                PatternType::AhoCorasick,
                PatternType::Sequence,
            ],
            max_correlation_depth: 5,
            supports_ml: true,
            supports_nlp: true,
            supports_anomaly_detection: true,
            cve_database_size: 250000,
            api_version: "v1.0".to_string(),
        }
    }
}

// ============================================================
// ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnalyzerContractError {
    InvalidData(String),
    InvalidConfiguration(String),
    IncompleteData(String),
    AnalysisNotFound(Uuid),
    AnalysisAlreadyRunning(Uuid),
    AnalysisTimedOut(Uuid),
    ModelLoadFailed(String),
    ModelInferenceFailed(String),
    CorrelationFailed(String),
    PatternCompilationFailed(String),
    CveDatabaseError(String),
    RiskCalculationError(String),
    TimeoutExceeded(u64),
    StageFailed(String),
    InternalError(String),
    UnsupportedFeature(String),
}

impl fmt::Display for AnalyzerContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalyzerContractError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            AnalyzerContractError::InvalidConfiguration(msg) => write!(f, "Invalid config: {}", msg),
            AnalyzerContractError::IncompleteData(msg) => write!(f, "Incomplete data: {}", msg),
            AnalyzerContractError::AnalysisNotFound(id) => write!(f, "Analysis not found: {}", id),
            AnalyzerContractError::AnalysisAlreadyRunning(id) => write!(f, "Analysis running: {}", id),
            AnalyzerContractError::AnalysisTimedOut(id) => write!(f, "Analysis timed out: {}", id),
            AnalyzerContractError::ModelLoadFailed(msg) => write!(f, "Model load failed: {}", msg),
            AnalyzerContractError::ModelInferenceFailed(msg) => write!(f, "Inference failed: {}", msg),
            AnalyzerContractError::CorrelationFailed(msg) => write!(f, "Correlation failed: {}", msg),
            AnalyzerContractError::PatternCompilationFailed(msg) => write!(f, "Pattern failed: {}", msg),
            AnalyzerContractError::CveDatabaseError(msg) => write!(f, "CVE database: {}", msg),
            AnalyzerContractError::RiskCalculationError(msg) => write!(f, "Risk calc: {}", msg),
            AnalyzerContractError::TimeoutExceeded(secs) => write!(f, "Timeout: {}s", secs),
            AnalyzerContractError::StageFailed(msg) => write!(f, "Stage failed: {}", msg),
            AnalyzerContractError::InternalError(msg) => write!(f, "Internal: {}", msg),
            AnalyzerContractError::UnsupportedFeature(msg) => write!(f, "Unsupported: {}", msg),
        }
    }
}

impl std::error::Error for AnalyzerContractError {}

impl AnalyzerContractError {
    pub fn code(&self) -> &str {
        match self {
            AnalyzerContractError::InvalidData(_) => "A2001",
            AnalyzerContractError::InvalidConfiguration(_) => "A2002",
            AnalyzerContractError::IncompleteData(_) => "A2003",
            AnalyzerContractError::AnalysisNotFound(_) => "A2004",
            AnalyzerContractError::AnalysisAlreadyRunning(_) => "A2005",
            AnalyzerContractError::AnalysisTimedOut(_) => "A2006",
            AnalyzerContractError::ModelLoadFailed(_) => "A2007",
            AnalyzerContractError::ModelInferenceFailed(_) => "A2008",
            AnalyzerContractError::CorrelationFailed(_) => "A2009",
            AnalyzerContractError::PatternCompilationFailed(_) => "A2010",
            AnalyzerContractError::CveDatabaseError(_) => "A2011",
            AnalyzerContractError::RiskCalculationError(_) => "A2012",
            AnalyzerContractError::TimeoutExceeded(_) => "A2013",
            AnalyzerContractError::StageFailed(_) => "A2014",
            AnalyzerContractError::InternalError(_) => "A2015",
            AnalyzerContractError::UnsupportedFeature(_) => "A2016",
        }
    }

    pub fn severity(&self) -> &str {
        match self {
            AnalyzerContractError::InvalidData(_) => "high",
            AnalyzerContractError::InvalidConfiguration(_) => "high",
            AnalyzerContractError::IncompleteData(_) => "high",
            AnalyzerContractError::AnalysisNotFound(_) => "medium",
            AnalyzerContractError::AnalysisAlreadyRunning(_) => "low",
            AnalyzerContractError::AnalysisTimedOut(_) => "medium",
            AnalyzerContractError::ModelLoadFailed(_) => "critical",
            AnalyzerContractError::ModelInferenceFailed(_) => "high",
            AnalyzerContractError::CorrelationFailed(_) => "medium",
            AnalyzerContractError::PatternCompilationFailed(_) => "medium",
            AnalyzerContractError::CveDatabaseError(_) => "high",
            AnalyzerContractError::RiskCalculationError(_) => "medium",
            AnalyzerContractError::TimeoutExceeded(_) => "medium",
            AnalyzerContractError::StageFailed(_) => "high",
            AnalyzerContractError::InternalError(_) => "critical",
            AnalyzerContractError::UnsupportedFeature(_) => "low",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            AnalyzerContractError::ModelLoadFailed(_)
                | AnalyzerContractError::ModelInferenceFailed(_)
                | AnalyzerContractError::CorrelationFailed(_)
                | AnalyzerContractError::TimeoutExceeded(_)
                | AnalyzerContractError::StageFailed(_)
        )
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_config_defaults() {
        let config = AnalysisConfig::default();
        assert!(config.enable_cross_reference);
        assert!(config.enable_cve_matching);
        assert_eq!(config.max_analysis_time_secs, 300);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_analysis_config_validation_bad_confidence() {
        let mut config = AnalysisConfig::default();
        config.confidence_threshold = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_analysis_config_validation_bad_timeout() {
        let mut config = AnalysisConfig::default();
        config.max_analysis_time_secs = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_quick_config() {
        let config = AnalysisConfig::quick();
        assert!(!config.enable_cross_reference);
        assert!(!config.enable_ml_classification);
        assert!(!config.enable_nlp_processing);
        assert_eq!(config.max_analysis_time_secs, 60);
    }

    #[test]
    fn test_deep_config() {
        let config = AnalysisConfig::deep();
        assert!(config.enable_cross_reference);
        assert!(config.enable_ml_classification);
        assert_eq!(config.max_analysis_time_secs, 600);
        assert_eq!(config.correlation_depth, 5);
    }

    #[test]
    fn test_severity_from_score() {
        assert_eq!(Severity::from_score(9.5), Severity::Critical);
        assert_eq!(Severity::from_score(7.5), Severity::High);
        assert_eq!(Severity::from_score(5.0), Severity::Medium);
        assert_eq!(Severity::from_score(2.0), Severity::Low);
        assert_eq!(Severity::from_score(0.0), Severity::Info);
    }

    #[test]
    fn test_severity_to_score() {
        assert_eq!(Severity::Critical.to_score(), 9.0);
        assert_eq!(Severity::High.to_score(), 7.0);
        assert_eq!(Severity::Medium.to_score(), 4.0);
        assert_eq!(Severity::Low.to_score(), 1.0);
        assert_eq!(Severity::Info.to_score(), 0.0);
    }

    #[test]
    fn test_confidence_from_score() {
        assert_eq!(Confidence::from_score(0.9), Confidence::Verified);
        assert_eq!(Confidence::from_score(0.75), Confidence::High);
        assert_eq!(Confidence::from_score(0.5), Confidence::Medium);
        assert_eq!(Confidence::from_score(0.2), Confidence::Low);
    }

    #[test]
    fn test_analysis_stage_lifecycle() {
        let mut stage = AnalysisStage::new("test_stage");
        assert_eq!(stage.progress, 0.0);

        stage.start();
        assert_eq!(stage.status, AnalysisStatus::PatternDetecting);

        stage.update_progress(50, 100);
        assert_eq!(stage.progress, 50.0);

        stage.complete();
        assert_eq!(stage.status, AnalysisStatus::Completed);
        assert_eq!(stage.progress, 100.0);
    }

    #[test]
    fn test_analysis_stage_fail() {
        let mut stage = AnalysisStage::new("failing_stage");
        stage.start();
        stage.fail("test error".to_string());
        assert_eq!(stage.status, AnalysisStatus::Failed);
        assert!(stage.error.is_some());
    }

    #[test]
    fn test_deduplicate_findings() {
        struct DummyAnalyzer;
        impl DummyAnalyzer {
            fn deduplicate_findings(&self, findings: &[Finding]) -> Vec<Finding> {
                let mut seen: HashMap<String, Finding> = HashMap::new();
                for finding in findings {
                    let key = format!(
                        "{}:{}:{}",
                        finding.finding_type,
                        finding.title,
                        finding.source_module
                    );
                    seen.entry(key)
                        .and_modify(|existing| {
                            if finding.confidence > existing.confidence {
                                *existing = finding.clone();
                            }
                        })
                        .or_insert_with(|| finding.clone());
                }
                seen.into_values().collect()
            }
        }

        let finding1 = Finding {
            id: Uuid::new_v4(),
            finding_type: FindingType::Vulnerability,
            title: "XSS Found".to_string(),
            description: "Test".to_string(),
            severity: Severity::High,
            confidence: Confidence::Medium,
            source_module: "xss_detector".to_string(),
            evidence: serde_json::json!({}),
            timestamp: Utc::now(),
            related_findings: vec![],
            false_positive_probability: 0.0,
        };

        let finding2 = Finding {
            id: Uuid::new_v4(),
            finding_type: FindingType::Vulnerability,
            title: "XSS Found".to_string(),
            description: "Test".to_string(),
            severity: Severity::High,
            confidence: Confidence::High,
            source_module: "xss_detector".to_string(),
            evidence: serde_json::json!({}),
            timestamp: Utc::now(),
            related_findings: vec![],
            false_positive_probability: 0.0,
        };

        let analyzer = DummyAnalyzer;
        let deduped = analyzer.deduplicate_findings(&[finding1, finding2]);
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].confidence, Confidence::High);
    }

    #[test]
    fn test_analyzer_error_codes() {
        let err = AnalyzerContractError::InvalidData("test".to_string());
        assert_eq!(err.code(), "A2001");

        let err = AnalyzerContractError::TimeoutExceeded(300);
        assert_eq!(err.code(), "A2013");
        assert!(err.to_string().contains("300s"));
    }

    #[test]
    fn test_analyzer_error_recoverable() {
        let timeout = AnalyzerContractError::TimeoutExceeded(300);
        assert!(timeout.is_recoverable());

        let invalid = AnalyzerContractError::InvalidData("test".to_string());
        assert!(!invalid.is_recoverable());
    }
}
