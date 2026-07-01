// shared/types/config_types.rs
// IWS v1.0 - Config Types
// Mendefinisikan tipe data untuk system configuration

use std::fmt;
use std::collections::HashMap;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use url::Url;

// ============================================================
// ENVIRONMENT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Environment {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "dev" | "development" => Environment::Development,
            "test" | "testing" => Environment::Testing,
            "staging" => Environment::Staging,
            "prod" | "production" => Environment::Production,
            _ => Environment::Development,
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "development"),
            Environment::Testing => write!(f, "testing"),
            Environment::Staging => write!(f, "staging"),
            Environment::Production => write!(f, "production"),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}

// ============================================================
// LOG LEVEL
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl LogLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "trace" => LogLevel::Trace,
            "debug" => LogLevel::Debug,
            "info" => LogLevel::Info,
            "warn" | "warning" => LogLevel::Warn,
            "error" => LogLevel::Error,
            "fatal" => LogLevel::Fatal,
            _ => LogLevel::Info,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Fatal => "fatal",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

// ============================================================
// FromEnv TRAIT (H2 FIX)
// ============================================================

/// Trait untuk loading konfigurasi dari environment variables
/// dengan prefix IWS_ dan fail-fast validation
pub trait FromEnv: Sized {
    /// Load dari environment variables dengan prefix IWS_
    /// Returns Err jika ada required variable yang missing atau invalid
    fn from_env() -> Result<Self, String>;

    /// Load dari environment variables dengan prefix IWS_, 
    /// lalu validasi dengan validate()
    fn from_env_validated() -> Result<Self, String>
    where
        Self: Validate,
    {
        let config = Self::from_env()?;
        config.validate().map_err(|e| format!("Validation failed: {}", e))?;
        Ok(config)
    }
}

/// Trait untuk validasi konfigurasi
pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

// ============================================================
// ENV HELPER FUNCTIONS
// ============================================================

fn env_string(key: &str, default: &str) -> String {
    std::env::var(format!("IWS_{}", key)).unwrap_or_else(|_| default.to_string())
}

fn env_string_optional(key: &str) -> Option<String> {
    std::env::var(format!("IWS_{}", key)).ok()
}

fn env_bool(key: &str, default: bool) -> bool {
    std::env::var(format!("IWS_{}", key))
        .map(|v| matches!(v.to_lowercase().as_str(), "true" | "1" | "yes" | "on"))
        .unwrap_or(default)
}

fn env_usize(key: &str, default: usize) -> usize {
    std::env::var(format!("IWS_{}", key))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_u64(key: &str, default: u64) -> u64 {
    std::env::var(format!("IWS_{}", key))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_u32(key: &str, default: u32) -> u32 {
    std::env::var(format!("IWS_{}", key))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_u16(key: &str, default: u16) -> u16 {
    std::env::var(format!("IWS_{}", key))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(format!("IWS_{}", key))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn env_vec_string(key: &str, default: Vec<String>) -> Vec<String> {
    std::env::var(format!("IWS_{}", key))
        .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or(default)
}

// ============================================================
// APPLICATION CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub environment: Environment,
    pub debug: bool,
    pub log_level: LogLevel,
    pub data_dir: String,
    pub temp_dir: String,
    pub max_concurrent_scans: usize,
    pub max_scan_duration_secs: u64,
    pub auto_save_interval_secs: u64,
    pub shutdown_timeout_secs: u64,
    pub health_check_interval_secs: u64,
    pub metrics_enabled: bool,
    pub telemetry_enabled: bool,
}

impl AppConfig {
    pub fn new() -> Self {
        AppConfig {
            app_name: "IWS".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: Environment::default(),
            debug: false,
            log_level: LogLevel::default(),
            data_dir: "./data".to_string(),
            temp_dir: "./data/temp".to_string(),
            max_concurrent_scans: 10,
            max_scan_duration_secs: 3600,
            auto_save_interval_secs: 30,
            shutdown_timeout_secs: 30,
            health_check_interval_secs: 30,
            metrics_enabled: true,
            telemetry_enabled: false,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.max_concurrent_scans == 0 || self.max_concurrent_scans > 100 {
            return Err(format!("max_concurrent_scans must be 1-100, got {}", self.max_concurrent_scans));
        }
        if self.max_scan_duration_secs < 60 {
            return Err(format!("max_scan_duration_secs must be >= 60, got {}", self.max_scan_duration_secs));
        }
        Ok(())
    }
}

impl Validate for AppConfig {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

impl FromEnv for AppConfig {
    fn from_env() -> Result<Self, String> {
        Ok(AppConfig {
            app_name: env_string("APP_NAME", "IWS"),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: Environment::from_str(&env_string("ENV", "development")),
            debug: env_bool("DEBUG", false),
            log_level: LogLevel::from_str(&env_string("LOG_LEVEL", "info")),
            data_dir: env_string("DATA_DIR", "./data"),
            temp_dir: env_string("TEMP_DIR", "./data/temp"),
            max_concurrent_scans: env_usize("MAX_CONCURRENT_SCANS", 10),
            max_scan_duration_secs: env_u64("MAX_SCAN_DURATION_SECS", 3600),
            auto_save_interval_secs: env_u64("AUTO_SAVE_INTERVAL_SECS", 30),
            shutdown_timeout_secs: env_u64("SHUTDOWN_TIMEOUT_SECS", 30),
            health_check_interval_secs: env_u64("HEALTH_CHECK_INTERVAL_SECS", 30),
            metrics_enabled: env_bool("METRICS_ENABLED", true),
            telemetry_enabled: env_bool("TELEMETRY_ENABLED", false),
        })
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig::new()
    }
}

// ============================================================
// DATABASE CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub pool_size: usize,
    pub pool_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub idle_timeout_secs: u64,
    pub connection_timeout_secs: u64,
    pub ssl_mode: SslMode,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub migrations_enabled: bool,
    pub migrations_path: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        DatabaseConfig {
            url: String::new(),
            host: "localhost".to_string(),
            port: 5432,
            database: "iws".to_string(),
            username: "iws".to_string(),
            password: String::new(),
            pool_size: 20,
            pool_timeout_secs: 30,
            max_lifetime_secs: 600,
            idle_timeout_secs: 300,
            connection_timeout_secs: 10,
            ssl_mode: SslMode::Prefer,
            max_retries: 3,
            retry_delay_ms: 1000,
            migrations_enabled: true,
            migrations_path: "./database/migrations/".to_string(),
        }
    }

    pub fn connection_string(&self) -> String {
        if !self.url.is_empty() {
            return self.url.clone();
        }
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database,
            self.ssl_mode.as_str(),
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() && (self.host.is_empty() || self.database.is_empty()) {
            return Err("Either url or (host + database) must be provided".to_string());
        }
        if self.pool_size == 0 || self.pool_size > 100 {
            return Err(format!("pool_size must be 1-100, got {}", self.pool_size));
        }
        Ok(())
    }
}

impl Validate for DatabaseConfig {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

impl FromEnv for DatabaseConfig {
    fn from_env() -> Result<Self, String> {
        Ok(DatabaseConfig {
            url: env_string("DATABASE_URL", ""),
            host: env_string("DATABASE_HOST", "localhost"),
            port: env_u16("DATABASE_PORT", 5432),
            database: env_string("DATABASE_NAME", "iws"),
            username: env_string("DATABASE_USERNAME", "iws"),
            password: env_string("DATABASE_PASSWORD", ""),
            pool_size: env_usize("DATABASE_POOL_SIZE", 20),
            pool_timeout_secs: env_u64("DATABASE_POOL_TIMEOUT_SECS", 30),
            max_lifetime_secs: env_u64("DATABASE_MAX_LIFETIME_SECS", 600),
            idle_timeout_secs: env_u64("DATABASE_IDLE_TIMEOUT_SECS", 300),
            connection_timeout_secs: env_u64("DATABASE_CONNECTION_TIMEOUT_SECS", 10),
            ssl_mode: SslMode::from_env_str(&env_string("DATABASE_SSL_MODE", "prefer")),
            max_retries: env_u32("DATABASE_MAX_RETRIES", 3),
            retry_delay_ms: env_u64("DATABASE_RETRY_DELAY_MS", 1000),
            migrations_enabled: env_bool("DATABASE_MIGRATIONS_ENABLED", true),
            migrations_path: env_string("DATABASE_MIGRATIONS_PATH", "./database/migrations/"),
        })
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SslMode {
    Disable,
    Allow,
    Prefer,
    Require,
    VerifyCa,
    VerifyFull,
}

impl SslMode {
    pub fn as_str(&self) -> &str {
        match self {
            SslMode::Disable => "disable",
            SslMode::Allow => "allow",
            SslMode::Prefer => "prefer",
            SslMode::Require => "require",
            SslMode::VerifyCa => "verify-ca",
            SslMode::VerifyFull => "verify-full",
        }
    }

    pub fn from_env_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "disable" => SslMode::Disable,
            "allow" => SslMode::Allow,
            "prefer" => SslMode::Prefer,
            "require" => SslMode::Require,
            "verify-ca" | "verifyca" => SslMode::VerifyCa,
            "verify-full" | "verifyfull" => SslMode::VerifyFull,
            _ => SslMode::Prefer,
        }
    }
}

// ============================================================
// REDIS CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub enabled: bool,
    pub url: String,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
    pub database: u8,
    pub pool_size: usize,
    pub timeout_secs: u64,
    pub key_prefix: String,
    pub ttl_default_secs: u64,
}

impl RedisConfig {
    pub fn new() -> Self {
        RedisConfig {
            enabled: true,
            url: String::new(),
            host: "localhost".to_string(),
            port: 6379,
            password: None,
            database: 0,
            pool_size: 10,
            timeout_secs: 5,
            key_prefix: "iws:".to_string(),
            ttl_default_secs: 3600,
        }
    }

    pub fn connection_string(&self) -> String {
        if !self.url.is_empty() {
            return self.url.clone();
        }
        format!("redis://{}:{}/{}", self.host, self.port, self.database)
    }
}

impl FromEnv for RedisConfig {
    fn from_env() -> Result<Self, String> {
        Ok(RedisConfig {
            enabled: env_bool("REDIS_ENABLED", true),
            url: env_string("REDIS_URL", ""),
            host: env_string("REDIS_HOST", "localhost"),
            port: env_u16("REDIS_PORT", 6379),
            password: env_string_optional("REDIS_PASSWORD"),
            database: env_u32("REDIS_DATABASE", 0) as u8,
            pool_size: env_usize("REDIS_POOL_SIZE", 10),
            timeout_secs: env_u64("REDIS_TIMEOUT_SECS", 5),
            key_prefix: env_string("REDIS_KEY_PREFIX", "iws:"),
            ttl_default_secs: env_u64("REDIS_TTL_DEFAULT_SECS", 3600),
        })
    }
}

impl Default for RedisConfig {
    fn default() -> Self {
        RedisConfig::new()
    }
}

// ============================================================
// SCANNING CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanningConfig {
    pub default_profile: String,
    pub max_threads: usize,
    pub default_timeout_secs: u64,
    pub default_delay_ms: u64,
    pub default_max_pages: usize,
    pub default_max_depth: u8,
    pub default_follow_redirects: bool,
    pub default_respect_robots: bool,
    pub user_agent_rotation: bool,
    pub proxy_enabled: bool,
    pub proxy_list_path: Option<String>,
    pub rate_limit_per_domain: u32,
    pub rate_limit_window_secs: u64,
    pub profiles: HashMap<String, ScanProfileConfig>,
}

impl ScanningConfig {
    pub fn new() -> Self {
        ScanningConfig {
            default_profile: "moderate".to_string(),
            max_threads: 200,
            default_timeout_secs: 30,
            default_delay_ms: 100,
            default_max_pages: 500,
            default_max_depth: 3,
            default_follow_redirects: true,
            default_respect_robots: true,
            user_agent_rotation: true,
            proxy_enabled: false,
            proxy_list_path: None,
            rate_limit_per_domain: 10,
            rate_limit_window_secs: 60,
            profiles: HashMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.max_threads == 0 || self.max_threads > 500 {
            return Err(format!("max_threads must be 1-500, got {}", self.max_threads));
        }
        if self.default_max_pages == 0 {
            return Err("default_max_pages must be > 0".to_string());
        }
        Ok(())
    }
}

impl Validate for ScanningConfig {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

impl FromEnv for ScanningConfig {
    fn from_env() -> Result<Self, String> {
        Ok(ScanningConfig {
            default_profile: env_string("SCAN_DEFAULT_PROFILE", "moderate"),
            max_threads: env_usize("SCAN_MAX_THREADS", 200),
            default_timeout_secs: env_u64("SCAN_DEFAULT_TIMEOUT_SECS", 30),
            default_delay_ms: env_u64("SCAN_DEFAULT_DELAY_MS", 100),
            default_max_pages: env_usize("SCAN_DEFAULT_MAX_PAGES", 500),
            default_max_depth: env_u32("SCAN_DEFAULT_MAX_DEPTH", 3) as u8,
            default_follow_redirects: env_bool("SCAN_FOLLOW_REDIRECTS", true),
            default_respect_robots: env_bool("SCAN_RESPECT_ROBOTS", true),
            user_agent_rotation: env_bool("SCAN_USER_AGENT_ROTATION", true),
            proxy_enabled: env_bool("SCAN_PROXY_ENABLED", false),
            proxy_list_path: env_string_optional("SCAN_PROXY_LIST_PATH"),
            rate_limit_per_domain: env_u32("SCAN_RATE_LIMIT_PER_DOMAIN", 10),
            rate_limit_window_secs: env_u64("SCAN_RATE_LIMIT_WINDOW_SECS", 60),
            profiles: HashMap::new(),
        })
    }
}

impl Default for ScanningConfig {
    fn default() -> Self {
        ScanningConfig::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProfileConfig {
    pub name: String,
    pub threads: usize,
    pub timeout_secs: u64,
    pub delay_ms: u64,
    pub max_pages: usize,
    pub max_depth: u8,
    pub follow_redirects: bool,
    pub respect_robots: bool,
    pub enable_js: bool,
    pub enable_stealth: bool,
}

impl ScanProfileConfig {
    pub fn moderate() -> Self {
        ScanProfileConfig {
            name: "moderate".to_string(),
            threads: 50,
            timeout_secs: 15,
            delay_ms: 100,
            max_pages: 500,
            max_depth: 3,
            follow_redirects: true,
            respect_robots: true,
            enable_js: false,
            enable_stealth: false,
        }
    }

    pub fn aggressive() -> Self {
        ScanProfileConfig {
            name: "aggressive".to_string(),
            threads: 100,
            timeout_secs: 10,
            delay_ms: 0,
            max_pages: 1000,
            max_depth: 5,
            follow_redirects: true,
            respect_robots: false,
            enable_js: false,
            enable_stealth: false,
        }
    }

    pub fn stealth() -> Self {
        ScanProfileConfig {
            name: "stealth".to_string(),
            threads: 10,
            timeout_secs: 30,
            delay_ms: 1000,
            max_pages: 100,
            max_depth: 2,
            follow_redirects: false,
            respect_robots: true,
            enable_js: false,
            enable_stealth: true,
        }
    }

    pub fn comprehensive() -> Self {
        ScanProfileConfig {
            name: "comprehensive".to_string(),
            threads: 30,
            timeout_secs: 20,
            delay_ms: 200,
            max_pages: 2000,
            max_depth: 5,
            follow_redirects: true,
            respect_robots: false,
            enable_js: true,
            enable_stealth: false,
        }
    }
}

// ============================================================
// API CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub cors_origins: Vec<String>,
    pub request_timeout_secs: u64,
    pub max_request_size_bytes: u64,
    pub max_response_size_bytes: u64,
    pub max_concurrent_requests: usize,
    pub enable_websocket: bool,
    pub enable_docs: bool,
    pub enable_metrics: bool,
    pub enable_request_logging: bool,
    pub rate_limit_enabled: bool,
    pub rate_limit_per_second: u32,
    pub rate_limit_burst: u32,
    pub jwt_secret: Option<String>,
    pub jwt_expiry_hours: u64,
}

impl ApiConfig {
    pub fn new() -> Self {
        ApiConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            cors_origins: vec!["*".to_string()],
            request_timeout_secs: 30,
            max_request_size_bytes: 10 * 1024 * 1024,
            max_response_size_bytes: 100 * 1024 * 1024,
            max_concurrent_requests: 100,
            enable_websocket: true,
            enable_docs: true,
            enable_metrics: true,
            enable_request_logging: true,
            rate_limit_enabled: true,
            rate_limit_per_second: 100,
            rate_limit_burst: 200,
            jwt_secret: None,
            jwt_expiry_hours: 24,
        }
    }

    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("port cannot be 0".to_string());
        }
        if self.request_timeout_secs == 0 {
            return Err("request_timeout_secs cannot be 0".to_string());
        }
        Ok(())
    }
}

impl Validate for ApiConfig {
    fn validate(&self) -> Result<(), String> {
        self.validate()
    }
}

impl FromEnv for ApiConfig {
    fn from_env() -> Result<Self, String> {
        Ok(ApiConfig {
            host: env_string("API_HOST", "0.0.0.0"),
            port: env_u16("API_PORT", 8080),
            tls_enabled: env_bool("API_TLS_ENABLED", false),
            tls_cert_path: env_string_optional("API_TLS_CERT_PATH"),
            tls_key_path: env_string_optional("API_TLS_KEY_PATH"),
            cors_origins: env_vec_string("API_CORS_ORIGINS", vec!["*".to_string()]),
            request_timeout_secs: env_u64("API_REQUEST_TIMEOUT_SECS", 30),
            max_request_size_bytes: env_u64("API_MAX_REQUEST_SIZE_BYTES", 10 * 1024 * 1024),
            max_response_size_bytes: env_u64("API_MAX_RESPONSE_SIZE_BYTES", 100 * 1024 * 1024),
            max_concurrent_requests: env_usize("API_MAX_CONCURRENT_REQUESTS", 100),
            enable_websocket: env_bool("API_ENABLE_WEBSOCKET", true),
            enable_docs: env_bool("API_ENABLE_DOCS", true),
            enable_metrics: env_bool("API_ENABLE_METRICS", true),
            enable_request_logging: env_bool("API_ENABLE_REQUEST_LOGGING", true),
            rate_limit_enabled: env_bool("API_RATE_LIMIT_ENABLED", true),
            rate_limit_per_second: env_u32("API_RATE_LIMIT_PER_SECOND", 100),
            rate_limit_burst: env_u32("API_RATE_LIMIT_BURST", 200),
            jwt_secret: env_string_optional("API_JWT_SECRET"),
            jwt_expiry_hours: env_u64("API_JWT_EXPIRY_HOURS", 24),
        })
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig::new()
    }
}

// ============================================================
// INTEGRATION CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub service_name: String,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub base_url: Url,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub rate_limit_rps: f64,
    pub cache_enabled: bool,
    pub cache_ttl_secs: u64,
    pub enabled: bool,
}

impl IntegrationConfig {
    pub fn new(service_name: &str, base_url: Url, api_key: &str) -> Self {
        IntegrationConfig {
            service_name: service_name.to_string(),
            api_key: api_key.to_string(),
            api_secret: None,
            base_url,
            timeout_secs: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
            rate_limit_rps: 1.0,
            cache_enabled: true,
            cache_ttl_secs: 3600,
            enabled: true,
        }
    }
}

impl FromEnv for IntegrationConfig {
    fn from_env() -> Result<Self, String> {
        let service_name = env_string("INTEGRATION_SERVICE_NAME", "");
        if service_name.is_empty() {
            return Err("INTEGRATION_SERVICE_NAME is required".to_string());
        }
        let base_url_str = env_string("INTEGRATION_BASE_URL", "");
        if base_url_str.is_empty() {
            return Err("INTEGRATION_BASE_URL is required".to_string());
        }
        let base_url = Url::parse(&base_url_str)
            .map_err(|e| format!("Invalid INTEGRATION_BASE_URL: {}", e))?;
        let api_key = env_string("INTEGRATION_API_KEY", "YOUR_API_KEY_HERE");

        Ok(IntegrationConfig {
            service_name,
            api_key,
            api_secret: env_string_optional("INTEGRATION_API_SECRET"),
            base_url,
            timeout_secs: env_u64("INTEGRATION_TIMEOUT_SECS", 30),
            max_retries: env_u32("INTEGRATION_MAX_RETRIES", 3),
            retry_delay_ms: env_u64("INTEGRATION_RETRY_DELAY_MS", 1000),
            rate_limit_rps: env_f64("INTEGRATION_RATE_LIMIT_RPS", 1.0),
            cache_enabled: env_bool("INTEGRATION_CACHE_ENABLED", true),
            cache_ttl_secs: env_u64("INTEGRATION_CACHE_TTL_SECS", 3600),
            enabled: env_bool("INTEGRATION_ENABLED", true),
        })
    }
}

// ============================================================
// STORAGE CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base_path: String,
    pub scans_path: String,
    pub reports_path: String,
    pub cache_path: String,
    pub logs_path: String,
    pub exports_path: String,
    pub temp_path: String,
    pub backup_path: String,
    pub max_file_size_bytes: u64,
    pub compression_enabled: bool,
    pub encryption_enabled: bool,
    pub retention_days: u32,
    pub auto_cleanup: bool,
    pub cleanup_interval_hours: u64,
}

impl StorageConfig {
    pub fn new() -> Self {
        StorageConfig {
            base_path: "./data".to_string(),
            scans_path: "./data/scans".to_string(),
            reports_path: "./data/reports".to_string(),
            cache_path: "./data/cache".to_string(),
            logs_path: "./data/logs".to_string(),
            exports_path: "./data/exports".to_string(),
            temp_path: "./data/temp".to_string(),
            backup_path: "./data/backups".to_string(),
            max_file_size_bytes: 100 * 1024 * 1024,
            compression_enabled: true,
            encryption_enabled: true,
            retention_days: 90,
            auto_cleanup: true,
            cleanup_interval_hours: 24,
        }
    }
}

impl FromEnv for StorageConfig {
    fn from_env() -> Result<Self, String> {
        let base_path = env_string("STORAGE_BASE_PATH", "./data");
        Ok(StorageConfig {
            scans_path: format!("{}/scans", base_path),
            reports_path: format!("{}/reports", base_path),
            cache_path: format!("{}/cache", base_path),
            logs_path: format!("{}/logs", base_path),
            exports_path: format!("{}/exports", base_path),
            temp_path: format!("{}/temp", base_path),
            backup_path: format!("{}/backups", base_path),
            base_path,
            max_file_size_bytes: env_u64("STORAGE_MAX_FILE_SIZE_BYTES", 100 * 1024 * 1024),
            compression_enabled: env_bool("STORAGE_COMPRESSION_ENABLED", true),
            encryption_enabled: env_bool("STORAGE_ENCRYPTION_ENABLED", true),
            retention_days: env_u32("STORAGE_RETENTION_DAYS", 90),
            auto_cleanup: env_bool("STORAGE_AUTO_CLEANUP", true),
            cleanup_interval_hours: env_u64("STORAGE_CLEANUP_INTERVAL_HOURS", 24),
        })
    }
}

impl Default for StorageConfig {
    fn default() -> Self {
        StorageConfig::new()
    }
}

// ============================================================
// MONITORING CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enabled: bool,
    pub prometheus_enabled: bool,
    pub prometheus_port: u16,
    pub grafana_enabled: bool,
    pub grafana_url: Option<String>,
    pub metrics_interval_secs: u64,
    pub health_check_enabled: bool,
    pub health_check_interval_secs: u64,
    pub alerting_enabled: bool,
    pub alert_channels: Vec<AlertChannel>,
    pub log_forwarding_enabled: bool,
    pub log_forwarding_url: Option<String>,
}

impl MonitoringConfig {
    pub fn new() -> Self {
        MonitoringConfig {
            enabled: true,
            prometheus_enabled: true,
            prometheus_port: 9090,
            grafana_enabled: false,
            grafana_url: None,
            metrics_interval_secs: 15,
            health_check_enabled: true,
            health_check_interval_secs: 30,
            alerting_enabled: true,
            alert_channels: vec![AlertChannel::default()],
            log_forwarding_enabled: false,
            log_forwarding_url: None,
        }
    }
}

impl FromEnv for MonitoringConfig {
    fn from_env() -> Result<Self, String> {
        Ok(MonitoringConfig {
            enabled: env_bool("MONITORING_ENABLED", true),
            prometheus_enabled: env_bool("MONITORING_PROMETHEUS_ENABLED", true),
            prometheus_port: env_u16("MONITORING_PROMETHEUS_PORT", 9090),
            grafana_enabled: env_bool("MONITORING_GRAFANA_ENABLED", false),
            grafana_url: env_string_optional("MONITORING_GRAFANA_URL"),
            metrics_interval_secs: env_u64("MONITORING_METRICS_INTERVAL_SECS", 15),
            health_check_enabled: env_bool("MONITORING_HEALTH_CHECK_ENABLED", true),
            health_check_interval_secs: env_u64("MONITORING_HEALTH_CHECK_INTERVAL_SECS", 30),
            alerting_enabled: env_bool("MONITORING_ALERTING_ENABLED", true),
            alert_channels: vec![AlertChannel::default()],
            log_forwarding_enabled: env_bool("MONITORING_LOG_FORWARDING_ENABLED", false),
            log_forwarding_url: env_string_optional("MONITORING_LOG_FORWARDING_URL"),
        })
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        MonitoringConfig::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertChannel {
    pub channel_type: AlertChannelType,
    pub enabled: bool,
    pub config: serde_json::Value,
}

impl AlertChannel {
    pub fn default() -> Self {
        AlertChannel {
            channel_type: AlertChannelType::Log,
            enabled: true,
            config: serde_json::json!({}),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertChannelType {
    Email,
    Slack,
    Discord,
    Telegram,
    Webhook,
    Log,
}

impl fmt::Display for AlertChannelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlertChannelType::Email => write!(f, "email"),
            AlertChannelType::Slack => write!(f, "slack"),
            AlertChannelType::Discord => write!(f, "discord"),
            AlertChannelType::Telegram => write!(f, "telegram"),
            AlertChannelType::Webhook => write!(f, "webhook"),
            AlertChannelType::Log => write!(f, "log"),
        }
    }
}

// ============================================================
// MASTER CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterConfig {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub scanning: ScanningConfig,
    pub api: ApiConfig,
    pub storage: StorageConfig,
    pub monitoring: MonitoringConfig,
    pub integrations: Vec<IntegrationConfig>,
    pub custom: serde_json::Value,
}

impl MasterConfig {
    pub fn new() -> Self {
        MasterConfig {
            app: AppConfig::default(),
            database: DatabaseConfig::default(),
            redis: RedisConfig::default(),
            scanning: ScanningConfig::default(),
            api: ApiConfig::default(),
            storage: StorageConfig::default(),
            monitoring: MonitoringConfig::default(),
            integrations: vec![],
            custom: serde_json::json!({}),
        }
    }

    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = vec![];

        if let Err(e) = self.app.validate() { errors.push(format!("app: {}", e)); }
        if let Err(e) = self.database.validate() { errors.push(format!("database: {}", e)); }
        if let Err(e) = self.scanning.validate() { errors.push(format!("scanning: {}", e)); }
        if let Err(e) = self.api.validate() { errors.push(format!("api: {}", e)); }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    pub fn get_integration(&self, service_name: &str) -> Option<&IntegrationConfig> {
        self.integrations.iter().find(|i| i.service_name == service_name)
    }

    pub fn is_production(&self) -> bool {
        self.app.environment.is_production()
    }
}

impl Validate for MasterConfig {
    fn validate(&self) -> Result<(), String> {
        self.validate().map_err(|errors| errors.join("; "))
    }
}

impl FromEnv for MasterConfig {
    fn from_env() -> Result<Self, String> {
        Ok(MasterConfig {
            app: AppConfig::from_env()?,
            database: DatabaseConfig::from_env()?,
            redis: RedisConfig::from_env()?,
            scanning: ScanningConfig::from_env()?,
            api: ApiConfig::from_env()?,
            storage: StorageConfig::from_env()?,
            monitoring: MonitoringConfig::from_env()?,
            integrations: vec![],
            custom: serde_json::json!({}),
        })
    }
}

impl Default for MasterConfig {
    fn default() -> Self {
        MasterConfig::new()
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_from_str() {
        assert_eq!(Environment::from_str("prod"), Environment::Production);
        assert_eq!(Environment::from_str("DEV"), Environment::Development);
        assert!(Environment::Production.is_production());
        assert!(!Environment::Development.is_production());
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Error > LogLevel::Warn);
        assert!(LogLevel::Debug < LogLevel::Info);
    }

    #[test]
    fn test_app_config_validation() {
        let mut config = AppConfig::default();
        assert!(config.validate().is_ok());

        config.max_concurrent_scans = 0;
        assert!(config.validate().is_err());

        config.max_concurrent_scans = 10;
        config.max_scan_duration_secs = 30;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_database_config_connection_string() {
        let config = DatabaseConfig::default();
        let conn_str = config.connection_string();
        assert!(conn_str.contains("postgresql://"));
        assert!(conn_str.contains("localhost"));
    }

    #[test]
    fn test_database_config_url_override() {
        let mut config = DatabaseConfig::default();
        config.url = "postgresql://custom:pass@host/db".to_string();
        assert_eq!(config.connection_string(), "postgresql://custom:pass@host/db");
    }

    #[test]
    fn test_redis_config_default() {
        let config = RedisConfig::default();
        assert!(config.enabled);
        assert_eq!(config.port, 6379);
    }

    #[test]
    fn test_scan_profile_presets() {
        let aggressive = ScanProfileConfig::aggressive();
        assert_eq!(aggressive.threads, 100);
        assert!(!aggressive.respect_robots);

        let stealth = ScanProfileConfig::stealth();
        assert_eq!(stealth.threads, 10);
        assert!(stealth.enable_stealth);
    }

    #[test]
    fn test_api_config_socket_addr() {
        let config = ApiConfig::default();
        assert_eq!(config.socket_addr(), "0.0.0.0:8080");
    }

    #[test]
    fn test_master_config_validate() {
        let config = MasterConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_master_config_get_integration() {
        let mut config = MasterConfig::default();
        config.integrations.push(IntegrationConfig::new(
            "shodan",
            Url::parse("https://api.shodan.io").unwrap(),
            "test-key",
        ));

        let shodan = config.get_integration("shodan");
        assert!(shodan.is_some());
        assert_eq!(shodan.unwrap().api_key, "test-key");

        assert!(config.get_integration("nonexistent").is_none());
    }

    #[test]
    fn test_ssl_modes() {
        assert_eq!(SslMode::Disable.as_str(), "disable");
        assert_eq!(SslMode::VerifyFull.as_str(), "verify-full");
        assert_eq!(SslMode::from_env_str("verify-ca"), SslMode::VerifyCa);
        assert_eq!(SslMode::from_env_str("invalid"), SslMode::Prefer);
    }

    #[test]
    fn test_alert_channel_types() {
        assert_eq!(AlertChannelType::Slack.to_string(), "slack");
        assert_eq!(AlertChannelType::Webhook.to_string(), "webhook");
    }

    #[test]
    fn test_from_env_app_config_defaults() {
        // Bersihkan env variables sebelum test
        std::env::remove_var("IWS_APP_NAME");
        std::env::remove_var("IWS_ENV");
        std::env::remove_var("IWS_DEBUG");
        std::env::remove_var("IWS_MAX_CONCURRENT_SCANS");

        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.app_name, "IWS");
        assert_eq!(config.environment, Environment::Development);
        assert!(!config.debug);
        assert_eq!(config.max_concurrent_scans, 10);
    }

    #[test]
    fn test_from_env_app_config_custom() {
        std::env::set_var("IWS_APP_NAME", "MyScanner");
        std::env::set_var("IWS_ENV", "production");
        std::env::set_var("IWS_DEBUG", "true");
        std::env::set_var("IWS_MAX_CONCURRENT_SCANS", "25");

        let config = AppConfig::from_env().unwrap();
        assert_eq!(config.app_name, "MyScanner");
        assert_eq!(config.environment, Environment::Production);
        assert!(config.debug);
        assert_eq!(config.max_concurrent_scans, 25);

        // Cleanup
        std::env::remove_var("IWS_APP_NAME");
        std::env::remove_var("IWS_ENV");
        std::env::remove_var("IWS_DEBUG");
        std::env::remove_var("IWS_MAX_CONCURRENT_SCANS");
    }

    #[test]
    fn test_from_env_validated() {
        std::env::set_var("IWS_MAX_CONCURRENT_SCANS", "0");
        let result = AppConfig::from_env_validated();
        assert!(result.is_err());

        std::env::set_var("IWS_MAX_CONCURRENT_SCANS", "10");
        let result = AppConfig::from_env_validated();
        assert!(result.is_ok());

        std::env::remove_var("IWS_MAX_CONCURRENT_SCANS");
    }

    #[test]
    fn test_from_env_master_config() {
        let config = MasterConfig::from_env().unwrap();
        assert_eq!(config.app.app_name, "IWS");
        assert_eq!(config.database.host, "localhost");
        assert_eq!(config.api.port, 8080);
    }

    #[test]
    fn test_env_vec_string() {
        std::env::set_var("IWS_API_CORS_ORIGINS", "http://a.com, http://b.com");
        let origins = env_vec_string("API_CORS_ORIGINS", vec![]);
        assert_eq!(origins.len(), 2);
        assert!(origins.contains(&"http://a.com".to_string()));

        std::env::remove_var("IWS_API_CORS_ORIGINS");
    }

    #[test]
    fn test_env_bool_variants() {
        for (val, expected) in &[("true", true), ("1", true), ("yes", true), ("on", true), ("false", false), ("0", false)] {
            std::env::set_var("IWS_TEST_BOOL", val);
            assert_eq!(env_bool("TEST_BOOL", false), *expected, "Failed for value: {}", val);
        }
        std::env::remove_var("IWS_TEST_BOOL");
    }
}
