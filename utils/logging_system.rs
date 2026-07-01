// utils/logging_system.rs
// IWS v1.0 - Logging System
// Menyediakan structured logging dengan rotation, remote forwarding, dan context

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error, debug, trace, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub component: String,
    pub request_id: Option<String>,
    pub scan_id: Option<String>,
    pub user_id: Option<String>,
    pub duration_ms: Option<u64>,
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct LoggingSystem {
    level: String,
    format: String,
    file_path: Option<PathBuf>,
    rotation_size: u64,
    remote_enabled: bool,
    mask_sensitive: bool,
    file_writer: Option<Mutex<File>>,
    current_size: Mutex<u64>,
}

impl LoggingSystem {
    pub fn new(level: &str, format: &str, file_path: Option<&str>, rotation_mb: u64, mask_sensitive: bool) -> Self {
        let file_writer = file_path.and_then(|p| {
            OpenOptions::new().create(true).append(true).open(p).ok().map(|f| Mutex::new(f))
        });

        LoggingSystem {
            level: level.to_string(),
            format: format.to_string(),
            file_path: file_path.map(PathBuf::from),
            rotation_size: rotation_mb * 1024 * 1024,
            remote_enabled: false,
            mask_sensitive,
            file_writer,
            current_size: Mutex::new(0),
        }
    }

    pub fn init_tracing(level: &str, json_format: bool) {
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(level));

        let subscriber = tracing_subscriber::registry().with(env_filter);

        if json_format {
            subscriber.with(fmt::Layer::new().json()).init();
        } else {
            subscriber.with(fmt::Layer::new().pretty()).init();
        }
    }

    pub fn log(&self, entry: LogEntry) {
        let masked_entry = if self.mask_sensitive { self.mask_entry(entry) } else { entry };

        // Console logging via tracing
        let msg = format!(
            "[{}] [{}] {} | component={}",
            masked_entry.timestamp, masked_entry.level, masked_entry.message, masked_entry.component
        );

        match masked_entry.level.as_str() {
            "error" | "ERROR" => error!("{}", msg),
            "warn" | "WARN" => warn!("{}", msg),
            "debug" | "DEBUG" => debug!("{}", msg),
            "trace" | "TRACE" => trace!("{}", msg),
            _ => info!("{}", msg),
        }

        // File logging dengan rotation
        if let Some(ref writer) = self.file_writer {
            if let Ok(mut file) = writer.lock() {
                let line = serde_json::to_string(&masked_entry).unwrap_or_default();
                if let Ok(_) = writeln!(file, "{}", line) {
                    let mut size = self.current_size.lock().unwrap();
                    *size += line.len() as u64;
                    if *size >= self.rotation_size {
                        *size = 0;
                        drop(file);
                        self.rotate_log();
                    }
                }
            }
        }
    }

    pub fn info(&self, component: &str, message: &str) {
        self.log(LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: message.to_string(),
            component: component.to_string(),
            request_id: None, scan_id: None, user_id: None, duration_ms: None,
            extra: serde_json::json!({}),
        });
    }

    pub fn error(&self, component: &str, message: &str) {
        self.log(LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: "ERROR".to_string(),
            message: message.to_string(),
            component: component.to_string(),
            request_id: None, scan_id: None, user_id: None, duration_ms: None,
            extra: serde_json::json!({}),
        });
    }

    pub fn with_context(&self, component: &str, message: &str, scan_id: Option<&str>, request_id: Option<&str>) {
        self.log(LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: message.to_string(),
            component: component.to_string(),
            request_id: request_id.map(|s| s.to_string()),
            scan_id: scan_id.map(|s| s.to_string()),
            user_id: None, duration_ms: None,
            extra: serde_json::json!({}),
        });
    }

    fn mask_entry(&self, mut entry: LogEntry) -> LogEntry {
        let sensitive_keys = ["password", "token", "secret", "api_key", "key", "credential", "auth"];
        if let serde_json::Value::Object(ref mut map) = entry.extra {
            for key in sensitive_keys {
                if map.contains_key(key) {
                    map.insert(key.to_string(), serde_json::json!("***MASKED***"));
                }
            }
        }
        entry
    }

    fn rotate_log(&self) {
        if let Some(ref path) = self.file_path {
            let backup = format!("{}.1", path.display());
            let _ = std::fs::rename(path, &backup);
        }
    }
}

impl Default for LoggingSystem {
    fn default() -> Self {
        LoggingSystem::new("info", "json", Some("./data/logs/iws.log"), 10, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_logging_system() {
        let logger = LoggingSystem::default();
        assert_eq!(logger.level, "info");
    }

    #[test]
    fn test_log_info() {
        let logger = LoggingSystem::new("debug", "text", None, 10, false);
        logger.info("test_component", "test message");
    }

    #[test]
    fn test_log_error() {
        let logger = LoggingSystem::new("debug", "text", None, 10, false);
        logger.error("scanner", "connection failed");
    }

    #[test]
    fn test_log_with_context() {
        let logger = LoggingSystem::new("debug", "text", None, 10, false);
        logger.with_context("scanner", "scan started", Some("scan-123"), Some("req-456"));
    }

    #[test]
    fn test_mask_sensitive() {
        let logger = LoggingSystem::new("debug", "text", None, 10, true);
        let mut entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: "INFO".to_string(),
            message: "auth".to_string(),
            component: "auth".to_string(),
            request_id: None, scan_id: None, user_id: None, duration_ms: None,
            extra: serde_json::json!({"password": "secret123", "username": "admin"}),
        };
        entry = logger.mask_entry(entry);
        assert_eq!(entry.extra["password"], "***MASKED***");
        assert_eq!(entry.extra["username"], "admin");
    }

    #[test]
    fn test_rotate_log() {
        let tmp = std::env::temp_dir().join("iws_test_log.txt");
        std::fs::write(&tmp, b"test content").unwrap();
        let logger = LoggingSystem::new("info", "json", Some(tmp.to_str().unwrap()), 0, false);
        logger.rotate_log();
        std::fs::remove_file(&tmp).ok();
        let backup = format!("{}.1", tmp.display());
        std::fs::remove_file(&backup).ok();
    }
}
