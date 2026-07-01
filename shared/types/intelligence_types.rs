// shared/types/intelligence_types.rs
// IWS v1.0 - Intelligence Types
// Mendefinisikan tipe data untuk threat intelligence

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::common_types::{Severity, Confidence};

// ============================================================
// THREAT INDICATOR
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IndicatorType {
    IPv4,
    IPv6,
    Domain,
    URL,
    FileHashMd5,
    FileHashSha1,
    FileHashSha256,
    Email,
    Cve,
    Malware,
    Yara,
    Mutex,
    RegistryKey,
    UserAgent,
    SslCertificate,
    Ja3Fingerprint,
    Other(String),
}

impl IndicatorType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ipv4" | "ip" => IndicatorType::IPv4,
            "ipv6" => IndicatorType::IPv6,
            "domain" | "hostname" => IndicatorType::Domain,
            "url" | "uri" => IndicatorType::URL,
            "hash_md5" | "md5" => IndicatorType::FileHashMd5,
            "hash_sha1" | "sha1" => IndicatorType::FileHashSha1,
            "hash_sha256" | "sha256" => IndicatorType::FileHashSha256,
            "email" | "mail" => IndicatorType::Email,
            "cve" => IndicatorType::Cve,
            "malware" => IndicatorType::Malware,
            "yara" => IndicatorType::Yara,
            "mutex" => IndicatorType::Mutex,
            "registry_key" | "regkey" => IndicatorType::RegistryKey,
            "user_agent" | "ua" => IndicatorType::UserAgent,
            "ssl_cert" | "certificate" => IndicatorType::SslCertificate,
            "ja3" => IndicatorType::Ja3Fingerprint,
            other => IndicatorType::Other(other.to_string()),
        }
    }
}

impl fmt::Display for IndicatorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IndicatorType::IPv4 => write!(f, "ipv4"),
            IndicatorType::IPv6 => write!(f, "ipv6"),
            IndicatorType::Domain => write!(f, "domain"),
            IndicatorType::URL => write!(f, "url"),
            IndicatorType::FileHashMd5 => write!(f, "hash_md5"),
            IndicatorType::FileHashSha1 => write!(f, "hash_sha1"),
            IndicatorType::FileHashSha256 => write!(f, "hash_sha256"),
            IndicatorType::Email => write!(f, "email"),
            IndicatorType::Cve => write!(f, "cve"),
            IndicatorType::Malware => write!(f, "malware"),
            IndicatorType::Yara => write!(f, "yara"),
            IndicatorType::Mutex => write!(f, "mutex"),
            IndicatorType::RegistryKey => write!(f, "registry_key"),
            IndicatorType::UserAgent => write!(f, "user_agent"),
            IndicatorType::SslCertificate => write!(f, "ssl_cert"),
            IndicatorType::Ja3Fingerprint => write!(f, "ja3"),
            IndicatorType::Other(s) => write!(f, "other:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub indicator_type: IndicatorType,
    pub value: String,
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    pub sightings: u64,
    pub severity: Severity,
    pub confidence: Confidence,
    pub sources: Vec<String>,
    pub tags: Vec<String>,
    pub description: Option<String>,
    pub related_indicators: Vec<String>,
    pub tlp: TlpLevel,
}

impl ThreatIndicator {
    pub fn new(indicator_type: IndicatorType, value: &str) -> Self {
        ThreatIndicator {
            indicator_type,
            value: value.to_string(),
            first_seen: None,
            last_seen: None,
            sightings: 1,
            severity: Severity::Medium,
            confidence: Confidence::Medium,
            sources: vec![],
            tags: vec![],
            description: None,
            related_indicators: vec![],
            tlp: TlpLevel::White,
        }
    }

    pub fn is_recent(&self, days: i64) -> bool {
        if let Some(last) = self.last_seen {
            (Utc::now() - last).num_days() <= days
        } else {
            false
        }
    }

    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= Confidence::High
    }
}

impl From<serde_json::Value> for ThreatIndicator {
    fn from(value: serde_json::Value) -> Self {
        let indicator_type = value
            .get("type")
            .and_then(|v| v.as_str())
            .map(IndicatorType::from_str)
            .unwrap_or(IndicatorType::Other("unknown".to_string()));

        let severity = value
            .get("severity")
            .and_then(|v| v.as_str())
            .and_then(Severity::from_str)
            .unwrap_or(Severity::Medium);

        let confidence = value
            .get("confidence")
            .and_then(|v| v.as_f64())
            .map(Confidence::from_score)
            .unwrap_or(Confidence::Medium);

        let tlp = value
            .get("tlp")
            .and_then(|v| v.as_str())
            .map(|s| match s.to_uppercase().as_str() {
                "WHITE" => TlpLevel::White,
                "GREEN" => TlpLevel::Green,
                "AMBER" => TlpLevel::Amber,
                "RED" => TlpLevel::Red,
                _ => TlpLevel::White,
            })
            .unwrap_or(TlpLevel::White);

        ThreatIndicator {
            indicator_type,
            value: value
                .get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            first_seen: value
                .get("first_seen")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            last_seen: value
                .get("last_seen")
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            sightings: value
                .get("sightings")
                .and_then(|v| v.as_u64())
                .unwrap_or(1),
            severity,
            confidence,
            sources: value
                .get("sources")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            tags: value
                .get("tags")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            description: value
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            related_indicators: value
                .get("related")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|s| s.as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default(),
            tlp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TlpLevel {
    White,
    Green,
    Amber,
    Red,
}

impl fmt::Display for TlpLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlpLevel::White => write!(f, "TLP:WHITE"),
            TlpLevel::Green => write!(f, "TLP:GREEN"),
            TlpLevel::Amber => write!(f, "TLP:AMBER"),
            TlpLevel::Red => write!(f, "TLP:RED"),
        }
    }
}

// ============================================================
// THREAT DATA (AGGREGATED)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatData {
    pub target: String,
    pub indicators: Vec<ThreatIndicator>,
    pub malware_families: Vec<String>,
    pub threat_actors: Vec<String>,
    pub campaigns: Vec<String>,
    pub severity: Severity,
    pub overall_confidence: Confidence,
    pub sources: Vec<ThreatSource>,
    pub total_indicators: usize,
    pub malicious_indicators: usize,
    pub suspicious_indicators: usize,
    pub clean_indicators: usize,
    pub query_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl ThreatData {
    pub fn new(target: &str) -> Self {
        ThreatData {
            target: target.to_string(),
            indicators: vec![],
            malware_families: vec![],
            threat_actors: vec![],
            campaigns: vec![],
            severity: Severity::Info,
            overall_confidence: Confidence::Low,
            sources: vec![],
            total_indicators: 0,
            malicious_indicators: 0,
            suspicious_indicators: 0,
            clean_indicators: 0,
            query_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn add_indicator(&mut self, indicator: ThreatIndicator) {
        self.total_indicators += 1;
        match indicator.severity {
            Severity::Critical | Severity::High => self.malicious_indicators += 1,
            Severity::Medium => self.suspicious_indicators += 1,
            _ => self.clean_indicators += 1,
        }
        self.indicators.push(indicator);
    }

    pub fn threat_ratio(&self) -> f32 {
        if self.total_indicators == 0 {
            return 0.0;
        }
        (self.malicious_indicators as f32 / self.total_indicators as f32) * 100.0
    }

    pub fn is_malicious(&self) -> bool {
        self.threat_ratio() > 50.0
    }

    pub fn has_threat_actors(&self) -> bool {
        !self.threat_actors.is_empty()
    }

    pub fn indicator_types_summary(&self) -> HashMap<String, usize> {
        let mut summary = HashMap::new();
        for indicator in &self.indicators {
            let key = indicator.indicator_type.to_string();
            *summary.entry(key).or_insert(0) += 1;
        }
        summary
    }
}

impl From<serde_json::Value> for ThreatData {
    fn from(value: serde_json::Value) -> Self {
        let target = value
            .get("target")
            .or_else(|| value.get("domain"))
            .or_else(|| value.get("url"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let mut data = ThreatData::new(target);

        // Parse indicators
        if let Some(indicators) = value.get("indicators").and_then(|v| v.as_array()) {
            for ind in indicators {
                data.add_indicator(ThreatIndicator::from(ind.clone()));
            }
        }

        // Parse individual indicator if single
        if data.indicators.is_empty() {
            if value.get("type").and_then(|v| v.as_str()).is_some()
                && value.get("value").and_then(|v| v.as_str()).is_some()
            {
                data.add_indicator(ThreatIndicator::from(value.clone()));
            }
        }

        // Parse malware families
        data.malware_families = value
            .get("malware_families")
            .or_else(|| value.get("malware"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Parse threat actors
        data.threat_actors = value
            .get("threat_actors")
            .or_else(|| value.get("actors"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Parse campaigns
        data.campaigns = value
            .get("campaigns")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|s| s.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        // Parse sources
        if let Some(sources) = value.get("sources").and_then(|v| v.as_array()) {
            data.sources = sources
                .iter()
                .map(|s| ThreatSource::from(s.clone()))
                .collect();
        }

        // Overall severity
        data.severity = value
            .get("severity")
            .and_then(|v| v.as_str())
            .and_then(Severity::from_str)
            .unwrap_or(if data.malicious_indicators > 0 {
                Severity::High
            } else if data.suspicious_indicators > 0 {
                Severity::Medium
            } else {
                Severity::Info
            });

        data.query_time_ms = value
            .get("query_time_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSource {
    pub name: String,
    pub url: Option<String>,
    pub confidence: Confidence,
    pub response_time_ms: u64,
    pub is_online: bool,
    pub error: Option<String>,
}

impl ThreatSource {
    pub fn new(name: &str) -> Self {
        ThreatSource {
            name: name.to_string(),
            url: None,
            confidence: Confidence::Medium,
            response_time_ms: 0,
            is_online: true,
            error: None,
        }
    }

    pub fn mark_offline(&mut self, error: &str) {
        self.is_online = false;
        self.error = Some(error.to_string());
    }
}

impl From<serde_json::Value> for ThreatSource {
    fn from(value: serde_json::Value) -> Self {
        ThreatSource {
            name: value
                .get("name")
                .or_else(|| value.get("source"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            url: value
                .get("url")
                .and_then(|v| v.as_str())
                .map(String::from),
            confidence: value
                .get("confidence")
                .and_then(|v| v.as_f64())
                .map(Confidence::from_score)
                .unwrap_or(Confidence::Medium),
            response_time_ms: value
                .get("response_time_ms")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            is_online: value
                .get("is_online")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            error: value
                .get("error")
                .and_then(|v| v.as_str())
                .map(String::from),
        }
    }
}

// ============================================================
// REPUTATION SCORE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReputationCategory {
    Malicious,
    Phishing,
    Suspicious,
    Benign,
    Unknown,
    Spam,
    Malware,
    Botnet,
    Scanner,
    Proxy,
    TorExit,
}

impl ReputationCategory {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "malicious" | "malware" => ReputationCategory::Malicious,
            "phishing" | "phish" => ReputationCategory::Phishing,
            "suspicious" => ReputationCategory::Suspicious,
            "benign" | "clean" | "safe" => ReputationCategory::Benign,
            "spam" => ReputationCategory::Spam,
            "botnet" | "bot" => ReputationCategory::Botnet,
            "scanner" => ReputationCategory::Scanner,
            "proxy" => ReputationCategory::Proxy,
            "tor_exit" | "tor" => ReputationCategory::TorExit,
            _ => ReputationCategory::Unknown,
        }
    }
}

impl fmt::Display for ReputationCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReputationCategory::Malicious => write!(f, "malicious"),
            ReputationCategory::Phishing => write!(f, "phishing"),
            ReputationCategory::Suspicious => write!(f, "suspicious"),
            ReputationCategory::Benign => write!(f, "benign"),
            ReputationCategory::Unknown => write!(f, "unknown"),
            ReputationCategory::Spam => write!(f, "spam"),
            ReputationCategory::Malware => write!(f, "malware"),
            ReputationCategory::Botnet => write!(f, "botnet"),
            ReputationCategory::Scanner => write!(f, "scanner"),
            ReputationCategory::Proxy => write!(f, "proxy"),
            ReputationCategory::TorExit => write!(f, "tor_exit"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationScore {
    pub target: String,
    pub overall_score: u8,
    pub categories: Vec<ReputationCategory>,
    pub vendors: Vec<VendorResult>,
    pub total_vendors: usize,
    pub malicious_vendors: usize,
    pub suspicious_vendors: usize,
    pub benign_vendors: usize,
    pub detection_rate: f32,
    pub last_analysis: Option<DateTime<Utc>>,
    pub timestamp: DateTime<Utc>,
}

impl ReputationScore {
    pub fn new(target: &str) -> Self {
        ReputationScore {
            target: target.to_string(),
            overall_score: 0,
            categories: vec![],
            vendors: vec![],
            total_vendors: 0,
            malicious_vendors: 0,
            suspicious_vendors: 0,
            benign_vendors: 0,
            detection_rate: 0.0,
            last_analysis: None,
            timestamp: Utc::now(),
        }
    }

    pub fn add_vendor_result(&mut self, vendor: VendorResult) {
        self.total_vendors += 1;
        match vendor.verdict {
            VendorVerdict::Malicious => self.malicious_vendors += 1,
            VendorVerdict::Suspicious => self.suspicious_vendors += 1,
            _ => self.benign_vendors += 1,
        }
        self.vendors.push(vendor);
        self.calculate_scores();
    }

    fn calculate_scores(&mut self) {
        if self.total_vendors > 0 {
            self.detection_rate = (self.malicious_vendors as f32 / self.total_vendors as f32) * 100.0;
        }
        self.overall_score = ((self.benign_vendors as f32 / self.total_vendors.max(1) as f32) * 100.0) as u8;
    }

    pub fn is_malicious(&self) -> bool {
        self.detection_rate >= 50.0 || self.malicious_vendors >= 3
    }

    pub fn is_suspicious(&self) -> bool {
        self.detection_rate > 0.0 && self.detection_rate < 50.0
    }

    pub fn is_clean(&self) -> bool {
        self.detection_rate == 0.0 && self.malicious_vendors == 0
    }

    pub fn reputation_level(&self) -> &str {
        if self.is_malicious() { return "malicious"; }
        if self.is_suspicious() { return "suspicious"; }
        "clean"
    }

    pub fn vendor_names(&self) -> Vec<String> {
        self.vendors.iter().map(|v| v.name.clone()).collect()
    }
}

impl From<serde_json::Value> for ReputationScore {
    fn from(value: serde_json::Value) -> Self {
        let target = value
            .get("target")
            .or_else(|| value.get("domain"))
            .or_else(|| value.get("url"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let mut score = ReputationScore::new(target);

        // Parse vendors/results
        if let Some(vendors) = value
            .get("vendors")
            .or_else(|| value.get("results"))
            .or_else(|| value.get("scans"))
            .and_then(|v| v.as_object())
        {
            for (name, result) in vendors {
                let verdict = if let Some(obj) = result.as_object() {
                    let detected = obj
                        .get("detected")
                        .or_else(|| obj.get("result"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    let category = obj
                        .get("category")
                        .or_else(|| obj.get("result"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    if detected {
                        match category.to_lowercase().as_str() {
                            "malicious" | "malware" | "phishing" => VendorVerdict::Malicious,
                            "suspicious" => VendorVerdict::Suspicious,
                            _ => VendorVerdict::Malicious,
                        }
                    } else {
                        VendorVerdict::Benign
                    }
                } else if let Some(s) = result.as_str() {
                    match s.to_lowercase().as_str() {
                        "malicious" | "malware" | "detected" => VendorVerdict::Malicious,
                        "suspicious" => VendorVerdict::Suspicious,
                        "clean" | "undetected" | "benign" => VendorVerdict::Benign,
                        _ => VendorVerdict::Unknown,
                    }
                } else {
                    VendorVerdict::Unknown
                };

                score.add_vendor_result(VendorResult::new(name, verdict));
            }
        }

        // Parse categories
        if let Some(categories) = value.get("categories").and_then(|v| v.as_array()) {
            score.categories = categories
                .iter()
                .filter_map(|c| c.as_str().map(ReputationCategory::from_str))
                .collect();
        }

        // Parse last analysis timestamp
        score.last_analysis = value
            .get("last_analysis")
            .or_else(|| value.get("scan_date"))
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        // Calculate final scores
        score.calculate_scores();
        score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorResult {
    pub name: String,
    pub verdict: VendorVerdict,
    pub category: Option<String>,
    pub detection_details: Option<String>,
    pub engine_version: Option<String>,
    pub engine_update: Option<DateTime<Utc>>,
}

impl VendorResult {
    pub fn new(name: &str, verdict: VendorVerdict) -> Self {
        VendorResult {
            name: name.to_string(),
            verdict,
            category: None,
            detection_details: None,
            engine_version: None,
            engine_update: None,
        }
    }
}

impl From<serde_json::Value> for VendorResult {
    fn from(value: serde_json::Value) -> Self {
        let verdict = value
            .get("verdict")
            .or_else(|| value.get("result"))
            .and_then(|v| v.as_str())
            .map(|s| match s.to_lowercase().as_str() {
                "malicious" | "detected" => VendorVerdict::Malicious,
                "suspicious" => VendorVerdict::Suspicious,
                "clean" | "benign" | "undetected" => VendorVerdict::Benign,
                "timeout" => VendorVerdict::Timeout,
                "error" => VendorVerdict::Error,
                _ => VendorVerdict::Unknown,
            })
            .unwrap_or(VendorVerdict::Unknown);

        VendorResult {
            name: value
                .get("name")
                .or_else(|| value.get("engine"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string(),
            verdict,
            category: value
                .get("category")
                .and_then(|v| v.as_str())
                .map(String::from),
            detection_details: value
                .get("details")
                .or_else(|| value.get("result"))
                .and_then(|v| v.as_str())
                .map(String::from),
            engine_version: value
                .get("version")
                .or_else(|| value.get("engine_version"))
                .and_then(|v| v.as_str())
                .map(String::from),
            engine_update: value
                .get("update_date")
                .or_else(|| value.get("engine_update"))
                .and_then(|v| v.as_str())
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VendorVerdict {
    Malicious,
    Suspicious,
    Benign,
    Unknown,
    Timeout,
    Error,
}

impl fmt::Display for VendorVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VendorVerdict::Malicious => write!(f, "malicious"),
            VendorVerdict::Suspicious => write!(f, "suspicious"),
            VendorVerdict::Benign => write!(f, "benign"),
            VendorVerdict::Unknown => write!(f, "unknown"),
            VendorVerdict::Timeout => write!(f, "timeout"),
            VendorVerdict::Error => write!(f, "error"),
        }
    }
}

// ============================================================
// BLACKLIST STATUS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlacklistSource {
    Spamhaus,
    SURBL,
    URIBL,
    DNSBL,
    SORBS,
    SpamCop,
    Barracuda,
    Invaluement,
    UceProtect,
    Custom(String),
}

impl BlacklistSource {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "spamhaus" | "zen" => BlacklistSource::Spamhaus,
            "surbl" => BlacklistSource::SURBL,
            "uribl" => BlacklistSource::URIBL,
            "dnsbl" => BlacklistSource::DNSBL,
            "sorbs" => BlacklistSource::SORBS,
            "spamcop" => BlacklistSource::SpamCop,
            "barracuda" | "bracuda" => BlacklistSource::Barracuda,
            "invaluement" => BlacklistSource::Invaluement,
            "uceprotect" | "uce" => BlacklistSource::UceProtect,
            other => BlacklistSource::Custom(other.to_string()),
        }
    }
}

impl fmt::Display for BlacklistSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlacklistSource::Spamhaus => write!(f, "Spamhaus"),
            BlacklistSource::SURBL => write!(f, "SURBL"),
            BlacklistSource::URIBL => write!(f, "URIBL"),
            BlacklistSource::DNSBL => write!(f, "DNSBL"),
            BlacklistSource::SORBS => write!(f, "SORBS"),
            BlacklistSource::SpamCop => write!(f, "SpamCop"),
            BlacklistSource::Barracuda => write!(f, "Barracuda"),
            BlacklistSource::Invaluement => write!(f, "Invaluement"),
            BlacklistSource::UceProtect => write!(f, "UCEProtect"),
            BlacklistSource::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BlacklistCategory {
    Spam,
    Malware,
    Phishing,
    Exploit,
    Abuse,
    Botnet,
    OpenRelay,
    OpenProxy,
    Ddos,
    Compromised,
    Other(String),
}

impl BlacklistCategory {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "spam" => BlacklistCategory::Spam,
            "malware" => BlacklistCategory::Malware,
            "phishing" | "phish" => BlacklistCategory::Phishing,
            "exploit" => BlacklistCategory::Exploit,
            "abuse" => BlacklistCategory::Abuse,
            "botnet" | "bot" => BlacklistCategory::Botnet,
            "open_relay" | "openrelay" => BlacklistCategory::OpenRelay,
            "open_proxy" | "openproxy" => BlacklistCategory::OpenProxy,
            "ddos" => BlacklistCategory::Ddos,
            "compromised" | "hacked" => BlacklistCategory::Compromised,
            other => BlacklistCategory::Other(other.to_string()),
        }
    }
}

impl fmt::Display for BlacklistCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlacklistCategory::Spam => write!(f, "spam"),
            BlacklistCategory::Malware => write!(f, "malware"),
            BlacklistCategory::Phishing => write!(f, "phishing"),
            BlacklistCategory::Exploit => write!(f, "exploit"),
            BlacklistCategory::Abuse => write!(f, "abuse"),
            BlacklistCategory::Botnet => write!(f, "botnet"),
            BlacklistCategory::OpenRelay => write!(f, "open_relay"),
            BlacklistCategory::OpenProxy => write!(f, "open_proxy"),
            BlacklistCategory::Ddos => write!(f, "ddos"),
            BlacklistCategory::Compromised => write!(f, "compromised"),
            BlacklistCategory::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistStatus {
    pub target: String,
    pub listed_in: Vec<BlacklistEntry>,
    pub total_checked: usize,
    pub total_listed: usize,
    pub categories: Vec<BlacklistCategory>,
    pub is_clean: bool,
    pub query_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl BlacklistStatus {
    pub fn new(target: &str) -> Self {
        BlacklistStatus {
            target: target.to_string(),
            listed_in: vec![],
            total_checked: 0,
            total_listed: 0,
            categories: vec![],
            is_clean: true,
            query_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn add_listing(&mut self, entry: BlacklistEntry) {
        self.total_listed += 1;
        self.is_clean = false;
        if !self.categories.contains(&entry.category) {
            self.categories.push(entry.category.clone());
        }
        self.listed_in.push(entry);
    }

    pub fn listing_percentage(&self) -> f32 {
        if self.total_checked == 0 {
            return 0.0;
        }
        (self.total_listed as f32 / self.total_checked as f32) * 100.0
    }

    pub fn is_heavily_blacklisted(&self) -> bool {
        self.listing_percentage() > 30.0
    }
}

impl From<serde_json::Value> for BlacklistStatus {
    fn from(value: serde_json::Value) -> Self {
        let target = value
            .get("target")
            .or_else(|| value.get("domain"))
            .or_else(|| value.get("ip"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let mut status = BlacklistStatus::new(target);

        // Parse listings/results
        if let Some(listings) = value
            .get("listings")
            .or_else(|| value.get("results"))
            .or_else(|| value.get("blacklists"))
            .and_then(|v| v.as_object())
        {
            for (source_name, result) in listings {
                let source = BlacklistSource::from_str(source_name);

                if let Some(obj) = result.as_object() {
                    let listed = obj
                        .get("listed")
                        .or_else(|| obj.get("detected"))
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);

                    if listed || obj.get("category").is_some() {
                        let category = obj
                            .get("category")
                            .and_then(|v| v.as_str())
                            .map(BlacklistCategory::from_str)
                            .unwrap_or(BlacklistCategory::Spam);

                        let entry = BlacklistEntry {
                            source,
                            category,
                            reason: obj
                                .get("reason")
                                .and_then(|v| v.as_str())
                                .map(String::from),
                            response_code: obj
                                .get("code")
                                .or_else(|| obj.get("return_code"))
                                .and_then(|v| v.as_str())
                                .map(String::from),
                            listed_since: obj
                                .get("listed_since")
                                .and_then(|v| v.as_str())
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc)),
                            expires: obj
                                .get("expires")
                                .and_then(|v| v.as_str())
                                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                                .map(|dt| dt.with_timezone(&Utc)),
                        };

                        status.add_listing(entry);
                    }
                } else if let Some(s) = result.as_str() {
                    if s.to_lowercase() != "clean" && s.to_lowercase() != "not_listed" {
                        status.add_listing(BlacklistEntry::new(
                            source,
                            BlacklistCategory::from_str(s),
                        ));
                    }
                }
            }
        }

        // Parse as array of listed sources
        if let Some(listed_arr) = value
            .get("listed_in")
            .or_else(|| value.get("blacklisted_in"))
            .and_then(|v| v.as_array())
        {
            for item in listed_arr {
                if let Some(source_str) = item.as_str() {
                    let source = BlacklistSource::from_str(source_str);
                    status.add_listing(BlacklistEntry::new(
                        source,
                        BlacklistCategory::Spam,
                    ));
                } else if let Some(obj) = item.as_object() {
                    let source = obj
                        .get("source")
                        .or_else(|| obj.get("name"))
                        .and_then(|v| v.as_str())
                        .map(BlacklistSource::from_str)
                        .unwrap_or(BlacklistSource::Custom("unknown".to_string()));

                    let category = obj
                        .get("category")
                        .and_then(|v| v.as_str())
                        .map(BlacklistCategory::from_str)
                        .unwrap_or(BlacklistCategory::Spam);

                    status.add_listing(BlacklistEntry::new(source, category));
                }
            }
        }

        // Parse total checked
        status.total_checked = value
            .get("total_checked")
            .or_else(|| value.get("total"))
            .and_then(|v| v.as_u64())
            .unwrap_or(status.total_listed.max(1) as u64) as usize;

        status.query_time_ms = value
            .get("query_time_ms")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);

        status
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistEntry {
    pub source: BlacklistSource,
    pub category: BlacklistCategory,
    pub reason: Option<String>,
    pub response_code: Option<String>,
    pub listed_since: Option<DateTime<Utc>>,
    pub expires: Option<DateTime<Utc>>,
}

impl BlacklistEntry {
    pub fn new(source: BlacklistSource, category: BlacklistCategory) -> Self {
        BlacklistEntry {
            source,
            category,
            reason: None,
            response_code: None,
            listed_since: None,
            expires: None,
        }
    }
}

// ============================================================
// HARVESTED EMAIL
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailCategory {
    Role,
    Personal,
    Generic,
    Disposable,
    Unknown,
}

impl EmailCategory {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "role" => EmailCategory::Role,
            "personal" => EmailCategory::Personal,
            "generic" => EmailCategory::Generic,
            "disposable" => EmailCategory::Disposable,
            _ => EmailCategory::Unknown,
        }
    }
}

impl fmt::Display for EmailCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailCategory::Role => write!(f, "role"),
            EmailCategory::Personal => write!(f, "personal"),
            EmailCategory::Generic => write!(f, "generic"),
            EmailCategory::Disposable => write!(f, "disposable"),
            EmailCategory::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailSource {
    HTML,
    JavaScript,
    Mailto,
    File,
    Comment,
    Header,
    Whois,
    Other(String),
}

impl EmailSource {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "html" | "page" => EmailSource::HTML,
            "javascript" | "js" => EmailSource::JavaScript,
            "mailto" | "mail" => EmailSource::Mailto,
            "file" | "document" => EmailSource::File,
            "comment" => EmailSource::Comment,
            "header" => EmailSource::Header,
            "whois" => EmailSource::Whois,
            other => EmailSource::Other(other.to_string()),
        }
    }
}

impl fmt::Display for EmailSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailSource::HTML => write!(f, "html"),
            EmailSource::JavaScript => write!(f, "javascript"),
            EmailSource::Mailto => write!(f, "mailto"),
            EmailSource::File => write!(f, "file"),
            EmailSource::Comment => write!(f, "comment"),
            EmailSource::Header => write!(f, "header"),
            EmailSource::Whois => write!(f, "whois"),
            EmailSource::Other(s) => write!(f, "other:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmailValidationStatus {
    Valid,
    Invalid,
    Disposable,
    RoleBased,
    CatchAll,
    Unknown,
}

impl EmailValidationStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "valid" => EmailValidationStatus::Valid,
            "invalid" => EmailValidationStatus::Invalid,
            "disposable" => EmailValidationStatus::Disposable,
            "role_based" | "role" => EmailValidationStatus::RoleBased,
            "catch_all" | "catchall" => EmailValidationStatus::CatchAll,
            _ => EmailValidationStatus::Unknown,
        }
    }
}

impl fmt::Display for EmailValidationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmailValidationStatus::Valid => write!(f, "valid"),
            EmailValidationStatus::Invalid => write!(f, "invalid"),
            EmailValidationStatus::Disposable => write!(f, "disposable"),
            EmailValidationStatus::RoleBased => write!(f, "role_based"),
            EmailValidationStatus::CatchAll => write!(f, "catch_all"),
            EmailValidationStatus::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarvestedEmail {
    pub email: String,
    pub local_part: String,
    pub domain: String,
    pub category: EmailCategory,
    pub source: EmailSource,
    pub source_url: Option<String>,
    pub validation_status: EmailValidationStatus,
    pub is_disposable: bool,
    pub is_role_based: bool,
    pub confidence: Confidence,
    pub timestamp: DateTime<Utc>,
}

impl HarvestedEmail {
    pub fn new(email: &str, source: EmailSource) -> Self {
        let (local_part, domain) = email.split_once('@').unwrap_or((email, "unknown"));
        let is_role = matches!(
            local_part.to_lowercase().as_str(),
            "admin" | "info" | "support" | "sales" | "contact" | "help" | "abuse" | "postmaster" | "webmaster" | "noreply" | "no-reply"
        );

        let category = if is_role {
            EmailCategory::Role
        } else {
            EmailCategory::Unknown
        };

        HarvestedEmail {
            email: email.to_string(),
            local_part: local_part.to_string(),
            domain: domain.to_string(),
            category,
            source,
            source_url: None,
            validation_status: EmailValidationStatus::Unknown,
            is_disposable: false,
            is_role_based: is_role,
            confidence: Confidence::Medium,
            timestamp: Utc::now(),
        }
    }

    pub fn mask_email(&self) -> String {
        format!("{}***@{}", &self.local_part[..self.local_part.len().min(2)], self.domain)
    }

    pub fn is_valid(&self) -> bool {
        self.validation_status == EmailValidationStatus::Valid
    }
}

impl From<serde_json::Value> for HarvestedEmail {
    fn from(value: serde_json::Value) -> Self {
        let email = value
            .get("email")
            .or_else(|| value.get("address"))
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let source = value
            .get("source")
            .and_then(|v| v.as_str())
            .map(EmailSource::from_str)
            .unwrap_or(EmailSource::Other("unknown".to_string()));

        let mut harvested = HarvestedEmail::new(email, source);

        harvested.source_url = value
            .get("source_url")
            .or_else(|| value.get("url"))
            .and_then(|v| v.as_str())
            .map(String::from);

        harvested.category = value
            .get("category")
            .and_then(|v| v.as_str())
            .map(EmailCategory::from_str)
            .unwrap_or(harvested.category);

        harvested.validation_status = value
            .get("validation")
            .or_else(|| value.get("status"))
            .and_then(|v| v.as_str())
            .map(EmailValidationStatus::from_str)
            .unwrap_or(EmailValidationStatus::Unknown);

        harvested.is_disposable = value
            .get("is_disposable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        harvested.is_role_based = value
            .get("is_role_based")
            .and_then(|v| v.as_bool())
            .unwrap_or(harvested.is_role_based);

        harvested.confidence = value
            .get("confidence")
            .and_then(|v| v.as_f64())
            .map(Confidence::from_score)
            .unwrap_or(Confidence::Medium);

        harvested
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailHarvestReport {
    pub target_url: String,
    pub emails: Vec<HarvestedEmail>,
    pub total_emails: usize,
    pub unique_emails: usize,
    pub role_emails: usize,
    pub personal_emails: usize,
    pub disposable_emails: usize,
    pub valid_emails: usize,
    pub invalid_emails: usize,
    pub sources: HashMap<String, usize>,
    pub domains: HashMap<String, usize>,
    pub query_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl EmailHarvestReport {
    pub fn new(target_url: &str) -> Self {
        EmailHarvestReport {
            target_url: target_url.to_string(),
            emails: vec![],
            total_emails: 0,
            unique_emails: 0,
            role_emails: 0,
            personal_emails: 0,
            disposable_emails: 0,
            valid_emails: 0,
            invalid_emails: 0,
            sources: HashMap::new(),
            domains: HashMap::new(),
            query_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn add_email(&mut self, email: HarvestedEmail) {
        self.total_emails += 1;
        match email.category {
            EmailCategory::Role => self.role_emails += 1,
            EmailCategory::Personal => self.personal_emails += 1,
            EmailCategory::Disposable => self.disposable_emails += 1,
            _ => {}
        }
        if email.is_valid() { self.valid_emails += 1; }
        else { self.invalid_emails += 1; }

        let source_key = email.source.to_string();
        *self.sources.entry(source_key).or_insert(0) += 1;
        *self.domains.entry(email.domain.clone()).or_insert(0) += 1;

        // Deduplication
        if !self.emails.iter().any(|e| e.email == email.email) {
            self.unique_emails += 1;
        }
        self.emails.push(email);
    }

    pub fn email_density(&self) -> f32 {
        self.unique_emails as f32
    }

    pub fn top_domains(&self, n: usize) -> Vec<(String, usize)> {
        let mut domains: Vec<(String, usize)> = self.domains.iter().map(|(k, v)| (k.clone(), *v)).collect();
        domains.sort_by(|a, b| b.1.cmp(&a.1));
        domains.truncate(n);
        domains
    }

    pub fn security_concern_level(&self) -> Severity {
        if self.disposable_emails > 0 { return Severity::Medium; }
        if self.valid_emails > 10 { return Severity::Low; }
        Severity::Info
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_indicator_recent() {
        let mut indicator = ThreatIndicator::new(IndicatorType::Domain, "evil.com");
        indicator.last_seen = Some(Utc::now());
        assert!(indicator.is_recent(7));
        assert!(!indicator.is_recent(0));
    }

    #[test]
    fn test_threat_data_add_indicator() {
        let mut data = ThreatData::new("example.com");
        let mut indicator = ThreatIndicator::new(IndicatorType::URL, "http://phish.com");
        indicator.severity = Severity::Critical;
        data.add_indicator(indicator);
        data.add_indicator(ThreatIndicator::new(IndicatorType::Domain, "safe.com"));

        assert_eq!(data.total_indicators, 2);
        assert_eq!(data.malicious_indicators, 1);
        assert!(data.threat_ratio() > 0.0);
    }

    #[test]
    fn test_threat_data_is_malicious() {
        let mut data = ThreatData::new("evil.com");
        let mut indicator = ThreatIndicator::new(IndicatorType::Domain, "evil.com");
        indicator.severity = Severity::Critical;
        data.add_indicator(indicator);
        assert!(data.is_malicious());
    }

    #[test]
    fn test_reputation_score_add_vendor() {
        let mut score = ReputationScore::new("example.com");
        score.add_vendor_result(VendorResult::new("VirusTotal", VendorVerdict::Malicious));
        score.add_vendor_result(VendorResult::new("Google", VendorVerdict::Benign));
        score.add_vendor_result(VendorResult::new("McAfee", VendorVerdict::Benign));

        assert_eq!(score.total_vendors, 3);
        assert_eq!(score.malicious_vendors, 1);
        assert!(score.detection_rate > 0.0);
        assert!(score.is_suspicious());
    }

    #[test]
    fn test_reputation_score_clean() {
        let mut score = ReputationScore::new("safe.com");
        score.add_vendor_result(VendorResult::new("VirusTotal", VendorVerdict::Benign));
        score.add_vendor_result(VendorResult::new("Google", VendorVerdict::Benign));
        assert!(score.is_clean());
        assert_eq!(score.reputation_level(), "clean");
    }

    #[test]
    fn test_blacklist_status_add_listing() {
        let mut status = BlacklistStatus::new("spam.com");
        status.total_checked = 10;
        status.add_listing(BlacklistEntry::new(
            BlacklistSource::Spamhaus,
            BlacklistCategory::Spam,
        ));
        status.add_listing(BlacklistEntry::new(
            BlacklistSource::SURBL,
            BlacklistCategory::Malware,
        ));

        assert!(!status.is_clean);
        assert_eq!(status.total_listed, 2);
        assert_eq!(status.listing_percentage(), 20.0);
        assert!(!status.is_heavily_blacklisted());
    }

    #[test]
    fn test_blacklist_heavily_listed() {
        let mut status = BlacklistStatus::new("bad.com");
        status.total_checked = 10;
        for _ in 0..5 {
            status.add_listing(BlacklistEntry::new(
                BlacklistSource::SORBS,
                BlacklistCategory::Spam,
            ));
        }
        assert!(status.is_heavily_blacklisted());
    }

    #[test]
    fn test_harvested_email_role_based() {
        let email = HarvestedEmail::new("admin@example.com", EmailSource::HTML);
        assert!(email.is_role_based);
        assert_eq!(email.category, EmailCategory::Role);
    }

    #[test]
    fn test_harvested_email_mask() {
        let email = HarvestedEmail::new("john.doe@example.com", EmailSource::Mailto);
        assert!(email.mask_email().contains("***@"));
    }

    #[test]
    fn test_email_harvest_report() {
        let mut report = EmailHarvestReport::new("https://example.com");
        report.add_email(HarvestedEmail::new("admin@example.com", EmailSource::HTML));
        report.add_email(HarvestedEmail::new("contact@example.com", EmailSource::Mailto));
        report.add_email(HarvestedEmail::new("user@gmail.com", EmailSource::JavaScript));

        assert_eq!(report.total_emails, 3);
        assert_eq!(report.unique_emails, 3);
        assert_eq!(report.role_emails, 2);
        assert_eq!(report.domains.len(), 2);
    }

    #[test]
    fn test_email_harvest_top_domains() {
        let mut report = EmailHarvestReport::new("https://example.com");
        report.add_email(HarvestedEmail::new("a@domain1.com", EmailSource::HTML));
        report.add_email(HarvestedEmail::new("b@domain1.com", EmailSource::HTML));
        report.add_email(HarvestedEmail::new("c@domain2.com", EmailSource::HTML));

        let top = report.top_domains(2);
        assert_eq!(top[0].0, "domain1.com");
        assert_eq!(top[0].1, 2);
    }

    #[test]
    fn test_threat_source_mark_offline() {
        let mut source = ThreatSource::new("Shodan");
        assert!(source.is_online);
        source.mark_offline("Connection timeout");
        assert!(!source.is_online);
        assert!(source.error.is_some());
    }

    // ============================================================
    // From<serde_json::Value> TESTS
    // ============================================================

    #[test]
    fn test_threat_indicator_from_json() {
        let json = serde_json::json!({
            "type": "domain",
            "value": "evil.com",
            "severity": "high",
            "confidence": 0.9,
            "tlp": "AMBER",
            "sightings": 42,
            "sources": ["AlienVault"],
            "tags": ["phishing"],
            "description": "Malicious domain"
        });

        let indicator = ThreatIndicator::from(json);
        assert_eq!(indicator.indicator_type, IndicatorType::Domain);
        assert_eq!(indicator.value, "evil.com");
        assert_eq!(indicator.severity, Severity::High);
        assert!(indicator.is_high_confidence());
        assert_eq!(indicator.tlp, TlpLevel::Amber);
        assert_eq!(indicator.sightings, 42);
    }

    #[test]
    fn test_threat_data_from_json() {
        let json = serde_json::json!({
            "target": "example.com",
            "indicators": [
                {"type": "domain", "value": "evil.com", "severity": "critical"},
                {"type": "ipv4", "value": "1.2.3.4", "severity": "medium"}
            ],
            "malware_families": ["emotet"],
            "threat_actors": ["APT29"],
            "sources": [{"name": "AlienVault", "is_online": true}]
        });

        let data = ThreatData::from(json);
        assert_eq!(data.target, "example.com");
        assert_eq!(data.total_indicators, 2);
        assert_eq!(data.malicious_indicators, 1);
        assert_eq!(data.suspicious_indicators, 1);
        assert!(data.has_threat_actors());
    }

    #[test]
    fn test_reputation_score_from_json_virustotal() {
        let json = serde_json::json!({
            "domain": "example.com",
            "vendors": {
                "VirusTotal": {"detected": true, "category": "malicious"},
                "Google": {"detected": false},
                "McAfee": {"detected": false}
            },
            "categories": ["malicious", "phishing"]
        });

        let score = ReputationScore::from(json);
        assert_eq!(score.total_vendors, 3);
        assert_eq!(score.malicious_vendors, 1);
        assert!(score.is_suspicious());
    }

    #[test]
    fn test_reputation_score_from_json_string_verdicts() {
        let json = serde_json::json!({
            "target": "safe.com",
            "vendors": {
                "VirusTotal": "clean",
                "Google": "benign",
                "McAfee": "undetected"
            }
        });

        let score = ReputationScore::from(json);
        assert_eq!(score.total_vendors, 3);
        assert!(score.is_clean());
    }

    #[test]
    fn test_blacklist_status_from_json() {
        let json = serde_json::json!({
            "target": "spam.com",
            "listings": {
                "spamhaus": {"listed": true, "category": "spam", "reason": "Known spammer"},
                "surbl": {"listed": true, "category": "malware"},
                "barracuda": {"listed": false}
            },
            "total_checked": 10
        });

        let status = BlacklistStatus::from(json);
        assert!(!status.is_clean);
        assert_eq!(status.total_listed, 2);
        assert_eq!(status.total_checked, 10);
    }

    #[test]
    fn test_blacklist_status_from_json_array() {
        let json = serde_json::json!({
            "domain": "bad.com",
            "blacklisted_in": ["spamhaus", "surbl", "sorbs"],
            "total": 10
        });

        let status = BlacklistStatus::from(json);
        assert_eq!(status.total_listed, 3);
        assert_eq!(status.listing_percentage(), 30.0);
    }

    #[test]
    fn test_harvested_email_from_json() {
        let json = serde_json::json!({
            "email": "admin@example.com",
            "source": "html",
            "category": "role",
            "validation": "valid",
            "is_disposable": false,
            "is_role_based": true,
            "confidence": 0.85
        });

        let email = HarvestedEmail::from(json);
        assert_eq!(email.email, "admin@example.com");
        assert_eq!(email.source, EmailSource::HTML);
        assert_eq!(email.category, EmailCategory::Role);
        assert!(email.is_valid());
        assert!(email.is_role_based);
    }

    #[test]
    fn test_indicator_type_from_str() {
        assert_eq!(IndicatorType::from_str("domain"), IndicatorType::Domain);
        assert_eq!(IndicatorType::from_str("IP"), IndicatorType::IPv4);
        assert_eq!(IndicatorType::from_str("sha256"), IndicatorType::FileHashSha256);
    }

    #[test]
    fn test_reputation_category_from_str() {
        assert_eq!(ReputationCategory::from_str("malware"), ReputationCategory::Malicious);
        assert_eq!(ReputationCategory::from_str("clean"), ReputationCategory::Benign);
    }

    #[test]
    fn test_blacklist_source_from_str() {
        assert_eq!(BlacklistSource::from_str("zen"), BlacklistSource::Spamhaus);
        assert_eq!(BlacklistSource::from_str("custom"), BlacklistSource::Custom("custom".to_string()));
    }
}
