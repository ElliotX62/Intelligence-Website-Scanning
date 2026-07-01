// shared/types/common_types.rs
// IWS v1.0 - Common Types
// Mendefinisikan tipe data fundamental yang digunakan di seluruh sistem

use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::time::Duration as StdDuration;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use url::Url;

// ============================================================
// URL WRAPPER
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TargetUrl {
    url: Url,
    original: String,
}

impl TargetUrl {
    pub fn new(input: &str) -> Result<Self, CommonTypeError> {
        let trimmed = input.trim().to_string();
        if trimmed.is_empty() {
            return Err(CommonTypeError::InvalidUrl("URL is empty".to_string()));
        }
        let normalized = if !trimmed.contains("://") {
            format!("https://{}", trimmed)
        } else {
            trimmed.clone()
        };
        let parsed = Url::parse(&normalized).map_err(|e| {
            CommonTypeError::InvalidUrl(format!("Failed to parse URL: {}", e))
        })?;
        let scheme = parsed.scheme();
        if scheme != "http" && scheme != "https" {
            return Err(CommonTypeError::InvalidUrl(format!(
                "Unsupported scheme: {}. Only http and https are allowed",
                scheme
            )));
        }
        if parsed.host_str().is_none() {
            return Err(CommonTypeError::InvalidUrl(
                "URL has no valid host".to_string()
            ));
        }
        Ok(TargetUrl {
            url: parsed,
            original: trimmed,
        })
    }

    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }

    pub fn scheme(&self) -> &str {
        self.url.scheme()
    }

    pub fn host(&self) -> Option<&str> {
        self.url.host_str()
    }

    pub fn domain(&self) -> Option<String> {
        self.url.host_str().map(|h| {
            let parts: Vec<&str> = h.split('.').collect();
            if parts.len() >= 2 {
                format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1])
            } else {
                h.to_string()
            }
        })
    }

    pub fn port(&self) -> Option<u16> {
        self.url.port()
    }

    pub fn path(&self) -> &str {
        self.url.path()
    }

    pub fn original(&self) -> &str {
        &self.original
    }

    pub fn to_url(&self) -> Url {
        self.url.clone()
    }
}

impl fmt::Display for TargetUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

// ============================================================
// DOMAIN
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Domain {
    value: String,
    is_valid: bool,
}

impl Domain {
    pub fn new(input: &str) -> Result<Self, CommonTypeError> {
        let trimmed = input.trim().to_lowercase();
        if trimmed.is_empty() {
            return Err(CommonTypeError::InvalidDomain("Domain is empty".to_string()));
        }
        if trimmed.len() > 253 {
            return Err(CommonTypeError::InvalidDomain(
                format!("Domain too long: {} chars (max 253)", trimmed.len())
            ));
        }
        if trimmed.contains(' ') {
            return Err(CommonTypeError::InvalidDomain(
                "Domain contains spaces".to_string()
            ));
        }
        let labels: Vec<&str> = trimmed.split('.').collect();
        for label in &labels {
            if label.is_empty() || label.len() > 63 {
                return Err(CommonTypeError::InvalidDomain(
                    format!("Invalid label length: '{}'", label)
                ));
            }
            if label.starts_with('-') || label.ends_with('-') {
                return Err(CommonTypeError::InvalidDomain(
                    format!("Label starts/ends with hyphen: '{}'", label)
                ));
            }
        }
        if labels.len() < 2 {
            return Err(CommonTypeError::InvalidDomain(
                "Domain must have at least a TLD".to_string()
            ));
        }
        Ok(Domain {
            value: trimmed,
            is_valid: true,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn tld(&self) -> Option<&str> {
        self.value.rsplit('.').next()
    }

    pub fn root_domain(&self) -> Option<String> {
        let parts: Vec<&str> = self.value.split('.').collect();
        if parts.len() >= 2 {
            Some(format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]))
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// ============================================================
// IP ADDRESS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum IpAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

impl IpAddress {
    pub fn new(input: &str) -> Result<Self, CommonTypeError> {
        if let Ok(v4) = Ipv4Addr::from_str(input) {
            return Ok(IpAddress::V4(v4));
        }
        if let Ok(v6) = Ipv6Addr::from_str(input) {
            return Ok(IpAddress::V6(v6));
        }
        Err(CommonTypeError::InvalidIp(format!("Invalid IP address: {}", input)))
    }

    pub fn is_private(&self) -> bool {
        match self {
            IpAddress::V4(ip) => {
                ip.is_private() || ip.is_loopback() || ip.is_link_local()
            }
            IpAddress::V6(ip) => {
                ip.is_loopback() || ip.is_unique_local()
            }
        }
    }

    pub fn is_loopback(&self) -> bool {
        match self {
            IpAddress::V4(ip) => ip.is_loopback(),
            IpAddress::V6(ip) => ip.is_loopback(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            IpAddress::V4(ip) => ip.to_string(),
            IpAddress::V6(ip) => ip.to_string(),
        }
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

// ============================================================
// TIMESTAMP
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Timestamp(Utc::now())
    }

    pub fn from_iso8601(s: &str) -> Result<Self, CommonTypeError> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| Timestamp(dt.with_timezone(&Utc)))
            .or_else(|_| {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                    .map(|ndt| Timestamp(ndt.and_utc()))
            })
            .map_err(|e| CommonTypeError::InvalidTimestamp(format!("{}", e)))
    }

    pub fn from_unix(secs: i64) -> Self {
        Timestamp(DateTime::from_timestamp(secs, 0).unwrap_or_else(Utc::now))
    }

    pub fn to_iso8601(&self) -> String {
        self.0.to_rfc3339()
    }

    pub fn to_unix(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn elapsed(&self) -> StdDuration {
        let now = Utc::now();
        let diff = now - self.0;
        diff.to_std().unwrap_or(StdDuration::from_secs(0))
    }

    pub fn add_seconds(&self, secs: i64) -> Self {
        Timestamp(self.0 + chrono::Duration::seconds(secs))
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        Timestamp(self.0 + chrono::Duration::minutes(minutes))
    }

    pub fn add_hours(&self, hours: i64) -> Self {
        Timestamp(self.0 + chrono::Duration::hours(hours))
    }

    pub fn is_before(&self, other: &Timestamp) -> bool {
        self.0 < other.0
    }

    pub fn is_after(&self, other: &Timestamp) -> bool {
        self.0 > other.0
    }

    pub fn format_human(&self) -> String {
        self.0.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_iso8601())
    }
}

// ============================================================
// DURATION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(StdDuration);

impl Duration {
    pub fn from_secs(secs: u64) -> Self {
        Duration(StdDuration::from_secs(secs))
    }

    pub fn from_millis(millis: u64) -> Self {
        Duration(StdDuration::from_millis(millis))
    }

    pub fn from_nanos(nanos: u64) -> Self {
        Duration(StdDuration::from_nanos(nanos))
    }

    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }

    pub fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    pub fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }

    pub fn to_std(&self) -> StdDuration {
        self.0
    }

    pub fn format_human(&self) -> String {
        let secs = self.0.as_secs();
        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m {}s", secs / 60, secs % 60)
        } else {
            format!("{}h {}m {}s", secs / 3600, (secs % 3600) / 60, secs % 60)
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_human())
    }
}

// ============================================================
// ID TYPES
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScanId(Uuid);

impl ScanId {
    pub fn new() -> Self {
        ScanId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        ScanId(uuid)
    }

    pub fn from_str(s: &str) -> Result<Self, CommonTypeError> {
        Uuid::parse_str(s)
            .map(ScanId)
            .map_err(|e| CommonTypeError::InvalidId(format!("Invalid ScanId: {}", e)))
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }

    pub fn nil() -> Self {
        ScanId(Uuid::nil())
    }

    pub fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

impl fmt::Display for ScanId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ScanId {
    fn default() -> Self {
        ScanId::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        UserId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        UserId(uuid)
    }

    pub fn from_str(s: &str) -> Result<Self, CommonTypeError> {
        Uuid::parse_str(s)
            .map(UserId)
            .map_err(|e| CommonTypeError::InvalidId(format!("Invalid UserId: {}", e)))
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }

    pub fn nil() -> Self {
        UserId(Uuid::nil())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for UserId {
    fn default() -> Self {
        UserId::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AnalysisId(Uuid);

impl AnalysisId {
    pub fn new() -> Self {
        AnalysisId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        AnalysisId(uuid)
    }

    pub fn from_str(s: &str) -> Result<Self, CommonTypeError> {
        Uuid::parse_str(s)
            .map(AnalysisId)
            .map_err(|e| CommonTypeError::InvalidId(format!("Invalid AnalysisId: {}", e)))
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }

    pub fn nil() -> Self {
        AnalysisId(Uuid::nil())
    }
}

impl fmt::Display for AnalysisId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for AnalysisId {
    fn default() -> Self {
        AnalysisId::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReportId(Uuid);

impl ReportId {
    pub fn new() -> Self {
        ReportId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        ReportId(uuid)
    }

    pub fn from_str(s: &str) -> Result<Self, CommonTypeError> {
        Uuid::parse_str(s)
            .map(ReportId)
            .map_err(|e| CommonTypeError::InvalidId(format!("Invalid ReportId: {}", e)))
    }

    pub fn to_uuid(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for ReportId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for ReportId {
    fn default() -> Self {
        ReportId::new()
    }
}

// ============================================================
// SEVERITY
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    Info = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl Severity {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "info" | "informational" => Some(Severity::Info),
            "low" => Some(Severity::Low),
            "medium" | "moderate" => Some(Severity::Medium),
            "high" => Some(Severity::High),
            "critical" => Some(Severity::Critical),
            _ => None,
        }
    }

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
            Severity::Info => 0.0,
            Severity::Low => 2.5,
            Severity::Medium => 5.0,
            Severity::High => 7.5,
            Severity::Critical => 10.0,
        }
    }

    pub fn to_emoji(&self) -> &str {
        match self {
            Severity::Info => "ℹ️",
            Severity::Low => "🟢",
            Severity::Medium => "🟡",
            Severity::High => "🟠",
            Severity::Critical => "🔴",
        }
    }

    pub fn is_actionable(&self) -> bool {
        matches!(self, Severity::High | Severity::Critical | Severity::Medium)
    }
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

// ============================================================
// STATUS
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Active,
    Completed,
    Failed,
    Cancelled,
    Pending,
    Paused,
    Running,
    Queued,
    Archived,
}

impl Status {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "active" => Some(Status::Active),
            "completed" | "done" | "success" => Some(Status::Completed),
            "failed" | "error" => Some(Status::Failed),
            "cancelled" | "canceled" => Some(Status::Cancelled),
            "pending" | "waiting" => Some(Status::Pending),
            "paused" => Some(Status::Paused),
            "running" => Some(Status::Running),
            "queued" => Some(Status::Queued),
            "archived" => Some(Status::Archived),
            _ => None,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Status::Completed | Status::Failed | Status::Cancelled | Status::Archived)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, Status::Active | Status::Running | Status::Pending | Status::Queued)
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "active"),
            Status::Completed => write!(f, "completed"),
            Status::Failed => write!(f, "failed"),
            Status::Cancelled => write!(f, "cancelled"),
            Status::Pending => write!(f, "pending"),
            Status::Paused => write!(f, "paused"),
            Status::Running => write!(f, "running"),
            Status::Queued => write!(f, "queued"),
            Status::Archived => write!(f, "archived"),
        }
    }
}

// ============================================================
// CONFIDENCE
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    Low = 0,
    Medium = 1,
    High = 2,
    Verified = 3,
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
            Confidence::Low => 0.25,
            Confidence::Medium => 0.5,
            Confidence::High => 0.8,
            Confidence::Verified => 0.95,
        }
    }
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

// ============================================================
// PRIORITY
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    P4 = 0,
    P3 = 1,
    P2 = 2,
    P1 = 3,
    P0 = 4,
}

impl Priority {
    pub fn rank(&self) -> u8 {
        *self as u8
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "P0" | "CRITICAL" => Some(Priority::P0),
            "P1" | "HIGH" => Some(Priority::P1),
            "P2" | "MEDIUM" => Some(Priority::P2),
            "P3" | "LOW" => Some(Priority::P3),
            "P4" | "INFO" => Some(Priority::P4),
            _ => None,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Priority::P0 => "P0-Critical",
            Priority::P1 => "P1-High",
            Priority::P2 => "P2-Medium",
            Priority::P3 => "P3-Low",
            Priority::P4 => "P4-Info",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

// ============================================================
// GEO LOCATION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeoLocation {
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
    pub asn: Option<u32>,
}

impl GeoLocation {
    pub fn new() -> Self {
        GeoLocation {
            country: None,
            country_code: None,
            city: None,
            region: None,
            latitude: None,
            longitude: None,
            timezone: None,
            isp: None,
            organization: None,
            asn: None,
        }
    }

    pub fn has_coordinates(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }

    pub fn format_short(&self) -> String {
        match (&self.city, &self.country) {
            (Some(city), Some(country)) => format!("{}, {}", city, country),
            (Some(city), None) => city.clone(),
            (None, Some(country)) => country.clone(),
            (None, None) => "Unknown".to_string(),
        }
    }
}

impl Default for GeoLocation {
    fn default() -> Self {
        GeoLocation::new()
    }
}

// ============================================================
// PAGINATION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

impl Pagination {
    pub fn new(page: u32, page_size: u32, total_items: u64) -> Self {
        let total_pages = if page_size == 0 {
            0
        } else {
            ((total_items as f64) / (page_size as f64)).ceil() as u32
        };
        Pagination {
            page: page.max(1),
            page_size: page_size.min(100),
            total_items,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }

    pub fn offset(&self) -> u64 {
        ((self.page - 1) * self.page_size) as u64
    }

    pub fn limit(&self) -> u32 {
        self.page_size
    }
}

// ============================================================
// FILE SIZE
// ============================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileSize(u64);

impl FileSize {
    pub fn from_bytes(bytes: u64) -> Self {
        FileSize(bytes)
    }

    pub fn from_kb(kb: f64) -> Self {
        FileSize((kb * 1024.0) as u64)
    }

    pub fn from_mb(mb: f64) -> Self {
        FileSize((mb * 1024.0 * 1024.0) as u64)
    }

    pub fn as_bytes(&self) -> u64 {
        self.0
    }

    pub fn as_kb(&self) -> f64 {
        self.0 as f64 / 1024.0
    }

    pub fn as_mb(&self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0)
    }

    pub fn as_gb(&self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0 * 1024.0)
    }

    pub fn format_human(&self) -> String {
        let bytes = self.0 as f64;
        if bytes < 1024.0 {
            format!("{} B", bytes)
        } else if bytes < 1024.0 * 1024.0 {
            format!("{:.1} KB", bytes / 1024.0)
        } else if bytes < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.2} MB", bytes / (1024.0 * 1024.0))
        } else {
            format!("{:.2} GB", bytes / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_human())
    }
}

// ============================================================
// ERROR
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommonTypeError {
    InvalidUrl(String),
    InvalidDomain(String),
    InvalidIp(String),
    InvalidTimestamp(String),
    InvalidId(String),
    InvalidFormat(String),
    ValidationFailed(String),
}

impl fmt::Display for CommonTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonTypeError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            CommonTypeError::InvalidDomain(msg) => write!(f, "Invalid domain: {}", msg),
            CommonTypeError::InvalidIp(msg) => write!(f, "Invalid IP: {}", msg),
            CommonTypeError::InvalidTimestamp(msg) => write!(f, "Invalid timestamp: {}", msg),
            CommonTypeError::InvalidId(msg) => write!(f, "Invalid ID: {}", msg),
            CommonTypeError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            CommonTypeError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
        }
    }
}

impl std::error::Error for CommonTypeError {}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_url_valid() {
        let url = TargetUrl::new("https://example.com").unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host(), Some("example.com"));
    }

    #[test]
    fn test_target_url_auto_https() {
        let url = TargetUrl::new("example.com").unwrap();
        assert!(url.as_str().starts_with("https://"));
    }

    #[test]
    fn test_target_url_invalid_scheme() {
        let result = TargetUrl::new("ftp://example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_target_url_empty() {
        let result = TargetUrl::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_target_url_domain_extraction() {
        let url = TargetUrl::new("https://sub.example.com/path").unwrap();
        assert_eq!(url.domain(), Some("example.com".to_string()));
    }

    #[test]
    fn test_domain_valid() {
        let domain = Domain::new("example.com").unwrap();
        assert!(domain.is_valid());
        assert_eq!(domain.tld(), Some("com"));
        assert_eq!(domain.root_domain(), Some("example.com".to_string()));
    }

    #[test]
    fn test_domain_invalid_empty() {
        assert!(Domain::new("").is_err());
    }

    #[test]
    fn test_domain_invalid_spaces() {
        assert!(Domain::new("example .com").is_err());
    }

    #[test]
    fn test_domain_invalid_length() {
        let long = format!("{}.com", "a".repeat(255));
        assert!(Domain::new(&long).is_err());
    }

    #[test]
    fn test_ip_address_v4() {
        let ip = IpAddress::new("192.168.1.1").unwrap();
        assert!(ip.is_private());
        assert_eq!(ip.to_string(), "192.168.1.1");
    }

    #[test]
    fn test_ip_address_v6() {
        let ip = IpAddress::new("::1").unwrap();
        assert!(ip.is_loopback());
    }

    #[test]
    fn test_ip_address_invalid() {
        assert!(IpAddress::new("not_an_ip").is_err());
    }

    #[test]
    fn test_timestamp_now() {
        let ts = Timestamp::now();
        assert!(ts.to_unix() > 0);
    }

    #[test]
    fn test_timestamp_iso8601() {
        let ts = Timestamp::from_iso8601("2024-01-15T10:30:00Z").unwrap();
        assert_eq!(ts.format_human(), "2024-01-15 10:30:00 UTC");
    }

    #[test]
    fn test_timestamp_elapsed() {
        let ts = Timestamp::now().add_seconds(-60);
        assert!(ts.elapsed().as_secs() >= 59);
    }

    #[test]
    fn test_duration_format() {
        let d = Duration::from_secs(3661);
        assert_eq!(d.format_human(), "1h 1m 1s");
    }

    #[test]
    fn test_scan_id_new() {
        let id = ScanId::new();
        assert!(!id.is_nil());
    }

    #[test]
    fn test_scan_id_roundtrip() {
        let id = ScanId::new();
        let s = id.to_string();
        let parsed = ScanId::from_str(&s).unwrap();
        assert_eq!(id, parsed);
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
    fn test_severity_to_emoji() {
        assert_eq!(Severity::Critical.to_emoji(), "🔴");
        assert_eq!(Severity::Info.to_emoji(), "ℹ️");
    }

    #[test]
    fn test_status_terminal() {
        assert!(Status::Completed.is_terminal());
        assert!(Status::Failed.is_terminal());
        assert!(!Status::Running.is_terminal());
    }

    #[test]
    fn test_confidence_from_score() {
        assert_eq!(Confidence::from_score(0.9), Confidence::Verified);
        assert_eq!(Confidence::from_score(0.75), Confidence::High);
        assert_eq!(Confidence::from_score(0.5), Confidence::Medium);
        assert_eq!(Confidence::from_score(0.2), Confidence::Low);
    }

    #[test]
    fn test_priority_rank() {
        assert_eq!(Priority::P0.rank(), 4);
        assert_eq!(Priority::P4.rank(), 0);
        assert!(Priority::P0 > Priority::P4);
    }

    #[test]
    fn test_geo_location_format() {
        let geo = GeoLocation {
            city: Some("Jakarta".to_string()),
            country: Some("Indonesia".to_string()),
            ..GeoLocation::new()
        };
        assert_eq!(geo.format_short(), "Jakarta, Indonesia");
    }

    #[test]
    fn test_pagination() {
        let p = Pagination::new(1, 10, 95);
        assert_eq!(p.total_pages, 10);
        assert!(p.has_next);
        assert!(!p.has_previous);
        assert_eq!(p.offset(), 0);
    }

    #[test]
    fn test_file_size_format() {
        assert_eq!(FileSize::from_bytes(500).format_human(), "500 B");
        assert_eq!(FileSize::from_kb(1.5).format_human(), "1.5 KB");
        assert_eq!(FileSize::from_mb(100.0).format_human(), "100.00 MB");
    }
}
