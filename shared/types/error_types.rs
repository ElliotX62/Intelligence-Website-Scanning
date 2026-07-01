// shared/types/error_types.rs
// IWS v1.0 - Error Types
// Mendefinisikan error types untuk semua komponen sistem

use std::fmt;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================
// ERROR SEVERITY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Fatal,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Debug => write!(f, "debug"),
            ErrorSeverity::Info => write!(f, "info"),
            ErrorSeverity::Warning => write!(f, "warning"),
            ErrorSeverity::Error => write!(f, "error"),
            ErrorSeverity::Critical => write!(f, "critical"),
            ErrorSeverity::Fatal => write!(f, "fatal"),
        }
    }
}

// ============================================================
// ERROR CONTEXT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub component: String,
    pub module: Option<String>,
    pub function: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub request_id: Option<Uuid>,
    pub scan_id: Option<Uuid>,
    pub user_id: Option<String>,
    pub target_url: Option<String>,
    pub additional: serde_json::Value,
}

impl ErrorContext {
    pub fn new(component: &str) -> Self {
        ErrorContext {
            component: component.to_string(),
            module: None,
            function: None,
            file: None,
            line: None,
            request_id: None,
            scan_id: None,
            user_id: None,
            target_url: None,
            additional: serde_json::json!({}),
        }
    }

    pub fn with_module(mut self, module: &str) -> Self {
        self.module = Some(module.to_string());
        self
    }

    pub fn with_scan(mut self, scan_id: Uuid) -> Self {
        self.scan_id = Some(scan_id);
        self
    }
}

// ============================================================
// ERROR CATEGORY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCategory {
    Network,
    Security,
    Configuration,
    Database,
    Storage,
    Authentication,
    Authorization,
    RateLimit,
    Validation,
    Integration,
    Internal,
    Timeout,
    Resource,
    Unknown,
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCategory::Network => write!(f, "network"),
            ErrorCategory::Security => write!(f, "security"),
            ErrorCategory::Configuration => write!(f, "configuration"),
            ErrorCategory::Database => write!(f, "database"),
            ErrorCategory::Storage => write!(f, "storage"),
            ErrorCategory::Authentication => write!(f, "authentication"),
            ErrorCategory::Authorization => write!(f, "authorization"),
            ErrorCategory::RateLimit => write!(f, "rate_limit"),
            ErrorCategory::Validation => write!(f, "validation"),
            ErrorCategory::Integration => write!(f, "integration"),
            ErrorCategory::Internal => write!(f, "internal"),
            ErrorCategory::Timeout => write!(f, "timeout"),
            ErrorCategory::Resource => write!(f, "resource"),
            ErrorCategory::Unknown => write!(f, "unknown"),
        }
    }
}

// ============================================================
// BASE ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScannerError {
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub timestamp: DateTime<Utc>,
    pub retry_after_secs: Option<u64>,
}

impl ScannerError {
    pub fn new(code: &str, message: &str, context: ErrorContext) -> Self {
        ScannerError {
            code: code.to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            recoverable: false,
            context,
            timestamp: Utc::now(),
            retry_after_secs: None,
        }
    }

    pub fn timeout(message: &str, context: ErrorContext) -> Self {
        ScannerError {
            code: "S1004".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::Timeout,
            recoverable: true,
            context,
            timestamp: Utc::now(),
            retry_after_secs: Some(5),
        }
    }

    pub fn connection_failed(message: &str, context: ErrorContext) -> Self {
        ScannerError {
            code: "S1005".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Network,
            recoverable: true,
            context,
            timestamp: Utc::now(),
            retry_after_secs: Some(2),
        }
    }

    pub fn invalid_target(message: &str, context: ErrorContext) -> Self {
        ScannerError {
            code: "S1001".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Validation,
            recoverable: false,
            context,
            timestamp: Utc::now(),
            retry_after_secs: None,
        }
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ScannerError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerError {
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub timestamp: DateTime<Utc>,
}

impl AnalyzerError {
    pub fn new(code: &str, message: &str, context: ErrorContext) -> Self {
        AnalyzerError {
            code: code.to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            recoverable: false,
            context,
            timestamp: Utc::now(),
        }
    }

    pub fn invalid_data(message: &str, context: ErrorContext) -> Self {
        AnalyzerError {
            code: "A2001".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Validation,
            recoverable: false,
            context,
            timestamp: Utc::now(),
        }
    }

    pub fn model_error(message: &str, context: ErrorContext) -> Self {
        AnalyzerError {
            code: "A2007".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Critical,
            category: ErrorCategory::Internal,
            recoverable: true,
            context,
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AnalyzerError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageError {
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub key: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl StorageError {
    pub fn new(code: &str, message: &str, context: ErrorContext) -> Self {
        StorageError {
            code: code.to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Storage,
            recoverable: false,
            context,
            key: None,
            timestamp: Utc::now(),
        }
    }

    pub fn key_not_found(key: &str, context: ErrorContext) -> Self {
        StorageError {
            code: "ST4003".to_string(),
            message: format!("Key not found: {}", key),
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::Storage,
            recoverable: false,
            context,
            key: Some(key.to_string()),
            timestamp: Utc::now(),
        }
    }

    pub fn connection_failed(message: &str, context: ErrorContext) -> Self {
        StorageError {
            code: "ST4005".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Critical,
            category: ErrorCategory::Database,
            recoverable: true,
            context,
            key: None,
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for StorageError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentError {
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub agent_id: Option<Uuid>,
    pub agent_type: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl AgentError {
    pub fn new(code: &str, message: &str, context: ErrorContext) -> Self {
        AgentError {
            code: code.to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            recoverable: false,
            context,
            agent_id: None,
            agent_type: None,
            timestamp: Utc::now(),
        }
    }

    pub fn heartbeat_missed(agent_id: Uuid, context: ErrorContext) -> Self {
        AgentError {
            code: "AG5007".to_string(),
            message: format!("Heartbeat missed for agent {}", agent_id),
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::Internal,
            recoverable: true,
            context,
            agent_id: Some(agent_id),
            agent_type: None,
            timestamp: Utc::now(),
        }
    }

    pub fn agent_crashed(agent_id: Uuid, message: &str, context: ErrorContext) -> Self {
        AgentError {
            code: "AG5011".to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Critical,
            category: ErrorCategory::Internal,
            recoverable: true,
            context,
            agent_id: Some(agent_id),
            agent_type: None,
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for AgentError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub http_status: u16,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub timestamp: DateTime<Utc>,
}

impl ApiError {
    pub fn new(code: &str, message: &str, http_status: u16, context: ErrorContext) -> Self {
        ApiError {
            code: code.to_string(),
            message: message.to_string(),
            http_status,
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            recoverable: false,
            context,
            timestamp: Utc::now(),
        }
    }

    pub fn not_found(resource: &str, context: ErrorContext) -> Self {
        ApiError {
            code: "API_004".to_string(),
            message: format!("{} not found", resource),
            http_status: 404,
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::Validation,
            recoverable: false,
            context,
            timestamp: Utc::now(),
        }
    }

    pub fn rate_limited(retry_after_secs: u64, context: ErrorContext) -> Self {
        ApiError {
            code: "API_002".to_string(),
            message: format!("Rate limit exceeded, retry after {}s", retry_after_secs),
            http_status: 429,
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::RateLimit,
            recoverable: true,
            context,
            timestamp: Utc::now(),
        }
    }

    pub fn internal_error(message: &str, context: ErrorContext) -> Self {
        ApiError {
            code: "API_003".to_string(),
            message: message.to_string(),
            http_status: 500,
            severity: ErrorSeverity::Critical,
            category: ErrorCategory::Internal,
            recoverable: false,
            context,
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] HTTP {} - {}", self.code, self.http_status, self.message)
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationError {
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub service_name: String,
    pub timestamp: DateTime<Utc>,
}

impl IntegrationError {
    pub fn new(code: &str, message: &str, service_name: &str, context: ErrorContext) -> Self {
        IntegrationError {
            code: code.to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Integration,
            recoverable: false,
            context,
            service_name: service_name.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn auth_failed(service: &str, context: ErrorContext) -> Self {
        IntegrationError {
            code: "I8002".to_string(),
            message: format!("Authentication failed for {}", service),
            severity: ErrorSeverity::Critical,
            category: ErrorCategory::Authentication,
            recoverable: false,
            context,
            service_name: service.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn rate_limited(service: &str, context: ErrorContext) -> Self {
        IntegrationError {
            code: "I8004".to_string(),
            message: format!("Rate limited by {}", service),
            severity: ErrorSeverity::Warning,
            category: ErrorCategory::RateLimit,
            recoverable: true,
            context,
            service_name: service.to_string(),
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {}", self.code, self.service_name, self.message)
    }
}

impl std::error::Error for IntegrationError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelError {
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub recoverable: bool,
    pub context: ErrorContext,
    pub model_name: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ModelError {
    pub fn new(code: &str, message: &str, context: ErrorContext) -> Self {
        ModelError {
            code: code.to_string(),
            message: message.to_string(),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            recoverable: false,
            context,
            model_name: None,
            timestamp: Utc::now(),
        }
    }

    pub fn load_failed(model_name: &str, message: &str, context: ErrorContext) -> Self {
        ModelError {
            code: "M6001".to_string(),
            message: format!("Failed to load model {}: {}", model_name, message),
            severity: ErrorSeverity::Critical,
            category: ErrorCategory::Internal,
            recoverable: true,
            context,
            model_name: Some(model_name.to_string()),
            timestamp: Utc::now(),
        }
    }

    pub fn inference_failed(model_name: &str, message: &str, context: ErrorContext) -> Self {
        ModelError {
            code: "M6002".to_string(),
            message: format!("Inference failed for {}: {}", model_name, message),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Internal,
            recoverable: true,
            context,
            model_name: Some(model_name.to_string()),
            timestamp: Utc::now(),
        }
    }
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ModelError {}

// ============================================================
// ERROR CONVERTER TRAIT
// ============================================================

pub trait ToErrorResponse {
    fn code(&self) -> &str;
    fn message(&self) -> &str;
    fn severity(&self) -> &ErrorSeverity;
    fn is_recoverable(&self) -> bool;
    fn to_json(&self) -> serde_json::Value;
}

impl ToErrorResponse for ScannerError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

impl ToErrorResponse for AnalyzerError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

impl ToErrorResponse for StorageError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "key": self.key,
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

impl ToErrorResponse for AgentError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "agent_id": self.agent_id.map(|id| id.to_string()),
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

impl ToErrorResponse for ApiError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "http_status": self.http_status,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

impl ToErrorResponse for IntegrationError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "service": self.service_name,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

impl ToErrorResponse for ModelError {
    fn code(&self) -> &str { &self.code }
    fn message(&self) -> &str { &self.message }
    fn severity(&self) -> &ErrorSeverity { &self.severity }
    fn is_recoverable(&self) -> bool { self.recoverable }
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "code": self.code,
            "message": self.message,
            "model": self.model_name,
            "severity": self.severity.to_string(),
            "category": self.category.to_string(),
            "recoverable": self.recoverable,
            "timestamp": self.timestamp.to_rfc3339(),
        })
    }
}

// ============================================================
// ERROR LOG ENTRY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLogEntry {
    pub error_id: Uuid,
    pub error_type: String,
    pub code: String,
    pub message: String,
    pub severity: ErrorSeverity,
    pub category: ErrorCategory,
    pub context: ErrorContext,
    pub stack_trace: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ErrorLogEntry {
    pub fn from_error<E: ToErrorResponse>(error: &E, context: ErrorContext) -> Self {
        ErrorLogEntry {
            error_id: Uuid::new_v4(),
            error_type: std::any::type_name::<E>().to_string(),
            code: error.code().to_string(),
            message: error.message().to_string(),
            severity: error.severity().clone(),
            category: ErrorCategory::Unknown,
            context,
            stack_trace: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_stack_trace(mut self, trace: &str) -> Self {
        self.stack_trace = Some(trace.to_string());
        self
    }

    pub fn to_log_string(&self) -> String {
        format!(
            "[{}] [{}] {} - {}",
            self.timestamp.to_rfc3339(),
            self.severity,
            self.code,
            self.message,
        )
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn test_context() -> ErrorContext {
        ErrorContext::new("test_component")
            .with_module("test_module")
    }

    #[test]
    fn test_scanner_error_factories() {
        let ctx = test_context();
        let err = ScannerError::timeout("Connection timed out", ctx.clone());
        assert_eq!(err.code, "S1004");
        assert!(err.recoverable);
        assert_eq!(err.retry_after_secs, Some(5));

        let err = ScannerError::invalid_target("Bad URL", ctx);
        assert_eq!(err.code, "S1001");
        assert!(!err.recoverable);
    }

    #[test]
    fn test_analyzer_error_factories() {
        let ctx = test_context();
        let err = AnalyzerError::invalid_data("Missing data", ctx);
        assert_eq!(err.code, "A2001");
        assert!(!err.recoverable);
    }

    #[test]
    fn test_storage_error_factories() {
        let ctx = test_context();
        let err = StorageError::key_not_found("scans/test", ctx);
        assert_eq!(err.code, "ST4003");
        assert!(err.key.is_some());
    }

    #[test]
    fn test_agent_error_factories() {
        let ctx = test_context();
        let agent_id = Uuid::new_v4();
        let err = AgentError::heartbeat_missed(agent_id, ctx);
        assert_eq!(err.code, "AG5007");
        assert!(err.recoverable);
    }

    #[test]
    fn test_api_error_factories() {
        let ctx = test_context();
        let err = ApiError::not_found("scan-123", ctx);
        assert_eq!(err.http_status, 404);

        let err = ApiError::rate_limited(60, test_context());
        assert_eq!(err.http_status, 429);
    }

    #[test]
    fn test_integration_error_factories() {
        let ctx = test_context();
        let err = IntegrationError::auth_failed("Shodan", ctx);
        assert_eq!(err.code, "I8002");
        assert_eq!(err.service_name, "Shodan");
    }

    #[test]
    fn test_model_error_factories() {
        let ctx = test_context();
        let err = ModelError::load_failed("bert-ner", "OOM", ctx);
        assert_eq!(err.code, "M6001");
        assert_eq!(err.model_name, Some("bert-ner".to_string()));
    }

    #[test]
    fn test_error_to_json() {
        let ctx = test_context();
        let err = ScannerError::timeout("timeout", ctx);
        let json = err.to_json();
        assert_eq!(json["code"], "S1004");
        assert_eq!(json["recoverable"], true);
    }

    #[test]
    fn test_error_log_entry() {
        let ctx = test_context();
        let err = ApiError::internal_error("Something broke", ctx.clone());
        let entry = ErrorLogEntry::from_error(&err, ctx)
            .with_stack_trace("at line 42");
        assert!(entry.to_log_string().contains("API_003"));
        assert!(entry.stack_trace.is_some());
    }

    #[test]
    fn test_error_severity_ordering() {
        assert!(ErrorSeverity::Critical > ErrorSeverity::Warning);
        assert!(ErrorSeverity::Fatal > ErrorSeverity::Error);
        assert!(ErrorSeverity::Debug < ErrorSeverity::Info);
    }
}
