// shared/interfaces/storage_interface.rs
// IWS v1.0 - Storage Interface
// Mendefinisikan trait Storage untuk data persistence

use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::shared::contracts::storage_contract::{
    StorageKey, StorageValue, DataQuery,
    BackupRequest, BackupInfo, RestoreRequest,
    StorageMetrics, StorageCapabilities, StorageContractError,
    ConsistencyLevel, RetentionPolicy,
};

// ============================================================
// STORAGE TRAIT
// ============================================================

#[async_trait]
pub trait Storage: Send + Sync {
    type Error: std::error::Error + From<StorageContractError> + Send + Sync;

    /// Simpan data
    async fn store(&self, key: StorageKey, value: StorageValue) -> Result<(), Self::Error>;

    /// Ambil data
    async fn retrieve(&self, key: &StorageKey) -> Result<Option<StorageValue>, Self::Error>;

    /// Hapus data
    async fn delete(&self, key: &StorageKey) -> Result<(), Self::Error>;

    /// Query data
    async fn query(&self, query: DataQuery) -> Result<Vec<StorageValue>, Self::Error>;

    /// Cek apakah key exists
    async fn exists(&self, key: &StorageKey) -> Result<bool, Self::Error>;

    /// Update data yang sudah ada
    async fn update(&self, key: StorageKey, value: StorageValue) -> Result<(), Self::Error>;

    /// Batch store
    async fn batch_store(
        &self,
        entries: Vec<(StorageKey, StorageValue)>,
    ) -> Result<Vec<StorageKey>, Self::Error>;

    /// Batch retrieve
    async fn batch_retrieve(
        &self,
        keys: Vec<StorageKey>,
    ) -> Result<HashMap<StorageKey, Option<StorageValue>>, Self::Error>;

    /// Batch delete
    async fn batch_delete(&self, keys: Vec<StorageKey>) -> Result<(), Self::Error>;

    /// Backup data
    async fn backup(&self, request: BackupRequest) -> Result<BackupInfo, Self::Error>;

    /// Restore dari backup
    async fn restore(&self, request: RestoreRequest) -> Result<(), Self::Error>;

    /// Health check
    async fn health_check(&self) -> Result<bool, Self::Error>;

    /// Durability check — verifikasi data persist
    async fn durability_check(&self) -> Result<bool, Self::Error> {
        let test_key = StorageKey::new(
            crate::shared::contracts::storage_contract::StorageNamespace::Temp,
            &format!("durability_test_{}", Uuid::new_v4()),
        );
        let test_value = StorageValue::new(
            b"durability_check".to_vec(),
            "application/octet-stream",
        );

        self.store(test_key.clone(), test_value).await?;
        let retrieved = self.retrieve(&test_key).await?;
        self.delete(&test_key).await?;

        Ok(retrieved.is_some())
    }

    /// Dapatkan metrics
    async fn metrics(&self) -> Result<StorageMetrics, Self::Error>;

    /// Compaction/optimization
    async fn compact(&self) -> Result<(), Self::Error>;

    /// List backups
    async fn list_backups(&self) -> Result<Vec<BackupInfo>, Self::Error>;

    /// Verifikasi integritas backup
    async fn verify_backup(&self, backup_id: Uuid) -> Result<bool, Self::Error>;

    /// Dapatkan capabilities
    fn capabilities(&self) -> StorageCapabilities;

    /// Validasi key
    fn validate_key(&self, key: &StorageKey) -> Result<(), Self::Error> {
        if key.id.is_empty() {
            return Err(StorageContractError::InvalidKey(
                "key id cannot be empty".to_string()
            ).into());
        }
        if key.id.len() > 1024 {
            return Err(StorageContractError::InvalidKey(
                format!("key id too long: {} chars (max 1024)", key.id.len())
            ).into());
        }
        Ok(())
    }

    /// Store dengan retry
    async fn store_with_retry(
        &self,
        key: StorageKey,
        value: StorageValue,
        max_retries: u32,
    ) -> Result<(), Self::Error> {
        let mut last_error = None;
        for attempt in 0..=max_retries {
            match self.store(key.clone(), value.clone()).await {
                Ok(()) => return Ok(()),
                Err(e) if attempt < max_retries => {
                    let backoff = std::time::Duration::from_millis(100 * 2u64.pow(attempt));
                    tokio::time::sleep(backoff).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }
        Err(last_error.unwrap_or_else(|| {
            StorageContractError::InternalError("Store retry exhausted".to_string()).into()
        }))
    }

    /// Retrieve dengan fallback consistency
    async fn retrieve_with_fallback(
        &self,
        key: &StorageKey,
        consistency: ConsistencyLevel,
    ) -> Result<Option<StorageValue>, Self::Error> {
        match consistency {
            ConsistencyLevel::StrongConsistency => self.retrieve(key).await,
            ConsistencyLevel::EventualConsistency => {
                // Coba retrieve, jika None coba lagi setelah short delay
                let result = self.retrieve(key).await?;
                if result.is_none() {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    return self.retrieve(key).await;
                }
                Ok(result)
            }
            ConsistencyLevel::ReadAfterWriteConsistency => {
                // Write dulu baru read untuk konsistensi
                self.retrieve(key).await
            }
        }
    }
}

// ============================================================
// STORAGE MANAGER
// ============================================================

#[async_trait]
pub trait StorageManager: Storage {
    /// Inisialisasi storage
    async fn initialize(&self) -> Result<(), <Self as Storage>::Error>;

    /// Shutdown storage dengan graceful cleanup
    async fn shutdown(&self) -> Result<(), <Self as Storage>::Error>;

    /// Cek apakah storage siap
    async fn is_ready(&self) -> bool;

    /// Dapatkan nama storage backend
    fn backend_name(&self) -> &str;

    /// Dapatkan konfigurasi retensi
    fn retention_policy(&self) -> &RetentionPolicy;

    /// Set retensi policy
    fn set_retention_policy(&mut self, policy: RetentionPolicy);

    /// Apply retensi — archive/delete data lama
    async fn apply_retention(&self) -> Result<u64, <Self as Storage>::Error>;

    /// Dapatkan statistik namespace
    async fn namespace_stats(&self, namespace: &str) -> Result<NamespaceStats, <Self as Storage>::Error>;

    /// Clear namespace
    async fn clear_namespace(&self, namespace: &str) -> Result<u64, <Self as Storage>::Error>;

    /// Export namespace ke file
    async fn export_namespace(
        &self,
        namespace: &str,
        output_path: &str,
    ) -> Result<u64, <Self as Storage>::Error>;

    /// Import namespace dari file
    async fn import_namespace(
        &self,
        namespace: &str,
        input_path: &str,
    ) -> Result<u64, <Self as Storage>::Error>;
}

// ============================================================
// NAMESPACE STATS
// ============================================================

#[derive(Debug, Clone)]
pub struct NamespaceStats {
    pub namespace: String,
    pub key_count: u64,
    pub total_size_bytes: u64,
    pub average_size_bytes: f64,
    pub oldest_entry: Option<chrono::DateTime<chrono::Utc>>,
    pub newest_entry: Option<chrono::DateTime<chrono::Utc>>,
    pub expired_count: u64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f64,
}

impl NamespaceStats {
    pub fn new(namespace: &str) -> Self {
        NamespaceStats {
            namespace: namespace.to_string(),
            key_count: 0,
            total_size_bytes: 0,
            average_size_bytes: 0.0,
            oldest_entry: None,
            newest_entry: None,
            expired_count: 0,
            compressed_size_bytes: 0,
            compression_ratio: 0.0,
        }
    }
}

// ============================================================
// STORAGE CACHE TRAIT
// ============================================================

#[async_trait]
pub trait StorageCache: Send + Sync {
    /// Cache get
    async fn cache_get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageContractError>;

    /// Cache set
    async fn cache_set(&self, key: &str, value: Vec<u8>, ttl_secs: u64) -> Result<(), StorageContractError>;

    /// Cache delete
    async fn cache_delete(&self, key: &str) -> Result<(), StorageContractError>;

    /// Cache clear
    async fn cache_clear(&self) -> Result<(), StorageContractError>;

    /// Cache hit rate
    async fn cache_hit_rate(&self) -> f64;

    /// Cache size
    async fn cache_size(&self) -> usize;
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::contracts::storage_contract::{
        StorageNamespace, StorageValue, CompressionType,
    };

    struct TestStorage {
        data: std::sync::Mutex<HashMap<String, StorageValue>>,
        retention: RetentionPolicy,
    }

    impl TestStorage {
        fn new() -> Self {
            TestStorage {
                data: std::sync::Mutex::new(HashMap::new()),
                retention: RetentionPolicy::default(),
            }
        }
    }

    #[async_trait]
    impl Storage for TestStorage {
        type Error = StorageContractError;

        async fn store(&self, key: StorageKey, value: StorageValue) -> Result<(), Self::Error> {
            let mut data = self.data.lock().unwrap();
            data.insert(key.to_string_key(), value);
            Ok(())
        }

        async fn retrieve(&self, key: &StorageKey) -> Result<Option<StorageValue>, Self::Error> {
            let data = self.data.lock().unwrap();
            Ok(data.get(&key.to_string_key()).cloned())
        }

        async fn delete(&self, key: &StorageKey) -> Result<(), Self::Error> {
            let mut data = self.data.lock().unwrap();
            data.remove(&key.to_string_key());
            Ok(())
        }

        async fn query(&self, _query: DataQuery) -> Result<Vec<StorageValue>, Self::Error> {
            let data = self.data.lock().unwrap();
            Ok(data.values().cloned().collect())
        }

        async fn exists(&self, key: &StorageKey) -> Result<bool, Self::Error> {
            let data = self.data.lock().unwrap();
            Ok(data.contains_key(&key.to_string_key()))
        }

        async fn update(&self, key: StorageKey, value: StorageValue) -> Result<(), Self::Error> {
            self.store(key, value).await
        }

        async fn batch_store(&self, entries: Vec<(StorageKey, StorageValue)>) -> Result<Vec<StorageKey>, Self::Error> {
            let keys: Vec<StorageKey> = entries.iter().map(|(k, _)| k.clone()).collect();
            for (key, value) in entries {
                self.store(key, value).await?;
            }
            Ok(keys)
        }

        async fn batch_retrieve(&self, keys: Vec<StorageKey>) -> Result<HashMap<StorageKey, Option<StorageValue>>, Self::Error> {
            let mut result = HashMap::new();
            for key in keys {
                let value = self.retrieve(&key).await?;
                result.insert(key, value);
            }
            Ok(result)
        }

        async fn batch_delete(&self, keys: Vec<StorageKey>) -> Result<(), Self::Error> {
            for key in keys {
                self.delete(&key).await?;
            }
            Ok(())
        }

        async fn backup(&self, _request: BackupRequest) -> Result<BackupInfo, Self::Error> {
            Err(StorageContractError::UnsupportedOperation("backup not implemented".to_string()))
        }

        async fn restore(&self, _request: RestoreRequest) -> Result<(), Self::Error> {
            Err(StorageContractError::UnsupportedOperation("restore not implemented".to_string()))
        }

        async fn health_check(&self) -> Result<bool, Self::Error> {
            Ok(true)
        }

        async fn metrics(&self) -> Result<StorageMetrics, Self::Error> {
            Ok(StorageMetrics::default())
        }

        async fn compact(&self) -> Result<(), Self::Error> {
            Ok(())
        }

        async fn list_backups(&self) -> Result<Vec<BackupInfo>, Self::Error> {
            Ok(vec![])
        }

        async fn verify_backup(&self, _backup_id: Uuid) -> Result<bool, Self::Error> {
            Ok(false)
        }

        fn capabilities(&self) -> StorageCapabilities {
            StorageCapabilities::default()
        }
    }

    #[async_trait]
    impl StorageManager for TestStorage {
        async fn initialize(&self) -> Result<(), Self::Error> { Ok(()) }
        async fn shutdown(&self) -> Result<(), Self::Error> { Ok(()) }
        async fn is_ready(&self) -> bool { true }
        fn backend_name(&self) -> &str { "test_memory" }
        fn retention_policy(&self) -> &RetentionPolicy { &self.retention }
        fn set_retention_policy(&mut self, policy: RetentionPolicy) { self.retention = policy; }
        async fn apply_retention(&self) -> Result<u64, Self::Error> { Ok(0) }
        async fn namespace_stats(&self, _ns: &str) -> Result<NamespaceStats, Self::Error> {
            Ok(NamespaceStats::new("test"))
        }
        async fn clear_namespace(&self, _ns: &str) -> Result<u64, Self::Error> { Ok(0) }
        async fn export_namespace(&self, _ns: &str, _path: &str) -> Result<u64, Self::Error> { Ok(0) }
        async fn import_namespace(&self, _ns: &str, _path: &str) -> Result<u64, Self::Error> { Ok(0) }
    }

    #[tokio::test]
    async fn test_storage_store_retrieve() {
        let storage = TestStorage::new();
        let key = StorageKey::new(StorageNamespace::Scan, "test-001");
        let value = StorageValue::new(b"hello world".to_vec(), "text/plain");

        storage.store(key.clone(), value).await.unwrap();
        let retrieved = storage.retrieve(&key).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().data, b"hello world".to_vec());
    }

    #[tokio::test]
    async fn test_storage_delete() {
        let storage = TestStorage::new();
        let key = StorageKey::new(StorageNamespace::Scan, "test-del");
        let value = StorageValue::new(b"data".to_vec(), "text/plain");

        storage.store(key.clone(), value).await.unwrap();
        assert!(storage.exists(&key).await.unwrap());

        storage.delete(&key).await.unwrap();
        assert!(!storage.exists(&key).await.unwrap());
    }

    #[tokio::test]
    async fn test_storage_batch_operations() {
        let storage = TestStorage::new();
        let entries: Vec<(StorageKey, StorageValue)> = (0..5)
            .map(|i| {
                let key = StorageKey::new(StorageNamespace::Scan, &format!("batch-{}", i));
                let value = StorageValue::new(vec![i as u8], "text/plain");
                (key, value)
            })
            .collect();

        let keys = entries.iter().map(|(k, _)| k.clone()).collect();
        let stored_keys = storage.batch_store(entries).await.unwrap();
        assert_eq!(stored_keys.len(), 5);

        let retrieved = storage.batch_retrieve(keys.clone()).await.unwrap();
        assert_eq!(retrieved.len(), 5);

        storage.batch_delete(keys).await.unwrap();
        for i in 0..5 {
            let key = StorageKey::new(StorageNamespace::Scan, &format!("batch-{}", i));
            assert!(!storage.exists(&key).await.unwrap());
        }
    }

    #[tokio::test]
    async fn test_storage_durability_check() {
        let storage = TestStorage::new();
        let result = storage.durability_check().await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[tokio::test]
    async fn test_storage_validate_key() {
        let storage = TestStorage::new();

        // Valid key
        let key = StorageKey::new(StorageNamespace::Scan, "valid-key");
        assert!(storage.validate_key(&key).is_ok());

        // Empty key
        let empty_key = StorageKey {
            namespace: StorageNamespace::Scan,
            id: String::new(),
            version: None,
            shard_key: None,
        };
        assert!(storage.validate_key(&empty_key).is_err());

        // Too long key
        let long_key = StorageKey {
            namespace: StorageNamespace::Scan,
            id: "a".repeat(2000),
            version: None,
            shard_key: None,
        };
        assert!(storage.validate_key(&long_key).is_err());
    }

    #[tokio::test]
    async fn test_storage_store_with_retry() {
        let storage = TestStorage::new();
        let key = StorageKey::new(StorageNamespace::Scan, "retry-test");
        let value = StorageValue::new(b"retry data".to_vec(), "text/plain");

        let result = storage.store_with_retry(key, value, 3).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_namespace_stats() {
        let stats = NamespaceStats::new("test_ns");
        assert_eq!(stats.namespace, "test_ns");
        assert_eq!(stats.key_count, 0);
    }
}
