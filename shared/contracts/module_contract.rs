// shared/contracts/module_contract.rs
// IWS v1.0 - Module Contract
// Mendefinisikan kontrak formal untuk semua functional modules

use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

// ============================================================
// API VERSION CONSTANT
// ============================================================

/// Versi API yang harus dikembalikan oleh semua module
pub const API_VERSION: &str = "v1.0";

// ============================================================
// MODULE TYPES & CAPABILITIES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModuleType {
    Network,
    Content,
    Security,
    Infrastructure,
    Intelligence,
    Custom(String),
}

impl fmt::Display for ModuleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleType::Network => write!(f, "network"),
            ModuleType::Content => write!(f, "content"),
            ModuleType::Security => write!(f, "security"),
            ModuleType::Infrastructure => write!(f, "infrastructure"),
            ModuleType::Intelligence => write!(f, "intelligence"),
            ModuleType::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

impl ModuleType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "network" => ModuleType::Network,
            "content" => ModuleType::Content,
            "security" => ModuleType::Security,
            "infrastructure" => ModuleType::Infrastructure,
            "intelligence" => ModuleType::Intelligence,
            other => ModuleType::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleCapabilities {
    pub capability_flags: u32,
    pub supported_formats: Vec<String>,
    pub requires_network: bool,
    pub requires_authentication: bool,
    pub max_concurrent_instances: u32,
    pub average_duration_ms: u64,
    pub timeout_ms: u64,
    pub is_thread_safe: bool,
    pub supports_cancellation: bool,
    pub supports_pause_resume: bool,
    pub dependencies: Vec<String>,
    pub optional_dependencies: Vec<String>,
    pub api_version: String,
}

impl Default for ModuleCapabilities {
    fn default() -> Self {
        ModuleCapabilities {
            capability_flags: 0,
            supported_formats: vec!["json".to_string()],
            requires_network: true,
            requires_authentication: false,
            max_concurrent_instances: 1,
            average_duration_ms: 5000,
            timeout_ms: 60000,
            is_thread_safe: true,
            supports_cancellation: true,
            supports_pause_resume: false,
            dependencies: vec![],
            optional_dependencies: vec![],
            api_version: API_VERSION.to_string(),
        }
    }
}

impl ModuleCapabilities {
    pub fn has_capability(&self, flag: u32) -> bool {
        (self.capability_flags & flag) != 0
    }

    pub fn set_capability(&mut self, flag: u32) {
        self.capability_flags |= flag;
    }

    pub fn clear_capability(&mut self, flag: u32) {
        self.capability_flags &= !flag;
    }

    pub fn is_api_compatible(&self) -> bool {
        self.api_version == API_VERSION
    }
}

// Capability flag constants
pub const CAP_NETWORK_SCAN: u32 = 0x01;
pub const CAP_CONTENT_ANALYSIS: u32 = 0x02;
pub const CAP_SECURITY_CHECK: u32 = 0x04;
pub const CAP_INTELLIGENCE: u32 = 0x08;
pub const CAP_INFRASTRUCTURE: u32 = 0x10;
pub const CAP_STEALTH: u32 = 0x20;
pub const CAP_HIGH_PERFORMANCE: u32 = 0x40;
pub const CAP_LOW_RESOURCE: u32 = 0x80;

// ============================================================
// MODULE CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub module_name: String,
    pub module_type: ModuleType,
    pub enabled: bool,
    pub timeout_ms: u64,
    pub retry_count: u32,
    pub retry_delay_ms: u64,
    pub custom_config: serde_json::Value,
    pub version: String,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        ModuleConfig {
            module_name: "unnamed".to_string(),
            module_type: ModuleType::Custom("unknown".to_string()),
            enabled: true,
            timeout_ms: 60000,
            retry_count: 3,
            retry_delay_ms: 1000,
            custom_config: serde_json::json!({}),
            version: API_VERSION.to_string(),
        }
    }
}

impl ModuleConfig {
    pub fn new(name: &str, module_type: ModuleType) -> Self {
        ModuleConfig {
            module_name: name.to_string(),
            module_type,
            ..Default::default()
        }
    }

    pub fn validate(&self) -> Result<(), ModuleContractError> {
        if self.module_name.is_empty() {
            return Err(ModuleContractError::InvalidConfiguration(
                "module_name cannot be empty".to_string()
            ));
        }
        if self.timeout_ms == 0 || self.timeout_ms > 600000 {
            return Err(ModuleContractError::InvalidConfiguration(
                format!("timeout_ms must be 1-600000, got {}", self.timeout_ms)
            ));
        }
        if self.retry_count > 20 {
            return Err(ModuleContractError::InvalidConfiguration(
                format!("retry_count must be 0-20, got {}", self.retry_count)
            ));
        }
        if self.version != API_VERSION {
            return Err(ModuleContractError::InvalidConfiguration(
                format!("version must be {}, got {}", API_VERSION, self.version)
            ));
        }
        Ok(())
    }
}

// ============================================================
// MODULE INPUT & OUTPUT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInput {
    pub scan_id: Uuid,
    pub target_url: String,
    pub target_ip: Option<String>,
    pub config: ModuleConfig,
    pub context: HashMap<String, serde_json::Value>,
    pub previous_results: Vec<ModuleOutput>,
    pub metadata: serde_json::Value,
}

impl ModuleInput {
    pub fn new(scan_id: Uuid, target_url: &str, config: ModuleConfig) -> Self {
        ModuleInput {
            scan_id,
            target_url: target_url.to_string(),
            target_ip: None,
            config,
            context: HashMap::new(),
            previous_results: vec![],
            metadata: serde_json::json!({}),
        }
    }

    pub fn with_ip(mut self, ip: &str) -> Self {
        self.target_ip = Some(ip.to_string());
        self
    }

    pub fn with_context(mut self, key: &str, value: serde_json::Value) -> Self {
        self.context.insert(key.to_string(), value);
        self
    }

    pub fn with_previous(mut self, results: Vec<ModuleOutput>) -> Self {
        self.previous_results = results;
        self
    }

    pub fn get_context<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.context
            .get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOutput {
    pub module_name: String,
    pub module_type: ModuleType,
    pub status: ModuleStatus,
    pub data: serde_json::Value,
    pub findings: Vec<ModuleFinding>,
    pub errors: Vec<ModuleError>,
    pub warnings: Vec<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub duration_ms: u64,
    pub metrics: ModuleMetrics,
    pub metadata: serde_json::Value,
}

impl ModuleOutput {
    pub fn new(module_name: &str, module_type: ModuleType) -> Self {
        let now = Utc::now();
        ModuleOutput {
            module_name: module_name.to_string(),
            module_type,
            status: ModuleStatus::Pending,
            data: serde_json::json!({}),
            findings: vec![],
            errors: vec![],
            warnings: vec![],
            start_time: now,
            end_time: now,
            duration_ms: 0,
            metrics: ModuleMetrics::default(),
            metadata: serde_json::json!({}),
        }
    }

    pub fn is_successful(&self) -> bool {
        self.status == ModuleStatus::Completed && self.errors.is_empty()
    }

    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }

    pub fn finding_count_by_severity(&self, severity: &str) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ModuleStatus {
    Pending,
    Initializing,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
    Skipped,
    TimedOut,
}

impl fmt::Display for ModuleStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleStatus::Pending => write!(f, "pending"),
            ModuleStatus::Initializing => write!(f, "initializing"),
            ModuleStatus::Running => write!(f, "running"),
            ModuleStatus::Paused => write!(f, "paused"),
            ModuleStatus::Completed => write!(f, "completed"),
            ModuleStatus::Failed => write!(f, "failed"),
            ModuleStatus::Cancelled => write!(f, "cancelled"),
            ModuleStatus::Skipped => write!(f, "skipped"),
            ModuleStatus::TimedOut => write!(f, "timed_out"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleFinding {
    pub finding_type: String,
    pub severity: String,
    pub title: String,
    pub description: String,
    pub details: serde_json::Value,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

impl ModuleFinding {
    pub fn new(
        finding_type: &str,
        severity: &str,
        title: &str,
        description: &str,
    ) -> Self {
        ModuleFinding {
            finding_type: finding_type.to_string(),
            severity: severity.to_string(),
            title: title.to_string(),
            description: description.to_string(),
            details: serde_json::json!({}),
            confidence: 0.5,
            timestamp: Utc::now(),
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.confidence = confidence.max(0.0).min(1.0);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleError {
    pub error_type: String,
    pub message: String,
    pub recoverable: bool,
    pub timestamp: DateTime<Utc>,
    pub details: Option<serde_json::Value>,
}

impl ModuleError {
    pub fn new(error_type: &str, message: &str, recoverable: bool) -> Self {
        ModuleError {
            error_type: error_type.to_string(),
            message: message.to_string(),
            recoverable,
            timestamp: Utc::now(),
            details: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetrics {
    pub requests_sent: u64,
    pub requests_failed: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub dns_queries: u64,
    pub tcp_connections: u64,
    pub pages_crawled: u64,
    pub forms_analyzed: u64,
    pub vulnerabilities_found: u64,
    pub cpu_time_ms: u64,
    pub memory_peak_mb: f64,
    pub network_time_ms: u64,
}

impl Default for ModuleMetrics {
    fn default() -> Self {
        ModuleMetrics {
            requests_sent: 0,
            requests_failed: 0,
            bytes_sent: 0,
            bytes_received: 0,
            dns_queries: 0,
            tcp_connections: 0,
            pages_crawled: 0,
            forms_analyzed: 0,
            vulnerabilities_found: 0,
            cpu_time_ms: 0,
            memory_peak_mb: 0.0,
            network_time_ms: 0,
        }
    }
}

// ============================================================
// MODULE VALIDATION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec![],
            suggestions: vec![],
        }
    }

    pub fn invalid(errors: Vec<String>) -> Self {
        ValidationResult {
            is_valid: false,
            errors,
            warnings: vec![],
            suggestions: vec![],
        }
    }

    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = warnings;
        self
    }

    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
}

// ============================================================
// MODULE DEPENDENCY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependency {
    pub module_name: String,
    pub required: bool,
    pub minimum_version: Option<String>,
    pub maximum_version: Option<String>,
    pub reason: String,
}

impl ModuleDependency {
    pub fn required(module_name: &str, reason: &str) -> Self {
        ModuleDependency {
            module_name: module_name.to_string(),
            required: true,
            minimum_version: None,
            maximum_version: None,
            reason: reason.to_string(),
        }
    }

    pub fn optional(module_name: &str, reason: &str) -> Self {
        ModuleDependency {
            module_name: module_name.to_string(),
            required: false,
            minimum_version: None,
            maximum_version: None,
            reason: reason.to_string(),
        }
    }
}

// ============================================================
// MODULE CONTRACT TRAIT
// ============================================================

#[async_trait]
pub trait ModuleContract: Send + Sync {
    /// Eksekusi module dengan input yang diberikan
    /// Preconditions: input valid, config valid
    /// Postconditions: output.status != Pending
    async fn execute(
        &self,
        input: ModuleInput,
    ) -> Result<ModuleOutput, ModuleContractError>;

    /// Validasi konfigurasi module
    fn validate_config(
        &self,
        config: &ModuleConfig,
    ) -> Result<ValidationResult, ModuleContractError>;

    /// Mendapatkan capabilities module
    fn get_capabilities(&self) -> ModuleCapabilities;

    /// Mendapatkan versi module — HARUS mengembalikan API_VERSION
    fn get_version(&self) -> String {
        API_VERSION.to_string()
    }

    /// Mendapatkan nama module
    fn get_name(&self) -> String;

    /// Mendapatkan tipe module
    fn get_type(&self) -> ModuleType;

    /// Mendapatkan dependencies module
    fn get_dependencies(&self) -> Vec<ModuleDependency>;

    /// Mendapatkan system requirements
    fn get_requirements(&self) -> Vec<ModuleRequirement>;

    /// Mendapatkan schema konfigurasi (JSON Schema)
    fn get_config_schema(&self) -> serde_json::Value;

    /// Eksekusi dengan timeout
    async fn execute_with_timeout(
        &self,
        input: ModuleInput,
        timeout: Duration,
    ) -> Result<ModuleOutput, ModuleContractError> {
        tokio::select! {
            result = self.execute(input) => result,
            _ = tokio::time::sleep(timeout) => {
                Err(ModuleContractError::Timeout(
                    format!("Module {} timed out after {:?}", self.get_name(), timeout)
                ))
            }
        }
    }

    /// Validasi input sebelum eksekusi
    fn validate_input(&self, input: &ModuleInput) -> Result<(), ModuleContractError> {
        if input.target_url.is_empty() {
            return Err(ModuleContractError::InvalidInput(
                "target_url cannot be empty".to_string()
            ));
        }
        input.config.validate()?;
        Ok(())
    }

    /// Retry wrapper untuk execute
    async fn execute_with_retry(
        &self,
        input: ModuleInput,
        max_retries: u32,
        delay_ms: u64,
    ) -> Result<ModuleOutput, ModuleContractError> {
        let mut last_error = None;

        for attempt in 0..=max_retries {
            match self.execute(input.clone()).await {
                Ok(output) => return Ok(output),
                Err(e) if attempt < max_retries && e.is_recoverable() => {
                    last_error = Some(e);
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or_else(|| {
            ModuleContractError::InternalError(
                "Retry exhausted with no error".to_string()
            )
        }))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleRequirement {
    pub requirement_type: RequirementType,
    pub name: String,
    pub minimum: Option<String>,
    pub recommended: Option<String>,
    pub description: String,
}

impl ModuleRequirement {
    pub fn new(
        requirement_type: RequirementType,
        name: &str,
        description: &str,
    ) -> Self {
        ModuleRequirement {
            requirement_type,
            name: name.to_string(),
            minimum: None,
            recommended: None,
            description: description.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequirementType {
    Memory,
    Disk,
    Cpu,
    Network,
    Os,
    Library,
    Binary,
    ApiKey,
    Permission,
    Other(String),
}

impl fmt::Display for RequirementType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequirementType::Memory => write!(f, "memory"),
            RequirementType::Disk => write!(f, "disk"),
            RequirementType::Cpu => write!(f, "cpu"),
            RequirementType::Network => write!(f, "network"),
            RequirementType::Os => write!(f, "os"),
            RequirementType::Library => write!(f, "library"),
            RequirementType::Binary => write!(f, "binary"),
            RequirementType::ApiKey => write!(f, "api_key"),
            RequirementType::Permission => write!(f, "permission"),
            RequirementType::Other(s) => write!(f, "other:{}", s),
        }
    }
}

// ============================================================
// MODULE REGISTRY
// ============================================================

#[derive(Debug, Clone)]
pub struct ModuleRegistry {
    modules: HashMap<String, Box<dyn ModuleContract>>,
    metadata: HashMap<String, ModuleMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMetadata {
    pub name: String,
    pub module_type: ModuleType,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub registered_at: DateTime<Utc>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry {
            modules: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        module: Box<dyn ModuleContract>,
        metadata: ModuleMetadata,
    ) -> Result<(), ModuleContractError> {
        let name = module.get_name();
        if self.modules.contains_key(&name) {
            return Err(ModuleContractError::ModuleAlreadyRegistered(name));
        }
        if metadata.version != API_VERSION {
            return Err(ModuleContractError::DependencyVersionMismatch(
                format!("Module {} requires API {}, got {}", name, API_VERSION, metadata.version)
            ));
        }
        self.modules.insert(name.clone(), module);
        self.metadata.insert(name, metadata);
        Ok(())
    }

    pub fn unregister(&mut self, name: &str) -> Result<(), ModuleContractError> {
        if !self.modules.contains_key(name) {
            return Err(ModuleContractError::ModuleNotFound(name.to_string()));
        }
        self.modules.remove(name);
        self.metadata.remove(name);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&dyn ModuleContract> {
        self.modules.get(name).map(|m| m.as_ref())
    }

    pub fn list_modules(&self) -> Vec<String> {
        self.modules.keys().cloned().collect()
    }

    pub fn list_by_type(&self, module_type: &ModuleType) -> Vec<String> {
        self.metadata
            .iter()
            .filter(|(_, m)| &m.module_type == module_type)
            .map(|(name, _)| name.clone())
            .collect()
    }

    pub fn count(&self) -> usize {
        self.modules.len()
    }

    pub fn get_all_metadata(&self) -> Vec<ModuleMetadata> {
        self.metadata.values().cloned().collect()
    }

    pub fn get_dependency_graph(&self) -> HashMap<String, Vec<String>> {
        let mut graph = HashMap::new();
        for (name, module) in &self.modules {
            let deps: Vec<String> = module
                .get_dependencies()
                .into_iter()
                .map(|d| d.module_name)
                .collect();
            graph.insert(name.clone(), deps);
        }
        graph
    }

    pub fn resolve_order(&self) -> Result<Vec<String>, ModuleContractError> {
        let graph = self.get_dependency_graph();
        let mut visited = HashMap::new();
        let mut order = Vec::new();
        let mut temp_marks = HashMap::new();

        for node in graph.keys() {
            visited.insert(node.clone(), false);
            temp_marks.insert(node.clone(), false);
        }

        fn visit(
            node: &str,
            graph: &HashMap<String, Vec<String>>,
            visited: &mut HashMap<String, bool>,
            temp_marks: &mut HashMap<String, bool>,
            order: &mut Vec<String>,
        ) -> Result<(), ModuleContractError> {
            if *temp_marks.get(node).unwrap_or(&false) {
                return Err(ModuleContractError::CircularDependency(
                    format!("Circular dependency detected at {}", node)
                ));
            }
            if *visited.get(node).unwrap_or(&false) {
                return Ok(());
            }
            temp_marks.insert(node.to_string(), true);
            if let Some(deps) = graph.get(node) {
                for dep in deps {
                    if graph.contains_key(dep) {
                        visit(dep, graph, visited, temp_marks, order)?;
                    }
                }
            }
            temp_marks.insert(node.to_string(), false);
            visited.insert(node.to_string(), true);
            order.push(node.to_string());
            Ok(())
        }

        for node in graph.keys() {
            if !visited.get(node).unwrap_or(&false) {
                visit(node, &graph, &mut visited, &mut temp_marks, &mut order)?;
            }
        }

        order.reverse();
        Ok(order)
    }
}

// ============================================================
// ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleContractError {
    InvalidInput(String),
    InvalidConfiguration(String),
    ModuleNotFound(String),
    ModuleAlreadyRegistered(String),
    ModuleExecutionFailed(String),
    ModuleNotEnabled(String),
    DependencyNotFound(String),
    DependencyVersionMismatch(String),
    CircularDependency(String),
    Timeout(String),
    ResourceExhausted(String),
    NetworkError(String),
    ParseError(String),
    RateLimited(String),
    PermissionDenied(String),
    InternalError(String),
}

impl fmt::Display for ModuleContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleContractError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ModuleContractError::InvalidConfiguration(msg) => write!(f, "Invalid config: {}", msg),
            ModuleContractError::ModuleNotFound(name) => write!(f, "Module not found: {}", name),
            ModuleContractError::ModuleAlreadyRegistered(name) => write!(f, "Module already registered: {}", name),
            ModuleContractError::ModuleExecutionFailed(msg) => write!(f, "Execution failed: {}", msg),
            ModuleContractError::ModuleNotEnabled(name) => write!(f, "Module not enabled: {}", name),
            ModuleContractError::DependencyNotFound(name) => write!(f, "Dependency not found: {}", name),
            ModuleContractError::DependencyVersionMismatch(msg) => write!(f, "Version mismatch: {}", msg),
            ModuleContractError::CircularDependency(msg) => write!(f, "Circular dependency: {}", msg),
            ModuleContractError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            ModuleContractError::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
            ModuleContractError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ModuleContractError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ModuleContractError::RateLimited(msg) => write!(f, "Rate limited: {}", msg),
            ModuleContractError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            ModuleContractError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for ModuleContractError {}

impl ModuleContractError {
    pub fn code(&self) -> &str {
        match self {
            ModuleContractError::InvalidInput(_) => "M7001",
            ModuleContractError::InvalidConfiguration(_) => "M7002",
            ModuleContractError::ModuleNotFound(_) => "M7003",
            ModuleContractError::ModuleAlreadyRegistered(_) => "M7004",
            ModuleContractError::ModuleExecutionFailed(_) => "M7005",
            ModuleContractError::ModuleNotEnabled(_) => "M7006",
            ModuleContractError::DependencyNotFound(_) => "M7007",
            ModuleContractError::DependencyVersionMismatch(_) => "M7008",
            ModuleContractError::CircularDependency(_) => "M7009",
            ModuleContractError::Timeout(_) => "M7010",
            ModuleContractError::ResourceExhausted(_) => "M7011",
            ModuleContractError::NetworkError(_) => "M7012",
            ModuleContractError::ParseError(_) => "M7013",
            ModuleContractError::RateLimited(_) => "M7014",
            ModuleContractError::PermissionDenied(_) => "M7015",
            ModuleContractError::InternalError(_) => "M7016",
        }
    }

    pub fn severity(&self) -> &str {
        match self {
            ModuleContractError::InvalidInput(_) => "high",
            ModuleContractError::InvalidConfiguration(_) => "high",
            ModuleContractError::ModuleNotFound(_) => "medium",
            ModuleContractError::ModuleAlreadyRegistered(_) => "low",
            ModuleContractError::ModuleExecutionFailed(_) => "high",
            ModuleContractError::ModuleNotEnabled(_) => "low",
            ModuleContractError::DependencyNotFound(_) => "high",
            ModuleContractError::DependencyVersionMismatch(_) => "medium",
            ModuleContractError::CircularDependency(_) => "critical",
            ModuleContractError::Timeout(_) => "medium",
            ModuleContractError::ResourceExhausted(_) => "critical",
            ModuleContractError::NetworkError(_) => "medium",
            ModuleContractError::ParseError(_) => "medium",
            ModuleContractError::RateLimited(_) => "medium",
            ModuleContractError::PermissionDenied(_) => "high",
            ModuleContractError::InternalError(_) => "critical",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            ModuleContractError::Timeout(_)
                | ModuleContractError::NetworkError(_)
                | ModuleContractError::RateLimited(_)
                | ModuleContractError::ParseError(_)
                | ModuleContractError::ResourceExhausted(_)
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
    fn test_api_version_constant() {
        assert_eq!(API_VERSION, "v1.0");
    }

    #[test]
    fn test_module_capabilities_api_version() {
        let caps = ModuleCapabilities::default();
        assert_eq!(caps.api_version, API_VERSION);
        assert!(caps.is_api_compatible());
    }

    #[test]
    fn test_module_config_default_version() {
        let config = ModuleConfig::default();
        assert_eq!(config.version, API_VERSION);
    }

    #[test]
    fn test_module_config_validation_wrong_version() {
        let mut config = ModuleConfig::default();
        config.version = "v0.9".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_get_version_default() {
        // Trait default implementation returns API_VERSION
        struct DummyModule;
        #[async_trait]
        impl ModuleContract for DummyModule {
            fn get_name(&self) -> String { "dummy".to_string() }
            fn get_type(&self) -> ModuleType { ModuleType::Custom("test".to_string()) }
            fn get_capabilities(&self) -> ModuleCapabilities { ModuleCapabilities::default() }
            fn get_dependencies(&self) -> Vec<ModuleDependency> { vec![] }
            fn get_requirements(&self) -> Vec<ModuleRequirement> { vec![] }
            fn get_config_schema(&self) -> serde_json::Value { serde_json::json!({}) }
            async fn execute(&self, _input: ModuleInput) -> Result<ModuleOutput, ModuleContractError> {
                Ok(ModuleOutput::new("dummy", ModuleType::Custom("test".to_string())))
            }
            fn validate_config(&self, _config: &ModuleConfig) -> Result<ValidationResult, ModuleContractError> {
                Ok(ValidationResult::valid())
            }
        }
        let module = DummyModule;
        assert_eq!(module.get_version(), API_VERSION);
    }

    #[test]
    fn test_module_capabilities_flags() {
        let mut caps = ModuleCapabilities::default();
        assert!(!caps.has_capability(CAP_NETWORK_SCAN));

        caps.set_capability(CAP_NETWORK_SCAN);
        assert!(caps.has_capability(CAP_NETWORK_SCAN));

        caps.set_capability(CAP_SECURITY_CHECK);
        assert!(caps.has_capability(CAP_NETWORK_SCAN));
        assert!(caps.has_capability(CAP_SECURITY_CHECK));

        caps.clear_capability(CAP_NETWORK_SCAN);
        assert!(!caps.has_capability(CAP_NETWORK_SCAN));
    }

    #[test]
    fn test_module_config_validation() {
        let config = ModuleConfig::default();
        assert!(config.validate().is_ok());

        let mut bad_timeout = ModuleConfig::default();
        bad_timeout.timeout_ms = 0;
        assert!(bad_timeout.validate().is_err());

        let mut bad_retry = ModuleConfig::default();
        bad_retry.retry_count = 100;
        assert!(bad_retry.validate().is_err());
    }

    #[test]
    fn test_module_input_builder() {
        let scan_id = Uuid::new_v4();
        let config = ModuleConfig::new("test_module", ModuleType::Network);
        let input = ModuleInput::new(scan_id, "https://example.com", config)
            .with_ip("93.184.216.34")
            .with_context("depth", serde_json::json!(3));

        assert_eq!(input.target_url, "https://example.com");
        assert_eq!(input.target_ip, Some("93.184.216.34".to_string()));
        assert!(input.get_context::<i32>("depth").is_some());
    }

    #[test]
    fn test_module_output_helpers() {
        let mut output = ModuleOutput::new("test", ModuleType::Security);
        assert!(!output.is_successful());

        output.status = ModuleStatus::Completed;
        assert!(output.is_successful());

        let finding = ModuleFinding::new(
            "vulnerability",
            "high",
            "XSS Found",
            "Cross-site scripting detected",
        );
        output.findings.push(finding);
        assert!(output.has_findings());
        assert_eq!(output.finding_count_by_severity("high"), 1);
    }

    #[test]
    fn test_module_finding_builder() {
        let finding = ModuleFinding::new(
            "misconfiguration",
            "medium",
            "Missing CSP",
            "Content-Security-Policy header missing",
        )
        .with_confidence(0.75)
        .with_details(serde_json::json!({"header": "CSP", "status": "missing"}));

        assert_eq!(finding.finding_type, "misconfiguration");
        assert_eq!(finding.confidence, 0.75);
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::valid();
        assert!(valid.is_valid);

        let invalid = ValidationResult::invalid(vec!["Error 1".to_string()]);
        assert!(!invalid.is_valid);

        let with_warnings = ValidationResult::valid()
            .with_warnings(vec!["Warning 1".to_string()])
            .with_suggestions(vec!["Suggestion 1".to_string()]);
        assert!(with_warnings.is_valid);
        assert_eq!(with_warnings.warnings.len(), 1);
        assert_eq!(with_warnings.suggestions.len(), 1);
    }

    #[test]
    fn test_module_dependency() {
        let required = ModuleDependency::required("dns_enum", "DNS data needed");
        assert!(required.required);

        let optional = ModuleDependency::optional("threat_intel", "Enhanced results");
        assert!(!optional.required);
    }

    #[test]
    fn test_module_error_codes() {
        let err = ModuleContractError::ModuleNotFound("test".to_string());
        assert_eq!(err.code(), "M7003");

        let err = ModuleContractError::CircularDependency("loop".to_string());
        assert_eq!(err.code(), "M7009");
    }

    #[test]
    fn test_module_error_recoverable() {
        let timeout = ModuleContractError::Timeout("test".to_string());
        assert!(timeout.is_recoverable());

        let invalid = ModuleContractError::InvalidInput("test".to_string());
        assert!(!invalid.is_recoverable());
    }

    #[test]
    fn test_requirement_type_display() {
        assert_eq!(RequirementType::Memory.to_string(), "memory");
        assert_eq!(RequirementType::ApiKey.to_string(), "api_key");
    }
}
