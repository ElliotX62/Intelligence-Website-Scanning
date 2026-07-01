// shared/types/storage_types.rs
// IWS v1.0 - Storage Types
// Mendefinisikan tipe data untuk storage management

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================
// STORAGE KEY & VALUE (REDEFINED DARI CONTRACT UNTUK TYPE SAFETY)
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StorageKey {
    pub namespace: String,
    pub id: String,
    pub version: Option<u32>,
    pub shard_key: Option<String>,
}

impl StorageKey {
    pub fn new(namespace: &str, id: &str) -> Self {
        StorageKey {
            namespace: namespace.to_string(),
            id: id.to_string(),
            version: None,
            shard_key: None,
        }
    }

    pub fn with_version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_shard(mut self, shard_key: &str) -> Self {
        self.shard_key = Some(shard_key.to_string());
        self
    }

    pub fn to_string_key(&self) -> String {
        let mut s = format!("{}:{}", self.namespace, self.id);
        if let Some(v) = self.version {
            s.push_str(&format!(":v{}", v));
        }
        s
    }

    pub fn shard_id(&self) -> String {
        self.shard_key.clone().unwrap_or_else(|| self.namespace.clone())
    }
}

impl fmt::Display for StorageKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_key())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageValue {
    pub data: Vec<u8>,
    pub content_type: String,
    pub encoding: StorageEncoding,
    pub compression: CompressionType,
    pub encrypted: bool,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
    pub checksum: Option<String>,
    pub tags: HashMap<String, String>,
    pub metadata: serde_json::Value,
    pub ttl_seconds: Option<u64>,
}

impl StorageValue {
    pub fn new(data: Vec<u8>, content_type: &str) -> Self {
        let now = Utc::now();
        let size = data.len() as u64;
        StorageValue {
            data,
            content_type: content_type.to_string(),
            encoding: StorageEncoding::Raw,
            compression: CompressionType::None,
            encrypted: false,
            size_bytes: size,
            created_at: now,
            updated_at: now,
            version: 1,
            checksum: None,
            tags: HashMap::new(),
            metadata: serde_json::json!({}),
            ttl_seconds: None,
        }
    }

    pub fn with_ttl(mut self, seconds: u64) -> Self {
        self.ttl_seconds = Some(seconds);
        self
    }

    pub fn with_tag(mut self, key: &str, value: &str) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }

    pub fn is_expired(&self) -> bool {
        if let Some(ttl) = self.ttl_seconds {
            let elapsed = Utc::now() - self.created_at;
            elapsed.num_seconds() as u64 > ttl
        } else {
            false
        }
    }

    pub fn age_seconds(&self) -> i64 {
        (Utc::now() - self.created_at).num_seconds()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageEncoding {
    Raw,
    Base64,
    Hex,
    Utf8,
    Binary,
}

impl fmt::Display for StorageEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageEncoding::Raw => write!(f, "raw"),
            StorageEncoding::Base64 => write!(f, "base64"),
            StorageEncoding::Hex => write!(f, "hex"),
            StorageEncoding::Utf8 => write!(f, "utf8"),
            StorageEncoding::Binary => write!(f, "binary"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionType {
    None,
    Gzip,
    Zstd,
    Lz4,
    Snappy,
    Brotli,
}

impl fmt::Display for CompressionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionType::None => write!(f, "none"),
            CompressionType::Gzip => write!(f, "gzip"),
            CompressionType::Zstd => write!(f, "zstd"),
            CompressionType::Lz4 => write!(f, "lz4"),
            CompressionType::Snappy => write!(f, "snappy"),
            CompressionType::Brotli => write!(f, "brotli"),
        }
    }
}

// ============================================================
// VERSION METADATA
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionMetadata {
    pub version: u32,
    pub timestamp: DateTime<Utc>,
    pub author: String,
    pub message: String,
    pub size_bytes: u64,
    pub checksum: Option<String>,
    pub is_deleted: bool,
}

impl VersionMetadata {
    pub fn new(version: u32, author: &str, message: &str, size_bytes: u64) -> Self {
        VersionMetadata {
            version,
            timestamp: Utc::now(),
            author: author.to_string(),
            message: message.to_string(),
            size_bytes,
            checksum: None,
            is_deleted: false,
        }
    }
}

// ============================================================
// STORAGE QUERY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageQuery {
    pub namespace: Option<String>,
    pub filters: Vec<QueryFilter>,
    pub sort: Option<SortConfig>,
    pub pagination: Option<PaginationConfig>,
    pub include_deleted: bool,
    pub include_expired: bool,
    pub consistency: ConsistencyLevel,
}

impl StorageQuery {
    pub fn new() -> Self {
        StorageQuery {
            namespace: None,
            filters: vec![],
            sort: None,
            pagination: None,
            include_deleted: false,
            include_expired: false,
            consistency: ConsistencyLevel::Strong,
        }
    }

    pub fn with_namespace(mut self, namespace: &str) -> Self {
        self.namespace = Some(namespace.to_string());
        self
    }

    pub fn with_filter(mut self, filter: QueryFilter) -> Self {
        self.filters.push(filter);
        self
    }
}

impl Default for StorageQuery {
    fn default() -> Self {
        StorageQuery::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

impl QueryFilter {
    pub fn eq(field: &str, value: serde_json::Value) -> Self {
        QueryFilter {
            field: field.to_string(),
            operator: FilterOperator::Equals,
            value,
        }
    }

    pub fn contains(field: &str, value: &str) -> Self {
        QueryFilter {
            field: field.to_string(),
            operator: FilterOperator::Contains,
            value: serde_json::json!(value),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
    In,
    Between,
    IsNull,
    Regex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortConfig {
    pub field: String,
    pub direction: SortDirection,
}

impl SortConfig {
    pub fn ascending(field: &str) -> Self {
        SortConfig { field: field.to_string(), direction: SortDirection::Ascending }
    }

    pub fn descending(field: &str) -> Self {
        SortConfig { field: field.to_string(), direction: SortDirection::Descending }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationConfig {
    pub page: u32,
    pub page_size: u32,
}

impl PaginationConfig {
    pub fn new(page: u32, page_size: u32) -> Self {
        PaginationConfig { page: page.max(1), page_size: page_size.min(1000) }
    }

    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.page_size
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsistencyLevel {
    Strong,
    Eventual,
    ReadAfterWrite,
}

// ============================================================
// STORAGE STATISTICS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_keys: u64,
    pub total_size_bytes: u64,
    pub namespace_counts: HashMap<String, u64>,
    pub read_ops: u64,
    pub write_ops: u64,
    pub delete_ops: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub average_read_latency_us: f64,
    pub average_write_latency_us: f64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f64,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
    pub last_compaction: Option<DateTime<Utc>>,
    pub last_backup: Option<DateTime<Utc>>,
}

impl StorageStats {
    pub fn new() -> Self {
        StorageStats {
            total_keys: 0,
            total_size_bytes: 0,
            namespace_counts: HashMap::new(),
            read_ops: 0,
            write_ops: 0,
            delete_ops: 0,
            cache_hits: 0,
            cache_misses: 0,
            average_read_latency_us: 0.0,
            average_write_latency_us: 0.0,
            compressed_size_bytes: 0,
            compression_ratio: 0.0,
            oldest_entry: None,
            newest_entry: None,
            last_compaction: None,
            last_backup: None,
        }
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 { 0.0 } else { self.cache_hits as f64 / total as f64 }
    }

    pub fn namespace_distribution(&self) -> Vec<(String, u64)> {
        let mut vec: Vec<(String, u64)> = self.namespace_counts.iter().map(|(k, v)| (k.clone(), *v)).collect();
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        vec
    }
}

// ============================================================
// BACKUP & RESTORE TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub backup_path: String,
    pub include_data: bool,
    pub include_cache: bool,
    pub include_logs: bool,
    pub compress: bool,
    pub compression_type: CompressionType,
    pub encrypt: bool,
    pub encryption_key_path: Option<String>,
    pub upload_to_cloud: bool,
    pub cloud_provider: Option<CloudProviderType>,
    pub cloud_bucket: Option<String>,
    pub schedule_cron: Option<String>,
    pub retention_days: u32,
}

impl Default for BackupConfig {
    fn default() -> Self {
        BackupConfig {
            backup_path: "./data/backups/".to_string(),
            include_data: true,
            include_cache: false,
            include_logs: true,
            compress: true,
            compression_type: CompressionType::Gzip,
            encrypt: true,
            encryption_key_path: None,
            upload_to_cloud: false,
            cloud_provider: None,
            cloud_bucket: None,
            schedule_cron: None,
            retention_days: 90,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudProviderType {
    AwsS3,
    Gcs,
    Azure,
    Minio,
    Custom(String),
}

// ============================================================
// BackupInfo — PRIMARY STRUCT (replaces BackupMetadata)
// M4 FIX: selaras dengan storage_contract.rs BackupInfo + fields tambahan
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub backup_id: Uuid,
    pub size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub location: BackupLocation,
    pub status: BackupStatus,
    pub checksum: Option<String>,
    pub checksum_sha256: Option<String>,
    pub encrypted: bool,
    pub encryption_used: Option<String>,
    pub compression: CompressionType,
    pub compression_used: Option<CompressionType>,
    pub file_count: usize,
    pub tags: Vec<String>,
    pub notes: Option<String>,
    pub created_by: String,
    pub version: String,
    pub metadata: serde_json::Value,
}

impl BackupInfo {
    pub fn new(backup_id: Uuid, created_by: &str) -> Self {
        BackupInfo {
            backup_id,
            created_at: Utc::now(),
            size_bytes: 0,
            compressed_size_bytes: 0,
            location: BackupLocation::Local("./data/backups/".to_string()),
            status: BackupStatus::Pending,
            checksum: None,
            checksum_sha256: None,
            encrypted: false,
            encryption_used: None,
            compression: CompressionType::None,
            compression_used: None,
            file_count: 0,
            tags: vec![],
            notes: None,
            created_by: created_by.to_string(),
            version: "1.0.0".to_string(),
            metadata: serde_json::json!({}),
        }
    }

    pub fn compression_ratio(&self) -> f64 {
        if self.size_bytes == 0 { 0.0 }
        else { 1.0 - (self.compressed_size_bytes as f64 / self.size_bytes as f64) }
    }

    pub fn is_complete(&self) -> bool {
        self.status == BackupStatus::Complete
    }

    pub fn is_failed(&self) -> bool {
        self.status == BackupStatus::Failed
    }

    pub fn age_days(&self) -> i64 {
        (Utc::now() - self.created_at).num_days()
    }

    /// Konversi ke format ringkasan untuk logging
    pub fn to_summary(&self) -> String {
        format!(
            "Backup {} | {} | {} | {} | ratio={:.1}%",
            self.backup_id,
            self.status,
            self.created_at.format("%Y-%m-%d %H:%M"),
            self.location,
            self.compression_ratio() * 100.0,
        )
    }
}

// Type alias untuk backward compatibility
pub type BackupMetadata = BackupInfo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupStatus {
    Pending,
    InProgress,
    Complete,
    Failed,
    Restoring,
    Restored,
    Expired,
    Deleted,
}

impl fmt::Display for BackupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupStatus::Pending => write!(f, "pending"),
            BackupStatus::InProgress => write!(f, "in_progress"),
            BackupStatus::Complete => write!(f, "complete"),
            BackupStatus::Failed => write!(f, "failed"),
            BackupStatus::Restoring => write!(f, "restoring"),
            BackupStatus::Restored => write!(f, "restored"),
            BackupStatus::Expired => write!(f, "expired"),
            BackupStatus::Deleted => write!(f, "deleted"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupLocation {
    Local(String),
    Cloud(String),
}

impl fmt::Display for BackupLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupLocation::Local(path) => write!(f, "local:{}", path),
            BackupLocation::Cloud(path) => write!(f, "cloud:{}", path),
        }
    }
}

// ============================================================
// RETENTION POLICY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub hot_storage_days: u32,
    pub warm_storage_days: u32,
    pub cold_storage_days: u32,
    pub archive_path: String,
    pub auto_archive: bool,
    pub auto_delete: bool,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        RetentionPolicy {
            hot_storage_days: 30,
            warm_storage_days: 90,
            cold_storage_days: 365,
            archive_path: "./data/archive/".to_string(),
            auto_archive: true,
            auto_delete: false,
        }
    }
}

impl RetentionPolicy {
    pub fn get_tier(&self, age_days: i64) -> StorageTier {
        let age = age_days as u32;
        if age <= self.hot_storage_days {
            StorageTier::Hot
        } else if age <= self.warm_storage_days {
            StorageTier::Warm
        } else if age <= self.cold_storage_days {
            StorageTier::Cold
        } else {
            StorageTier::Archive
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageTier {
    Hot,
    Warm,
    Cold,
    Archive,
}

impl fmt::Display for StorageTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageTier::Hot => write!(f, "hot"),
            StorageTier::Warm => write!(f, "warm"),
            StorageTier::Cold => write!(f, "cold"),
            StorageTier::Archive => write!(f, "archive"),
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
    fn test_storage_key_string() {
        let key = StorageKey::new("scans", "scan-001").with_version(3);
        assert!(key.to_string_key().contains("scans"));
        assert!(key.to_string_key().contains("scan-001"));
        assert!(key.to_string_key().contains("v3"));
    }

    #[test]
    fn test_storage_key_shard() {
        let key = StorageKey::new("reports", "rpt-001").with_shard("shard-a");
        assert_eq!(key.shard_id(), "shard-a");

        let key_no_shard = StorageKey::new("scans", "s-001");
        assert_eq!(key_no_shard.shard_id(), "scans");
    }

    #[test]
    fn test_storage_value_expiration() {
        let value = StorageValue::new(vec![1, 2, 3], "text/plain").with_ttl(0);
        assert!(value.is_expired());
    }

    #[test]
    fn test_storage_value_not_expired() {
        let value = StorageValue::new(vec![1, 2, 3], "text/plain").with_ttl(3600);
        assert!(!value.is_expired());
    }

    #[test]
    fn test_storage_value_tags() {
        let value = StorageValue::new(vec![], "text/plain")
            .with_tag("source", "scanner")
            .with_tag("severity", "high");
        assert_eq!(value.tags.get("source").unwrap(), "scanner");
        assert_eq!(value.tags.len(), 2);
    }

    #[test]
    fn test_query_filter_eq() {
        let filter = QueryFilter::eq("status", serde_json::json!("completed"));
        assert_eq!(filter.operator, FilterOperator::Equals);
    }

    #[test]
    fn test_pagination_config() {
        let pagination = PaginationConfig::new(3, 50);
        assert_eq!(pagination.page, 3);
        assert_eq!(pagination.offset(), 100);
    }

    #[test]
    fn test_storage_stats_cache_hit_rate() {
        let mut stats = StorageStats::new();
        stats.cache_hits = 80;
        stats.cache_misses = 20;
        assert!((stats.cache_hit_rate() - 0.8).abs() < 0.01);
    }

    #[test]
    fn test_backup_info_compression_ratio() {
        let mut info = BackupInfo::new(Uuid::new_v4(), "admin");
        info.size_bytes = 1000;
        info.compressed_size_bytes = 300;
        assert!((info.compression_ratio() - 0.7).abs() < 0.01);
    }

    #[test]
    fn test_backup_info_status_checks() {
        let mut info = BackupInfo::new(Uuid::new_v4(), "system");
        assert!(!info.is_complete());
        assert!(!info.is_failed());

        info.status = BackupStatus::Complete;
        assert!(info.is_complete());

        info.status = BackupStatus::Failed;
        assert!(info.is_failed());
    }

    #[test]
    fn test_backup_info_summary() {
        let info = BackupInfo::new(Uuid::new_v4(), "admin");
        let summary = info.to_summary();
        assert!(summary.contains("Backup"));
        assert!(summary.contains("pending"));
    }

    #[test]
    fn test_backup_info_age() {
        let info = BackupInfo::new(Uuid::new_v4(), "admin");
        assert_eq!(info.age_days(), 0);
    }

    #[test]
    fn test_backup_metadata_alias() {
        // Verifikasi bahwa BackupMetadata adalah alias untuk BackupInfo
        let meta: BackupMetadata = BackupInfo::new(Uuid::new_v4(), "test");
        assert_eq!(meta.version, "1.0.0");
        // Type check: bisa digunakan di mana BackupInfo digunakan
        let _info: BackupInfo = meta;
    }

    #[test]
    fn test_backup_info_encryption_fields() {
        let mut info = BackupInfo::new(Uuid::new_v4(), "admin");
        info.encrypted = true;
        info.encryption_used = Some("AES-256-GCM".to_string());
        info.compression_used = Some(CompressionType::Gzip);

        assert!(info.encrypted);
        assert_eq!(info.encryption_used, Some("AES-256-GCM".to_string()));
        assert_eq!(info.compression_used, Some(CompressionType::Gzip));
    }

    #[test]
    fn test_backup_location_display() {
        assert_eq!(
            BackupLocation::Local("/backups".to_string()).to_string(),
            "local:/backups"
        );
        assert_eq!(
            BackupLocation::Cloud("s3://bucket".to_string()).to_string(),
            "cloud:s3://bucket"
        );
    }

    #[test]
    fn test_retention_policy_tiers() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.get_tier(10), StorageTier::Hot);
        assert_eq!(policy.get_tier(50), StorageTier::Warm);
        assert_eq!(policy.get_tier(200), StorageTier::Cold);
        assert_eq!(policy.get_tier(500), StorageTier::Archive);
    }

    #[test]
    fn test_storage_query_builder() {
        let query = StorageQuery::new()
            .with_namespace("scans")
            .with_filter(QueryFilter::eq("status", serde_json::json!("active")));

        assert_eq!(query.namespace, Some("scans".to_string()));
        assert_eq!(query.filters.len(), 1);
    }
}
