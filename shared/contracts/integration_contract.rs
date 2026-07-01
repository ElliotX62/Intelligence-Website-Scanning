// shared/contracts/integration_contract.rs
// IWS v1.0 - Integration Contract
// Mendefinisikan kontrak formal untuk semua third-party integrations

use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use url::Url;

// ============================================================
// INTEGRATION TYPES & CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IntegrationType {
    Shodan,
    Censys,
    VirusTotal,
    AlienVaultOTX,
    UrlScan,
    SecurityTrails,
    CrtSh,
    Dnsdb,
    GreyHatWarfare,
    Custom(String),
}

impl fmt::Display for IntegrationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationType::Shodan => write!(f, "shodan"),
            IntegrationType::Censys => write!(f, "censys"),
            IntegrationType::VirusTotal => write!(f, "virustotal"),
            IntegrationType::AlienVaultOTX => write!(f, "alienvault_otx"),
            IntegrationType::UrlScan => write!(f, "urlscan"),
            IntegrationType::SecurityTrails => write!(f, "securitytrails"),
            IntegrationType::CrtSh => write!(f, "crtsh"),
            IntegrationType::Dnsdb => write!(f, "dnsdb"),
            IntegrationType::GreyHatWarfare => write!(f, "greyhat_warfare"),
            IntegrationType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticated,
    Degraded,
    Reconnecting,
    Disconnecting,
    Failed,
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionState::Disconnected => write!(f, "disconnected"),
            ConnectionState::Connecting => write!(f, "connecting"),
            ConnectionState::Connected => write!(f, "connected"),
            ConnectionState::Authenticated => write!(f, "authenticated"),
            ConnectionState::Degraded => write!(f, "degraded"),
            ConnectionState::Reconnecting => write!(f, "reconnecting"),
            ConnectionState::Disconnecting => write!(f, "disconnecting"),
            ConnectionState::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub integration_type: IntegrationType,
    pub base_url: Url,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub rate_limit_rps: f64,
    pub max_connections: usize,
    pub connection_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub enable_cache: bool,
    pub cache_ttl_secs: u64,
    pub headers: HashMap<String, String>,
    pub custom_config: serde_json::Value,
}

impl IntegrationConfig {
    pub fn new(
        integration_type: IntegrationType,
        base_url: Url,
        api_key: &str,
    ) -> Self {
        IntegrationConfig {
            integration_type,
            base_url,
            api_key: api_key.to_string(),
            api_secret: None,
            timeout_secs: 30,
            max_retries: 3,
            rate_limit_rps: 1.0,
            max_connections: 5,
            connection_timeout_secs: 10,
            idle_timeout_secs: 300,
            enable_cache: true,
            cache_ttl_secs: 3600,
            headers: HashMap::new(),
            custom_config: serde_json::json!({}),
        }
    }

    pub fn with_secret(mut self, secret: &str) -> Self {
        self.api_secret = Some(secret.to_string());
        self
    }

    pub fn with_rate_limit(mut self, rps: f64) -> Self {
        self.rate_limit_rps = rps;
        self
    }

    pub fn validate(&self) -> Result<(), IntegrationContractError> {
        if self.api_key.is_empty() || self.api_key == "YOUR_API_KEY_HERE" {
            return Err(IntegrationContractError::AuthenticationFailed(
                "API key is missing or still using placeholder".to_string()
            ));
        }
        if self.timeout_secs == 0 || self.timeout_secs > 300 {
            return Err(IntegrationContractError::InvalidConfiguration(
                format!("timeout_secs must be 1-300, got {}", self.timeout_secs)
            ));
        }
        if self.rate_limit_rps <= 0.0 || self.rate_limit_rps > 100.0 {
            return Err(IntegrationContractError::InvalidConfiguration(
                format!("rate_limit_rps must be 0-100, got {}", self.rate_limit_rps)
            ));
        }
        Ok(())
    }
}

impl Default for IntegrationConfig {
    fn default() -> Self {
        IntegrationConfig {
            integration_type: IntegrationType::Custom("unknown".to_string()),
            base_url: Url::parse("http://localhost").unwrap(),
            api_key: "YOUR_API_KEY_HERE".to_string(),
            api_secret: None,
            timeout_secs: 30,
            max_retries: 3,
            rate_limit_rps: 1.0,
            max_connections: 5,
            connection_timeout_secs: 10,
            idle_timeout_secs: 300,
            enable_cache: true,
            cache_ttl_secs: 3600,
            headers: HashMap::new(),
            custom_config: serde_json::json!({}),
        }
    }
}

// ============================================================
// RETRY POLICY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_attempts: u32,
    pub initial_backoff_secs: f64,
    pub max_backoff_secs: f64,
    pub multiplier: f64,
    pub jitter: f64,
    pub retry_on_status: Vec<u16>,
    pub retry_on_timeout: bool,
    pub retry_on_connection_error: bool,
    pub retry_on_rate_limit: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        RetryPolicy {
            max_attempts: 3,
            initial_backoff_secs: 1.0,
            max_backoff_secs: 30.0,
            multiplier: 2.0,
            jitter: 0.1,
            retry_on_status: vec![408, 429, 500, 502, 503, 504],
            retry_on_timeout: true,
            retry_on_connection_error: true,
            retry_on_rate_limit: true,
        }
    }
}

impl RetryPolicy {
    pub fn calculate_backoff(&self, attempt: u32) -> Duration {
        let base = self.initial_backoff_secs * self.multiplier.powi(attempt as i32);
        let capped = base.min(self.max_backoff_secs);
        let jitter_amount = capped * self.jitter;
        let with_jitter = capped + (rand::random::<f64>() * jitter_amount * 2.0 - jitter_amount);
        Duration::from_secs_f64(with_jitter.max(0.0))
    }

    pub fn should_retry_http_status(&self, status: u16) -> bool {
        self.retry_on_status.contains(&status)
    }
}

// ============================================================
// REQUEST & RESPONSE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationRequest {
    pub request_id: Uuid,
    pub method: HttpMethod,
    pub path: String,
    pub query_params: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub content_type: Option<String>,
    pub timeout_secs: u64,
    pub retry_policy: RetryPolicy,
    pub metadata: serde_json::Value,
}

impl IntegrationRequest {
    pub fn new(method: HttpMethod, path: &str) -> Self {
        IntegrationRequest {
            request_id: Uuid::new_v4(),
            method,
            path: path.to_string(),
            query_params: HashMap::new(),
            headers: HashMap::new(),
            body: None,
            content_type: None,
            timeout_secs: 30,
            retry_policy: RetryPolicy::default(),
            metadata: serde_json::json!({}),
        }
    }

    pub fn with_query_param(mut self, key: &str, value: &str) -> Self {
        self.query_params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_json_body<T: Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(serde_json::to_vec(body)?);
        self.content_type = Some("application/json".to_string());
        Ok(self)
    }

    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationResponse {
    pub request_id: Uuid,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub elapsed_ms: u64,
    pub retry_count: u32,
    pub cached: bool,
    pub timestamp: DateTime<Utc>,
}

impl IntegrationResponse {
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    pub fn is_rate_limited(&self) -> bool {
        self.status_code == 429
    }

    pub fn is_server_error(&self) -> bool {
        self.status_code >= 500
    }

    pub fn get_header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn parse_json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    pub fn get_retry_after_secs(&self) -> Option<u64> {
        self.headers
            .get("retry-after")
            .and_then(|v| v.parse::<u64>().ok())
    }
}

// ============================================================
// RATE LIMIT STATUS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitStatus {
    pub limit: u64,
    pub remaining: u64,
    pub reset_at: DateTime<Utc>,
    pub window_secs: u64,
}

impl RateLimitStatus {
    pub fn is_exhausted(&self) -> bool {
        self.remaining == 0
    }

    pub fn usage_percent(&self) -> f64 {
        if self.limit == 0 {
            return 100.0;
        }
        ((self.limit - self.remaining) as f64 / self.limit as f64) * 100.0
    }

    pub fn time_until_reset(&self) -> Duration {
        let now = Utc::now();
        if self.reset_at > now {
            let diff = self.reset_at - now;
            Duration::from_secs(diff.num_seconds() as u64)
        } else {
            Duration::from_secs(0)
        }
    }
}

// ============================================================
// INTEGRATION HEALTH
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
            HealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationHealth {
    pub status: HealthStatus,
    pub last_check: DateTime<Utc>,
    pub latency_ms: u64,
    pub success_rate: f64,
    pub error_rate: f64,
    pub uptime_percent: f64,
    pub message: Option<String>,
}

impl IntegrationHealth {
    pub fn healthy(latency_ms: u64) -> Self {
        IntegrationHealth {
            status: HealthStatus::Healthy,
            last_check: Utc::now(),
            latency_ms,
            success_rate: 100.0,
            error_rate: 0.0,
            uptime_percent: 100.0,
            message: None,
        }
    }

    pub fn unhealthy(message: &str) -> Self {
        IntegrationHealth {
            status: HealthStatus::Unhealthy,
            last_check: Utc::now(),
            latency_ms: 0,
            success_rate: 0.0,
            error_rate: 100.0,
            uptime_percent: 0.0,
            message: Some(message.to_string()),
        }
    }
}

// ============================================================
// INTEGRATION CAPABILITIES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCapabilities {
    pub supports_ip_lookup: bool,
    pub supports_domain_lookup: bool,
    pub supports_url_scan: bool,
    pub supports_file_scan: bool,
    pub supports_certificate_lookup: bool,
    pub supports_dns_history: bool,
    pub supports_whois: bool,
    pub supports_threat_intel: bool,
    pub supports_reputation: bool,
    pub supports_blacklist: bool,
    pub max_batch_size: usize,
    pub requires_authentication: bool,
    pub api_version: String,
}

impl Default for IntegrationCapabilities {
    fn default() -> Self {
        IntegrationCapabilities {
            supports_ip_lookup: false,
            supports_domain_lookup: true,
            supports_url_scan: false,
            supports_file_scan: false,
            supports_certificate_lookup: false,
            supports_dns_history: false,
            supports_whois: false,
            supports_threat_intel: false,
            supports_reputation: false,
            supports_blacklist: false,
            max_batch_size: 1,
            requires_authentication: true,
            api_version: "v1".to_string(),
        }
    }
}

// ============================================================
// INTEGRATION METRICS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub rate_limited_requests: u64,
    pub average_latency_ms: f64,
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub retry_count: u64,
    pub last_request_time: Option<DateTime<Utc>>,
    pub last_error_time: Option<DateTime<Utc>>,
}

impl Default for IntegrationMetrics {
    fn default() -> Self {
        IntegrationMetrics {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            rate_limited_requests: 0,
            average_latency_ms: 0.0,
            p50_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
            bytes_sent: 0,
            bytes_received: 0,
            cache_hits: 0,
            cache_misses: 0,
            retry_count: 0,
            last_request_time: None,
            last_error_time: None,
        }
    }
}

// ============================================================
// CREDENTIALS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub integration_type: IntegrationType,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub custom_fields: HashMap<String, String>,
}

impl Credentials {
    pub fn api_key(integration_type: IntegrationType, key: &str) -> Self {
        Credentials {
            integration_type,
            api_key: key.to_string(),
            api_secret: None,
            username: None,
            password: None,
            token: None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn api_key_secret(integration_type: IntegrationType, key: &str, secret: &str) -> Self {
        Credentials {
            integration_type,
            api_key: key.to_string(),
            api_secret: Some(secret.to_string()),
            username: None,
            password: None,
            token: None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn mask_sensitive(&self) -> Self {
        let mask = |s: &str| -> String {
            if s.len() <= 8 {
                return "***".to_string();
            }
            format!("{}...{}", &s[..4], &s[s.len()-4..])
        };

        Credentials {
            integration_type: self.integration_type.clone(),
            api_key: mask(&self.api_key),
            api_secret: self.api_secret.as_ref().map(|s| mask(s)),
            username: self.username.clone(),
            password: self.password.as_ref().map(|_| "***".to_string()),
            token: self.token.as_ref().map(|s| mask(s)),
            custom_fields: self.custom_fields.iter().map(|(k, _)| (k.clone(), "***".to_string())).collect(),
        }
    }
}

// ============================================================
// INTEGRATION CONTRACT TRAIT
// ============================================================

#[async_trait]
pub trait IntegrationContract: Send + Sync {
    /// Koneksi ke service
    /// Postconditions: connection_state == Connected || Authenticated
    async fn connect(
        &mut self,
        config: IntegrationConfig,
    ) -> Result<(), IntegrationContractError>;

    /// Query ke service
    async fn query(
        &self,
        request: IntegrationRequest,
    ) -> Result<IntegrationResponse, IntegrationContractError>;

    /// Validasi credentials
    async fn validate_credentials(
        &self,
        creds: Credentials,
    ) -> Result<bool, IntegrationContractError>;

    /// Disconnect dari service
    async fn disconnect(&mut self) -> Result<(), IntegrationContractError>;

    /// Mendapatkan connection state
    fn get_connection_state(&self) -> ConnectionState;

    /// Mendapatkan rate limit status
    async fn get_rate_limit_status(&self) -> Result<RateLimitStatus, IntegrationContractError>;

    /// Mendapatkan health status
    async fn get_health_status(&self) -> Result<IntegrationHealth, IntegrationContractError>;

    /// Mendapatkan capabilities
    fn get_capabilities(&self) -> IntegrationCapabilities;

    /// Mendapatkan metrics
    fn get_metrics(&self) -> IntegrationMetrics;

    /// Mendapatkan tipe integrasi
    fn get_type(&self) -> IntegrationType;

    /// Cek rate limit sebelum request
    async fn check_rate_limit(&self) -> Result<bool, IntegrationContractError> {
        let status = self.get_rate_limit_status().await?;
        Ok(!status.is_exhausted())
    }

    /// Query dengan retry
    async fn query_with_retry(
        &self,
        request: IntegrationRequest,
    ) -> Result<IntegrationResponse, IntegrationContractError> {
        let mut last_error = None;
        let max_attempts = request.retry_policy.max_attempts;

        for attempt in 0..=max_attempts {
            match self.query(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) if attempt < max_attempts && e.is_recoverable() => {
                    let backoff = request.retry_policy.calculate_backoff(attempt);
                    tokio::time::sleep(backoff).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or_else(|| {
            IntegrationContractError::InternalError(
                "Retry exhausted with no error".to_string()
            )
        }))
    }
}

// ============================================================
// INTEGRATION CONNECTION
// ============================================================

#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_id: Uuid,
    pub integration_type: IntegrationType,
    pub state: ConnectionState,
    pub established_at: Option<DateTime<Utc>>,
    pub last_activity: Option<DateTime<Utc>>,
    pub config: IntegrationConfig,
}

impl Connection {
    pub fn new(config: IntegrationConfig) -> Self {
        Connection {
            connection_id: Uuid::new_v4(),
            integration_type: config.integration_type.clone(),
            state: ConnectionState::Disconnected,
            established_at: None,
            last_activity: None,
            config,
        }
    }

    pub fn is_idle(&self, idle_timeout_secs: u64) -> bool {
        if let Some(last) = self.last_activity {
            let elapsed = Utc::now() - last;
            elapsed.num_seconds() as u64 > idle_timeout_secs
        } else {
            false
        }
    }

    pub fn needs_reconnect(&self) -> bool {
        matches!(
            self.state,
            ConnectionState::Disconnected
                | ConnectionState::Failed
                | ConnectionState::Degraded
        )
    }
}

// ============================================================
// ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationContractError {
    InvalidConfiguration(String),
    AuthenticationFailed(String),
    AuthorizationFailed(String),
    RateLimitExceeded(String),
    ConnectionFailed(String),
    ConnectionTimeout(String),
    RequestTimeout(String),
    InvalidResponse(String),
    ServiceUnavailable(String),
    QuotaExceeded(String),
    InvalidCredentials(String),
    UnsupportedFeature(String),
    CacheError(String),
    ParseError(String),
    InternalError(String),
}

impl fmt::Display for IntegrationContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegrationContractError::InvalidConfiguration(msg) => write!(f, "Invalid config: {}", msg),
            IntegrationContractError::AuthenticationFailed(msg) => write!(f, "Auth failed: {}", msg),
            IntegrationContractError::AuthorizationFailed(msg) => write!(f, "Authorization failed: {}", msg),
            IntegrationContractError::RateLimitExceeded(msg) => write!(f, "Rate limit: {}", msg),
            IntegrationContractError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            IntegrationContractError::ConnectionTimeout(msg) => write!(f, "Connection timeout: {}", msg),
            IntegrationContractError::RequestTimeout(msg) => write!(f, "Request timeout: {}", msg),
            IntegrationContractError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            IntegrationContractError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
            IntegrationContractError::QuotaExceeded(msg) => write!(f, "Quota exceeded: {}", msg),
            IntegrationContractError::InvalidCredentials(msg) => write!(f, "Invalid credentials: {}", msg),
            IntegrationContractError::UnsupportedFeature(msg) => write!(f, "Unsupported: {}", msg),
            IntegrationContractError::CacheError(msg) => write!(f, "Cache error: {}", msg),
            IntegrationContractError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            IntegrationContractError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for IntegrationContractError {}

impl IntegrationContractError {
    pub fn code(&self) -> &str {
        match self {
            IntegrationContractError::InvalidConfiguration(_) => "I8001",
            IntegrationContractError::AuthenticationFailed(_) => "I8002",
            IntegrationContractError::AuthorizationFailed(_) => "I8003",
            IntegrationContractError::RateLimitExceeded(_) => "I8004",
            IntegrationContractError::ConnectionFailed(_) => "I8005",
            IntegrationContractError::ConnectionTimeout(_) => "I8006",
            IntegrationContractError::RequestTimeout(_) => "I8007",
            IntegrationContractError::InvalidResponse(_) => "I8008",
            IntegrationContractError::ServiceUnavailable(_) => "I8009",
            IntegrationContractError::QuotaExceeded(_) => "I8010",
            IntegrationContractError::InvalidCredentials(_) => "I8011",
            IntegrationContractError::UnsupportedFeature(_) => "I8012",
            IntegrationContractError::CacheError(_) => "I8013",
            IntegrationContractError::ParseError(_) => "I8014",
            IntegrationContractError::InternalError(_) => "I8015",
        }
    }

    pub fn severity(&self) -> &str {
        match self {
            IntegrationContractError::InvalidConfiguration(_) => "high",
            IntegrationContractError::AuthenticationFailed(_) => "critical",
            IntegrationContractError::AuthorizationFailed(_) => "high",
            IntegrationContractError::RateLimitExceeded(_) => "medium",
            IntegrationContractError::ConnectionFailed(_) => "high",
            IntegrationContractError::ConnectionTimeout(_) => "medium",
            IntegrationContractError::RequestTimeout(_) => "medium",
            IntegrationContractError::InvalidResponse(_) => "medium",
            IntegrationContractError::ServiceUnavailable(_) => "high",
            IntegrationContractError::QuotaExceeded(_) => "medium",
            IntegrationContractError::InvalidCredentials(_) => "critical",
            IntegrationContractError::UnsupportedFeature(_) => "low",
            IntegrationContractError::CacheError(_) => "low",
            IntegrationContractError::ParseError(_) => "medium",
            IntegrationContractError::InternalError(_) => "critical",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            IntegrationContractError::RateLimitExceeded(_)
                | IntegrationContractError::ConnectionFailed(_)
                | IntegrationContractError::ConnectionTimeout(_)
                | IntegrationContractError::RequestTimeout(_)
                | IntegrationContractError::ServiceUnavailable(_)
                | IntegrationContractError::CacheError(_)
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
    fn test_integration_config_validation_missing_key() {
        let config = IntegrationConfig::default();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_integration_config_validation_valid() {
        let config = IntegrationConfig::new(
            IntegrationType::Shodan,
            Url::parse("https://api.shodan.io").unwrap(),
            "valid_api_key_12345",
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_retry_policy_backoff() {
        let policy = RetryPolicy::default();
        let backoff_0 = policy.calculate_backoff(0);
        let backoff_1 = policy.calculate_backoff(1);
        let backoff_2 = policy.calculate_backoff(2);
        assert!(backoff_1 > backoff_0);
        assert!(backoff_2 > backoff_1);
    }

    #[test]
    fn test_retry_policy_http_status() {
        let policy = RetryPolicy::default();
        assert!(policy.should_retry_http_status(429));
        assert!(policy.should_retry_http_status(503));
        assert!(!policy.should_retry_http_status(200));
        assert!(!policy.should_retry_http_status(404));
    }

    #[test]
    fn test_integration_response_helpers() {
        let response = IntegrationResponse {
            request_id: Uuid::new_v4(),
            status_code: 200,
            headers: HashMap::new(),
            body: b"{\"test\": true}".to_vec(),
            elapsed_ms: 150,
            retry_count: 0,
            cached: false,
            timestamp: Utc::now(),
        };
        assert!(response.is_success());
        assert!(!response.is_rate_limited());
        assert!(!response.is_server_error());

        let parsed: serde_json::Value = response.parse_json().unwrap();
        assert_eq!(parsed["test"], true);
    }

    #[test]
    fn test_rate_limit_status() {
        let status = RateLimitStatus {
            limit: 100,
            remaining: 0,
            reset_at: Utc::now() + chrono::Duration::seconds(60),
            window_secs: 60,
        };
        assert!(status.is_exhausted());
        assert_eq!(status.usage_percent(), 100.0);
        assert!(status.time_until_reset() > Duration::from_secs(0));
    }

    #[test]
    fn test_credentials_masking() {
        let creds = Credentials::api_key_secret(
            IntegrationType::Censys,
            "my-secret-api-key-12345678",
            "my-secret-value-87654321",
        );
        let masked = creds.mask_sensitive();
        assert_ne!(masked.api_key, creds.api_key);
        assert!(masked.api_key.contains("..."));
        assert_ne!(masked.api_secret, creds.api_secret);
    }

    #[test]
    fn test_connection_idle_check() {
        let mut conn = Connection::new(IntegrationConfig::default());
        assert!(!conn.is_idle(300));

        conn.last_activity = Some(
            Utc::now() - chrono::Duration::seconds(400),
        );
        assert!(conn.is_idle(300));
    }

    #[test]
    fn test_connection_needs_reconnect() {
        let conn = Connection::new(IntegrationConfig::default());
        assert!(conn.needs_reconnect());
    }

    #[test]
    fn test_integration_error_codes() {
        let err = IntegrationContractError::RateLimitExceeded("test".to_string());
        assert_eq!(err.code(), "I8004");
        assert!(err.is_recoverable());

        let err = IntegrationContractError::AuthenticationFailed("test".to_string());
        assert_eq!(err.code(), "I8002");
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_health_status() {
        let healthy = IntegrationHealth::healthy(50);
        assert_eq!(healthy.status, HealthStatus::Healthy);
        assert_eq!(healthy.success_rate, 100.0);

        let unhealthy = IntegrationHealth::unhealthy("service down");
        assert_eq!(unhealthy.status, HealthStatus::Unhealthy);
        assert_eq!(unhealthy.error_rate, 100.0);
    }

    #[test]
    fn test_integration_request_builder() {
        let request = IntegrationRequest::new(HttpMethod::GET, "/api/v1/test")
            .with_query_param("key", "value")
            .with_header("Authorization", "Bearer token123")
            .with_timeout(60);

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path, "/api/v1/test");
        assert_eq!(request.query_params.get("key").unwrap(), "value");
        assert_eq!(request.timeout_secs, 60);
    }
}
