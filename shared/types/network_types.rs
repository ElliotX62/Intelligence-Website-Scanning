// shared/types/network_types.rs
// IWS v1.0 - Network Types
// Mendefinisikan tipe data untuk semua operasi jaringan

use std::fmt;
use std::net::IpAddr;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use super::common_types::{IpAddress, GeoLocation, Severity, Timestamp};

// ============================================================
// DNS RECORD
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DnsRecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SOA,
    SRV,
    PTR,
    CAA,
    DS,
    DNSKEY,
    RRSIG,
    NSEC,
    NSEC3,
    TLSA,
    SMIMEA,
    SSHFP,
    HIP,
    SPF,
    DMARC,
    DKIM,
    Unknown(String),
}

impl DnsRecordType {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "A" => DnsRecordType::A,
            "AAAA" => DnsRecordType::AAAA,
            "CNAME" => DnsRecordType::CNAME,
            "MX" => DnsRecordType::MX,
            "TXT" => DnsRecordType::TXT,
            "NS" => DnsRecordType::NS,
            "SOA" => DnsRecordType::SOA,
            "SRV" => DnsRecordType::SRV,
            "PTR" => DnsRecordType::PTR,
            "CAA" => DnsRecordType::CAA,
            "DS" => DnsRecordType::DS,
            "DNSKEY" => DnsRecordType::DNSKEY,
            "RRSIG" => DnsRecordType::RRSIG,
            "NSEC" => DnsRecordType::NSEC,
            "NSEC3" => DnsRecordType::NSEC3,
            "TLSA" => DnsRecordType::TLSA,
            "SMIMEA" => DnsRecordType::SMIMEA,
            "SSHFP" => DnsRecordType::SSHFP,
            "HIP" => DnsRecordType::HIP,
            "SPF" => DnsRecordType::SPF,
            "DMARC" => DnsRecordType::DMARC,
            "DKIM" => DnsRecordType::DKIM,
            other => DnsRecordType::Unknown(other.to_string()),
        }
    }

    pub fn is_common(&self) -> bool {
        matches!(
            self,
            DnsRecordType::A
                | DnsRecordType::AAAA
                | DnsRecordType::CNAME
                | DnsRecordType::MX
                | DnsRecordType::TXT
                | DnsRecordType::NS
                | DnsRecordType::SOA
        )
    }
}

impl fmt::Display for DnsRecordType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DnsRecordType::A => write!(f, "A"),
            DnsRecordType::AAAA => write!(f, "AAAA"),
            DnsRecordType::CNAME => write!(f, "CNAME"),
            DnsRecordType::MX => write!(f, "MX"),
            DnsRecordType::TXT => write!(f, "TXT"),
            DnsRecordType::NS => write!(f, "NS"),
            DnsRecordType::SOA => write!(f, "SOA"),
            DnsRecordType::SRV => write!(f, "SRV"),
            DnsRecordType::PTR => write!(f, "PTR"),
            DnsRecordType::CAA => write!(f, "CAA"),
            DnsRecordType::DS => write!(f, "DS"),
            DnsRecordType::DNSKEY => write!(f, "DNSKEY"),
            DnsRecordType::RRSIG => write!(f, "RRSIG"),
            DnsRecordType::NSEC => write!(f, "NSEC"),
            DnsRecordType::NSEC3 => write!(f, "NSEC3"),
            DnsRecordType::TLSA => write!(f, "TLSA"),
            DnsRecordType::SMIMEA => write!(f, "SMIMEA"),
            DnsRecordType::SSHFP => write!(f, "SSHFP"),
            DnsRecordType::HIP => write!(f, "HIP"),
            DnsRecordType::SPF => write!(f, "SPF"),
            DnsRecordType::DMARC => write!(f, "DMARC"),
            DnsRecordType::DKIM => write!(f, "DKIM"),
            DnsRecordType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecord {
    pub record_type: DnsRecordType,
    pub name: String,
    pub value: String,
    pub ttl: u32,
    pub priority: Option<u16>,
    pub class: String,
    pub raw_data: Option<Vec<u8>>,
}

impl DnsRecord {
    pub fn new(record_type: DnsRecordType, name: &str, value: &str, ttl: u32) -> Self {
        DnsRecord {
            record_type,
            name: name.to_string(),
            value: value.to_string(),
            ttl,
            priority: None,
            class: "IN".to_string(),
            raw_data: None,
        }
    }

    pub fn with_priority(mut self, priority: u16) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn is_expired(&self, reference_time: DateTime<Utc>) -> bool {
        // TTL-based expiration tidak bisa dicek tanpa timestamp saat record didapat
        // Placeholder untuk implementasi dengan cache timestamp
        false
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsEnumResult {
    pub domain: String,
    pub records: Vec<DnsRecord>,
    pub nameservers: Vec<String>,
    pub zone_transfer_possible: bool,
    pub dnssec_enabled: bool,
    pub query_time_ms: u64,
    pub errors: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl DnsEnumResult {
    pub fn new(domain: &str) -> Self {
        DnsEnumResult {
            domain: domain.to_string(),
            records: vec![],
            nameservers: vec![],
            zone_transfer_possible: false,
            dnssec_enabled: false,
            query_time_ms: 0,
            errors: vec![],
            timestamp: Utc::now(),
        }
    }

    pub fn get_records_by_type(&self, record_type: &DnsRecordType) -> Vec<&DnsRecord> {
        self.records
            .iter()
            .filter(|r| r.record_type == *record_type)
            .collect()
    }

    pub fn get_a_records(&self) -> Vec<&DnsRecord> {
        self.get_records_by_type(&DnsRecordType::A)
    }

    pub fn get_mx_records(&self) -> Vec<&DnsRecord> {
        self.get_records_by_type(&DnsRecordType::MX)
    }

    pub fn get_txt_records(&self) -> Vec<&DnsRecord> {
        self.get_records_by_type(&DnsRecordType::TXT)
    }

    pub fn total_records(&self) -> usize {
        self.records.len()
    }
}

// ============================================================
// PORT SCAN
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
    Both,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::TCP => write!(f, "tcp"),
            Protocol::UDP => write!(f, "udp"),
            Protocol::Both => write!(f, "both"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PortState {
    Open,
    Closed,
    Filtered,
    OpenFiltered,
    Unfiltered,
}

impl fmt::Display for PortState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PortState::Open => write!(f, "open"),
            PortState::Closed => write!(f, "closed"),
            PortState::Filtered => write!(f, "filtered"),
            PortState::OpenFiltered => write!(f, "open|filtered"),
            PortState::Unfiltered => write!(f, "unfiltered"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanResult {
    pub port: u16,
    pub protocol: Protocol,
    pub state: PortState,
    pub service: Option<String>,
    pub version: Option<String>,
    pub banner: Option<String>,
    pub product: Option<String>,
    pub os_type: Option<String>,
    pub device_type: Option<String>,
    pub cpe: Option<String>,
    pub confidence: f32,
}

impl PortScanResult {
    pub fn new(port: u16, protocol: Protocol, state: PortState) -> Self {
        PortScanResult {
            port,
            protocol,
            state,
            service: None,
            version: None,
            banner: None,
            product: None,
            os_type: None,
            device_type: None,
            cpe: None,
            confidence: 0.5,
        }
    }

    pub fn with_service(mut self, service: &str) -> Self {
        self.service = Some(service.to_string());
        self
    }

    pub fn with_version(mut self, version: &str) -> Self {
        self.version = Some(version.to_string());
        self
    }

    pub fn with_banner(mut self, banner: &str) -> Self {
        self.banner = Some(banner.to_string());
        self
    }

    pub fn is_common_service(&self) -> bool {
        matches!(
            self.port,
            21 | 22 | 23 | 25 | 53 | 80 | 110 | 143 | 443 | 993 | 995 | 3306 | 3389 | 5432 | 6379 | 8080 | 8443 | 27017
        )
    }

    pub fn risk_level(&self) -> Severity {
        match self.state {
            PortState::Open => {
                if self.port <= 1024 {
                    Severity::Medium
                } else {
                    Severity::Low
                }
            }
            PortState::Filtered => Severity::Info,
            _ => Severity::Info,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortScanSummary {
    pub target_ip: String,
    pub ports_scanned: usize,
    pub ports_open: usize,
    pub ports_closed: usize,
    pub ports_filtered: usize,
    pub results: Vec<PortScanResult>,
    pub scan_duration_ms: u64,
    pub scan_type: ScanType,
    pub timestamp: DateTime<Utc>,
}

impl PortScanSummary {
    pub fn new(target_ip: &str) -> Self {
        PortScanSummary {
            target_ip: target_ip.to_string(),
            ports_scanned: 0,
            ports_open: 0,
            ports_closed: 0,
            ports_filtered: 0,
            results: vec![],
            scan_duration_ms: 0,
            scan_type: ScanType::TcpConnect,
            timestamp: Utc::now(),
        }
    }

    pub fn open_ports(&self) -> Vec<&PortScanResult> {
        self.results
            .iter()
            .filter(|r| r.state == PortState::Open)
            .collect()
    }

    pub fn common_services(&self) -> Vec<&PortScanResult> {
        self.results.iter().filter(|r| r.is_common_service()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanType {
    TcpConnect,
    TcpSyn,
    UdpScan,
    FinScan,
    NullScan,
    XmasScan,
    AckScan,
    WindowScan,
    MaimonScan,
}

impl fmt::Display for ScanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanType::TcpConnect => write!(f, "tcp_connect"),
            ScanType::TcpSyn => write!(f, "tcp_syn"),
            ScanType::UdpScan => write!(f, "udp"),
            ScanType::FinScan => write!(f, "fin"),
            ScanType::NullScan => write!(f, "null"),
            ScanType::XmasScan => write!(f, "xmas"),
            ScanType::AckScan => write!(f, "ack"),
            ScanType::WindowScan => write!(f, "window"),
            ScanType::MaimonScan => write!(f, "maimon"),
        }
    }
}

// ============================================================
// WHOIS DATA
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

impl Contact {
    pub fn new() -> Self {
        Contact {
            name: None,
            organization: None,
            email: None,
            phone: None,
            fax: None,
            address: None,
            city: None,
            state: None,
            postal_code: None,
            country: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.organization.is_none()
            && self.email.is_none()
    }

    pub fn format_short(&self) -> String {
        match (&self.name, &self.organization) {
            (Some(name), Some(org)) => format!("{} ({})", name, org),
            (Some(name), None) => name.clone(),
            (None, Some(org)) => org.clone(),
            (None, None) => "Unknown".to_string(),
        }
    }
}

impl Default for Contact {
    fn default() -> Self {
        Contact::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhoisData {
    pub domain: String,
    pub registrar: Option<String>,
    pub creation_date: Option<DateTime<Utc>>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub updated_date: Option<DateTime<Utc>>,
    pub nameservers: Vec<String>,
    pub registrant: Option<Contact>,
    pub admin_contact: Option<Contact>,
    pub tech_contact: Option<Contact>,
    pub status: Vec<String>,
    pub dnssec: Option<String>,
    pub raw_text: Option<String>,
    pub query_time_ms: u64,
    pub is_redacted: bool,
    pub timestamp: DateTime<Utc>,
}

impl WhoisData {
    pub fn new(domain: &str) -> Self {
        WhoisData {
            domain: domain.to_string(),
            registrar: None,
            creation_date: None,
            expiration_date: None,
            updated_date: None,
            nameservers: vec![],
            registrant: None,
            admin_contact: None,
            tech_contact: None,
            status: vec![],
            dnssec: None,
            raw_text: None,
            query_time_ms: 0,
            is_redacted: false,
            timestamp: Utc::now(),
        }
    }

    pub fn domain_age_days(&self) -> Option<i64> {
        self.creation_date.map(|d| {
            (Utc::now() - d).num_days()
        })
    }

    pub fn days_until_expiry(&self) -> Option<i64> {
        self.expiration_date.map(|d| {
            (d - Utc::now()).num_days()
        })
    }

    pub fn is_expired(&self) -> bool {
        self.days_until_expiry()
            .map(|d| d <= 0)
            .unwrap_or(false)
    }

    pub fn is_expiring_soon(&self, days: i64) -> bool {
        self.days_until_expiry()
            .map(|d| d > 0 && d <= days)
            .unwrap_or(false)
    }
}

// ============================================================
// TRACEROUTE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteHop {
    pub hop_number: u32,
    pub ip: IpAddr,
    pub hostname: Option<String>,
    pub rtt_ms: Vec<f64>,
    pub avg_rtt_ms: f64,
    pub min_rtt_ms: f64,
    pub max_rtt_ms: f64,
    pub location: Option<GeoLocation>,
    pub asn: Option<u32>,
    pub asn_org: Option<String>,
    pub packet_loss_percent: f64,
    pub timestamp: DateTime<Utc>,
}

impl TracerouteHop {
    pub fn new(hop_number: u32, ip: IpAddr) -> Self {
        TracerouteHop {
            hop_number,
            ip,
            hostname: None,
            rtt_ms: vec![],
            avg_rtt_ms: 0.0,
            min_rtt_ms: 0.0,
            max_rtt_ms: 0.0,
            location: None,
            asn: None,
            asn_org: None,
            packet_loss_percent: 0.0,
            timestamp: Utc::now(),
        }
    }

    pub fn add_rtt(&mut self, rtt_ms: f64) {
        self.rtt_ms.push(rtt_ms);
        let sum: f64 = self.rtt_ms.iter().sum();
        let count = self.rtt_ms.len() as f64;
        self.avg_rtt_ms = sum / count;
        self.min_rtt_ms = self.rtt_ms.iter().cloned().fold(f64::MAX, f64::min);
        self.max_rtt_ms = self.rtt_ms.iter().cloned().fold(0.0, f64::max);
    }

    pub fn is_timeout(&self) -> bool {
        self.rtt_ms.is_empty()
    }

    pub fn format_short(&self) -> String {
        if self.is_timeout() {
            return format!("{}: *", self.hop_number);
        }
        match &self.hostname {
            Some(host) => format!("{}: {} ({}) {:.1}ms", self.hop_number, self.ip, host, self.avg_rtt_ms),
            None => format!("{}: {} {:.1}ms", self.hop_number, self.ip, self.avg_rtt_ms),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracerouteResult {
    pub target: String,
    pub target_ip: IpAddr,
    pub hops: Vec<TracerouteHop>,
    pub total_hops: usize,
    pub reached_target: bool,
    pub total_time_ms: u64,
    pub max_hops: u32,
    pub protocol: TraceProtocol,
    pub timestamp: DateTime<Utc>,
}

impl TracerouteResult {
    pub fn new(target: &str, target_ip: IpAddr, max_hops: u32) -> Self {
        TracerouteResult {
            target: target.to_string(),
            target_ip,
            hops: vec![],
            total_hops: 0,
            reached_target: false,
            total_time_ms: 0,
            max_hops,
            protocol: TraceProtocol::UDP,
            timestamp: Utc::now(),
        }
    }

    pub fn add_hop(&mut self, hop: TracerouteHop) {
        self.total_hops = hop.hop_number as usize;
        self.hops.push(hop);
    }

    pub fn average_rtt(&self) -> f64 {
        let valid_hops: Vec<&TracerouteHop> = self.hops.iter().filter(|h| !h.is_timeout()).collect();
        if valid_hops.is_empty() {
            return 0.0;
        }
        let sum: f64 = valid_hops.iter().map(|h| h.avg_rtt_ms).sum();
        sum / valid_hops.len() as f64
    }

    pub fn packet_loss_percent(&self) -> f64 {
        let timeouts = self.hops.iter().filter(|h| h.is_timeout()).count();
        if self.hops.is_empty() {
            return 0.0;
        }
        (timeouts as f64 / self.hops.len() as f64) * 100.0
    }

    pub fn to_ascii_visualization(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("Traceroute to {} ({})\n", self.target, self.target_ip));
        output.push_str(&format!("Max hops: {}, Protocol: {}\n", self.max_hops, self.protocol));
        for hop in &self.hops {
            if hop.is_timeout() {
                output.push_str(&format!("{:2d}. * * *\n", hop.hop_number));
            } else {
                output.push_str(&format!(
                    "{:2d}. {} ({}) {:.1}ms {:.1}ms {:.1}ms\n",
                    hop.hop_number,
                    hop.ip,
                    hop.hostname.as_deref().unwrap_or("unknown"),
                    hop.min_rtt_ms,
                    hop.avg_rtt_ms,
                    hop.max_rtt_ms,
                ));
            }
        }
        output
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraceProtocol {
    ICMP,
    UDP,
    TCP,
}

impl fmt::Display for TraceProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TraceProtocol::ICMP => write!(f, "icmp"),
            TraceProtocol::UDP => write!(f, "udp"),
            TraceProtocol::TCP => write!(f, "tcp"),
        }
    }
}

// ============================================================
// SSL/TLS CERTIFICATE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyType {
    RSA,
    ECDSA,
    Ed25519,
    Unknown(String),
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::RSA => write!(f, "RSA"),
            KeyType::ECDSA => write!(f, "ECDSA"),
            KeyType::Ed25519 => write!(f, "Ed25519"),
            KeyType::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherSuite {
    pub name: String,
    pub protocol: String,
    pub key_exchange: String,
    pub encryption: String,
    pub mac: String,
    pub strength: CipherStrength,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CipherStrength {
    Weak,
    Medium,
    Strong,
    Unknown,
}

impl fmt::Display for CipherStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherStrength::Weak => write!(f, "weak"),
            CipherStrength::Medium => write!(f, "medium"),
            CipherStrength::Strong => write!(f, "strong"),
            CipherStrength::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslCertificate {
    pub subject: String,
    pub common_name: String,
    pub issuer: String,
    pub issuer_country: Option<String>,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub san: Vec<String>,
    pub serial_number: String,
    pub fingerprint_sha1: String,
    pub fingerprint_sha256: String,
    pub key_type: KeyType,
    pub key_size: u16,
    pub signature_algorithm: String,
    pub cipher_suites: Vec<CipherSuite>,
    pub protocols: Vec<String>,
    pub is_valid: bool,
    pub is_self_signed: bool,
    pub is_wildcard: bool,
    pub is_expired: bool,
    pub is_revoked: bool,
    pub has_weak_ciphers: bool,
    pub has_weak_protocols: bool,
    pub chain_length: usize,
    pub cert_chain: Vec<CertificateChainEntry>,
    pub rating: Option<CertRating>,
    pub raw_pem: Option<String>,
    pub query_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl SslCertificate {
    pub fn new(subject: &str, common_name: &str, issuer: &str) -> Self {
        SslCertificate {
            subject: subject.to_string(),
            common_name: common_name.to_string(),
            issuer: issuer.to_string(),
            issuer_country: None,
            valid_from: Utc::now(),
            valid_to: Utc::now(),
            san: vec![],
            serial_number: String::new(),
            fingerprint_sha1: String::new(),
            fingerprint_sha256: String::new(),
            key_type: KeyType::Unknown("unknown".to_string()),
            key_size: 0,
            signature_algorithm: String::new(),
            cipher_suites: vec![],
            protocols: vec![],
            is_valid: true,
            is_self_signed: false,
            is_wildcard: false,
            is_expired: false,
            is_revoked: false,
            has_weak_ciphers: false,
            has_weak_protocols: false,
            chain_length: 0,
            cert_chain: vec![],
            rating: None,
            raw_pem: None,
            query_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn days_until_expiry(&self) -> i64 {
        (self.valid_to - Utc::now()).num_days()
    }

    pub fn is_expiring_soon(&self, days: i64) -> bool {
        let remaining = self.days_until_expiry();
        remaining > 0 && remaining <= days
    }

    pub fn calculate_rating(&mut self) -> CertRating {
        let mut score = 100u8;

        if self.is_expired { score = score.saturating_sub(50); }
        if self.is_self_signed { score = score.saturating_sub(20); }
        if self.is_revoked { score = score.saturating_sub(50); }
        if self.has_weak_ciphers { score = score.saturating_sub(15); }
        if self.has_weak_protocols { score = score.saturating_sub(15); }
        if self.key_size < 2048 && matches!(self.key_type, KeyType::RSA) {
            score = score.saturating_sub(20);
        }
        if self.days_until_expiry() < 30 && !self.is_expired {
            score = score.saturating_sub(10);
        }

        let grade = match score {
            90..=100 => CertGrade::A,
            80..=89 => CertGrade::B,
            65..=79 => CertGrade::C,
            50..=64 => CertGrade::D,
            _ => CertGrade::F,
        };

        let rating = CertRating {
            score,
            grade,
            issues: vec![],
            recommendations: vec![],
        };

        self.rating = Some(rating.clone());
        rating
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateChainEntry {
    pub subject: String,
    pub issuer: String,
    pub valid_from: DateTime<Utc>,
    pub valid_to: DateTime<Utc>,
    pub fingerprint: String,
    pub is_ca: bool,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertRating {
    pub score: u8,
    pub grade: CertGrade,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum CertGrade {
    A,
    B,
    C,
    D,
    F,
}

impl fmt::Display for CertGrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CertGrade::A => write!(f, "A"),
            CertGrade::B => write!(f, "B"),
            CertGrade::C => write!(f, "C"),
            CertGrade::D => write!(f, "D"),
            CertGrade::F => write!(f, "F"),
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
    fn test_dns_record_type_from_str() {
        assert_eq!(DnsRecordType::from_str("A"), DnsRecordType::A);
        assert_eq!(DnsRecordType::from_str("aaaa"), DnsRecordType::AAAA);
        assert_eq!(DnsRecordType::from_str("unknown"), DnsRecordType::Unknown("unknown".to_string()));
    }

    #[test]
    fn test_dns_record_type_is_common() {
        assert!(DnsRecordType::A.is_common());
        assert!(DnsRecordType::MX.is_common());
        assert!(!DnsRecordType::SSHFP.is_common());
    }

    #[test]
    fn test_dns_enum_result_filtering() {
        let mut result = DnsEnumResult::new("example.com");
        result.records.push(DnsRecord::new(DnsRecordType::A, "example.com", "93.184.216.34", 300));
        result.records.push(DnsRecord::new(DnsRecordType::MX, "example.com", "mail.example.com", 3600).with_priority(10));
        result.records.push(DnsRecord::new(DnsRecordType::TXT, "example.com", "v=spf1 -all", 3600));

        assert_eq!(result.get_a_records().len(), 1);
        assert_eq!(result.get_mx_records().len(), 1);
        assert_eq!(result.total_records(), 3);
    }

    #[test]
    fn test_port_scan_result_helpers() {
        let result = PortScanResult::new(443, Protocol::TCP, PortState::Open)
            .with_service("https")
            .with_version("nginx/1.24.0");

        assert!(result.is_common_service());
        assert_eq!(result.risk_level(), Severity::Medium);
    }

    #[test]
    fn test_port_scan_summary() {
        let mut summary = PortScanSummary::new("192.168.1.1");
        summary.results.push(PortScanResult::new(80, Protocol::TCP, PortState::Open));
        summary.results.push(PortScanResult::new(443, Protocol::TCP, PortState::Open));
        summary.results.push(PortScanResult::new(22, Protocol::TCP, PortState::Closed));

        assert_eq!(summary.open_ports().len(), 2);
    }

    #[test]
    fn test_whois_domain_age() {
        let mut whois = WhoisData::new("example.com");
        whois.creation_date = Some(Utc::now() - chrono::Duration::days(365 * 5));
        assert!(whois.domain_age_days().unwrap() >= 365 * 5);
    }

    #[test]
    fn test_whois_expiry_checks() {
        let mut whois = WhoisData::new("example.com");
        whois.expiration_date = Some(Utc::now() + chrono::Duration::days(15));
        assert!(!whois.is_expired());
        assert!(whois.is_expiring_soon(30));

        whois.expiration_date = Some(Utc::now() - chrono::Duration::days(1));
        assert!(whois.is_expired());
    }

    #[test]
    fn test_contact_format() {
        let mut contact = Contact::new();
        assert_eq!(contact.format_short(), "Unknown");

        contact.name = Some("John Doe".to_string());
        assert_eq!(contact.format_short(), "John Doe");

        contact.organization = Some("ACME Corp".to_string());
        assert_eq!(contact.format_short(), "John Doe (ACME Corp)");
    }

    #[test]
    fn test_traceroute_hop_add_rtt() {
        let mut hop = TracerouteHop::new(1, "8.8.8.8".parse().unwrap());
        hop.add_rtt(10.5);
        hop.add_rtt(12.3);
        hop.add_rtt(11.1);

        assert!((hop.avg_rtt_ms - 11.3).abs() < 0.1);
        assert!((hop.min_rtt_ms - 10.5).abs() < 0.01);
        assert!((hop.max_rtt_ms - 12.3).abs() < 0.01);
    }

    #[test]
    fn test_traceroute_result() {
        let target_ip: IpAddr = "8.8.8.8".parse().unwrap();
        let mut result = TracerouteResult::new("google.com", target_ip, 30);

        let mut hop = TracerouteHop::new(1, "192.168.1.1".parse().unwrap());
        hop.add_rtt(1.5);
        result.add_hop(hop);

        assert_eq!(result.total_hops, 1);
        assert!(result.average_rtt() > 0.0);
    }

    #[test]
    fn test_ssl_certificate_rating() {
        let mut cert = SslCertificate::new("CN=example.com", "example.com", "CN=CA");
        cert.key_type = KeyType::RSA;
        cert.key_size = 2048;
        cert.valid_to = Utc::now() + chrono::Duration::days(365);
        let rating = cert.calculate_rating();
        assert!(rating.score >= 90);
        assert_eq!(rating.grade, CertGrade::A);
    }

    #[test]
    fn test_ssl_certificate_expired_rating() {
        let mut cert = SslCertificate::new("CN=example.com", "example.com", "CN=CA");
        cert.is_expired = true;
        cert.valid_to = Utc::now() - chrono::Duration::days(30);
        let rating = cert.calculate_rating();
        assert!(rating.score <= 50);
        assert_eq!(rating.grade, CertGrade::F);
    }

    #[test]
    fn test_ssl_certificate_weak_cipher_rating() {
        let mut cert = SslCertificate::new("CN=example.com", "example.com", "CN=CA");
        cert.has_weak_ciphers = true;
        cert.has_weak_protocols = true;
        cert.key_type = KeyType::RSA;
        cert.key_size = 2048;
        cert.valid_to = Utc::now() + chrono::Duration::days(365);
        let rating = cert.calculate_rating();
        assert!(rating.score <= 70);
    }
}
