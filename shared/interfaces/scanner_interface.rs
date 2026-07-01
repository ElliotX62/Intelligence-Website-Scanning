// shared/interfaces/scanner_interface.rs
// IWS v1.0 - Scanner Interface
// Mendefinisikan trait Scanner sebagai base abstraction untuk semua scanner

use std::net::IpAddr;
use std::time::Duration;
use async_trait::async_trait;
use uuid::Uuid;
use url::Url;
use anyhow::Result;

use crate::shared::contracts::scanner_contract::{
    ScanProfile, ScanResult, ScanProgress, ScanStatus, ScanRequest,
    ScannerContractError, RetryConfig, ScannerCapabilities,
};

// ============================================================
// SCANNER TRAIT
// ============================================================

#[async_trait]
pub trait Scanner: Send + Sync {
    type Error: std::error::Error + From<ScannerContractError> + Send + Sync;

    /// Scan URL target dengan profile default
    #[must_use = "scan result must be handled to ensure scan completion"]
    async fn scan(&self, url: Url) -> Result<ScanResult, Self::Error>;

    /// Scan URL target dengan profile tertentu
    #[must_use = "scan result must be handled to ensure scan completion"]
    async fn scan_with_profile(
        &self,
        url: Url,
        profile: ScanProfile,
    ) -> Result<ScanResult, Self::Error>;

    /// Batalkan scan yang sedang berjalan
    #[must_use = "cancel result must be checked to confirm cancellation"]
    async fn cancel(&self, scan_id: Uuid) -> Result<(), Self::Error>;

    /// Dapatkan status scan
    #[must_use = "status result must be handled"]
    async fn status(&self, scan_id: Uuid) -> Result<ScanStatus, Self::Error>;

    /// Dapatkan progress scan detail
    #[must_use = "progress result must be handled"]
    async fn progress(&self, scan_id: Uuid) -> Result<ScanProgress, Self::Error>;

    /// Dapatkan hasil scan lengkap
    #[must_use = "scan result must be consumed"]
    async fn result(&self, scan_id: Uuid) -> Result<ScanResult, Self::Error>;

    /// Jeda scan
    #[must_use = "pause result must be checked"]
    async fn pause(&self, scan_id: Uuid) -> Result<(), Self::Error>;

    /// Lanjutkan scan yang dijeda
    #[must_use = "resume result must be checked"]
    async fn resume(&self, scan_id: Uuid) -> Result<(), Self::Error>;

    /// Dapatkan capabilities scanner — SPEC FIX #8
    fn get_capabilities(&self) -> ScannerCapabilities;

    /// Scan dengan retry otomatis
    #[must_use = "scan result must be handled to ensure scan completion"]
    async fn scan_with_retry(
        &self,
        url: Url,
        profile: ScanProfile,
        max_retries: u8,
    ) -> Result<ScanResult, Self::Error> {
        let retry_config = RetryConfig {
            max_attempts: max_retries as u32,
            ..Default::default()
        };

        let mut last_error: Option<Self::Error> = None;

        for attempt in 0..=max_retries {
            match self.scan_with_profile(url.clone(), profile.clone()).await {
                Ok(result) => {
                    if result.status == ScanStatus::Completed {
                        return Ok(result);
                    }
                    last_error = Some(
                        ScannerContractError::ScanNotRunning(result.scan_id).into()
                    );
                }
                Err(e) => {
                    last_error = Some(e);
                }
            }

            if attempt < max_retries {
                let backoff = retry_config.calculate_backoff(attempt as u32);
                tokio::time::sleep(backoff).await;
            }
        }

        Err(last_error.unwrap_or_else(|| {
            ScannerContractError::InternalError(
                "Retry exhausted with no error".to_string()
            ).into()
        }))
    }

    /// Validasi target sebelum scan
    #[must_use = "validation result must be checked before proceeding"]
    fn validate_target(&self, url: &Url) -> Result<(), Self::Error> {
        if url.as_str().is_empty() {
            return Err(ScannerContractError::InvalidUrl("URL is empty".to_string()).into());
        }
        let scheme = url.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(ScannerContractError::InvalidUrl(
                format!("Unsupported scheme: {}. Only http/https allowed", scheme)
            ).into());
        }
        if url.host_str().is_none() {
            return Err(ScannerContractError::InvalidUrl(
                "URL has no valid host".to_string()
            ).into());
        }
        Ok(())
    }

    /// Cek apakah scanner sedang sibuk
    async fn is_busy(&self) -> bool {
        false
    }

    /// Dapatkan jumlah scan yang sedang berjalan
    async fn active_scan_count(&self) -> usize {
        0
    }
}

// ============================================================
// SCAN CONTEXT
// ============================================================

#[derive(Debug, Clone)]
pub struct ScanContext {
    pub scan_id: Uuid,
    pub target_url: Url,
    pub target_ip: Option<IpAddr>,
    pub profile: ScanProfile,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub timeout: Duration,
    pub metadata: serde_json::Value,
}

impl ScanContext {
    pub fn new(
        scan_id: Uuid,
        target_url: Url,
        profile: ScanProfile,
        timeout: Duration,
    ) -> Self {
        ScanContext {
            scan_id,
            target_url,
            target_ip: None,
            profile,
            started_at: chrono::Utc::now(),
            timeout,
            metadata: serde_json::json!({}),
        }
    }

    pub fn with_ip(mut self, ip: IpAddr) -> Self {
        self.target_ip = Some(ip);
        self
    }

    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn elapsed(&self) -> Duration {
        let now = chrono::Utc::now();
        let elapsed = now - self.started_at;
        Duration::from_secs(elapsed.num_seconds() as u64)
    }

    pub fn is_timed_out(&self) -> bool {
        self.elapsed() > self.timeout
    }

    pub fn remaining(&self) -> Duration {
        let elapsed = self.elapsed();
        if elapsed >= self.timeout {
            Duration::from_secs(0)
        } else {
            self.timeout - elapsed
        }
    }
}

// ============================================================
// SCAN RESULT BUILDER
// ============================================================

#[derive(Debug)]
pub struct ScanResultBuilder {
    scan_id: Option<Uuid>,
    target_url: Option<Url>,
    target_ip: Option<IpAddr>,
    profile: Option<ScanProfile>,
    status: ScanStatus,
    start_time: Option<chrono::DateTime<chrono::Utc>>,
    end_time: Option<chrono::DateTime<chrono::Utc>>,
    modules_results: Vec<crate::shared::contracts::scanner_contract::ModuleResult>,
    errors: Vec<crate::shared::contracts::scanner_contract::ScanErrorEntry>,
    summary: Option<String>,
}

impl ScanResultBuilder {
    pub fn new() -> Self {
        ScanResultBuilder {
            scan_id: None,
            target_url: None,
            target_ip: None,
            profile: None,
            status: ScanStatus::Pending,
            start_time: None,
            end_time: None,
            modules_results: vec![],
            errors: vec![],
            summary: None,
        }
    }

    pub fn scan_id(mut self, id: Uuid) -> Self {
        self.scan_id = Some(id);
        self
    }

    pub fn target_url(mut self, url: Url) -> Self {
        self.target_url = Some(url);
        self
    }

    pub fn target_ip(mut self, ip: IpAddr) -> Self {
        self.target_ip = Some(ip);
        self
    }

    pub fn profile(mut self, profile: ScanProfile) -> Self {
        self.profile = Some(profile);
        self
    }

    pub fn status(mut self, status: ScanStatus) -> Self {
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

    pub fn add_module_result(
        mut self,
        result: crate::shared::contracts::scanner_contract::ModuleResult,
    ) -> Self {
        self.modules_results.push(result);
        self
    }

    pub fn add_error(
        mut self,
        error: crate::shared::contracts::scanner_contract::ScanErrorEntry,
    ) -> Self {
        self.errors.push(error);
        self
    }

    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_string());
        self
    }

    #[must_use = "build result must be handled — returns Result<ScanResult, ScannerContractError>"]
    pub fn build(self) -> Result<ScanResult, ScannerContractError> {
        let scan_id = self.scan_id.unwrap_or_else(Uuid::new_v4);
        let target_url = self.target_url.ok_or_else(|| {
            ScannerContractError::InvalidUrl("target_url is required".to_string())
        })?;
        let profile = self.profile.ok_or_else(|| {
            ScannerContractError::InvalidConfiguration("profile is required".to_string())
        })?;
        let start_time = self.start_time.unwrap_or_else(chrono::Utc::now);
        let end_time = self.end_time.unwrap_or_else(chrono::Utc::now);
        let duration_secs = (end_time - start_time).num_seconds() as u64;

        let findings_count = self.modules_results.iter()
            .map(|m| m.data.get("findings").and_then(|f| f.as_array()).map(|a| a.len()).unwrap_or(0))
            .sum();

        let vulnerabilities_count = self.modules_results.iter()
            .filter(|m| m.module_type == crate::shared::contracts::scanner_contract::ModuleType::Security)
            .count();

        Ok(ScanResult {
            scan_id,
            target_url,
            target_ip: self.target_ip,
            profile,
            status: self.status,
            start_time,
            end_time: Some(end_time),
            duration_secs,
            modules_results: self.modules_results,
            findings_count,
            vulnerabilities_count,
            risk_score: None,
            summary: self.summary,
            errors: self.errors,
            metadata: crate::shared::contracts::scanner_contract::ScanMetadata::default(),
        })
    }
}

impl Default for ScanResultBuilder {
    fn default() -> Self {
        ScanResultBuilder::new()
    }
}

// ============================================================
// SCAN EVENT EMITTER
// ============================================================

#[async_trait]
pub trait ScanEventEmitter: Send + Sync {
    /// Emit event saat scan dimulai
    async fn emit_scan_started(&self, scan_id: Uuid, url: &Url);

    /// Emit event progress scan
    async fn emit_scan_progress(&self, scan_id: Uuid, progress: f32, current_module: &str);

    /// Emit event module selesai
    async fn emit_module_completed(&self, scan_id: Uuid, module_name: &str, duration_ms: u64);

    /// Emit event module gagal
    async fn emit_module_failed(&self, scan_id: Uuid, module_name: &str, error: &str);

    /// Emit event scan selesai
    async fn emit_scan_completed(&self, scan_id: Uuid, findings_count: usize, risk_score: Option<f32>);

    /// Emit event scan gagal
    async fn emit_scan_failed(&self, scan_id: Uuid, error: &str);

    /// Emit event scan dibatalkan
    async fn emit_scan_cancelled(&self, scan_id: Uuid, reason: Option<&str>);
}

// ============================================================
// DEFAULT EVENT EMITTER (LOG-BASED)
// ============================================================

pub struct LogEventEmitter;

impl LogEventEmitter {
    pub fn new() -> Self {
        LogEventEmitter
    }
}

#[async_trait]
impl ScanEventEmitter for LogEventEmitter {
    async fn emit_scan_started(&self, scan_id: Uuid, url: &Url) {
        tracing::info!(
            scan_id = %scan_id,
            url = %url,
            "Scan started"
        );
    }

    async fn emit_scan_progress(&self, scan_id: Uuid, progress: f32, current_module: &str) {
        tracing::debug!(
            scan_id = %scan_id,
            progress = progress,
            module = %current_module,
            "Scan progress"
        );
    }

    async fn emit_module_completed(&self, scan_id: Uuid, module_name: &str, duration_ms: u64) {
        tracing::info!(
            scan_id = %scan_id,
            module = %module_name,
            duration_ms = duration_ms,
            "Module completed"
        );
    }

    async fn emit_module_failed(&self, scan_id: Uuid, module_name: &str, error: &str) {
        tracing::warn!(
            scan_id = %scan_id,
            module = %module_name,
            error = %error,
            "Module failed"
        );
    }

    async fn emit_scan_completed(&self, scan_id: Uuid, findings_count: usize, risk_score: Option<f32>) {
        tracing::info!(
            scan_id = %scan_id,
            findings = findings_count,
            risk_score = ?risk_score,
            "Scan completed"
        );
    }

    async fn emit_scan_failed(&self, scan_id: Uuid, error: &str) {
        tracing::error!(
            scan_id = %scan_id,
            error = %error,
            "Scan failed"
        );
    }

    async fn emit_scan_cancelled(&self, scan_id: Uuid, reason: Option<&str>) {
        tracing::warn!(
            scan_id = %scan_id,
            reason = ?reason,
            "Scan cancelled"
        );
    }
}

impl Default for LogEventEmitter {
    fn default() -> Self {
        LogEventEmitter::new()
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::contracts::scanner_contract::{
        ScanProfile, ScanStatus, ScanProfileType,
        ModuleResult, ModuleType, ScanErrorEntry,
    };

    struct TestScanner;

    #[async_trait]
    impl Scanner for TestScanner {
        type Error = ScannerContractError;

        async fn scan(&self, _url: Url) -> Result<ScanResult, Self::Error> {
            Err(ScannerContractError::InternalError("not implemented".to_string()))
        }

        async fn scan_with_profile(
            &self,
            _url: Url,
            _profile: ScanProfile,
        ) -> Result<ScanResult, Self::Error> {
            Err(ScannerContractError::InternalError("not implemented".to_string()))
        }

        async fn cancel(&self, _scan_id: Uuid) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn status(&self, _scan_id: Uuid) -> Result<ScanStatus, Self::Error> {
            Ok(ScanStatus::Completed)
        }

        async fn progress(&self, _scan_id: Uuid) -> Result<ScanProgress, Self::Error> {
            Err(ScannerContractError::ScanNotFound(Uuid::nil()))
        }

        async fn result(&self, _scan_id: Uuid) -> Result<ScanResult, Self::Error> {
            Err(ScannerContractError::ScanNotFound(Uuid::nil()))
        }

        async fn pause(&self, _scan_id: Uuid) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn resume(&self, _scan_id: Uuid) -> Result<(), Self::Error> {
            Ok(())
        }

        fn get_capabilities(&self) -> ScannerCapabilities {
            ScannerCapabilities::default()
        }
    }

    #[test]
    fn test_scanner_get_capabilities() {
        let scanner = TestScanner;
        let caps = scanner.get_capabilities();
        assert!(caps.supported_modules.len() > 0);
        assert_eq!(caps.api_version, "v1.0");
    }

    #[test]
    fn test_scan_context_creation() {
        let scan_id = Uuid::new_v4();
        let url = Url::parse("https://example.com").unwrap();
        let profile = ScanProfile::moderate();
        let timeout = Duration::from_secs(300);

        let ctx = ScanContext::new(scan_id, url.clone(), profile, timeout);
        assert_eq!(ctx.scan_id, scan_id);
        assert_eq!(ctx.target_url, url);
        assert!(!ctx.is_timed_out());
    }

    #[test]
    fn test_scan_context_timeout() {
        let scan_id = Uuid::new_v4();
        let url = Url::parse("https://example.com").unwrap();
        let profile = ScanProfile::moderate();
        let timeout = Duration::from_secs(0);

        let ctx = ScanContext::new(scan_id, url, profile, timeout);
        std::thread::sleep(Duration::from_millis(10));
        assert!(ctx.is_timed_out());
        assert_eq!(ctx.remaining(), Duration::from_secs(0));
    }

    #[test]
    fn test_scan_result_builder() {
        let scan_id = Uuid::new_v4();
        let url = Url::parse("https://example.com").unwrap();
        let now = chrono::Utc::now();

        let result = ScanResultBuilder::new()
            .scan_id(scan_id)
            .target_url(url.clone())
            .profile(ScanProfile::moderate())
            .status(ScanStatus::Completed)
            .started_at(now)
            .completed_at(now + chrono::Duration::seconds(60))
            .summary("Test scan completed")
            .build();

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.scan_id, scan_id);
        assert_eq!(result.target_url, url);
        assert_eq!(result.status, ScanStatus::Completed);
        assert_eq!(result.duration_secs, 60);
    }

    #[test]
    fn test_scan_result_builder_missing_url() {
        let result = ScanResultBuilder::new()
            .profile(ScanProfile::moderate())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_target_valid() {
        let scanner = TestScanner;
        let url = Url::parse("https://example.com").unwrap();
        assert!(scanner.validate_target(&url).is_ok());
    }

    #[test]
    fn test_validate_target_invalid_scheme() {
        let scanner = TestScanner;
        let url = Url::parse("ftp://example.com").unwrap();
        assert!(scanner.validate_target(&url).is_err());
    }

    #[test]
    fn test_validate_target_empty() {
        let scanner = TestScanner;
        let url = Url::parse("").unwrap_or_else(|_| Url::parse("http://localhost").unwrap());
        assert!(scanner.validate_target(&url).is_err());
    }

    #[test]
    fn test_log_event_emitter() {
        let emitter = LogEventEmitter::new();
        let scan_id = Uuid::new_v4();
        let url = Url::parse("https://example.com").unwrap();

        tokio_test::block_on(async {
            emitter.emit_scan_started(scan_id, &url).await;
            emitter.emit_scan_progress(scan_id, 50.0, "port_scanner").await;
            emitter.emit_module_completed(scan_id, "dns_enum", 1500).await;
            emitter.emit_module_failed(scan_id, "xss_detector", "timeout").await;
            emitter.emit_scan_completed(scan_id, 5, Some(7.5)).await;
            emitter.emit_scan_failed(scan_id, "connection lost").await;
            emitter.emit_scan_cancelled(scan_id, Some("user requested")).await;
        });
    }

    #[test]
    fn test_scanner_default_methods() {
        let scanner = TestScanner;
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            assert!(!scanner.is_busy().await);
            assert_eq!(scanner.active_scan_count().await, 0);
        });
    }

    #[test]
    fn test_scan_context_with_ip() {
        let scan_id = Uuid::new_v4();
        let url = Url::parse("https://example.com").unwrap();
        let profile = ScanProfile::moderate();
        let ip: IpAddr = "93.184.216.34".parse().unwrap();

        let ctx = ScanContext::new(scan_id, url, profile, Duration::from_secs(300))
            .with_ip(ip)
            .with_metadata(serde_json::json!({"priority": "high"}));

        assert_eq!(ctx.target_ip, Some(ip));
        assert_eq!(ctx.metadata["priority"], "high");
    }
}
