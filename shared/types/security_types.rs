// shared/types/security_types.rs
// IWS v1.0 - Security Types
// Mendefinisikan tipe data untuk security assessment

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::common_types::{Severity, Confidence, Priority};

// ============================================================
// HTTP HEADER ANALYSIS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderAnalysis {
    pub header_name: String,
    pub value: Option<String>,
    pub score: u8,
    pub max_score: u8,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub is_present: bool,
    pub is_valid: bool,
    pub severity: Severity,
    pub description: String,
    pub references: Vec<String>,
}

impl HeaderAnalysis {
    pub fn new(header_name: &str) -> Self {
        HeaderAnalysis {
            header_name: header_name.to_string(),
            value: None,
            score: 0,
            max_score: 10,
            issues: vec![],
            recommendations: vec![],
            is_present: false,
            is_valid: false,
            severity: Severity::Info,
            description: String::new(),
            references: vec![],
        }
    }

    pub fn score_percentage(&self) -> f32 {
        if self.max_score == 0 {
            return 0.0;
        }
        (self.score as f32 / self.max_score as f32) * 100.0
    }

    pub fn is_secure(&self) -> bool {
        self.score_percentage() >= 70.0
    }

    pub fn add_issue(&mut self, issue: &str) {
        self.issues.push(issue.to_string());
    }

    pub fn add_recommendation(&mut self, recommendation: &str) {
        self.recommendations.push(recommendation.to_string());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHeadersReport {
    pub hsts: HeaderAnalysis,
    pub csp: HeaderAnalysis,
    pub x_frame_options: HeaderAnalysis,
    pub x_content_type_options: HeaderAnalysis,
    pub referrer_policy: HeaderAnalysis,
    pub permissions_policy: HeaderAnalysis,
    pub x_xss_protection: HeaderAnalysis,
    pub cross_origin_policies: HeaderAnalysis,
    pub overall_score: f32,
    pub total_headers_checked: usize,
    pub headers_present: usize,
    pub headers_secure: usize,
    pub critical_issues: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl SecurityHeadersReport {
    pub fn new() -> Self {
        SecurityHeadersReport {
            hsts: HeaderAnalysis::new("Strict-Transport-Security"),
            csp: HeaderAnalysis::new("Content-Security-Policy"),
            x_frame_options: HeaderAnalysis::new("X-Frame-Options"),
            x_content_type_options: HeaderAnalysis::new("X-Content-Type-Options"),
            referrer_policy: HeaderAnalysis::new("Referrer-Policy"),
            permissions_policy: HeaderAnalysis::new("Permissions-Policy"),
            x_xss_protection: HeaderAnalysis::new("X-XSS-Protection"),
            cross_origin_policies: HeaderAnalysis::new("Cross-Origin-Resource-Policy"),
            overall_score: 0.0,
            total_headers_checked: 8,
            headers_present: 0,
            headers_secure: 0,
            critical_issues: vec![],
            timestamp: Utc::now(),
        }
    }

    pub fn calculate_score(&mut self) {
        let headers = [
            &self.hsts,
            &self.csp,
            &self.x_frame_options,
            &self.x_content_type_options,
            &self.referrer_policy,
            &self.permissions_policy,
            &self.x_xss_protection,
            &self.cross_origin_policies,
        ];

        self.headers_present = headers.iter().filter(|h| h.is_present).count();
        self.headers_secure = headers.iter().filter(|h| h.is_secure()).count();

        let total_score: u32 = headers.iter().map(|h| h.score as u32).sum();
        let total_max: u32 = headers.iter().map(|h| h.max_score as u32).sum();

        self.overall_score = if total_max > 0 {
            (total_score as f32 / total_max as f32) * 100.0
        } else {
            0.0
        };
    }

    pub fn overall_grade(&self) -> SecurityGrade {
        SecurityGrade::from_score(self.overall_score)
    }

    pub fn missing_critical_headers(&self) -> Vec<String> {
        let mut missing = vec![];
        if !self.hsts.is_present { missing.push("HSTS".to_string()); }
        if !self.csp.is_present { missing.push("CSP".to_string()); }
        if !self.x_frame_options.is_present { missing.push("X-Frame-Options".to_string()); }
        missing
    }
}

// ============================================================
// COOKIE ANALYSIS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SameSite {
    Strict,
    Lax,
    None,
    NotSet,
}

impl fmt::Display for SameSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SameSite::Strict => write!(f, "Strict"),
            SameSite::Lax => write!(f, "Lax"),
            SameSite::None => write!(f, "None"),
            SameSite::NotSet => write!(f, "NotSet"),
        }
    }
}

impl SameSite {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "strict" => SameSite::Strict,
            "lax" => SameSite::Lax,
            "none" => SameSite::None,
            _ => SameSite::NotSet,
        }
    }

    pub fn is_secure(&self) -> bool {
        matches!(self, SameSite::Strict | SameSite::Lax)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieAnalysis {
    pub name: String,
    pub value_masked: String,
    pub secure: bool,
    pub httponly: bool,
    pub samesite: SameSite,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<DateTime<Utc>>,
    pub max_age: Option<i64>,
    pub is_session: bool,
    pub is_persistent: bool,
    pub size_bytes: usize,
    pub issues: Vec<String>,
    pub security_score: u8,
}

impl CookieAnalysis {
    pub fn new(name: &str, value: &str) -> Self {
        let value_masked = if value.len() > 8 {
            format!("{}...", &value[..8])
        } else {
            "***".to_string()
        };

        CookieAnalysis {
            name: name.to_string(),
            value_masked,
            secure: false,
            httponly: false,
            samesite: SameSite::NotSet,
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            is_session: true,
            is_persistent: false,
            size_bytes: value.len(),
            issues: vec![],
            security_score: 0,
        }
    }

    pub fn analyze(&mut self) {
        let mut score = 0u8;
        let max_score = 3u8;

        if self.secure { score += 1; } else { self.issues.push("Missing Secure flag".to_string()); }
        if self.httponly { score += 1; } else { self.issues.push("Missing HttpOnly flag".to_string()); }
        if self.samesite.is_secure() { score += 1; } else { self.issues.push(format!("SameSite is {:?}", self.samesite)); }

        self.security_score = (score as f32 / max_score as f32 * 100.0) as u8;
    }

    pub fn is_secure_cookie(&self) -> bool {
        self.secure && self.httponly && self.samesite.is_secure()
    }

    pub fn is_session_cookie(&self) -> bool {
        self.expires.is_none() && self.max_age.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CookieReport {
    pub cookies: Vec<CookieAnalysis>,
    pub total_cookies: usize,
    pub secure_cookies: usize,
    pub insecure_cookies: usize,
    pub session_cookies: usize,
    pub persistent_cookies: usize,
    pub third_party_cookies: usize,
    pub issues: Vec<String>,
    pub overall_score: f32,
    pub timestamp: DateTime<Utc>,
}

impl CookieReport {
    pub fn new() -> Self {
        CookieReport {
            cookies: vec![],
            total_cookies: 0,
            secure_cookies: 0,
            insecure_cookies: 0,
            session_cookies: 0,
            persistent_cookies: 0,
            third_party_cookies: 0,
            issues: vec![],
            overall_score: 0.0,
            timestamp: Utc::now(),
        }
    }

    pub fn add_cookie(&mut self, mut cookie: CookieAnalysis) {
        cookie.analyze();
        self.total_cookies += 1;
        if cookie.is_secure_cookie() { self.secure_cookies += 1; }
        else { self.insecure_cookies += 1; }
        if cookie.is_session_cookie() { self.session_cookies += 1; }
        else { self.persistent_cookies += 1; }
        self.issues.extend(cookie.issues.clone());
        self.cookies.push(cookie);
    }

    pub fn calculate_score(&mut self) {
        if self.total_cookies == 0 {
            self.overall_score = 100.0;
            return;
        }
        self.overall_score = (self.secure_cookies as f32 / self.total_cookies as f32) * 100.0;
    }
}

// ============================================================
// VULNERABILITY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareVersion {
    pub name: String,
    pub version: String,
    pub version_ranges: Option<String>,
    pub cpe: Option<String>,
}

impl SoftwareVersion {
    pub fn new(name: &str, version: &str) -> Self {
        SoftwareVersion {
            name: name.to_string(),
            version: version.to_string(),
            version_ranges: None,
            cpe: None,
        }
    }

    pub fn with_cpe(mut self, cpe: &str) -> Self {
        self.cpe = Some(cpe.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    pub id: String,
    pub cve_id: Option<String>,
    pub title: String,
    pub description: String,
    pub severity: Severity,
    pub cvss_score: f32,
    pub cvss_vector: Option<String>,
    pub epss_score: Option<f32>,
    pub affected_software: Vec<SoftwareVersion>,
    pub references: Vec<String>,
    pub remediation: Option<String>,
    pub exploit_available: bool,
    pub exploit_maturity: ExploitMaturity,
    pub discovered_at: DateTime<Utc>,
    pub status: VulnerabilityStatus,
    pub confidence: Confidence,
    pub cvss_metrics: Option<CvssMetrics>,
}

impl Vulnerability {
    pub fn new(title: &str, description: &str, severity: Severity) -> Self {
        Vulnerability {
            id: format!("VULN-{}", uuid::Uuid::new_v4()),
            cve_id: None,
            title: title.to_string(),
            description: description.to_string(),
            severity,
            cvss_score: severity.to_score(),
            cvss_vector: None,
            epss_score: None,
            affected_software: vec![],
            references: vec![],
            remediation: None,
            exploit_available: false,
            exploit_maturity: ExploitMaturity::Unknown,
            discovered_at: Utc::now(),
            status: VulnerabilityStatus::Open,
            confidence: Confidence::Medium,
            cvss_metrics: None,
        }
    }

    pub fn is_critical(&self) -> bool {
        self.severity == Severity::Critical || self.cvss_score >= 9.0
    }

    pub fn needs_immediate_action(&self) -> bool {
        self.is_critical() && self.exploit_available
    }

    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.discovered_at).num_days()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExploitMaturity {
    Unproven,
    ProofOfConcept,
    Functional,
    High,
    Weaponized,
    Unknown,
}

impl fmt::Display for ExploitMaturity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExploitMaturity::Unproven => write!(f, "unproven"),
            ExploitMaturity::ProofOfConcept => write!(f, "poc"),
            ExploitMaturity::Functional => write!(f, "functional"),
            ExploitMaturity::High => write!(f, "high"),
            ExploitMaturity::Weaponized => write!(f, "weaponized"),
            ExploitMaturity::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnerabilityStatus {
    Open,
    InProgress,
    Fixed,
    WontFix,
    FalsePositive,
    Duplicate,
}

impl fmt::Display for VulnerabilityStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VulnerabilityStatus::Open => write!(f, "open"),
            VulnerabilityStatus::InProgress => write!(f, "in_progress"),
            VulnerabilityStatus::Fixed => write!(f, "fixed"),
            VulnerabilityStatus::WontFix => write!(f, "wont_fix"),
            VulnerabilityStatus::FalsePositive => write!(f, "false_positive"),
            VulnerabilityStatus::Duplicate => write!(f, "duplicate"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssMetrics {
    pub attack_vector: AttackVector,
    pub attack_complexity: AttackComplexity,
    pub privileges_required: PrivilegesRequired,
    pub user_interaction: UserInteraction,
    pub scope: Scope,
    pub confidentiality_impact: Impact,
    pub integrity_impact: Impact,
    pub availability_impact: Impact,
}

impl CvssMetrics {
    pub fn to_vector_string(&self) -> String {
        format!(
            "CVSS:3.1/AV:{}/AC:{}/PR:{}/UI:{}/S:{}/C:{}/I:{}/A:{}",
            self.attack_vector, self.attack_complexity, self.privileges_required,
            self.user_interaction, self.scope,
            self.confidentiality_impact, self.integrity_impact, self.availability_impact
        )
    }

    pub fn calculate_base_score(&self) -> f32 {
        let av = self.attack_vector.weight();
        let ac = self.attack_complexity.weight();
        let pr = self.privileges_required.weight(self.scope);
        let ui = self.user_interaction.weight();
        let c = self.confidentiality_impact.weight();
        let i = self.integrity_impact.weight();
        let a = self.availability_impact.weight();

        let iss = 1.0 - ((1.0 - c) * (1.0 - i) * (1.0 - a));
        let impact = if self.scope == Scope::Unchanged {
            6.42 * iss
        } else {
            7.52 * (iss - 0.029) - 3.25 * (iss - 0.02).powf(15.0)
        };

        let exploitability = 8.22 * av * ac * pr * ui;
        let base = if impact <= 0.0 {
            0.0
        } else if self.scope == Scope::Unchanged {
            (impact + exploitability).min(10.0)
        } else {
            (1.08 * (impact + exploitability)).min(10.0)
        };

        (base * 10.0).round() / 10.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttackVector { Network, Adjacent, Local, Physical }

impl AttackVector {
    fn weight(&self) -> f32 {
        match self { AttackVector::Network => 0.85, AttackVector::Adjacent => 0.62, AttackVector::Local => 0.55, AttackVector::Physical => 0.2 }
    }
}

impl fmt::Display for AttackVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { AttackVector::Network => write!(f, "N"), AttackVector::Adjacent => write!(f, "A"), AttackVector::Local => write!(f, "L"), AttackVector::Physical => write!(f, "P") }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AttackComplexity { Low, High }

impl AttackComplexity {
    fn weight(&self) -> f32 { match self { AttackComplexity::Low => 0.77, AttackComplexity::High => 0.44 } }
}

impl fmt::Display for AttackComplexity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { AttackComplexity::Low => write!(f, "L"), AttackComplexity::High => write!(f, "H") }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrivilegesRequired { None, Low, High }

impl PrivilegesRequired {
    fn weight(&self, scope: Scope) -> f32 {
        match (self, scope) {
            (PrivilegesRequired::None, _) => 0.85,
            (PrivilegesRequired::Low, Scope::Changed) => 0.68,
            (PrivilegesRequired::Low, Scope::Unchanged) => 0.62,
            (PrivilegesRequired::High, Scope::Changed) => 0.50,
            (PrivilegesRequired::High, Scope::Unchanged) => 0.27,
        }
    }
}

impl fmt::Display for PrivilegesRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { PrivilegesRequired::None => write!(f, "N"), PrivilegesRequired::Low => write!(f, "L"), PrivilegesRequired::High => write!(f, "H") }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UserInteraction { None, Required }

impl UserInteraction {
    fn weight(&self) -> f32 { match self { UserInteraction::None => 0.85, UserInteraction::Required => 0.62 } }
}

impl fmt::Display for UserInteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { UserInteraction::None => write!(f, "N"), UserInteraction::Required => write!(f, "R") }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Scope { Unchanged, Changed }

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Scope::Unchanged => write!(f, "U"), Scope::Changed => write!(f, "C") }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Impact { None, Low, High }

impl Impact {
    fn weight(&self) -> f32 { match self { Impact::None => 0.0, Impact::Low => 0.22, Impact::High => 0.56 } }
}

impl fmt::Display for Impact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self { Impact::None => write!(f, "N"), Impact::Low => write!(f, "L"), Impact::High => write!(f, "H") }
    }
}

// ============================================================
// XSS FINDING
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum XssType {
    Reflected,
    Stored,
    DomBased,
    Blind,
}

impl fmt::Display for XssType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XssType::Reflected => write!(f, "reflected"),
            XssType::Stored => write!(f, "stored"),
            XssType::DomBased => write!(f, "dom_based"),
            XssType::Blind => write!(f, "blind"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum XssContext {
    Html,
    Attribute,
    JavaScript,
    Css,
    Url,
    Comment,
}

impl fmt::Display for XssContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            XssContext::Html => write!(f, "html"),
            XssContext::Attribute => write!(f, "attribute"),
            XssContext::JavaScript => write!(f, "javascript"),
            XssContext::Css => write!(f, "css"),
            XssContext::Url => write!(f, "url"),
            XssContext::Comment => write!(f, "comment"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XssFinding {
    pub xss_type: XssType,
    pub location: String,
    pub parameter: String,
    pub payload: String,
    pub context: XssContext,
    pub confidence: Confidence,
    pub evidence: Option<String>,
    pub was_sanitized: bool,
    pub was_encoded: bool,
    pub waf_detected: bool,
    pub timestamp: DateTime<Utc>,
}

impl XssFinding {
    pub fn new(xss_type: XssType, location: &str, parameter: &str, payload: &str, context: XssContext) -> Self {
        XssFinding {
            xss_type,
            location: location.to_string(),
            parameter: parameter.to_string(),
            payload: payload.to_string(),
            context,
            confidence: Confidence::Medium,
            evidence: None,
            was_sanitized: false,
            was_encoded: false,
            waf_detected: false,
            timestamp: Utc::now(),
        }
    }

    pub fn is_high_risk(&self) -> bool {
        matches!(self.xss_type, XssType::Stored) && self.confidence >= Confidence::High
    }
}

// ============================================================
// SQL INJECTION FINDING
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SqlInjectionType {
    BooleanBased,
    TimeBased,
    ErrorBased,
    UnionBased,
    Stacked,
    OutOfBand,
}

impl fmt::Display for SqlInjectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlInjectionType::BooleanBased => write!(f, "boolean_based"),
            SqlInjectionType::TimeBased => write!(f, "time_based"),
            SqlInjectionType::ErrorBased => write!(f, "error_based"),
            SqlInjectionType::UnionBased => write!(f, "union_based"),
            SqlInjectionType::Stacked => write!(f, "stacked"),
            SqlInjectionType::OutOfBand => write!(f, "out_of_band"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlInjectionFinding {
    pub injection_type: SqlInjectionType,
    pub parameter: String,
    pub payload: String,
    pub error_message: Option<String>,
    pub database_type: Option<String>,
    pub database_version: Option<String>,
    pub confidence: Confidence,
    pub response_time_ms: Option<u64>,
    pub evidence: Option<String>,
    pub waf_detected: bool,
    pub timestamp: DateTime<Utc>,
}

impl SqlInjectionFinding {
    pub fn new(injection_type: SqlInjectionType, parameter: &str, payload: &str) -> Self {
        SqlInjectionFinding {
            injection_type,
            parameter: parameter.to_string(),
            payload: payload.to_string(),
            error_message: None,
            database_type: None,
            database_version: None,
            confidence: Confidence::Medium,
            response_time_ms: None,
            evidence: None,
            waf_detected: false,
            timestamp: Utc::now(),
        }
    }

    pub fn is_blind(&self) -> bool {
        matches!(
            self.injection_type,
            SqlInjectionType::BooleanBased | SqlInjectionType::TimeBased
        )
    }

    pub fn database_identified(&self) -> bool {
        self.database_type.is_some()
    }
}

// ============================================================
// CSRF FINDING
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsrfFinding {
    pub form_action: String,
    pub method: String,
    pub has_csrf_token: bool,
    pub token_name: Option<String>,
    pub token_valid: Option<bool>,
    pub samesite_cookie: SameSite,
    pub custom_headers: bool,
    pub referer_validation: bool,
    pub origin_validation: bool,
    pub is_vulnerable: bool,
    pub risk_level: Severity,
    pub recommendations: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl CsrfFinding {
    pub fn new(form_action: &str, method: &str) -> Self {
        CsrfFinding {
            form_action: form_action.to_string(),
            method: method.to_string(),
            has_csrf_token: false,
            token_name: None,
            token_valid: None,
            samesite_cookie: SameSite::NotSet,
            custom_headers: false,
            referer_validation: false,
            origin_validation: false,
            is_vulnerable: true,
            risk_level: Severity::High,
            recommendations: vec![],
            timestamp: Utc::now(),
        }
    }

    pub fn assess_risk(&mut self) {
        let mut protections = 0u8;
        if self.has_csrf_token && self.token_valid.unwrap_or(false) { protections += 1; }
        if self.samesite_cookie.is_secure() { protections += 1; }
        if self.custom_headers { protections += 1; }
        if self.referer_validation || self.origin_validation { protections += 1; }

        self.is_vulnerable = protections < 2;
        self.risk_level = match protections {
            0 => Severity::Critical,
            1 => Severity::High,
            2 => Severity::Medium,
            3 => Severity::Low,
            _ => Severity::Info,
        };
    }
}

// ============================================================
// SECURITY GRADE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityGrade {
    APlus,
    A,
    B,
    C,
    D,
    E,
    F,
}

impl fmt::Display for SecurityGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityGrade::APlus => write!(f, "A+"),
            SecurityGrade::A => write!(f, "A"),
            SecurityGrade::B => write!(f, "B"),
            SecurityGrade::C => write!(f, "C"),
            SecurityGrade::D => write!(f, "D"),
            SecurityGrade::E => write!(f, "E"),
            SecurityGrade::F => write!(f, "F"),
        }
    }
}

impl SecurityGrade {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 95.0 => SecurityGrade::APlus,
            s if s >= 85.0 => SecurityGrade::A,
            s if s >= 70.0 => SecurityGrade::B,
            s if s >= 55.0 => SecurityGrade::C,
            s if s >= 40.0 => SecurityGrade::D,
            s if s >= 25.0 => SecurityGrade::E,
            _ => SecurityGrade::F,
        }
    }

    pub fn to_color(&self) -> &str {
        match self {
            SecurityGrade::APlus | SecurityGrade::A => "#00cc00",
            SecurityGrade::B => "#66cc00",
            SecurityGrade::C => "#cccc00",
            SecurityGrade::D => "#cc6600",
            SecurityGrade::E => "#cc3300",
            SecurityGrade::F => "#cc0000",
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
    fn test_header_analysis_scoring() {
        let mut header = HeaderAnalysis::new("HSTS");
        header.is_present = true;
        header.score = 8;
        header.max_score = 10;
        assert_eq!(header.score_percentage(), 80.0);
        assert!(header.is_secure());
    }

    #[test]
    fn test_security_headers_report_calculation() {
        let mut report = SecurityHeadersReport::new();
        report.hsts.is_present = true;
        report.hsts.score = 10;
        report.csp.is_present = true;
        report.csp.score = 5;
        report.calculate_score();
        assert!(report.overall_score > 0.0);
        assert_eq!(report.headers_present, 2);
    }

    #[test]
    fn test_security_headers_missing_critical() {
        let mut report = SecurityHeadersReport::new();
        report.hsts.is_present = false;
        report.csp.is_present = false;
        report.x_frame_options.is_present = true;
        let missing = report.missing_critical_headers();
        assert_eq!(missing.len(), 2);
        assert!(missing.contains(&"HSTS".to_string()));
        assert!(missing.contains(&"CSP".to_string()));
    }

    #[test]
    fn test_cookie_analysis_secure() {
        let mut cookie = CookieAnalysis::new("session", "abc123456789");
        cookie.secure = true;
        cookie.httponly = true;
        cookie.samesite = SameSite::Strict;
        cookie.analyze();
        assert!(cookie.is_secure_cookie());
        assert_eq!(cookie.security_score, 100);
    }

    #[test]
    fn test_cookie_analysis_insecure() {
        let mut cookie = CookieAnalysis::new("tracking", "data123");
        cookie.analyze();
        assert!(!cookie.is_secure_cookie());
        assert!(!cookie.issues.is_empty());
    }

    #[test]
    fn test_cookie_report_add() {
        let mut report = CookieReport::new();
        let mut cookie = CookieAnalysis::new("session", "secret1234");
        cookie.secure = true;
        cookie.httponly = true;
        cookie.samesite = SameSite::Lax;
        report.add_cookie(cookie);
        report.calculate_score();
        assert_eq!(report.total_cookies, 1);
        assert_eq!(report.secure_cookies, 1);
    }

    #[test]
    fn test_vulnerability_critical() {
        let vuln = Vulnerability::new("RCE", "Remote code execution", Severity::Critical);
        assert!(vuln.is_critical());
        assert!(!vuln.needs_immediate_action());
    }

    #[test]
    fn test_vulnerability_immediate_action() {
        let mut vuln = Vulnerability::new("RCE", "Remote code execution", Severity::Critical);
        vuln.exploit_available = true;
        assert!(vuln.needs_immediate_action());
    }

    #[test]
    fn test_cvss_base_score_calculation() {
        let metrics = CvssMetrics {
            attack_vector: AttackVector::Network,
            attack_complexity: AttackComplexity::Low,
            privileges_required: PrivilegesRequired::None,
            user_interaction: UserInteraction::None,
            scope: Scope::Unchanged,
            confidentiality_impact: Impact::High,
            integrity_impact: Impact::High,
            availability_impact: Impact::High,
        };
        let score = metrics.calculate_base_score();
        assert!(score > 9.0);
        assert!(score <= 10.0);
    }

    #[test]
    fn test_cvss_vector_string() {
        let metrics = CvssMetrics {
            attack_vector: AttackVector::Network,
            attack_complexity: AttackComplexity::Low,
            privileges_required: PrivilegesRequired::None,
            user_interaction: UserInteraction::None,
            scope: Scope::Changed,
            confidentiality_impact: Impact::High,
            integrity_impact: Impact::High,
            availability_impact: Impact::High,
        };
        let vector = metrics.to_vector_string();
        assert!(vector.starts_with("CVSS:3.1/"));
        assert!(vector.contains("AV:N"));
    }

    #[test]
    fn test_xss_finding_high_risk() {
        let finding = XssFinding::new(
            XssType::Stored,
            "/comment",
            "body",
            "<script>alert(1)</script>",
            XssContext::Html,
        );
        assert!(finding.is_high_risk());
    }

    #[test]
    fn test_sql_injection_finding_blind() {
        let finding = SqlInjectionFinding::new(
            SqlInjectionType::TimeBased,
            "id",
            "1 AND SLEEP(5)",
        );
        assert!(finding.is_blind());
        assert!(!finding.database_identified());
    }

    #[test]
    fn test_csrf_finding_assess_risk() {
        let mut finding = CsrfFinding::new("/login", "POST");
        finding.assess_risk();
        assert!(finding.is_vulnerable);
        assert_eq!(finding.risk_level, Severity::Critical);

        finding.has_csrf_token = true;
        finding.token_valid = Some(true);
        finding.samesite_cookie = SameSite::Lax;
        finding.assess_risk();
        assert_eq!(finding.risk_level, Severity::Medium);
    }

    #[test]
    fn test_security_grade_from_score() {
        assert_eq!(SecurityGrade::from_score(98.0), SecurityGrade::APlus);
        assert_eq!(SecurityGrade::from_score(88.0), SecurityGrade::A);
        assert_eq!(SecurityGrade::from_score(75.0), SecurityGrade::B);
        assert_eq!(SecurityGrade::from_score(60.0), SecurityGrade::C);
        assert_eq!(SecurityGrade::from_score(45.0), SecurityGrade::D);
        assert_eq!(SecurityGrade::from_score(30.0), SecurityGrade::E);
        assert_eq!(SecurityGrade::from_score(10.0), SecurityGrade::F);
    }

    #[test]
    fn test_same_site_from_str() {
        assert_eq!(SameSite::from_str("Strict"), SameSite::Strict);
        assert_eq!(SameSite::from_str("lax"), SameSite::Lax);
        assert_eq!(SameSite::from_str("None"), SameSite::None);
        assert_eq!(SameSite::from_str("invalid"), SameSite::NotSet);
    }
}
