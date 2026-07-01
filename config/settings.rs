// config/settings.rs
// IWS v1.0 - Settings Configuration
// Hierarchical configuration: default -> file -> environment variables (prefix IWS_)

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use config::{Config, Environment, File};

// ============================================================
// SETTINGS STRUCT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub redis: RedisSettings,
    pub scanning: ScanningSettings,
    pub api: ApiSettings,
    pub storage: StorageSettings,
    pub logging: LoggingSettings,
    pub monitoring: MonitoringSettings,
    pub integrations: HashMap<String, IntegrationSettings>,
    pub custom: serde_json::Value,
}

impl Settings {
    /// Load konfigurasi dari multiple sources dengan hierarki:
    /// 1. Default values (hardcoded)
    /// 2. Config file (config/settings.{env}.toml)
    /// 3. Environment variables (prefix IWS_)
    pub fn load(env: &str) -> Result<Self, config::ConfigError> {
        let config = Config::builder()
            // Layer 1: Default values
            .add_source(Config::try_from(&Settings::default()).unwrap())
            // Layer 2: Config file berdasarkan environment
            .add_source(File::with_name(&format!("config/settings.{}", env)).required(false))
            // Layer 3: Environment variables dengan prefix IWS_
            .add_source(
                Environment::with_prefix("IWS")
                    .separator("__")
                    .try_parsing(true)
            )
            .build()?;

        config.try_deserialize()
    }

    /// Reload konfigurasi tanpa restart (SIGHUP handler)
    pub fn reload(&mut self, env: &str) -> Result<(), config::ConfigError> {
        let new_settings = Settings::load(env)?;
        *self = new_settings;
        Ok(())
    }

    /// Validate semua settings
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Err(e) = self.app.validate() { errors.push(format!("app: {}", e)); }
        if let Err(e) = self.database.validate() { errors.push(format!("database: {}", e)); }
        if let Err(e) = self.scanning.validate() { errors.push(format!("scanning: {}", e)); }
        if let Err(e) = self.api.validate() { errors.push(format!("api: {}", e)); }
        if let Err(e) = self.logging.validate() { errors.push(format!("logging: {}", e)); }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Export current config ke file untuk debugging
    pub fn export(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let toml = toml::to_string_pretty(self)?;
        std::fs::write(path, toml)?;
        Ok(())
    }

    pub fn is_production(&self) -> bool {
        self.app.environment == "production"
    }

    pub fn is_development(&self) -> bool {
        self.app.environment == "development"
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            app: AppSettings::default(),
            database: DatabaseSettings::default(),
            redis: RedisSettings::default(),
            scanning: ScanningSettings::default(),
            api: ApiSettings::default(),
            storage: StorageSettings::default(),
            logging: LoggingSettings::default(),
            monitoring: MonitoringSettings::default(),
            integrations: HashMap::new(),
            custom: serde_json::json!({}),
        }
    }
}

// ============================================================
// APP SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
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

impl AppSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.max_concurrent_scans == 0 || self.max_concurrent_scans > 100 {
            return Err(format!("max_concurrent_scans must be 1-100, got {}", self.max_concurrent_scans));
        }
        if self.max_scan_duration_secs < 60 {
            return Err(format!("max_scan_duration_secs >= 60, got {}", self.max_scan_duration_secs));
        }
        Ok(())
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            name: "IWS".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
            debug: false,
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
}

// ============================================================
// DATABASE SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub username: String,
    pub password: String,
    pub pool_size: usize,
    pub pool_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub idle_timeout_secs: u64,
    pub connection_timeout_secs: u64,
    pub ssl_mode: String,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub migrations_enabled: bool,
    pub migrations_path: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        if !self.url.is_empty() {
            return self.url.clone();
        }
        format!(
            "postgresql://{}:{}@{}:{}/{}?sslmode={}",
            self.username, self.password, self.host, self.port, self.name, self.ssl_mode
        )
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.url.is_empty() && (self.host.is_empty() || self.name.is_empty()) {
            return Err("Either url or (host + name) required".to_string());
        }
        if self.pool_size == 0 || self.pool_size > 100 {
            return Err(format!("pool_size must be 1-100, got {}", self.pool_size));
        }
        Ok(())
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings {
            url: String::new(),
            host: "localhost".to_string(),
            port: 5432,
            name: "iws".to_string(),
            username: "iws".to_string(),
            password: String::new(),
            pool_size: 20,
            pool_timeout_secs: 30,
            max_lifetime_secs: 600,
            idle_timeout_secs: 300,
            connection_timeout_secs: 10,
            ssl_mode: "prefer".to_string(),
            max_retries: 3,
            retry_delay_ms: 1000,
            migrations_enabled: true,
            migrations_path: "./database/migrations/".to_string(),
        }
    }
}

// ============================================================
// REDIS SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisSettings {
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

impl RedisSettings {
    pub fn connection_string(&self) -> String {
        if !self.url.is_empty() {
            return self.url.clone();
        }
        format!("redis://{}:{}/{}", self.host, self.port, self.database)
    }
}

impl Default for RedisSettings {
    fn default() -> Self {
        RedisSettings {
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
}

// ============================================================
// SCANNING SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanningSettings {
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
    pub profiles: HashMap<String, ScanProfileSettings>,
}

impl ScanningSettings {
    pub fn validate(&self) -> Result<(), String> {
        if self.max_threads == 0 || self.max_threads > 500 {
            return Err(format!("max_threads 1-500, got {}", self.max_threads));
        }
        if self.default_max_pages == 0 {
            return Err("default_max_pages must be > 0".to_string());
        }
        Ok(())
    }
}

impl Default for ScanningSettings {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert("moderate".to_string(), ScanProfileSettings::moderate());
        profiles.insert("aggressive".to_string(), ScanProfileSettings::aggressive());
        profiles.insert("stealth".to_string(), ScanProfileSettings::stealth());
        profiles.insert("comprehensive".to_string(), ScanProfileSettings::comprehensive());

        ScanningSettings {
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
            profiles,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProfileSettings {
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

impl ScanProfileSettings {
    pub fn moderate() -> Self {
        ScanProfileSettings { name: "moderate".into(), threads: 50, timeout_secs: 15, delay_ms: 100, max_pages: 500, max_depth: 3, follow_redirects: true, respect_robots: true, enable_js: false, enable_stealth: false }
    }
    pub fn aggressive() -> Self {
        ScanProfileSettings { name: "aggressive".into(), threads: 100, timeout_secs: 10, delay_ms: 0, max_pages: 1000, max_depth: 5, follow_redirects: true, respect_robots: false, enable_js: false, enable_stealth: false }
    }
    pub fn stealth() -> Self {
        ScanProfileSettings { name: "stealth".into(), threads: 10, timeout_secs: 30, delay_ms: 1000, max_pages: 100, max_depth: 2, follow_redirects: false, respect_robots: true, enable_js: false, enable_stealth: true }
    }
    pub fn comprehensive() -> Self {
        ScanProfileSettings { name: "comprehensive".into(), threads: 30, timeout_secs: 20, delay_ms: 200, max_pages: 2000, max_depth: 5, follow_redirects: true, respect_robots: false, enable_js: true, enable_stealth: false }
    }
}

// ============================================================
// API SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSettings {
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

impl ApiSettings {
    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 { return Err("port cannot be 0".into()); }
        if self.request_timeout_secs == 0 { return Err("request_timeout_secs cannot be 0".into()); }
        Ok(())
    }
}

impl Default for ApiSettings {
    fn default() -> Self {
        ApiSettings {
            host: "0.0.0.0".into(), port: 8080, tls_enabled: false,
            tls_cert_path: None, tls_key_path: None,
            cors_origins: vec!["*".into()], request_timeout_secs: 30,
            max_request_size_bytes: 10*1024*1024, max_response_size_bytes: 100*1024*1024,
            max_concurrent_requests: 100, enable_websocket: true,
            enable_docs: true, enable_metrics: true, enable_request_logging: true,
            rate_limit_enabled: true, rate_limit_per_second: 100, rate_limit_burst: 200,
            jwt_secret: None, jwt_expiry_hours: 24,
        }
    }
}

// ============================================================
// STORAGE SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSettings {
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

impl Default for StorageSettings {
    fn default() -> Self {
        StorageSettings {
            base_path: "./data".into(), scans_path: "./data/scans".into(),
            reports_path: "./data/reports".into(), cache_path: "./data/cache".into(),
            logs_path: "./data/logs".into(), exports_path: "./data/exports".into(),
            temp_path: "./data/temp".into(), backup_path: "./data/backups".into(),
            max_file_size_bytes: 100*1024*1024, compression_enabled: true,
            encryption_enabled: true, retention_days: 90,
            auto_cleanup: true, cleanup_interval_hours: 24,
        }
    }
}

// ============================================================
// LOGGING SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String,
    pub output: String,
    pub file_path: Option<String>,
    pub rotation_size_mb: u64,
    pub rotation_count: u32,
    pub remote_enabled: bool,
    pub remote_url: Option<String>,
    pub mask_sensitive_data: bool,
}

impl LoggingSettings {
    pub fn validate(&self) -> Result<(), String> {
        let valid_levels = ["trace", "debug", "info", "warn", "error", "fatal"];
        if !valid_levels.contains(&self.level.as_str()) {
            return Err(format!("Invalid log level: {}", self.level));
        }
        Ok(())
    }
}

impl Default for LoggingSettings {
    fn default() -> Self {
        LoggingSettings {
            level: "info".into(), format: "json".into(), output: "stdout".into(),
            file_path: Some("./data/logs/iws.log".into()),
            rotation_size_mb: 10, rotation_count: 5,
            remote_enabled: false, remote_url: None, mask_sensitive_data: true,
        }
    }
}

// ============================================================
// MONITORING SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    pub enabled: bool,
    pub prometheus_enabled: bool,
    pub prometheus_port: u16,
    pub grafana_enabled: bool,
    pub grafana_url: Option<String>,
    pub metrics_interval_secs: u64,
    pub health_check_enabled: bool,
    pub health_check_interval_secs: u64,
    pub alerting_enabled: bool,
    pub alert_channels: Vec<String>,
    pub log_forwarding_enabled: bool,
    pub log_forwarding_url: Option<String>,
}

impl Default for MonitoringSettings {
    fn default() -> Self {
        MonitoringSettings {
            enabled: true, prometheus_enabled: true, prometheus_port: 9090,
            grafana_enabled: false, grafana_url: None,
            metrics_interval_secs: 15, health_check_enabled: true,
            health_check_interval_secs: 30, alerting_enabled: true,
            alert_channels: vec!["log".into()],
            log_forwarding_enabled: false, log_forwarding_url: None,
        }
    }
}

// ============================================================
// INTEGRATION SETTINGS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSettings {
    pub service_name: String,
    pub api_key: String,
    pub api_secret: Option<String>,
    pub base_url: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub rate_limit_rps: f64,
    pub cache_enabled: bool,
    pub cache_ttl_secs: u64,
    pub enabled: bool,
}

impl IntegrationSettings {
    pub fn new(service_name: &str, base_url: &str, api_key: &str) -> Self {
        IntegrationSettings {
            service_name: service_name.into(), api_key: api_key.into(),
            api_secret: None, base_url: base_url.into(),
            timeout_secs: 30, max_retries: 3, retry_delay_ms: 1000,
            rate_limit_rps: 1.0, cache_enabled: true, cache_ttl_secs: 3600,
            enabled: true,
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
    fn test_default_settings_valid() {
        let settings = Settings::default();
        assert!(settings.validate().is_ok());
        assert!(settings.is_development());
    }

    #[test]
    fn test_app_settings_validation() {
        let mut app = AppSettings::default();
        assert!(app.validate().is_ok());
        app.max_concurrent_scans = 0;
        assert!(app.validate().is_err());
    }

    #[test]
    fn test_database_connection_string() {
        let db = DatabaseSettings::default();
        let conn = db.connection_string();
        assert!(conn.contains("postgresql://"));
        assert!(conn.contains("localhost"));
    }

    #[test]
    fn test_database_url_override() {
        let mut db = DatabaseSettings::default();
        db.url = "postgresql://custom:pass@host/db".into();
        assert_eq!(db.connection_string(), "postgresql://custom:pass@host/db");
    }

    #[test]
    fn test_redis_connection_string() {
        let redis = RedisSettings::default();
        let conn = redis.connection_string();
        assert!(conn.contains("redis://"));
    }

    #[test]
    fn test_scan_profiles_exist() {
        let settings = ScanningSettings::default();
        assert!(settings.profiles.contains_key("moderate"));
        assert!(settings.profiles.contains_key("aggressive"));
        assert!(settings.profiles.contains_key("stealth"));
        assert!(settings.profiles.contains_key("comprehensive"));
    }

    #[test]
    fn test_scan_profile_values() {
        let aggressive = ScanProfileSettings::aggressive();
        assert_eq!(aggressive.threads, 100);
        assert!(!aggressive.respect_robots);

        let stealth = ScanProfileSettings::stealth();
        assert_eq!(stealth.delay_ms, 1000);
        assert!(stealth.enable_stealth);
    }

    #[test]
    fn test_api_socket_addr() {
        let api = ApiSettings::default();
        assert_eq!(api.socket_addr(), "0.0.0.0:8080");
    }

    #[test]
    fn test_logging_validation() {
        let log = LoggingSettings::default();
        assert!(log.validate().is_ok());

        let mut bad = LoggingSettings::default();
        bad.level = "invalid".into();
        assert!(bad.validate().is_err());
    }

    #[test]
    fn test_settings_reload() {
        let mut settings = Settings::default();
        settings.app.max_concurrent_scans = 50;
        assert!(settings.reload("development").is_ok());
    }

    #[test]
    fn test_export_settings() {
        let settings = Settings::default();
        let tmp = std::env::temp_dir().join("iws_test_settings.toml");
        let path = tmp.to_str().unwrap();
        assert!(settings.export(path).is_ok());
        assert!(tmp.exists());
        std::fs::remove_file(&tmp).ok();
    }
}
