// shared/types/infrastructure_types.rs
// IWS v1.0 - Infrastructure Types
// Mendefinisikan tipe data untuk infrastructure intelligence

use std::fmt;
use std::net::IpAddr;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::common_types::{GeoLocation, Confidence};

// ============================================================
// SERVER FINGERPRINT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerType {
    Apache,
    Nginx,
    IIS,
    Lighttpd,
    Caddy,
    Tomcat,
    Jetty,
    NodeJS,
    Gunicorn,
    Uvicorn,
    Unknown(String),
}

impl ServerType {
    pub fn from_header(header: &str) -> Self {
        let lower = header.to_lowercase();
        if lower.contains("apache") { return ServerType::Apache; }
        if lower.contains("nginx") { return ServerType::Nginx; }
        if lower.contains("iis") || lower.contains("microsoft") { return ServerType::IIS; }
        if lower.contains("lighttpd") { return ServerType::Lighttpd; }
        if lower.contains("caddy") { return ServerType::Caddy; }
        if lower.contains("tomcat") { return ServerType::Tomcat; }
        if lower.contains("jetty") { return ServerType::Jetty; }
        if lower.contains("node") || lower.contains("express") { return ServerType::NodeJS; }
        if lower.contains("gunicorn") { return ServerType::Gunicorn; }
        if lower.contains("uvicorn") { return ServerType::Uvicorn; }
        ServerType::Unknown(header.to_string())
    }

    pub fn is_common(&self) -> bool {
        matches!(self, ServerType::Apache | ServerType::Nginx | ServerType::IIS)
    }
}

impl fmt::Display for ServerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerType::Apache => write!(f, "Apache"),
            ServerType::Nginx => write!(f, "Nginx"),
            ServerType::IIS => write!(f, "Microsoft IIS"),
            ServerType::Lighttpd => write!(f, "Lighttpd"),
            ServerType::Caddy => write!(f, "Caddy"),
            ServerType::Tomcat => write!(f, "Apache Tomcat"),
            ServerType::Jetty => write!(f, "Jetty"),
            ServerType::NodeJS => write!(f, "Node.js"),
            ServerType::Gunicorn => write!(f, "Gunicorn"),
            ServerType::Uvicorn => write!(f, "Uvicorn"),
            ServerType::Unknown(s) => write!(f, "Unknown ({})", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub server_type: ServerType,
    pub version: Option<String>,
    pub os: Option<String>,
    pub powered_by: Option<String>,
    pub framework: Option<String>,
    pub platform: Option<String>,
    pub generator: Option<String>,
    pub headers: Vec<ServerHeader>,
    pub jarm_hash: Option<String>,
    pub error_page_fingerprint: Option<String>,
    pub confidence: Confidence,
    pub timestamp: DateTime<Utc>,
}

impl ServerInfo {
    pub fn new() -> Self {
        ServerInfo {
            server_type: ServerType::Unknown("unknown".to_string()),
            version: None,
            os: None,
            powered_by: None,
            framework: None,
            platform: None,
            generator: None,
            headers: vec![],
            jarm_hash: None,
            error_page_fingerprint: None,
            confidence: Confidence::Low,
            timestamp: Utc::now(),
        }
    }

    pub fn is_outdated(&self) -> bool {
        // Simplified check — production implementation would check against version database
        if let Some(ref version) = self.version {
            let parts: Vec<&str> = version.split('.').collect();
            if let Some(major) = parts.first().and_then(|v| v.parse::<u32>().ok()) {
                return match self.server_type {
                    ServerType::Apache => major < 2,
                    ServerType::Nginx => major < 1,
                    ServerType::IIS => major < 10,
                    _ => false,
                };
            }
        }
        false
    }

    pub fn full_identity(&self) -> String {
        let mut identity = self.server_type.to_string();
        if let Some(ref v) = self.version { identity.push_str(&format!(" {}", v)); }
        if let Some(ref os) = self.os { identity.push_str(&format!(" on {}", os)); }
        if let Some(ref fw) = self.framework { identity.push_str(&format!(" ({})", fw)); }
        identity
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHeader {
    pub name: String,
    pub value: String,
    pub is_security_relevant: bool,
    pub exposes_version: bool,
}

impl ServerHeader {
    pub fn new(name: &str, value: &str) -> Self {
        let is_security = matches!(
            name.to_lowercase().as_str(),
            "server" | "x-powered-by" | "x-aspnet-version" | "x-generator" | "x-drupal-cache" | "x-drupal-dynamic-cache"
        );
        let exposes_ver = value.chars().any(|c| c.is_numeric());

        ServerHeader {
            name: name.to_string(),
            value: value.to_string(),
            is_security_relevant: is_security,
            exposes_version: exposes_ver,
        }
    }
}

// ============================================================
// CLOUD DETECTION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudProviderType {
    AWS,
    GCP,
    Azure,
    DigitalOcean,
    Linode,
    Vultr,
    Heroku,
    Oracle,
    Alibaba,
    IBM,
    Unknown(String),
}

impl fmt::Display for CloudProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudProviderType::AWS => write!(f, "AWS"),
            CloudProviderType::GCP => write!(f, "GCP"),
            CloudProviderType::Azure => write!(f, "Azure"),
            CloudProviderType::DigitalOcean => write!(f, "DigitalOcean"),
            CloudProviderType::Linode => write!(f, "Linode"),
            CloudProviderType::Vultr => write!(f, "Vultr"),
            CloudProviderType::Heroku => write!(f, "Heroku"),
            CloudProviderType::Oracle => write!(f, "Oracle Cloud"),
            CloudProviderType::Alibaba => write!(f, "Alibaba Cloud"),
            CloudProviderType::IBM => write!(f, "IBM Cloud"),
            CloudProviderType::Unknown(s) => write!(f, "Unknown ({})", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDetection {
    pub provider: CloudProviderType,
    pub region: Option<String>,
    pub availability_zone: Option<String>,
    pub services: Vec<CloudService>,
    pub confidence: Confidence,
    pub indicators: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl CloudDetection {
    pub fn new() -> Self {
        CloudDetection {
            provider: CloudProviderType::Unknown("unknown".to_string()),
            region: None,
            availability_zone: None,
            services: vec![],
            confidence: Confidence::Low,
            indicators: vec![],
            timestamp: Utc::now(),
        }
    }

    pub fn is_cloud_hosted(&self) -> bool {
        !matches!(self.provider, CloudProviderType::Unknown(_))
    }

    pub fn has_service(&self, service_name: &str) -> bool {
        self.services.iter().any(|s| s.name.to_lowercase() == service_name.to_lowercase())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudService {
    pub name: String,
    pub service_type: CloudServiceType,
    pub detected_via: String,
    pub confidence: Confidence,
}

impl CloudService {
    pub fn new(name: &str, service_type: CloudServiceType) -> Self {
        CloudService {
            name: name.to_string(),
            service_type,
            detected_via: String::new(),
            confidence: Confidence::Medium,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudServiceType {
    Compute,
    Storage,
    CDN,
    Database,
    Function,
    LoadBalancer,
    DNS,
    WAF,
    ObjectStorage,
    Container,
    Unknown(String),
}

impl fmt::Display for CloudServiceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudServiceType::Compute => write!(f, "compute"),
            CloudServiceType::Storage => write!(f, "storage"),
            CloudServiceType::CDN => write!(f, "cdn"),
            CloudServiceType::Database => write!(f, "database"),
            CloudServiceType::Function => write!(f, "function"),
            CloudServiceType::LoadBalancer => write!(f, "load_balancer"),
            CloudServiceType::DNS => write!(f, "dns"),
            CloudServiceType::WAF => write!(f, "waf"),
            CloudServiceType::ObjectStorage => write!(f, "object_storage"),
            CloudServiceType::Container => write!(f, "container"),
            CloudServiceType::Unknown(s) => write!(f, "unknown:{}", s),
        }
    }
}

// ============================================================
// CDN DETECTION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CdnProvider {
    Cloudflare,
    Akamai,
    Fastly,
    CloudFront,
    GCPCDN,
    AzureCDN,
    StackPath,
    KeyCDN,
    BunnyCDN,
    Sucuri,
    Imperva,
    Unknown(String),
}

impl fmt::Display for CdnProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CdnProvider::Cloudflare => write!(f, "Cloudflare"),
            CdnProvider::Akamai => write!(f, "Akamai"),
            CdnProvider::Fastly => write!(f, "Fastly"),
            CdnProvider::CloudFront => write!(f, "AWS CloudFront"),
            CdnProvider::GCPCDN => write!(f, "Google Cloud CDN"),
            CdnProvider::AzureCDN => write!(f, "Azure CDN"),
            CdnProvider::StackPath => write!(f, "StackPath"),
            CdnProvider::KeyCDN => write!(f, "KeyCDN"),
            CdnProvider::BunnyCDN => write!(f, "BunnyCDN"),
            CdnProvider::Sucuri => write!(f, "Sucuri"),
            CdnProvider::Imperva => write!(f, "Imperva"),
            CdnProvider::Unknown(s) => write!(f, "Unknown ({})", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CacheStatus {
    Hit,
    Miss,
    Bypass,
    Expired,
    Dynamic,
    Unknown,
}

impl fmt::Display for CacheStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheStatus::Hit => write!(f, "HIT"),
            CacheStatus::Miss => write!(f, "MISS"),
            CacheStatus::Bypass => write!(f, "BYPASS"),
            CacheStatus::Expired => write!(f, "EXPIRED"),
            CacheStatus::Dynamic => write!(f, "DYNAMIC"),
            CacheStatus::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CdnInfo {
    pub provider: CdnProvider,
    pub is_cdn: bool,
    pub edge_locations: Vec<String>,
    pub cache_status: Option<CacheStatus>,
    pub cache_hit_ratio: Option<f32>,
    pub ssl_enabled: bool,
    pub waf_enabled: bool,
    pub bot_protection: bool,
    pub http2_support: bool,
    pub http3_support: bool,
    pub ipv6_support: bool,
    pub confidence: Confidence,
    pub indicators: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl CdnInfo {
    pub fn new() -> Self {
        CdnInfo {
            provider: CdnProvider::Unknown("unknown".to_string()),
            is_cdn: false,
            edge_locations: vec![],
            cache_status: None,
            cache_hit_ratio: None,
            ssl_enabled: false,
            waf_enabled: false,
            bot_protection: false,
            http2_support: false,
            http3_support: false,
            ipv6_support: false,
            confidence: Confidence::Low,
            indicators: vec![],
            timestamp: Utc::now(),
        }
    }

    pub fn is_using_cdn(&self) -> bool {
        self.is_cdn && !matches!(self.provider, CdnProvider::Unknown(_))
    }

    pub fn performance_grade(&self) -> &str {
        if self.http3_support { return "A+"; }
        if self.http2_support { return "A"; }
        "C"
    }
}

// ============================================================
// LOAD BALANCER DETECTION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancerType {
    Hardware,
    Software,
    Cloud,
    Unknown,
}

impl fmt::Display for LoadBalancerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadBalancerType::Hardware => write!(f, "hardware"),
            LoadBalancerType::Software => write!(f, "software"),
            LoadBalancerType::Cloud => write!(f, "cloud"),
            LoadBalancerType::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancerAlgorithm {
    RoundRobin,
    LeastConnections,
    IPHash,
    WeightedRoundRobin,
    LeastTime,
    Random,
    Unknown,
}

impl fmt::Display for LoadBalancerAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadBalancerAlgorithm::RoundRobin => write!(f, "round_robin"),
            LoadBalancerAlgorithm::LeastConnections => write!(f, "least_connections"),
            LoadBalancerAlgorithm::IPHash => write!(f, "ip_hash"),
            LoadBalancerAlgorithm::WeightedRoundRobin => write!(f, "weighted_round_robin"),
            LoadBalancerAlgorithm::LeastTime => write!(f, "least_time"),
            LoadBalancerAlgorithm::Random => write!(f, "random"),
            LoadBalancerAlgorithm::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancerInfo {
    pub lb_type: LoadBalancerType,
    pub vendor: Option<String>,
    pub algorithm: LoadBalancerAlgorithm,
    pub backend_count: Option<usize>,
    pub sticky_session: bool,
    pub session_cookie_name: Option<String>,
    pub health_check_enabled: bool,
    pub ssl_termination: bool,
    pub detected_via: Vec<String>,
    pub confidence: Confidence,
    pub timestamp: DateTime<Utc>,
}

impl LoadBalancerInfo {
    pub fn new() -> Self {
        LoadBalancerInfo {
            lb_type: LoadBalancerType::Unknown,
            vendor: None,
            algorithm: LoadBalancerAlgorithm::Unknown,
            backend_count: None,
            sticky_session: false,
            session_cookie_name: None,
            health_check_enabled: false,
            ssl_termination: false,
            detected_via: vec![],
            confidence: Confidence::Low,
            timestamp: Utc::now(),
        }
    }

    pub fn is_load_balanced(&self) -> bool {
        !matches!(self.lb_type, LoadBalancerType::Unknown)
            || self.backend_count.unwrap_or(1) > 1
            || !self.detected_via.is_empty()
    }
}

// ============================================================
// HOSTING PROVIDER
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostingInfo {
    pub provider: Option<String>,
    pub isp: Option<String>,
    pub organization: Option<String>,
    pub asn: Option<u32>,
    pub asn_name: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub timezone: Option<String>,
    pub data_center: Option<String>,
    pub ip_range: Option<String>,
    pub hostname: Option<String>,
    pub is_cloud: bool,
    pub is_cdn: bool,
    pub is_hosting: bool,
    pub confidence: Confidence,
    pub timestamp: DateTime<Utc>,
}

impl HostingInfo {
    pub fn new() -> Self {
        HostingInfo {
            provider: None,
            isp: None,
            organization: None,
            asn: None,
            asn_name: None,
            country: None,
            city: None,
            latitude: None,
            longitude: None,
            timezone: None,
            data_center: None,
            ip_range: None,
            hostname: None,
            is_cloud: false,
            is_cdn: false,
            is_hosting: false,
            confidence: Confidence::Low,
            timestamp: Utc::now(),
        }
    }

    pub fn location_summary(&self) -> String {
        match (&self.city, &self.country) {
            (Some(city), Some(country)) => format!("{}, {}", city, country),
            (Some(city), None) => city.clone(),
            (None, Some(country)) => country.clone(),
            (None, None) => "Unknown location".to_string(),
        }
    }

    pub fn network_summary(&self) -> String {
        let mut summary = String::new();
        if let Some(ref isp) = self.isp { summary.push_str(&format!("ISP: {}\n", isp)); }
        if let Some(asn) = self.asn { summary.push_str(&format!("ASN: AS{}\n", asn)); }
        if let Some(ref org) = self.organization { summary.push_str(&format!("Org: {}\n", org)); }
        if summary.is_empty() { summary.push_str("No network info"); }
        summary
    }
}

// ============================================================
// INFRASTRUCTURE SUMMARY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureSummary {
    pub target_url: String,
    pub target_ip: Option<String>,
    pub server: ServerInfo,
    pub cloud: CloudDetection,
    pub cdn: CdnInfo,
    pub load_balancer: LoadBalancerInfo,
    pub hosting: HostingInfo,
    pub geo: Option<GeoLocation>,
    pub overall_confidence: Confidence,
    pub scan_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl InfrastructureSummary {
    pub fn new(target_url: &str) -> Self {
        InfrastructureSummary {
            target_url: target_url.to_string(),
            target_ip: None,
            server: ServerInfo::new(),
            cloud: CloudDetection::new(),
            cdn: CdnInfo::new(),
            load_balancer: LoadBalancerInfo::new(),
            hosting: HostingInfo::new(),
            geo: None,
            overall_confidence: Confidence::Low,
            scan_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn is_behind_cdn(&self) -> bool {
        self.cdn.is_using_cdn()
    }

    pub fn is_cloud_hosted(&self) -> bool {
        self.cloud.is_cloud_hosted()
    }

    pub fn has_load_balancer(&self) -> bool {
        self.load_balancer.is_load_balanced()
    }

    pub fn infrastructure_type(&self) -> InfrastructureType {
        if self.is_behind_cdn() && self.is_cloud_hosted() {
            InfrastructureType::CloudCdn
        } else if self.is_cloud_hosted() {
            InfrastructureType::Cloud
        } else if self.is_behind_cdn() {
            InfrastructureType::CdnOnly
        } else if self.has_load_balancer() {
            InfrastructureType::LoadBalanced
        } else {
            InfrastructureType::Standard
        }
    }

    pub fn brief_summary(&self) -> String {
        format!(
            "Server: {} | Cloud: {} | CDN: {} | Host: {} | Type: {}",
            self.server.full_identity(),
            self.cloud.provider,
            self.cdn.provider,
            self.hosting.location_summary(),
            self.infrastructure_type(),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InfrastructureType {
    Standard,
    Cloud,
    CdnOnly,
    CloudCdn,
    LoadBalanced,
    Unknown,
}

impl fmt::Display for InfrastructureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InfrastructureType::Standard => write!(f, "Standard"),
            InfrastructureType::Cloud => write!(f, "Cloud-hosted"),
            InfrastructureType::CdnOnly => write!(f, "CDN-fronted"),
            InfrastructureType::CloudCdn => write!(f, "Cloud + CDN"),
            InfrastructureType::LoadBalanced => write!(f, "Load-balanced"),
            InfrastructureType::Unknown => write!(f, "Unknown"),
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
    fn test_server_type_from_header() {
        assert_eq!(ServerType::from_header("Apache/2.4.49"), ServerType::Apache);
        assert_eq!(ServerType::from_header("nginx/1.24.0"), ServerType::Nginx);
        assert_eq!(ServerType::from_header("Microsoft-IIS/10.0"), ServerType::IIS);
        assert!(matches!(ServerType::from_header("UnknownServer/1.0"), ServerType::Unknown(_)));
    }

    #[test]
    fn test_server_type_is_common() {
        assert!(ServerType::Apache.is_common());
        assert!(ServerType::Nginx.is_common());
        assert!(ServerType::IIS.is_common());
        assert!(!ServerType::Caddy.is_common());
    }

    #[test]
    fn test_server_info_full_identity() {
        let mut info = ServerInfo::new();
        info.server_type = ServerType::Nginx;
        info.version = Some("1.24.0".to_string());
        info.os = Some("Ubuntu".to_string());
        assert!(info.full_identity().contains("Nginx 1.24.0 on Ubuntu"));
    }

    #[test]
    fn test_server_info_outdated() {
        let mut info = ServerInfo::new();
        info.server_type = ServerType::Apache;
        info.version = Some("1.3.0".to_string());
        assert!(info.is_outdated());

        info.version = Some("2.4.50".to_string());
        assert!(!info.is_outdated());
    }

    #[test]
    fn test_server_header_detection() {
        let header = ServerHeader::new("Server", "Apache/2.4.49");
        assert!(header.is_security_relevant);
        assert!(header.exposes_version);

        let header = ServerHeader::new("Content-Type", "text/html");
        assert!(!header.is_security_relevant);
    }

    #[test]
    fn test_cloud_detection_is_hosted() {
        let mut detection = CloudDetection::new();
        assert!(!detection.is_cloud_hosted());

        detection.provider = CloudProviderType::AWS;
        assert!(detection.is_cloud_hosted());
    }

    #[test]
    fn test_cloud_detection_has_service() {
        let mut detection = CloudDetection::new();
        detection.services.push(CloudService::new("EC2", CloudServiceType::Compute));
        detection.services.push(CloudService::new("S3", CloudServiceType::ObjectStorage));
        assert!(detection.has_service("EC2"));
        assert!(detection.has_service("s3"));
        assert!(!detection.has_service("Lambda"));
    }

    #[test]
    fn test_cdn_info_performance_grade() {
        let mut cdn = CdnInfo::new();
        assert_eq!(cdn.performance_grade(), "C");

        cdn.http2_support = true;
        assert_eq!(cdn.performance_grade(), "A");

        cdn.http3_support = true;
        assert_eq!(cdn.performance_grade(), "A+");
    }

    #[test]
    fn test_cdn_info_is_using() {
        let mut cdn = CdnInfo::new();
        assert!(!cdn.is_using_cdn());

        cdn.is_cdn = true;
        cdn.provider = CdnProvider::Cloudflare;
        assert!(cdn.is_using_cdn());
    }

    #[test]
    fn test_load_balancer_is_balanced() {
        let mut lb = LoadBalancerInfo::new();
        assert!(!lb.is_load_balanced());

        lb.backend_count = Some(3);
        assert!(lb.is_load_balanced());
    }

    #[test]
    fn test_hosting_info_location() {
        let mut hosting = HostingInfo::new();
        assert_eq!(hosting.location_summary(), "Unknown location");

        hosting.city = Some("Jakarta".to_string());
        hosting.country = Some("Indonesia".to_string());
        assert_eq!(hosting.location_summary(), "Jakarta, Indonesia");
    }

    #[test]
    fn test_hosting_info_network() {
        let mut hosting = HostingInfo::new();
        hosting.isp = Some("Telkom".to_string());
        hosting.asn = Some(7713);
        let summary = hosting.network_summary();
        assert!(summary.contains("Telkom"));
        assert!(summary.contains("7713"));
    }

    #[test]
    fn test_infrastructure_summary_type() {
        let mut summary = InfrastructureSummary::new("https://example.com");
        assert_eq!(summary.infrastructure_type(), InfrastructureType::Standard);

        summary.cloud.provider = CloudProviderType::AWS;
        assert_eq!(summary.infrastructure_type(), InfrastructureType::Cloud);

        summary.cdn.is_cdn = true;
        summary.cdn.provider = CdnProvider::Cloudflare;
        assert_eq!(summary.infrastructure_type(), InfrastructureType::CloudCdn);
    }

    #[test]
    fn test_infrastructure_summary_brief() {
        let mut summary = InfrastructureSummary::new("https://example.com");
        summary.server.server_type = ServerType::Nginx;
        summary.hosting.city = Some("Singapore".to_string());
        summary.hosting.country = Some("Singapore".to_string());
        let brief = summary.brief_summary();
        assert!(brief.contains("Nginx"));
        assert!(brief.contains("Singapore"));
        assert!(brief.contains("Standard"));
    }
}
