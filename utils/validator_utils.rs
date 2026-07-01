// utils/validator_utils.rs
// IWS v1.0 - Validator Utilities
// Menyediakan fungsi validasi untuk berbagai tipe data dan input sanitization

use regex::Regex;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use url::Url;
use once_cell::sync::Lazy;

// ============================================================
// REGEX PATTERNS (COMPILED ONCE)
// ============================================================

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

static URL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap()
});

static DOMAIN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}$").unwrap()
});

static IPV4_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap()
});

static PATH_TRAVERSAL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\.\.[/\\]").unwrap()
});

static SQL_INJECTION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(union\s+select|drop\s+table|exec\s*\(|execute\s*\(|--|\bOR\b\s+1=1)").unwrap()
});

static XSS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)(<script|javascript:|onerror=|onload=|<img[^>]+src=)").unwrap()
});

static HTML_TAG_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<[^>]*>").unwrap()
});

// ============================================================
// VALIDATOR UTILS STRUCT
// ============================================================

#[derive(Debug, Clone)]
pub struct ValidatorUtils;

impl ValidatorUtils {
    // ============================================================
    // EMAIL VALIDATION
    // ============================================================

    /// Validasi format email (RFC 5322 compliant regex)
    pub fn validate_email(email: &str) -> bool {
        if email.is_empty() || email.len() > 254 {
            return false;
        }
        EMAIL_RE.is_match(email)
    }

    /// Validasi dan ekstrak bagian email
    pub fn parse_email(email: &str) -> Option<(String, String)> {
        if !ValidatorUtils::validate_email(email) {
            return None;
        }
        email.split_once('@').map(|(local, domain)| (local.to_string(), domain.to_string()))
    }

    // ============================================================
    // URL VALIDATION
    // ============================================================

    /// Validasi URL (harus http/https)
    pub fn validate_url(url_str: &str) -> bool {
        if url_str.is_empty() {
            return false;
        }
        match Url::parse(url_str) {
            Ok(url) => {
                let scheme = url.scheme();
                (scheme == "http" || scheme == "https") && url.host_str().is_some()
            }
            Err(_) => false,
        }
    }

    /// Normalisasi URL (tambah https:// jika belum ada)
    pub fn normalize_url(url_str: &str) -> Option<String> {
        let trimmed = url_str.trim();
        if trimmed.is_empty() {
            return None;
        }
        if !trimmed.contains("://") {
            return Some(format!("https://{}", trimmed));
        }
        Some(trimmed.to_string())
    }

    // ============================================================
    // IP VALIDATION
    // ============================================================

    /// Validasi IP address (IPv4 atau IPv6)
    pub fn validate_ip(ip_str: &str) -> bool {
        IpAddr::from_str(ip_str).is_ok()
    }

    /// Validasi IPv4
    pub fn validate_ipv4(ip_str: &str) -> bool {
        Ipv4Addr::from_str(ip_str).is_ok()
    }

    /// Validasi IPv6
    pub fn validate_ipv6(ip_str: &str) -> bool {
        Ipv6Addr::from_str(ip_str).is_ok()
    }

    /// Cek apakah IP private
    pub fn is_private_ip(ip_str: &str) -> bool {
        if let Ok(ip) = IpAddr::from_str(ip_str) {
            match ip {
                IpAddr::V4(v4) => v4.is_private() || v4.is_loopback() || v4.is_link_local(),
                IpAddr::V6(v6) => v6.is_loopback(),
            }
        } else {
            false
        }
    }

    // ============================================================
    // DOMAIN VALIDATION
    // ============================================================

    /// Validasi domain name
    pub fn validate_domain(domain: &str) -> bool {
        if domain.is_empty() || domain.len() > 253 {
            return false;
        }
        DOMAIN_RE.is_match(domain)
    }

    /// Ekstrak root domain
    pub fn extract_root_domain(domain: &str) -> Option<String> {
        let parts: Vec<&str> = domain.split('.').collect();
        if parts.len() >= 2 {
            Some(format!("{}.{}", parts[parts.len() - 2], parts[parts.len() - 1]))
        } else {
            None
        }
    }

    /// Ekstrak TLD
    pub fn extract_tld(domain: &str) -> Option<&str> {
        domain.rsplit('.').next()
    }

    // ============================================================
    // PORT VALIDATION
    // ============================================================

    /// Validasi port number (1-65535)
    pub fn validate_port(port: u16) -> bool {
        port > 0
    }

    /// Validasi port range
    pub fn validate_port_range(start: u16, end: u16) -> bool {
        start > 0 && end > 0 && start <= end && end <= 65535
    }

    /// Cek apakah port umum
    pub fn is_common_port(port: u16) -> bool {
        matches!(port, 21 | 22 | 23 | 25 | 53 | 80 | 110 | 143 | 443 | 993 | 995 | 3306 | 3389 | 5432 | 6379 | 8080 | 8443 | 27017)
    }

    // ============================================================
    // INPUT SANITIZATION
    // ============================================================

    /// Sanitasi input dari HTML tags
    pub fn sanitize_html(input: &str) -> String {
        HTML_TAG_RE.replace_all(input, "").to_string()
    }

    /// Sanitasi input dengan HTML entity escaping
    pub fn sanitize_html_escape(input: &str) -> String {
        input
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
    }

    /// Sanitasi untuk SQL injection prevention
    pub fn sanitize_sql(input: &str) -> String {
        input.replace('\'', "''").replace('\\', "\\\\")
    }

    /// Deteksi potensi SQL injection
    pub fn detect_sql_injection(input: &str) -> bool {
        SQL_INJECTION_RE.is_match(input)
    }

    /// Deteksi potensi XSS
    pub fn detect_xss(input: &str) -> bool {
        XSS_RE.is_match(input)
    }

    /// Sanitasi filename (hapus karakter berbahaya)
    pub fn sanitize_filename(filename: &str) -> String {
        filename
            .replace('/', "_")
            .replace('\\', "_")
            .replace(':', "_")
            .replace('*', "_")
            .replace('?', "_")
            .replace('"', "_")
            .replace('<', "_")
            .replace('>', "_")
            .replace('|', "_")
            .replace('\0', "_")
    }

    // ============================================================
    // PATH VALIDATION
    // ============================================================

    /// Cek path traversal attack
    pub fn validate_path(path: &str) -> bool {
        !PATH_TRAVERSAL_RE.is_match(path)
    }

    /// Sanitasi path
    pub fn sanitize_path(path: &str) -> String {
        PATH_TRAVERSAL_RE.replace_all(path, "").to_string()
    }

    // ============================================================
    // CONTENT TYPE VALIDATION
    // ============================================================

    /// Validasi MIME type
    pub fn validate_content_type(content_type: &str) -> bool {
        let valid_types = [
            "text/html", "text/plain", "text/css", "text/javascript",
            "application/json", "application/xml", "application/pdf",
            "application/octet-stream", "application/zip",
            "image/png", "image/jpeg", "image/gif", "image/svg+xml",
            "multipart/form-data", "application/x-www-form-urlencoded",
        ];
        valid_types.contains(&content_type) || content_type.contains('/')
    }

    // ============================================================
    // ENCODING VALIDATION
    // ============================================================

    /// Validasi character encoding
    pub fn validate_encoding(encoding: &str) -> bool {
        let valid = ["utf-8", "utf-16", "utf-32", "ascii", "iso-8859-1", "windows-1252", "latin1"];
        valid.contains(&encoding.to_lowercase().as_str())
    }

    // ============================================================
    // GENERAL VALIDATION
    // ============================================================

    /// Validasi string tidak kosong
    pub fn validate_not_empty(s: &str) -> bool {
        !s.trim().is_empty()
    }

    /// Validasi panjang string
    pub fn validate_length(s: &str, min: usize, max: usize) -> bool {
        s.len() >= min && s.len() <= max
    }

    /// Validasi alphanumeric
    pub fn validate_alphanumeric(s: &str) -> bool {
        s.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// Validasi numeric
    pub fn validate_numeric(s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    /// Validasi UUID
    pub fn validate_uuid(s: &str) -> bool {
        uuid::Uuid::parse_str(s).is_ok()
    }

    /// Validasi SHA256 hash
    pub fn validate_sha256(s: &str) -> bool {
        s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Validasi MD5 hash
    pub fn validate_md5(s: &str) -> bool {
        s.len() == 32 && s.chars().all(|c| c.is_ascii_hexdigit())
    }
}

// ============================================================
// VALIDATION RESULT
// ============================================================

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        ValidationResult { is_valid: true, errors: vec![], warnings: vec![] }
    }

    pub fn invalid(error: &str) -> Self {
        ValidationResult { is_valid: false, errors: vec![error.to_string()], warnings: vec![] }
    }

    pub fn with_warning(mut self, warning: &str) -> Self {
        self.warnings.push(warning.to_string());
        self
    }

    pub fn merge(&mut self, other: ValidationResult) {
        self.is_valid = self.is_valid && other.is_valid;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valid() {
        assert!(ValidatorUtils::validate_email("user@example.com"));
        assert!(ValidatorUtils::validate_email("test.user+tag@domain.co.id"));
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(!ValidatorUtils::validate_email(""));
        assert!(!ValidatorUtils::validate_email("not-email"));
        assert!(!ValidatorUtils::validate_email("@domain.com"));
    }

    #[test]
    fn test_parse_email() {
        let result = ValidatorUtils::parse_email("user@example.com");
        assert!(result.is_some());
        let (local, domain) = result.unwrap();
        assert_eq!(local, "user");
        assert_eq!(domain, "example.com");
    }

    #[test]
    fn test_validate_url_valid() {
        assert!(ValidatorUtils::validate_url("https://example.com"));
        assert!(ValidatorUtils::validate_url("http://example.com/path"));
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(!ValidatorUtils::validate_url(""));
        assert!(!ValidatorUtils::validate_url("ftp://example.com"));
        assert!(!ValidatorUtils::validate_url("not-a-url"));
    }

    #[test]
    fn test_normalize_url() {
        assert_eq!(ValidatorUtils::normalize_url("example.com"), Some("https://example.com".to_string()));
        assert_eq!(ValidatorUtils::normalize_url("https://example.com"), Some("https://example.com".to_string()));
        assert_eq!(ValidatorUtils::normalize_url(""), None);
    }

    #[test]
    fn test_validate_ipv4() {
        assert!(ValidatorUtils::validate_ipv4("192.168.1.1"));
        assert!(!ValidatorUtils::validate_ipv4("999.999.999.999"));
    }

    #[test]
    fn test_validate_ipv6() {
        assert!(ValidatorUtils::validate_ipv6("::1"));
        assert!(ValidatorUtils::validate_ipv6("2001:db8::1"));
    }

    #[test]
    fn test_is_private_ip() {
        assert!(ValidatorUtils::is_private_ip("192.168.1.1"));
        assert!(ValidatorUtils::is_private_ip("10.0.0.1"));
        assert!(ValidatorUtils::is_private_ip("::1"));
        assert!(!ValidatorUtils::is_private_ip("8.8.8.8"));
    }

    #[test]
    fn test_validate_domain() {
        assert!(ValidatorUtils::validate_domain("example.com"));
        assert!(ValidatorUtils::validate_domain("sub.example.co.id"));
        assert!(!ValidatorUtils::validate_domain(""));
        assert!(!ValidatorUtils::validate_domain("-invalid.com"));
    }

    #[test]
    fn test_extract_root_domain() {
        assert_eq!(ValidatorUtils::extract_root_domain("sub.example.com"), Some("example.com".to_string()));
        assert_eq!(ValidatorUtils::extract_root_domain("example.com"), Some("example.com".to_string()));
    }

    #[test]
    fn test_validate_port() {
        assert!(ValidatorUtils::validate_port(80));
        assert!(ValidatorUtils::validate_port(65535));
        assert!(!ValidatorUtils::validate_port(0));
    }

    #[test]
    fn test_is_common_port() {
        assert!(ValidatorUtils::is_common_port(443));
        assert!(!ValidatorUtils::is_common_port(12345));
    }

    #[test]
    fn test_sanitize_html() {
        assert_eq!(ValidatorUtils::sanitize_html("<p>Hello</p>"), "Hello");
        assert_eq!(ValidatorUtils::sanitize_html("<script>alert(1)</script>"), "alert(1)");
    }

    #[test]
    fn test_sanitize_html_escape() {
        let result = ValidatorUtils::sanitize_html_escape("<script>");
        assert!(result.contains("&lt;"));
        assert!(result.contains("&gt;"));
    }

    #[test]
    fn test_detect_sql_injection() {
        assert!(ValidatorUtils::detect_sql_injection("' OR 1=1 --"));
        assert!(ValidatorUtils::detect_sql_injection("UNION SELECT * FROM users"));
        assert!(!ValidatorUtils::detect_sql_injection("normal input"));
    }

    #[test]
    fn test_detect_xss() {
        assert!(ValidatorUtils::detect_xss("<script>alert(1)</script>"));
        assert!(ValidatorUtils::detect_xss("javascript:alert(1)"));
        assert!(!ValidatorUtils::detect_xss("normal text"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(ValidatorUtils::sanitize_filename("file/name:test*"), "file_name_test_");
    }

    #[test]
    fn test_validate_path() {
        assert!(ValidatorUtils::validate_path("/safe/path"));
        assert!(!ValidatorUtils::validate_path("../etc/passwd"));
        assert!(!ValidatorUtils::validate_path("..\\windows\\system32"));
    }

    #[test]
    fn test_validate_uuid() {
        assert!(ValidatorUtils::validate_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!ValidatorUtils::validate_uuid("not-a-uuid"));
    }

    #[test]
    fn test_validation_result_merge() {
        let mut result = ValidationResult::valid();
        result.merge(ValidationResult::invalid("error1"));
        result.merge(ValidationResult::valid().with_warning("warning1"));
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.warnings.len(), 1);
    }
}
