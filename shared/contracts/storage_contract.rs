// shared/contracts/storage_contract.rs
// IWS v1.0 - Storage Contract
// Mendefinisikan kontrak formal untuk semua storage operations

use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

// ============================================================
// PERFORMANCE SLA CONSTANTS (SPEC FIX #2)
// ============================================================

/// SLA: store_data harus selesai < 100ms
pub const SLA_STORE_MAX_MS: u64 = 100;

/// SLA: retrieve_data harus selesai < 50ms
pub const SLA_RETRIEVE_MAX_MS: u64 = 50;

/// SLA: delete_data harus selesai < 100ms
pub const SLA_DELETE_MAX_MS: u64 = 100;

/// SLA: query_data untuk 1000 records harus selesai < 500ms
pub const SLA_QUERY_MAX_MS: u64 = 500;

/// SLA: batch_store untuk 1000 entries harus selesai < 5000ms
pub const SLA_BATCH_STORE_MAX_MS: u64 = 5000;

/// SLA: batch_retrieve untuk 1000 entries harus selesai < 2000ms
pub const SLA_BATCH_RETRIEVE_MAX_MS: u64 = 2000;

/// SLA: batch_delete untuk 1000 entries harus selesai < 3000ms
pub const SLA_BATCH_DELETE_MAX_MS: u64 = 3000;

/// SLA: backup harus selesai < 60000ms (1 menit) per GB data
pub const SLA_BACKUP_MAX_MS_PER_GB: u64 = 60000;

/// SLA: restore harus selesai < 120000ms (2 menit) per GB data
pub const SLA_RESTORE_MAX_MS_PER_GB: u64 = 120000;

/// SLA: health_check harus selesai < 5000ms
pub const SLA_HEALTH_CHECK_MAX_MS: u64 = 5000;

/// SLA: durability_check harus selesai < 1000ms
pub const SLA_DURABILITY_CHECK_MAX_MS: u64 = 1000;

// ============================================================
// STORAGE KEY & VALUE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StorageKey {
    pub namespace: StorageNamespace,
    pub id: String,
    pub version: Option<u32>,
}

impl StorageKey {
    pub fn new(namespace: StorageNamespace, id: &str) -> Self {
        StorageKey {
            namespace,
            id: id.to_string(),
            version: None,
        }
    }

    pub fn with_version(namespace: StorageNamespace, id: &str, version: u32) -> Self {
        StorageKey {
            namespace,
            id: id.to_string(),
            version: Some(version),
        }
    }

    pub fn to_string(&self) -> String {
        match self.version {
            Some(v) => format!("{}:{}:v{}", self.namespace, self.id, v),
            None => format!("{}:{}", self.namespace, self.id),
        }
    }

    pub fn from_string(s: &str) -> Result<Self, StorageContractError> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 2 {
            return Err(StorageContractError::InvalidKey(format!(
                "invalid storage key format: {}", s
            )));
        }
        let namespace = StorageNamespace::from_str(parts[0])?;
        let id = parts[1].to_string();
        let version = if parts.len() >= 3 && parts[2].starts_with('v') {
            Some(parts[2][1..].parse::<u32>().map_err(|_| {
                StorageContractError::InvalidKey(format!("invalid version in key: {}", s))
            })?)
        } else {
            None
        };
        Ok(StorageKey { namespace, id, version })
    }
}

impl fmt::Display for StorageKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageNamespace {
    Scan,
    ScanActive,
    ScanCompleted,
    ScanArchived,
    Report,
    ReportJson,
    ReportTxt,
    ReportDocs,
    ReportCsv,
    ReportHtml,
    ReportPdf,
    Export,
    ExportJson,
    ExportTxt,
    ExportDocs,
    ExportCsv,
    ExportHtml,
    ExportPdf,
    Cache,
    CacheDns,
    CacheHtml,
    CacheEntity,
    Log,
    LogAccess,
    LogError,
    LogScanner,
    LogAgent,
    Config,
    AgentState,
    Backup,
    Temp,
    Custom(String),
}

impl StorageNamespace {
    pub fn from_str(s: &str) -> Result<Self, StorageContractError> {
        match s.to_lowercase().as_str() {
            "scan" => Ok(StorageNamespace::Scan),
            "scan_active" => Ok(StorageNamespace::ScanActive),
            "scan_completed" => Ok(StorageNamespace::ScanCompleted),
            "scan_archived" => Ok(StorageNamespace::ScanArchived),
            "report" => Ok(StorageNamespace::Report),
            "report_json" => Ok(StorageNamespace::ReportJson),
            "report_txt" => Ok(StorageNamespace::ReportTxt),
            "report_docs" => Ok(StorageNamespace::ReportDocs),
            "report_csv" => Ok(StorageNamespace::ReportCsv),
            "report_html" => Ok(StorageNamespace::ReportHtml),
            "report_pdf" => Ok(StorageNamespace::ReportPdf),
            "export" => Ok(StorageNamespace::Export),
            "export_json" => Ok(StorageNamespace::ExportJson),
            "export_txt" => Ok(StorageNamespace::ExportTxt),
            "export_docs" => Ok(StorageNamespace::ExportDocs),
            "export_csv" => Ok(StorageNamespace::ExportCsv),
            "export_html" => Ok(StorageNamespace::ExportHtml),
            "export_pdf" => Ok(StorageNamespace::ExportPdf),
            "cache" => Ok(StorageNamespace::Cache),
            "cache_dns" => Ok(StorageNamespace::CacheDns),
            "cache_html" => Ok(StorageNamespace::CacheHtml),
            "cache_entity" => Ok(StorageNamespace::CacheEntity),
            "log" => Ok(StorageNamespace::Log),
            "log_access" => Ok(StorageNamespace::LogAccess),
            "log_error" => Ok(StorageNamespace::LogError),
            "log_scanner" => Ok(StorageNamespace::LogScanner),
            "log_agent" => Ok(StorageNamespace::LogAgent),
            "config" => Ok(StorageNamespace::Config),
            "agent_state" => Ok(StorageNamespace::AgentState),
            "backup" => Ok(StorageNamespace::Backup),
            "temp" => Ok(StorageNamespace::Temp),
            other => Ok(StorageNamespace::Custom(other.to_string())),
        }
    }

    pub fn is_cache(&self) -> bool {
        matches!(
            self,
            StorageNamespace::Cache
                | StorageNamespace::CacheDns
                | StorageNamespace::CacheHtml
                | StorageNamespace::CacheEntity
        )
    }

    pub fn is_log(&self) -> bool {
        matches!(
            self,
            StorageNamespace::Log
                | StorageNamespace::LogAccess
                | StorageNamespace::LogError
                | StorageNamespace::LogScanner
                | StorageNamespace::LogAgent
        )
    }

    pub fn is_report(&self) -> bool {
        matches!(
            self,
            StorageNamespace::Report
                | StorageNamespace::ReportJson
                | StorageNamespace::ReportTxt
                | StorageNamespace::ReportDocs
                | StorageNamespace::ReportCsv
                | StorageNamespace::ReportHtml
                | StorageNamespace::ReportPdf
        )
    }

    pub fn is_export(&self) -> bool {
        matches!(
            self,
            StorageNamespace::Export
                | StorageNamespace::ExportJson
                | StorageNamespace::ExportTxt
                | StorageNamespace::ExportDocs
                | StorageNamespace::ExportCsv
                | StorageNamespace::ExportHtml
                | StorageNamespace::ExportPdf
        )
    }
}

impl fmt::Display for StorageNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageNamespace::Scan => write!(f, "scan"),
            StorageNamespace::ScanActive => write!(f, "scan_active"),
            StorageNamespace::ScanCompleted => write!(f, "scan_completed"),
            StorageNamespace::ScanArchived => write!(f, "scan_archived"),
            StorageNamespace::Report => write!(f, "report"),
            StorageNamespace::ReportJson => write!(f, "report_json"),
            StorageNamespace::ReportTxt => write!(f, "report_txt"),
            StorageNamespace::ReportDocs => write!(f, "report_docs"),
            StorageNamespace::ReportCsv => write!(f, "report_csv"),
            StorageNamespace::ReportHtml => write!(f, "report_html"),
            StorageNamespace::ReportPdf => write!(f, "report_pdf"),
            StorageNamespace::Export => write!(f, "export"),
            StorageNamespace::ExportJson => write!(f, "export_json"),
            StorageNamespace::ExportTxt => write!(f, "export_txt"),
            StorageNamespace::ExportDocs => write!(f, "export_docs"),
            StorageNamespace::ExportCsv => write!(f, "export_csv"),
            StorageNamespace::ExportHtml => write!(f, "export_html"),
            StorageNamespace::ExportPdf => write!(f, "export_pdf"),
            StorageNamespace::Cache => write!(f, "cache"),
            StorageNamespace::CacheDns => write!(f, "cache_dns"),
            StorageNamespace::CacheHtml => write!(f, "cache_html"),
            StorageNamespace::CacheEntity => write!(f, "cache_entity"),
            StorageNamespace::Log => write!(f, "log"),
            StorageNamespace::LogAccess => write!(f, "log_access"),
            StorageNamespace::LogError => write!(f, "log_error"),
            StorageNamespace::LogScanner => write!(f, "log_scanner"),
            StorageNamespace::LogAgent => write!(f, "log_agent"),
            StorageNamespace::Config => write!(f, "config"),
            StorageNamespace::AgentState => write!(f, "agent_state"),
            StorageNamespace::Backup => write!(f, "backup"),
            StorageNamespace::Temp => write!(f, "temp"),
            StorageNamespace::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageValue {
    pub data: Vec<u8>,
    pub metadata: ValueMetadata,
}

impl StorageValue {
    pub fn new(data: Vec<u8>, content_type: &str) -> Self {
        let now = Utc::now();
        StorageValue {
            data,
            metadata: ValueMetadata {
                content_type: content_type.to_string(),
                size: 0,
                created_at: now,
                updated_at: now,
                version: 1,
                checksum: None,
                compression: CompressionType::None,
                tags: HashMap::new(),
                custom: serde_json::json!({}),
            },
        }
    }

    pub fn with_compression(mut self, compression: CompressionType) -> Self {
        self.metadata.compression = compression;
        self
    }

    pub fn with_tags(mut self, tags: HashMap<String, String>) -> Self {
        self.metadata.tags = tags;
        self
    }

    pub fn with_checksum(mut self, checksum: &str) -> Self {
        self.metadata.checksum = Some(checksum.to_string());
        self
    }

    pub fn update_data(&mut self, data: Vec<u8>) {
        self.data = data;
        self.metadata.size = self.data.len() as u64;
        self.metadata.updated_at = Utc::now();
        self.metadata.version += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueMetadata {
    pub content_type: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
    pub checksum: Option<String>,
    pub compression: CompressionType,
    pub tags: HashMap<String, String>,
    pub custom: serde_json::Value,
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
// CONSISTENCY LEVEL
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsistencyLevel {
    StrongConsistency,
    EventualConsistency,
    ReadAfterWriteConsistency,
}

impl fmt::Display for ConsistencyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConsistencyLevel::StrongConsistency => write!(f, "strong"),
            ConsistencyLevel::EventualConsistency => write!(f, "eventual"),
            ConsistencyLevel::ReadAfterWriteConsistency => write!(f, "read_after_write"),
        }
    }
}

impl Default for ConsistencyLevel {
    fn default() -> Self {
        ConsistencyLevel::StrongConsistency
    }
}

// ============================================================
// DATA QUERY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuery {
    pub namespace: StorageNamespace,
    pub filters: Vec<QueryFilter>,
    pub sort: Option<SortOrder>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub consistency: ConsistencyLevel,
}

impl DataQuery {
    pub fn new(namespace: StorageNamespace) -> Self {
        DataQuery {
            namespace,
            filters: vec![],
            sort: None,
            limit: None,
            offset: None,
            consistency: ConsistencyLevel::default(),
        }
    }

    pub fn with_filter(mut self, filter: QueryFilter) -> Self {
        self.filters.push(filter);
        self
    }

    pub fn with_sort(mut self, field: &str, direction: SortDirection) -> Self {
        self.sort = Some(SortOrder {
            field: field.to_string(),
            direction,
        });
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn with_consistency(mut self, consistency: ConsistencyLevel) -> Self {
        self.consistency = consistency;
        self
    }

    pub fn validate(&self) -> Result<(), StorageContractError> {
        for filter in &self.filters {
            filter.validate()?;
        }
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 10000 {
                return Err(StorageContractError::InvalidQuery(
                    format!("limit must be 1-10000, got {}", limit)
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFilter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
    pub case_sensitive: bool,
}

impl QueryFilter {
    pub fn new(field: &str, operator: FilterOperator, value: serde_json::Value) -> Self {
        QueryFilter {
            field: field.to_string(),
            operator,
            value,
            case_sensitive: false,
        }
    }

    pub fn case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }

    pub fn validate(&self) -> Result<(), StorageContractError> {
        if self.field.is_empty() {
            return Err(StorageContractError::InvalidQuery(
                "filter field cannot be empty".to_string()
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    Between,
    IsNull,
    IsNotNull,
    Regex,
}

impl fmt::Display for FilterOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterOperator::Equals => write!(f, "="),
            FilterOperator::NotEquals => write!(f, "!="),
            FilterOperator::GreaterThan => write!(f, ">"),
            FilterOperator::GreaterThanOrEqual => write!(f, ">="),
            FilterOperator::LessThan => write!(f, "<"),
            FilterOperator::LessThanOrEqual => write!(f, "<="),
            FilterOperator::Contains => write!(f, "contains"),
            FilterOperator::NotContains => write!(f, "not_contains"),
            FilterOperator::StartsWith => write!(f, "starts_with"),
            FilterOperator::EndsWith => write!(f, "ends_with"),
            FilterOperator::In => write!(f, "in"),
            FilterOperator::NotIn => write!(f, "not_in"),
            FilterOperator::Between => write!(f, "between"),
            FilterOperator::IsNull => write!(f, "is_null"),
            FilterOperator::IsNotNull => write!(f, "is_not_null"),
            FilterOperator::Regex => write!(f, "regex"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortOrder {
    pub field: String,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SortDirection::Ascending => write!(f, "asc"),
            SortDirection::Descending => write!(f, "desc"),
        }
    }
}

// ============================================================
// BACKUP & RESTORE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRequest {
    pub include_databases: bool,
    pub include_files: bool,
    pub include_cache: bool,
    pub include_logs: bool,
    pub compress: bool,
    pub encrypt: bool,
    pub upload_to_cloud: bool,
    pub cloud_provider: Option<CloudProvider>,
    pub backup_path: Option<String>,
    pub tags: Vec<String>,
}

impl Default for BackupRequest {
    fn default() -> Self {
        BackupRequest {
            include_databases: true,
            include_files: true,
            include_cache: false,
            include_logs: true,
            compress: true,
            encrypt: true,
            upload_to_cloud: false,
            cloud_provider: None,
            backup_path: None,
            tags: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CloudProvider {
    AwsS3,
    GoogleCloudStorage,
    AzureBlob,
    Minio,
    Custom(String),
}

impl fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CloudProvider::AwsS3 => write!(f, "aws_s3"),
            CloudProvider::GoogleCloudStorage => write!(f, "gcs"),
            CloudProvider::AzureBlob => write!(f, "azure_blob"),
            CloudProvider::Minio => write!(f, "minio"),
            CloudProvider::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub backup_id: Uuid,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
    pub location: BackupLocation,
    pub status: BackupStatus,
    pub checksum: Option<String>,
    pub encryption_used: Option<String>,
    pub compression_used: Option<CompressionType>,
    pub tags: Vec<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupLocation {
    Local(String),
    S3(String),
    Gcs(String),
    Azure(String),
    Custom(String),
}

impl fmt::Display for BackupLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupLocation::Local(path) => write!(f, "local:{}", path),
            BackupLocation::S3(bucket) => write!(f, "s3:{}", bucket),
            BackupLocation::Gcs(bucket) => write!(f, "gcs:{}", bucket),
            BackupLocation::Azure(container) => write!(f, "azure:{}", container),
            BackupLocation::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupStatus {
    Pending,
    Running,
    Complete,
    Failed,
    Restoring,
    Restored,
    Expired,
}

impl fmt::Display for BackupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BackupStatus::Pending => write!(f, "pending"),
            BackupStatus::Running => write!(f, "running"),
            BackupStatus::Complete => write!(f, "complete"),
            BackupStatus::Failed => write!(f, "failed"),
            BackupStatus::Restoring => write!(f, "restoring"),
            BackupStatus::Restored => write!(f, "restored"),
            BackupStatus::Expired => write!(f, "expired"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRequest {
    pub backup_id: Uuid,
    pub restore_databases: bool,
    pub restore_files: bool,
    pub restore_cache: bool,
    pub restore_logs: bool,
    pub target_path: Option<String>,
    pub overwrite_existing: bool,
    pub validate_checksum: bool,
}

impl Default for RestoreRequest {
    fn default() -> Self {
        RestoreRequest {
            backup_id: Uuid::nil(),
            restore_databases: true,
            restore_files: true,
            restore_cache: false,
            restore_logs: false,
            target_path: None,
            overwrite_existing: false,
            validate_checksum: true,
        }
    }
}

// ============================================================
// RETENTION POLICY
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub hot_retention_days: u32,
    pub warm_retention_days: u32,
    pub cold_retention_days: u32,
    pub backup_retention: BackupRetention,
    pub auto_archive: bool,
    pub auto_delete: bool,
}

impl Default for RetentionPolicy {
    fn default() -> Self {
        RetentionPolicy {
            hot_retention_days: 90,
            warm_retention_days: 365,
            cold_retention_days: 730,
            backup_retention: BackupRetention::default(),
            auto_archive: true,
            auto_delete: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRetention {
    pub daily_backups: u32,
    pub weekly_backups: u32,
    pub monthly_backups: u32,
}

impl Default for BackupRetention {
    fn default() -> Self {
        BackupRetention {
            daily_backups: 7,
            weekly_backups: 4,
            monthly_backups: 12,
        }
    }
}

// ============================================================
// STORAGE METRICS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub total_entries: u64,
    pub total_size_bytes: u64,
    pub namespace_counts: HashMap<String, u64>,
    pub read_qps: f64,
    pub write_qps: f64,
    pub delete_qps: f64,
    pub average_read_latency_ms: f64,
    pub average_write_latency_ms: f64,
    pub cache_hit_rate: f64,
    pub compression_ratio: f64,
    pub disk_usage_percent: f64,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
    pub last_backup: Option<DateTime<Utc>>,
    pub last_compaction: Option<DateTime<Utc>>,
}

impl Default for StorageMetrics {
    fn default() -> Self {
        StorageMetrics {
            total_entries: 0,
            total_size_bytes: 0,
            namespace_counts: HashMap::new(),
            read_qps: 0.0,
            write_qps: 0.0,
            delete_qps: 0.0,
            average_read_latency_ms: 0.0,
            average_write_latency_ms: 0.0,
            cache_hit_rate: 0.0,
            compression_ratio: 0.0,
            disk_usage_percent: 0.0,
            oldest_entry: None,
            newest_entry: None,
            last_backup: None,
            last_compaction: None,
        }
    }
}

/// Cek apakah metrik melanggar SLA
impl StorageMetrics {
    pub fn check_sla(&self) -> Vec<String> {
        let mut violations = Vec::new();

        if self.average_read_latency_ms > SLA_RETRIEVE_MAX_MS as f64 {
            violations.push(format!(
                "Read latency {:.1}ms exceeds SLA {}ms",
                self.average_read_latency_ms, SLA_RETRIEVE_MAX_MS
            ));
        }
        if self.average_write_latency_ms > SLA_STORE_MAX_MS as f64 {
            violations.push(format!(
                "Write latency {:.1}ms exceeds SLA {}ms",
                self.average_write_latency_ms, SLA_STORE_MAX_MS
            ));
        }

        violations
    }
}

// ============================================================
// STORAGE CONTRACT TRAIT
// ============================================================

#[async_trait]
pub trait StorageContract: Send + Sync {
    /// Menyimpan data ke storage
    /// Preconditions: key valid, data tidak kosong
    /// Postconditions: data tersimpan, retrieve(key) mengembalikan data
    /// SLA: < 100ms (SLA_STORE_MAX_MS)
    async fn store_data(
        &self,
        key: StorageKey,
        data: StorageValue,
    ) -> Result<(), StorageContractError>;

    /// Mengambil data dari storage
    /// SLA: < 50ms (SLA_RETRIEVE_MAX_MS)
    async fn retrieve_data(
        &self,
        key: &StorageKey,
    ) -> Result<Option<StorageValue>, StorageContractError>;

    /// Menghapus data dari storage
    /// SLA: < 100ms (SLA_DELETE_MAX_MS)
    async fn delete_data(
        &self,
        key: &StorageKey,
    ) -> Result<(), StorageContractError>;

    /// Query data berdasarkan kriteria
    /// SLA: < 500ms untuk 1000 records (SLA_QUERY_MAX_MS)
    async fn query_data(
        &self,
        query: DataQuery,
    ) -> Result<Vec<StorageValue>, StorageContractError>;

    /// Mengecek apakah key exists
    async fn exists(&self, key: &StorageKey) -> Result<bool, StorageContractError>;

    /// Update data yang sudah ada
    async fn update_data(
        &self,
        key: StorageKey,
        data: StorageValue,
    ) -> Result<(), StorageContractError>;

    /// Batch store multiple entries
    /// SLA: < 5000ms untuk 1000 entries (SLA_BATCH_STORE_MAX_MS)
    async fn batch_store(
        &self,
        entries: Vec<(StorageKey, StorageValue)>,
    ) -> Result<Vec<StorageKey>, StorageContractError>;

    /// Batch retrieve multiple entries
    /// SLA: < 2000ms untuk 1000 entries (SLA_BATCH_RETRIEVE_MAX_MS)
    async fn batch_retrieve(
        &self,
        keys: Vec<StorageKey>,
    ) -> Result<HashMap<StorageKey, Option<StorageValue>>, StorageContractError>;

    /// Batch delete multiple entries
    /// SLA: < 3000ms untuk 1000 entries (SLA_BATCH_DELETE_MAX_MS)
    async fn batch_delete(
        &self,
        keys: Vec<StorageKey>,
    ) -> Result<(), StorageContractError>;

    /// Membuat backup
    /// SLA: < 60000ms per GB (SLA_BACKUP_MAX_MS_PER_GB)
    async fn backup(
        &self,
        request: BackupRequest,
    ) -> Result<BackupInfo, StorageContractError>;

    /// Merestore dari backup
    /// SLA: < 120000ms per GB (SLA_RESTORE_MAX_MS_PER_GB)
    async fn restore(
        &self,
        request: RestoreRequest,
    ) -> Result<(), StorageContractError>;

    /// Health check storage
    /// SLA: < 5000ms (SLA_HEALTH_CHECK_MAX_MS)
    async fn health_check(&self) -> Result<bool, StorageContractError>;

    /// Verifikasi bahwa data persist across system restarts
    /// Returns true jika durability terverifikasi, false jika tidak
    /// SLA: < 1000ms (SLA_DURABILITY_CHECK_MAX_MS)
    async fn durability_check(&self) -> Result<bool, StorageContractError> {
        let test_key = StorageKey::new(
            StorageNamespace::Temp,
            &format!("durability_test_{}", Uuid::new_v4()),
        );
        let test_value = StorageValue::new(
            b"durability_check".to_vec(),
            "application/octet-stream",
        );

        self.store_data(test_key.clone(), test_value).await?;
        let retrieved = self.retrieve_data(&test_key).await?;
        self.delete_data(&test_key).await?;

        Ok(retrieved.is_some())
    }

    /// Mendapatkan metrics storage
    async fn get_metrics(&self) -> Result<StorageMetrics, StorageContractError>;

    /// Melakukan compaction/optimization
    async fn compact(&self) -> Result<(), StorageContractError>;

    /// List backups yang tersedia
    async fn list_backups(&self) -> Result<Vec<BackupInfo>, StorageContractError>;

    /// Verifikasi integritas backup
    async fn verify_backup(
        &self,
        backup_id: Uuid,
    ) -> Result<bool, StorageContractError>;

    /// Mendapatkan capabilities storage
    fn get_capabilities(&self) -> StorageCapabilities;

    /// Validasi key sebelum operasi
    fn validate_key(&self, key: &StorageKey) -> Result<(), StorageContractError> {
        if key.id.is_empty() {
            return Err(StorageContractError::InvalidKey(
                "key id cannot be empty".to_string()
            ));
        }
        if key.id.len() > 1024 {
            return Err(StorageContractError::InvalidKey(
                format!("key id too long: {} chars (max 1024)", key.id.len())
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    pub max_value_size_bytes: u64,
    pub max_key_length: usize,
    pub supports_compression: bool,
    pub supported_compressions: Vec<CompressionType>,
    pub supports_encryption: bool,
    pub supports_versioning: bool,
    pub supports_batch_operations: bool,
    pub supports_backup: bool,
    pub supports_restore: bool,
    pub supports_query: bool,
    pub supports_full_text_search: bool,
    pub max_batch_size: usize,
    pub api_version: String,
}

impl Default for StorageCapabilities {
    fn default() -> Self {
        StorageCapabilities {
            max_value_size_bytes: 100 * 1024 * 1024,
            max_key_length: 1024,
            supports_compression: true,
            supported_compressions: vec![
                CompressionType::Gzip,
                CompressionType::Zstd,
                CompressionType::Lz4,
                CompressionType::Snappy,
                CompressionType::Brotli,
            ],
            supports_encryption: true,
            supports_versioning: true,
            supports_batch_operations: true,
            supports_backup: true,
            supports_restore: true,
            supports_query: true,
            supports_full_text_search: true,
            max_batch_size: 1000,
            api_version: "v1.0".to_string(),
        }
    }
}

// ============================================================
// ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorageContractError {
    InvalidKey(String),
    InvalidQuery(String),
    KeyNotFound(StorageKey),
    KeyAlreadyExists(StorageKey),
    ConnectionFailed(String),
    IntegrityError(String),
    SerializationError(String),
    DeserializationError(String),
    CompressionError(String),
    EncryptionError(String),
    StorageFull(String),
    Timeout(String),
    BackupFailed(String),
    RestoreFailed(String),
    InvalidBackup(Uuid),
    InternalError(String),
    UnsupportedOperation(String),
    PermissionDenied(String),
    QuotaExceeded(String),
    SlaViolation(String),
}

impl fmt::Display for StorageContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageContractError::InvalidKey(msg) => write!(f, "Invalid key: {}", msg),
            StorageContractError::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
            StorageContractError::KeyNotFound(key) => write!(f, "Key not found: {}", key),
            StorageContractError::KeyAlreadyExists(key) => write!(f, "Key already exists: {}", key),
            StorageContractError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            StorageContractError::IntegrityError(msg) => write!(f, "Integrity error: {}", msg),
            StorageContractError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            StorageContractError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            StorageContractError::CompressionError(msg) => write!(f, "Compression error: {}", msg),
            StorageContractError::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
            StorageContractError::StorageFull(msg) => write!(f, "Storage full: {}", msg),
            StorageContractError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            StorageContractError::BackupFailed(msg) => write!(f, "Backup failed: {}", msg),
            StorageContractError::RestoreFailed(msg) => write!(f, "Restore failed: {}", msg),
            StorageContractError::InvalidBackup(id) => write!(f, "Invalid backup: {}", id),
            StorageContractError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            StorageContractError::UnsupportedOperation(msg) => write!(f, "Unsupported: {}", msg),
            StorageContractError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            StorageContractError::QuotaExceeded(msg) => write!(f, "Quota exceeded: {}", msg),
            StorageContractError::SlaViolation(msg) => write!(f, "SLA violation: {}", msg),
        }
    }
}

impl std::error::Error for StorageContractError {}

impl StorageContractError {
    pub fn code(&self) -> &str {
        match self {
            StorageContractError::InvalidKey(_) => "ST4001",
            StorageContractError::InvalidQuery(_) => "ST4002",
            StorageContractError::KeyNotFound(_) => "ST4003",
            StorageContractError::KeyAlreadyExists(_) => "ST4004",
            StorageContractError::ConnectionFailed(_) => "ST4005",
            StorageContractError::IntegrityError(_) => "ST4006",
            StorageContractError::SerializationError(_) => "ST4007",
            StorageContractError::DeserializationError(_) => "ST4008",
            StorageContractError::CompressionError(_) => "ST4009",
            StorageContractError::EncryptionError(_) => "ST4010",
            StorageContractError::StorageFull(_) => "ST4011",
            StorageContractError::Timeout(_) => "ST4012",
            StorageContractError::BackupFailed(_) => "ST4013",
            StorageContractError::RestoreFailed(_) => "ST4014",
            StorageContractError::InvalidBackup(_) => "ST4015",
            StorageContractError::InternalError(_) => "ST4016",
            StorageContractError::UnsupportedOperation(_) => "ST4017",
            StorageContractError::PermissionDenied(_) => "ST4018",
            StorageContractError::QuotaExceeded(_) => "ST4019",
            StorageContractError::SlaViolation(_) => "ST4020",
        }
    }

    pub fn severity(&self) -> &str {
        match self {
            StorageContractError::InvalidKey(_) => "high",
            StorageContractError::InvalidQuery(_) => "medium",
            StorageContractError::KeyNotFound(_) => "low",
            StorageContractError::KeyAlreadyExists(_) => "low",
            StorageContractError::ConnectionFailed(_) => "critical",
            StorageContractError::IntegrityError(_) => "critical",
            StorageContractError::SerializationError(_) => "medium",
            StorageContractError::DeserializationError(_) => "medium",
            StorageContractError::CompressionError(_) => "medium",
            StorageContractError::EncryptionError(_) => "high",
            StorageContractError::StorageFull(_) => "critical",
            StorageContractError::Timeout(_) => "medium",
            StorageContractError::BackupFailed(_) => "high",
            StorageContractError::RestoreFailed(_) => "high",
            StorageContractError::InvalidBackup(_) => "medium",
            StorageContractError::InternalError(_) => "critical",
            StorageContractError::UnsupportedOperation(_) => "low",
            StorageContractError::PermissionDenied(_) => "high",
            StorageContractError::QuotaExceeded(_) => "medium",
            StorageContractError::SlaViolation(_) => "high",
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            StorageContractError::ConnectionFailed(_)
                | StorageContractError::Timeout(_)
                | StorageContractError::BackupFailed(_)
                | StorageContractError::SerializationError(_)
                | StorageContractError::CompressionError(_)
                | StorageContractError::SlaViolation(_)
        )
    }
}

// ============================================================
// STORAGE HEALTH
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealth {
    pub is_healthy: bool,
    pub connection_ok: bool,
    pub read_ok: bool,
    pub write_ok: bool,
    pub disk_ok: bool,
    pub latency_ms: f64,
    pub message: Option<String>,
    pub checked_at: DateTime<Utc>,
}

impl StorageHealth {
    pub fn healthy() -> Self {
        StorageHealth {
            is_healthy: true,
            connection_ok: true,
            read_ok: true,
            write_ok: true,
            disk_ok: true,
            latency_ms: 0.0,
            message: None,
            checked_at: Utc::now(),
        }
    }

    pub fn unhealthy(message: &str) -> Self {
        StorageHealth {
            is_healthy: false,
            connection_ok: false,
            read_ok: false,
            write_ok: false,
            disk_ok: false,
            latency_ms: 0.0,
            message: Some(message.to_string()),
            checked_at: Utc::now(),
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
    fn test_sla_constants_defined() {
        assert_eq!(SLA_STORE_MAX_MS, 100);
        assert_eq!(SLA_RETRIEVE_MAX_MS, 50);
        assert_eq!(SLA_DELETE_MAX_MS, 100);
        assert_eq!(SLA_QUERY_MAX_MS, 500);
        assert_eq!(SLA_BATCH_STORE_MAX_MS, 5000);
        assert_eq!(SLA_BATCH_RETRIEVE_MAX_MS, 2000);
        assert_eq!(SLA_BATCH_DELETE_MAX_MS, 3000);
        assert_eq!(SLA_BACKUP_MAX_MS_PER_GB, 60000);
        assert_eq!(SLA_RESTORE_MAX_MS_PER_GB, 120000);
        assert_eq!(SLA_HEALTH_CHECK_MAX_MS, 5000);
        assert_eq!(SLA_DURABILITY_CHECK_MAX_MS, 1000);
    }

    #[test]
    fn test_metrics_sla_check_no_violation() {
        let mut metrics = StorageMetrics::default();
        metrics.average_read_latency_ms = 25.0;  // < 50ms
        metrics.average_write_latency_ms = 50.0; // < 100ms
        let violations = metrics.check_sla();
        assert!(violations.is_empty());
    }

    #[test]
    fn test_metrics_sla_check_violation() {
        let mut metrics = StorageMetrics::default();
        metrics.average_read_latency_ms = 150.0; // > 50ms
        metrics.average_write_latency_ms = 200.0; // > 100ms
        let violations = metrics.check_sla();
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_sla_violation_error() {
        let err = StorageContractError::SlaViolation("Read latency 150ms exceeds SLA 50ms".to_string());
        assert_eq!(err.code(), "ST4020");
        assert_eq!(err.severity(), "high");
        assert!(err.is_recoverable());
    }

    #[test]
    fn test_storage_key_creation() {
        let key = StorageKey::new(StorageNamespace::Scan, "test-scan-001");
        assert_eq!(key.namespace, StorageNamespace::Scan);
        assert_eq!(key.id, "test-scan-001");
        assert_eq!(key.version, None);
    }

    #[test]
    fn test_storage_key_with_version() {
        let key = StorageKey::with_version(StorageNamespace::ScanCompleted, "scan-001", 3);
        assert_eq!(key.version, Some(3));
        assert!(key.to_string().contains("v3"));
    }

    #[test]
    fn test_storage_key_roundtrip() {
        let original = StorageKey::with_version(StorageNamespace::ReportJson, "report-001", 2);
        let serialized = original.to_string();
        let deserialized = StorageKey::from_string(&serialized).unwrap();
        assert_eq!(original.namespace, deserialized.namespace);
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.version, deserialized.version);
    }

    #[test]
    fn test_storage_key_invalid_format() {
        let result = StorageKey::from_string("invalid");
        assert!(result.is_err());

        let result = StorageKey::from_string("too:many:parts:here");
        assert!(result.is_ok());
    }

    #[test]
    fn test_namespace_from_str() {
        assert_eq!(
            StorageNamespace::from_str("scan").unwrap(),
            StorageNamespace::Scan
        );
        assert_eq!(
            StorageNamespace::from_str("SCAN_COMPLETED").unwrap(),
            StorageNamespace::ScanCompleted
        );
        assert_eq!(
            StorageNamespace::from_str("unknown_ns").unwrap(),
            StorageNamespace::Custom("unknown_ns".to_string())
        );
    }

    #[test]
    fn test_namespace_is_methods() {
        assert!(StorageNamespace::CacheDns.is_cache());
        assert!(StorageNamespace::LogAccess.is_log());
        assert!(StorageNamespace::ReportHtml.is_report());
        assert!(!StorageNamespace::Scan.is_cache());
    }

    #[test]
    fn test_storage_value_creation() {
        let data = b"test data".to_vec();
        let value = StorageValue::new(data.clone(), "application/json");
        assert_eq!(value.data, data);
        assert_eq!(value.metadata.content_type, "application/json");
        assert_eq!(value.metadata.version, 1);
    }

    #[test]
    fn test_storage_value_update() {
        let mut value = StorageValue::new(b"old data".to_vec(), "text/plain");
        let old_version = value.metadata.version;
        value.update_data(b"new data".to_vec());
        assert_eq!(value.data, b"new data".to_vec());
        assert_eq!(value.metadata.version, old_version + 1);
        assert_eq!(value.metadata.size, 8);
    }

    #[test]
    fn test_data_query_builder() {
        let query = DataQuery::new(StorageNamespace::Scan)
            .with_filter(QueryFilter::new(
                "status",
                FilterOperator::Equals,
                serde_json::json!("completed"),
            ))
            .with_sort("created_at", SortDirection::Descending)
            .with_limit(50)
            .with_offset(0);

        assert_eq!(query.filters.len(), 1);
        assert_eq!(query.limit, Some(50));
        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_data_query_validation_invalid_limit() {
        let query = DataQuery::new(StorageNamespace::Scan).with_limit(0);
        assert!(query.validate().is_err());

        let query = DataQuery::new(StorageNamespace::Scan).with_limit(10001);
        assert!(query.validate().is_err());
    }

    #[test]
    fn test_filter_operator_display() {
        assert_eq!(FilterOperator::Equals.to_string(), "=");
        assert_eq!(FilterOperator::Contains.to_string(), "contains");
        assert_eq!(FilterOperator::IsNull.to_string(), "is_null");
    }

    #[test]
    fn test_backup_request_default() {
        let req = BackupRequest::default();
        assert!(req.include_databases);
        assert!(req.include_files);
        assert!(!req.include_cache);
        assert!(req.compress);
        assert!(req.encrypt);
        assert!(!req.upload_to_cloud);
    }

    #[test]
    fn test_storage_error_codes() {
        let err = StorageContractError::KeyNotFound(StorageKey::new(
            StorageNamespace::Scan,
            "test",
        ));
        assert_eq!(err.code(), "ST4003");

        let err = StorageContractError::StorageFull("disk full".to_string());
        assert_eq!(err.code(), "ST4011");
    }

    #[test]
    fn test_storage_error_recoverable() {
        let timeout = StorageContractError::Timeout("timeout".to_string());
        assert!(timeout.is_recoverable());

        let integrity = StorageContractError::IntegrityError("corrupt".to_string());
        assert!(!integrity.is_recoverable());
    }

    #[test]
    fn test_storage_health() {
        let healthy = StorageHealth::healthy();
        assert!(healthy.is_healthy);
        assert!(healthy.connection_ok);

        let unhealthy = StorageHealth::unhealthy("disk full");
        assert!(!unhealthy.is_healthy);
        assert!(unhealthy.message.is_some());
    }

    #[test]
    fn test_retention_policy_defaults() {
        let policy = RetentionPolicy::default();
        assert_eq!(policy.hot_retention_days, 90);
        assert_eq!(policy.warm_retention_days, 365);
        assert_eq!(policy.cold_retention_days, 730);
        assert!(policy.auto_archive);
        assert_eq!(policy.backup_retention.daily_backups, 7);
    }
}
