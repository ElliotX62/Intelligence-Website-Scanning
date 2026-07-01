// storage/data_warehouse.rs
// IWS v1.0 - Data Warehouse
// Mengelola penyimpanan data utama dengan indexing, versioning, dan query optimization

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: u32,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    pub namespace: Option<String>,
    pub tags: HashMap<String, String>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub limit: usize,
    pub offset: usize,
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams {
            namespace: None, tags: HashMap::new(),
            since: None, until: None, limit: 100, offset: 0,
        }
    }
}

#[derive(Debug)]
struct Shard {
    entries: HashMap<String, Vec<DataEntry>>,
    index: HashMap<String, Vec<String>>,
}

impl Shard {
    fn new() -> Self {
        Shard { entries: HashMap::new(), index: HashMap::new() }
    }
}

pub struct DataWarehouse {
    shards: RwLock<HashMap<String, Shard>>,
    base_path: String,
    max_entries_per_shard: usize,
}

impl DataWarehouse {
    pub fn new(base_path: &str, shard_count: usize) -> Self {
        let mut shards = HashMap::new();
        for i in 0..shard_count {
            shards.insert(format!("shard_{}", i), Shard::new());
        }
        DataWarehouse {
            shards: RwLock::new(shards),
            base_path: base_path.to_string(),
            max_entries_per_shard: 10000,
        }
    }

    fn get_shard_key(&self, key: &str) -> String {
        let hash = key.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
        let shard_count = self.shards.read().unwrap().len();
        format!("shard_{}", hash % shard_count as u64)
    }

    pub fn store(&self, namespace: &str, id: &str, data: Vec<u8>, content_type: &str) -> Result<(), String> {
        let key = format!("{}:{}", namespace, id);
        let entry = DataEntry {
            key: key.clone(), data, content_type: content_type.to_string(),
            created_at: Utc::now(), updated_at: Utc::now(), version: 1,
            tags: HashMap::new(),
        };

        let shard_key = self.get_shard_key(&key);
        let mut shards = self.shards.write().unwrap();
        let shard = shards.get_mut(&shard_key).ok_or("Shard not found")?;

        shard.entries.entry(key.clone()).or_insert_with(Vec::new).push(entry);
        shard.index.entry(namespace.to_string()).or_insert_with(Vec::new).push(key);
        Ok(())
    }

    pub fn retrieve(&self, key: &str) -> Option<DataEntry> {
        let shard_key = self.get_shard_key(key);
        let shards = self.shards.read().unwrap();
        shards.get(&shard_key)?.entries.get(key)?.last().cloned()
    }

    pub fn query(&self, params: &QueryParams) -> Vec<DataEntry> {
        let shards = self.shards.read().unwrap();
        let mut results = Vec::new();

        for shard in shards.values() {
            for entries in shard.entries.values() {
                for entry in entries.iter().rev() {
                    if params.namespace.as_ref().map_or(true, |ns| entry.key.starts_with(ns)) {
                        if params.since.map_or(true, |s| entry.created_at >= s)
                            && params.until.map_or(true, |u| entry.created_at <= u) {
                            results.push(entry.clone());
                        }
                    }
                }
            }
        }

        results.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        let start = params.offset.min(results.len());
        let end = (start + params.limit).min(results.len());
        results[start..end].to_vec()
    }

    pub fn delete(&self, key: &str) -> Result<(), String> {
        let shard_key = self.get_shard_key(key);
        let mut shards = self.shards.write().unwrap();
        let shard = shards.get_mut(&shard_key).ok_or("Shard not found")?;
        shard.entries.remove(key);
        Ok(())
    }

    pub fn stats(&self) -> WarehouseStats {
        let shards = self.shards.read().unwrap();
        let mut stats = WarehouseStats::default();
        for shard in shards.values() {
            stats.total_entries += shard.entries.len();
            stats.total_keys += shard.entries.values().map(|v| v.len()).sum::<usize>();
        }
        stats.shard_count = shards.len();
        stats
    }
}

#[derive(Debug, Clone, Default)]
pub struct WarehouseStats {
    pub total_entries: usize,
    pub total_keys: usize,
    pub shard_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_retrieve() {
        let dw = DataWarehouse::new("./data", 4);
        dw.store("scans", "test-1", b"hello".to_vec(), "text/plain").unwrap();
        let entry = dw.retrieve("scans:test-1");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().data, b"hello");
    }

    #[test]
    fn test_query_namespace() {
        let dw = DataWarehouse::new("./data", 2);
        dw.store("scans", "a", b"1".to_vec(), "text").unwrap();
        dw.store("reports", "b", b"2".to_vec(), "text").unwrap();

        let params = QueryParams { namespace: Some("scans".into()), ..Default::default() };
        let results = dw.query(&params);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_delete() {
        let dw = DataWarehouse::new("./data", 2);
        dw.store("scans", "del", b"x".to_vec(), "text").unwrap();
        assert!(dw.retrieve("scans:del").is_some());
        dw.delete("scans:del").unwrap();
        assert!(dw.retrieve("scans:del").is_none());
    }
}
