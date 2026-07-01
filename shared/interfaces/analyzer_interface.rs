// shared/interfaces/analyzer_interface.rs
// IWS v1.0 - Analyzer Interface
// Mendefinisikan trait Analyzer untuk semua analyzer components

use std::time::Duration;
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use uuid::Uuid;
use anyhow::Result;

use crate::shared::contracts::analyzer_contract::{
    AnalysisConfig, AnalysisResult, AnalysisStatus, AnalysisStage,
    CrossReferenceResult, RiskAssessment, AnalyzerCapabilities,
    AnalyzerContractError, Finding, Vulnerability, Severity,
    Confidence, Correlation, Anomaly, PatternMatch,
};
use crate::shared::contracts::scanner_contract::ScanResult;

// ============================================================
// ANALYZER TRAIT
// ============================================================

#[async_trait]
pub trait Analyzer: Send + Sync {
    type Error: std::error::Error + From<AnalyzerContractError> + Send + Sync;

    /// Analisis data hasil scanning
    async fn analyze(
        &self,
        data: ScanResult,
        config: AnalysisConfig,
    ) -> Result<AnalysisResult, Self::Error>;

    /// Cross-reference analysis antar findings
    async fn cross_reference(
        &self,
        result: AnalysisResult,
    ) -> Result<CrossReferenceResult, Self::Error>;

    /// Hitung risk score
    async fn calculate_risk(
        &self,
        result: AnalysisResult,
    ) -> Result<RiskAssessment, Self::Error>;

    /// Dapatkan progress analisis (0-100) — SPEC FIX #3
    /// Default implementation: mengembalikan progress dari internal tracker
    /// Jika analyzer tidak memiliki internal tracker, return 0.0
    async fn get_analysis_progress(&self) -> f32 {
        0.0
    }

    /// Dapatkan progress analisis spesifik (0-100)
    async fn progress(&self, analysis_id: Uuid) -> Result<f32, Self::Error>;

    /// Dapatkan daftar stage dan statusnya
    async fn stages(&self, analysis_id: Uuid) -> Result<Vec<AnalysisStage>, Self::Error>;

    /// Batalkan analisis
    async fn cancel(&self, analysis_id: Uuid) -> Result<(), Self::Error>;

    /// Dapatkan capabilities analyzer
    fn capabilities(&self) -> AnalyzerCapabilities;

    /// Validasi data sebelum analisis
    fn validate_data(&self, data: &ScanResult) -> Result<(), Self::Error> {
        if data.status != crate::shared::contracts::scanner_contract::ScanStatus::Completed {
            return Err(AnalyzerContractError::InvalidData(
                format!("Scan must be completed, current status: {}", data.status)
            ).into());
        }
        if data.modules_results.is_empty() {
            return Err(AnalyzerContractError::InvalidData(
                "No module results to analyze".to_string()
            ).into());
        }
        Ok(())
    }

    /// Deduplikasi findings
    fn deduplicate(&self, findings: Vec<Finding>) -> Vec<Finding> {
        let mut seen: HashMap<String, Finding> = HashMap::new();
        for finding in findings {
            let key = format!(
                "{}:{}:{}",
                finding.finding_type,
                finding.title.to_lowercase(),
                finding.source_module
            );
            seen.entry(key)
                .and_modify(|existing| {
                    if finding.confidence > existing.confidence {
                        *existing = finding.clone();
                    }
                })
                .or_insert_with(|| finding);
        }
        seen.into_values().collect()
    }

    /// Filter findings berdasarkan severity minimum
    fn filter_by_severity(&self, findings: &[Finding], min_severity: Severity) -> Vec<Finding> {
        findings.iter()
            .filter(|f| f.severity >= min_severity)
            .cloned()
            .collect()
    }

    /// Filter findings berdasarkan confidence minimum
    fn filter_by_confidence(&self, findings: &[Finding], min_confidence: Confidence) -> Vec<Finding> {
        findings.iter()
            .filter(|f| f.confidence >= min_confidence)
            .cloned()
            .collect()
    }

    /// Kelompokkan findings berdasarkan tipe
    fn group_by_type(&self, findings: &[Finding]) -> HashMap<String, Vec<Finding>> {
        let mut groups: HashMap<String, Vec<Finding>> = HashMap::new();
        for finding in findings {
            groups.entry(finding.finding_type.to_string())
                .or_default()
                .push(finding.clone());
        }
        groups
    }

    /// Hitung distribusi severity
    fn severity_distribution(&self, findings: &[Finding]) -> HashMap<Severity, usize> {
        let mut dist = HashMap::new();
        for finding in findings {
            *dist.entry(finding.severity).or_insert(0) += 1;
        }
        dist
    }

    /// Analisis dengan timeout
    async fn analyze_with_timeout(
        &self,
        data: ScanResult,
        config: AnalysisConfig,
        timeout: Duration,
    ) -> Result<AnalysisResult, Self::Error> {
        tokio::select! {
            result = self.analyze(data, config) => result,
            _ = tokio::time::sleep(timeout) => {
                Err(AnalyzerContractError::TimeoutExceeded(timeout.as_secs()).into())
            }
        }
    }

    /// Analisis dengan retry
    async fn analyze_with_retry(
        &self,
        data: ScanResult,
        config: AnalysisConfig,
        max_retries: u32,
    ) -> Result<AnalysisResult, Self::Error> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match self.analyze(data.clone(), config.clone()).await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < max_retries => {
                    let backoff = Duration::from_secs(2u64.pow(attempt));
                    tokio::time::sleep(backoff).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or_else(|| {
            AnalyzerContractError::InternalError("Retry exhausted".to_string()).into())
        }))
    }
}

// ============================================================
// PROGRESS TRACKER — untuk mendukung get_analysis_progress()
// ============================================================

#[derive(Debug, Clone)]
pub struct ProgressTracker {
    inner: Arc<Mutex<ProgressState>>,
}

#[derive(Debug, Clone)]
struct ProgressState {
    current: f32,
    current_stage: String,
    active_analysis_id: Option<Uuid>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        ProgressTracker {
            inner: Arc::new(Mutex::new(ProgressState {
                current: 0.0,
                current_stage: "idle".to_string(),
                active_analysis_id: None,
            })),
        }
    }

    pub async fn update(&self, progress: f32, stage: &str) {
        let mut state = self.inner.lock().await;
        state.current = progress.clamp(0.0, 100.0);
        state.current_stage = stage.to_string();
    }

    pub async fn get(&self) -> f32 {
        let state = self.inner.lock().await;
        state.current
    }

    pub async fn get_stage(&self) -> String {
        let state = self.inner.lock().await;
        state.current_stage.clone()
    }

    pub async fn set_active(&self, analysis_id: Uuid) {
        let mut state = self.inner.lock().await;
        state.active_analysis_id = Some(analysis_id);
    }

    pub async fn clear_active(&self) {
        let mut state = self.inner.lock().await;
        state.active_analysis_id = None;
        state.current = 0.0;
        state.current_stage = "idle".to_string();
    }

    pub async fn get_active_id(&self) -> Option<Uuid> {
        let state = self.inner.lock().await;
        state.active_analysis_id
    }

    pub async fn is_idle(&self) -> bool {
        let state = self.inner.lock().await;
        state.active_analysis_id.is_none()
    }
}

impl Default for ProgressTracker {
    fn default() -> Self {
        ProgressTracker::new()
    }
}

// ============================================================
// ANALYSIS PIPELINE
// ============================================================

#[derive(Debug, Clone)]
pub struct AnalysisPipeline {
    pub stages: Vec<AnalysisStage>,
    pub config: AnalysisConfig,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl AnalysisPipeline {
    pub fn new(config: AnalysisConfig) -> Self {
        let stages = vec![
            AnalysisStage::new("preprocessing"),
            AnalysisStage::new("pattern_detection"),
            AnalysisStage::new("vulnerability_matching"),
            AnalysisStage::new("cross_referencing"),
            AnalysisStage::new("risk_scoring"),
            AnalysisStage::new("summarization"),
        ];

        AnalysisPipeline {
            stages,
            config,
            started_at: None,
            completed_at: None,
        }
    }

    pub fn start(&mut self) {
        self.started_at = Some(chrono::Utc::now());
        if let Some(stage) = self.stages.first_mut() {
            stage.start();
        }
    }

    pub fn advance_stage(&mut self, stage_name: &str) -> Result<(), AnalyzerContractError> {
        if let Some(current) = self.stages.iter_mut().find(|s| s.status == AnalysisStatus::PatternDetecting || s.status == AnalysisStatus::Pending) {
            current.complete();
        }

        if let Some(next) = self.stages.iter_mut().find(|s| s.name == stage_name && s.status == AnalysisStatus::Pending) {
            next.start();
            return Ok(());
        }

        Err(AnalyzerContractError::StageFailed(
            format!("Stage '{}' not found or already completed", stage_name)
        ))
    }

    pub fn complete(&mut self) {
        for stage in &mut self.stages {
            if stage.status != AnalysisStatus::Completed && stage.status != AnalysisStatus::Failed {
                stage.complete();
            }
        }
        self.completed_at = Some(chrono::Utc::now());
    }

    pub fn fail_stage(&mut self, stage_name: &str, error: &str) {
        if let Some(stage) = self.stages.iter_mut().find(|s| s.name == stage_name) {
            stage.fail(error.to_string());
        }
    }

    pub fn overall_progress(&self) -> f32 {
        if self.stages.is_empty() {
            return 0.0;
        }
        let completed = self.stages.iter()
            .filter(|s| s.status == AnalysisStatus::Completed)
            .count();
        (completed as f32 / self.stages.len() as f32) * 100.0
    }

    pub fn current_stage(&self) -> Option<&AnalysisStage> {
        self.stages.iter()
            .find(|s| s.status == AnalysisStatus::PatternDetecting)
    }

    pub fn duration_secs(&self) -> u64 {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => (end - start).num_seconds() as u64,
            (Some(start), None) => (chrono::Utc::now() - start).num_seconds() as u64,
            _ => 0,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.stages.iter().all(|s| {
            s.status == AnalysisStatus::Completed || s.status == AnalysisStatus::Failed
        })
    }

    pub fn has_failures(&self) -> bool {
        self.stages.iter().any(|s| s.status == AnalysisStatus::Failed)
    }
}

// ============================================================
// ANALYSIS RESULT BUILDER
// ============================================================

#[derive(Debug)]
pub struct AnalysisResultBuilder {
    analysis_id: Option<Uuid>,
    scan_id: Option<Uuid>,
    target_url: Option<String>,
    status: AnalysisStatus,
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    end_time: Option<chrono::DateTime<chrono::Utc>>,
    findings: Vec<Finding>,
    vulnerabilities: Vec<Vulnerability>,
    risk_assessment: Option<RiskAssessment>,
    correlations: Vec<Correlation>,
    anomalies: Vec<Anomaly>,
    patterns: Vec<PatternMatch>,
    summary: Option<String>,
    errors: Vec<crate::shared::contracts::analyzer_contract::AnalysisError>,
    stages_completed: Vec<String>,
    stages_failed: Vec<String>,
}

impl AnalysisResultBuilder {
    pub fn new() -> Self {
        AnalysisResultBuilder {
            analysis_id: None,
            scan_id: None,
            target_url: None,
            status: AnalysisStatus::Pending,
            start_time: None,
            end_time: None,
            findings: vec![],
            vulnerabilities: vec![],
            risk_assessment: None,
            correlations: vec![],
            anomalies: vec![],
            patterns: vec![],
            summary: None,
            errors: vec![],
            stages_completed: vec![],
            stages_failed: vec![],
        }
    }

    pub fn analysis_id(mut self, id: Uuid) -> Self {
        self.analysis_id = Some(id);
        self
    }

    pub fn scan_id(mut self, id: Uuid) -> Self {
        self.scan_id = Some(id);
        self
    }

    pub fn target_url(mut self, url: &str) -> Self {
        self.target_url = Some(url.to_string());
        self
    }

    pub fn status(mut self, status: AnalysisStatus) -> Self {
        self.status = status;
        self
    }

    pub fn started_at(mut self, time: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_time = Some(time);
        self
    }

    pub fn completed_at(mut self, time: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_time = Some(time);
        self
    }

    pub fn add_finding(mut self, finding: Finding) -> Self {
        self.findings.push(finding);
        self
    }

    pub fn add_findings(mut self, findings: Vec<Finding>) -> Self {
        self.findings.extend(findings);
        self
    }

    pub fn add_vulnerability(mut self, vuln: Vulnerability) -> Self {
        self.vulnerabilities.push(vuln);
        self
    }

    pub fn risk_assessment(mut self, assessment: RiskAssessment) -> Self {
        self.risk_assessment = Some(assessment);
        self
    }

    pub fn add_correlation(mut self, correlation: Correlation) -> Self {
        self.correlations.push(correlation);
        self
    }

    pub fn add_anomaly(mut self, anomaly: Anomaly) -> Self {
        self.anomalies.push(anomaly);
        self
    }

    pub fn add_pattern(mut self, pattern: PatternMatch) -> Self {
        self.patterns.push(pattern);
        self
    }

    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_string());
        self
    }

    pub fn add_error(
        mut self,
        error: crate::shared::contracts::analyzer_contract::AnalysisError,
    ) -> Self {
        self.errors.push(error);
        self
    }

    pub fn stage_completed(mut self, stage: &str) -> Self {
        self.stages_completed.push(stage.to_string());
        self
    }

    pub fn stage_failed(mut self, stage: &str) -> Self {
        self.stages_failed.push(stage.to_string());
        self
    }

    pub fn build(self) -> Result<AnalysisResult, AnalyzerContractError> {
        let analysis_id = self.analysis_id.unwrap_or_else(Uuid::new_v4);
        let scan_id = self.scan_id.unwrap_or_else(Uuid::nil);
        let target_url = self.target_url.unwrap_or_default();
        let start_time = self.start_time.unwrap_or_else(chrono::Utc::now);
        let end_time = self.end_time.unwrap_or_else(chrono::Utc::now);
        let duration_secs = (end_time - start_time).num_seconds() as u64;

        let total_findings = self.findings.len();
        let confidence_sum: f32 = self.findings.iter()
            .map(|f| f.confidence.to_score())
            .sum();
        let confidence_score = if total_findings > 0 {
            confidence_sum / total_findings as f32
        } else {
            0.0
        };

        Ok(AnalysisResult {
            analysis_id,
            scan_id,
            target_url,
            status: self.status,
            start_time,
            end_time: Some(end_time),
            duration_secs,
            findings: self.findings,
            vulnerabilities: self.vulnerabilities,
            risk_assessment: self.risk_assessment,
            correlations: self.correlations,
            anomalies: self.anomalies,
            patterns: self.patterns,
            summary: self.summary,
            confidence_score,
            stages_completed: self.stages_completed,
            stages_failed: self.stages_failed,
            errors: self.errors,
            metadata: crate::shared::contracts::analyzer_contract::AnalysisMetadata::default(),
        })
    }
}

impl Default for AnalysisResultBuilder {
    fn default() -> Self {
        AnalysisResultBuilder::new()
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::contracts::analyzer_contract::{
        FindingType, Severity, Confidence, AnalysisConfig,
        CvssVersion, RiskFormula,
    };

    struct TestAnalyzer {
        tracker: ProgressTracker,
    }

    impl TestAnalyzer {
        fn new() -> Self {
            TestAnalyzer {
                tracker: ProgressTracker::new(),
            }
        }
    }

    #[async_trait]
    impl Analyzer for TestAnalyzer {
        type Error = AnalyzerContractError;

        async fn analyze(
            &self,
            _data: ScanResult,
            _config: AnalysisConfig,
        ) -> Result<AnalysisResult, Self::Error> {
            self.tracker.update(50.0, "analyzing").await;
            Err(AnalyzerContractError::InternalError("not implemented".to_string()))
        }

        async fn cross_reference(
            &self,
            _result: AnalysisResult,
        ) -> Result<CrossReferenceResult, Self::Error> {
            Err(AnalyzerContractError::InternalError("not implemented".to_string()))
        }

        async fn calculate_risk(
            &self,
            _result: AnalysisResult,
        ) -> Result<RiskAssessment, Self::Error> {
            Err(AnalyzerContractError::InternalError("not implemented".to_string()))
        }

        async fn get_analysis_progress(&self) -> f32 {
            self.tracker.get().await
        }

        async fn progress(&self, _analysis_id: Uuid) -> Result<f32, Self::Error> {
            Ok(self.tracker.get().await)
        }

        async fn stages(&self, _analysis_id: Uuid) -> Result<Vec<AnalysisStage>, Self::Error> {
            Ok(vec![])
        }

        async fn cancel(&self, _analysis_id: Uuid) -> Result<(), Self::Error> {
            self.tracker.clear_active().await;
            Ok(())
        }

        fn capabilities(&self) -> AnalyzerCapabilities {
            AnalyzerCapabilities::default()
        }
    }

    fn make_finding(title: &str, severity: Severity, confidence: Confidence, module: &str) -> Finding {
        Finding {
            id: Uuid::new_v4(),
            finding_type: FindingType::Vulnerability,
            title: title.to_string(),
            description: "Test finding".to_string(),
            severity,
            confidence,
            source_module: module.to_string(),
            evidence: serde_json::json!({}),
            timestamp: chrono::Utc::now(),
            related_findings: vec![],
            false_positive_probability: 0.0,
        }
    }

    #[tokio::test]
    async fn test_progress_tracker_update() {
        let tracker = ProgressTracker::new();
        assert_eq!(tracker.get().await, 0.0);

        tracker.update(75.5, "vulnerability_matching").await;
        assert!((tracker.get().await - 75.5).abs() < 0.01);
        assert_eq!(tracker.get_stage().await, "vulnerability_matching");
    }

    #[tokio::test]
    async fn test_progress_tracker_clamp() {
        let tracker = ProgressTracker::new();
        tracker.update(150.0, "test").await;
        assert_eq!(tracker.get().await, 100.0);

        tracker.update(-10.0, "test").await;
        assert_eq!(tracker.get().await, 0.0);
    }

    #[tokio::test]
    async fn test_progress_tracker_active_id() {
        let tracker = ProgressTracker::new();
        assert!(tracker.is_idle().await);

        let id = Uuid::new_v4();
        tracker.set_active(id).await;
        assert!(!tracker.is_idle().await);
        assert_eq!(tracker.get_active_id().await, Some(id));

        tracker.clear_active().await;
        assert!(tracker.is_idle().await);
    }

    #[tokio::test]
    async fn test_analyzer_get_analysis_progress() {
        let analyzer = TestAnalyzer::new();
        // Default sebelum update
        assert_eq!(analyzer.get_analysis_progress().await, 0.0);

        // Update via analyze call
        analyzer.tracker.update(42.0, "testing").await;
        assert!((analyzer.get_analysis_progress().await - 42.0).abs() < 0.01);
    }

    #[test]
    fn test_analysis_pipeline_lifecycle() {
        let config = AnalysisConfig::default();
        let mut pipeline = AnalysisPipeline::new(config);

        assert_eq!(pipeline.stages.len(), 6);
        assert_eq!(pipeline.overall_progress(), 0.0);

        pipeline.start();
        assert!(pipeline.started_at.is_some());

        pipeline.advance_stage("preprocessing").unwrap();
        pipeline.advance_stage("pattern_detection").unwrap();

        assert!(pipeline.overall_progress() > 0.0);

        pipeline.fail_stage("vulnerability_matching", "CVE database not loaded");
        assert!(pipeline.has_failures());

        pipeline.complete();
        assert!(pipeline.is_complete());
        assert!(pipeline.duration_secs() > 0);
    }

    #[test]
    fn test_analysis_pipeline_advance_invalid_stage() {
        let config = AnalysisConfig::default();
        let mut pipeline = AnalysisPipeline::new(config);
        pipeline.start();

        let result = pipeline.advance_stage("nonexistent_stage");
        assert!(result.is_err());
    }

    #[test]
    fn test_analyzer_deduplicate() {
        let analyzer = TestAnalyzer::new();
        let f1 = make_finding("XSS Found", Severity::High, Confidence::Medium, "xss_detector");
        let f2 = make_finding("XSS Found", Severity::High, Confidence::High, "xss_detector");

        let deduped = analyzer.deduplicate(vec![f1, f2]);
        assert_eq!(deduped.len(), 1);
        assert_eq!(deduped[0].confidence, Confidence::High);
    }

    #[test]
    fn test_analyzer_filter_by_severity() {
        let analyzer = TestAnalyzer::new();
        let f1 = make_finding("Critical bug", Severity::Critical, Confidence::High, "module1");
        let f2 = make_finding("Low info", Severity::Info, Confidence::Medium, "module2");
        let f3 = make_finding("High vuln", Severity::High, Confidence::High, "module1");

        let filtered = analyzer.filter_by_severity(&[f1, f2, f3], Severity::High);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_analyzer_filter_by_confidence() {
        let analyzer = TestAnalyzer::new();
        let f1 = make_finding("A", Severity::High, Confidence::High, "m1");
        let f2 = make_finding("B", Severity::High, Confidence::Low, "m1");

        let filtered = analyzer.filter_by_confidence(&[f1, f2], Confidence::Medium);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn test_analyzer_group_by_type() {
        let analyzer = TestAnalyzer::new();
        let f1 = make_finding("A", Severity::High, Confidence::High, "m1");
        let f2 = make_finding("B", Severity::Medium, Confidence::Medium, "m1");
        let f3 = make_finding("C", Severity::Low, Confidence::Low, "m2");

        let groups = analyzer.group_by_type(&[f1, f2, f3]);
        assert_eq!(groups.len(), 1);
    }

    #[test]
    fn test_analyzer_severity_distribution() {
        let analyzer = TestAnalyzer::new();
        let f1 = make_finding("A", Severity::Critical, Confidence::High, "m1");
        let f2 = make_finding("B", Severity::High, Confidence::High, "m1");
        let f3 = make_finding("C", Severity::High, Confidence::Medium, "m2");

        let dist = analyzer.severity_distribution(&[f1, f2, f3]);
        assert_eq!(dist.get(&Severity::Critical), Some(&1));
        assert_eq!(dist.get(&Severity::High), Some(&2));
    }

    #[test]
    fn test_analysis_result_builder() {
        let analysis_id = Uuid::new_v4();
        let scan_id = Uuid::new_v4();
        let now = chrono::Utc::now();

        let result = AnalysisResultBuilder::new()
            .analysis_id(analysis_id)
            .scan_id(scan_id)
            .target_url("https://example.com")
            .status(AnalysisStatus::Completed)
            .started_at(now)
            .completed_at(now + chrono::Duration::seconds(120))
            .add_finding(make_finding("XSS", Severity::High, Confidence::High, "xss_detector"))
            .summary("Analysis complete")
            .stage_completed("preprocessing")
            .stage_completed("pattern_detection")
            .build();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.analysis_id, analysis_id);
        assert_eq!(result.scan_id, scan_id);
        assert_eq!(result.findings.len(), 1);
        assert_eq!(result.duration_secs, 120);
        assert_eq!(result.stages_completed.len(), 2);
    }

    #[test]
    fn test_analysis_result_builder_with_vulnerabilities() {
        let vuln = Vulnerability {
            id: Uuid::new_v4(),
            cve_id: Some("CVE-2024-0001".to_string()),
            title: "Test CVE".to_string(),
            description: "Test".to_string(),
            severity: Severity::Critical,
            cvss_score: 9.8,
            cvss_vector: Some("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H".to_string()),
            epss_score: Some(0.95),
            affected_component: "test".to_string(),
            affected_version: "1.0".to_string(),
            fixed_version: Some("1.1".to_string()),
            references: vec!["https://nvd.nist.gov".to_string()],
            remediation: Some("Update to version 1.1".to_string()),
            exploit_available: true,
            exploit_maturity: crate::shared::contracts::analyzer_contract::ExploitMaturity::Functional,
            discovered_at: chrono::Utc::now(),
            status: crate::shared::contracts::analyzer_contract::VulnerabilityStatus::Open,
        };

        let result = AnalysisResultBuilder::new()
            .scan_id(Uuid::new_v4())
            .target_url("https://vuln.com")
            .add_vulnerability(vuln)
            .build();

        assert!(result.is_ok());
        assert_eq!(result.unwrap().vulnerabilities.len(), 1);
    }
}
