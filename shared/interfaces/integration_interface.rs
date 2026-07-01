// shared/interfaces/integration_interface.rs
// IWS v1.0 - Integration Interface
// Mendefinisikan trait Integration untuk third-party integrations

use std::collections::HashMap;
use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

use crate::shared::contracts::integration_contract::{
    IntegrationType, IntegrationConfig, IntegrationRequest,
    IntegrationResponse, HttpMethod, ConnectionState,
    RateLimitStatus, IntegrationHealth, HealthStatus,
    IntegrationCapabilities, IntegrationMetrics,
    Credentials, RetryPolicy, IntegrationContractError,
};

// ============================================================
// INTEGRATION TRAIT
// ============================================================

#[async_trait]
pub trait Integration: Send + Sync {
    type Error: std::error::Error + From<IntegrationContractError> + Send + Sync;

    /// Koneksi ke service
    async fn connect(&mut self, config: IntegrationConfig) -> Result<(), Self::Error>;

    /// Query ke service
    async fn query(&self, request: IntegrationRequest) -> Result<IntegrationResponse, Self::Error>;

    /// Validasi credentials
    async fn validate_credentials(&self, creds: Credentials) -> Result<bool, Self::Error>;

    /// Disconnect dari service
    async fn disconnect(&mut self) -> Result<(), Self::Error>;

    /// Dapatkan connection state
    fn connection_state(&self) -> ConnectionState;

    /// Dapatkan rate limit status
    async fn rate_limit_status(&self) -> Result<RateLimitStatus, Self::Error>;

    /// Dapatkan health status
    async fn health_status(&self) -> Result<IntegrationHealth, Self::Error>;

    /// Dapatkan capabilities
    fn capabilities(&self) -> IntegrationCapabilities;

    /// Dapatkan metrics
    fn metrics(&self) -> IntegrationMetrics;

    /// Dapatkan tipe integrasi
    fn integration_type(&self) -> IntegrationType;

    /// Cek rate limit
    async fn check_rate_limit(&self) -> Result<bool, Self::Error> {
        let status = self.rate_limit_status().await?;
        Ok(!status.is_exhausted())
    }

    /// Query dengan retry
    async fn query_with_retry(
        &self,
        request: IntegrationRequest,
    ) -> Result<IntegrationResponse, Self::Error> {
        let max_attempts = request.retry_policy.max_attempts;
        let mut last_error = None;

        for attempt in 0..=max_attempts {
            match self.query(request.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) if attempt < max_attempts => {
                    let backoff = request.retry_policy.calculate_backoff(attempt);
                    tokio::time::sleep(backoff).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or_else(|| {
            IntegrationContractError::InternalError("Retry exhausted".to_string()).into()
        }))
    }

    /// GET request helper
    async fn get(
        &self,
        path: &str,
        params: HashMap<String, String>,
    ) -> Result<IntegrationResponse, Self::Error> {
        let mut request = IntegrationRequest::new(HttpMethod::GET, path);
        for (k, v) in params {
            request = request.with_query_param(&k, &v);
        }
        self.query(request).await
    }

    /// POST request helper
    async fn post(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<IntegrationResponse, Self::Error> {
        let request = IntegrationRequest::new(HttpMethod::POST, path)
            .with_json_body(&body)
            .map_err(|e| IntegrationContractError::InvalidResponse(
                format!("Failed to serialize body: {}", e)
            ))?;
        self.query(request).await
    }

    /// Cek apakah terkoneksi
    fn is_connected(&self) -> bool {
        matches!(
            self.connection_state(),
            ConnectionState::Connected | ConnectionState::Authenticated
        )
    }

    /// Cek apakah sehat
    async fn is_healthy(&self) -> bool {
        self.health_status().await
            .map(|h| h.status == HealthStatus::Healthy)
            .unwrap_or(false)
    }

    /// Dapatkan nama service
    fn service_name(&self) -> &str;
}

// ============================================================
// INTEGRATION CACHE
// ============================================================

pub struct IntegrationCache {
    cache: std::sync::Mutex<HashMap<String, (IntegrationResponse, chrono::DateTime<chrono::Utc>)>>,
    ttl_secs: u64,
    max_entries: usize,
    hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
}

impl IntegrationCache {
    pub fn new(ttl_secs: u64, max_entries: usize) -> Self {
        IntegrationCache {
            cache: std::sync::Mutex::new(HashMap::new()),
            ttl_secs,
            max_entries,
            hits: std::sync::atomic::AtomicU64::new(0),
            misses: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn get(&self, key: &str) -> Option<IntegrationResponse> {
        let cache = self.cache.lock().unwrap();
        if let Some((response, cached_at)) = cache.get(key) {
            let elapsed = (chrono::Utc::now() - *cached_at).num_seconds() as u64;
            if elapsed <= self.ttl_secs {
                self.hits.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                return Some(response.clone());
            }
        }
        self.misses.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        None
    }

    pub fn put(&self, key: &str, response: IntegrationResponse) {
        let mut cache = self.cache.lock().unwrap();
        if cache.len() >= self.max_entries {
            // Remove oldest entry
            let oldest = cache.iter()
                .min_by_key(|(_, (_, cached_at))| *cached_at)
                .map(|(k, _)| k.clone());
            if let Some(old_key) = oldest {
                cache.remove(&old_key);
            }
        }
        cache.insert(key.to_string(), (response, chrono::Utc::now()));
    }

    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }

    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(std::sync::atomic::Ordering::Relaxed) as f64;
        let misses = self.misses.load(std::sync::atomic::Ordering::Relaxed) as f64;
        let total = hits + misses;
        if total == 0.0 { 0.0 } else { hits / total }
    }

    pub fn size(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    pub fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "size": self.size(),
            "max_entries": self.max_entries,
            "hits": self.hits.load(std::sync::atomic::Ordering::Relaxed),
            "misses": self.misses.load(std::sync::atomic::Ordering::Relaxed),
            "hit_rate": self.hit_rate(),
            "ttl_secs": self.ttl_secs,
        })
    }
}

// ============================================================
// INTEGRATION MANAGER
// ============================================================

#[async_trait]
pub trait IntegrationManager: Send + Sync {
    /// Register integration
    async fn register(
        &mut self,
        integration: Box<dyn Integration<Error = IntegrationContractError>>,
    ) -> Result<(), IntegrationContractError>;

    /// Unregister integration
    async fn unregister(&mut self, service_type: IntegrationType) -> Result<(), IntegrationContractError>;

    /// Dapatkan integration
    fn get(&self, service_type: &IntegrationType) -> Option<&dyn Integration<Error = IntegrationContractError>>;

    /// List semua integration
    fn list(&self) -> Vec<IntegrationType>;

    /// Connect semua integration
    async fn connect_all(&mut self) -> Result<(), IntegrationContractError>;

    /// Disconnect semua integration
    async fn disconnect_all(&mut self) -> Result<(), IntegrationContractError>;

    /// Health check semua
    async fn health_check_all(&self) -> HashMap<IntegrationType, bool>;

    /// Dapatkan semua metrics
    fn metrics_all(&self) -> HashMap<IntegrationType, IntegrationMetrics>;
}

// ============================================================
// INTEGRATION POOL
// ============================================================

pub struct IntegrationPool {
    integrations: HashMap<IntegrationType, Box<dyn Integration<Error = IntegrationContractError>>>,
    configs: HashMap<IntegrationType, IntegrationConfig>,
}

impl IntegrationPool {
    pub fn new() -> Self {
        IntegrationPool {
            integrations: HashMap::new(),
            configs: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        integration: Box<dyn Integration<Error = IntegrationContractError>>,
        config: IntegrationConfig,
    ) -> Result<(), IntegrationContractError> {
        let itype = integration.integration_type();
        if self.integrations.contains_key(&itype) {
            return Err(IntegrationContractError::InternalError(
                format!("Integration {} already registered", itype)
            ));
        }
        self.configs.insert(itype.clone(), config);
        self.integrations.insert(itype, integration);
        Ok(())
    }

    pub fn remove(&mut self, itype: &IntegrationType) -> Result<(), IntegrationContractError> {
        self.integrations.remove(itype)
            .ok_or_else(|| IntegrationContractError::InternalError(
                format!("Integration {} not found", itype)
            ))?;
        self.configs.remove(itype);
        Ok(())
    }

    pub fn get(&self, itype: &IntegrationType) -> Option<&dyn Integration<Error = IntegrationContractError>> {
        self.integrations.get(itype).map(|i| i.as_ref())
    }

    pub fn get_mut(&mut self, itype: &IntegrationType) -> Option<&mut Box<dyn Integration<Error = IntegrationContractError>>> {
        self.integrations.get_mut(itype)
    }

    pub fn list_types(&self) -> Vec<IntegrationType> {
        self.integrations.keys().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.integrations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.integrations.is_empty()
    }

    pub fn get_config(&self, itype: &IntegrationType) -> Option<&IntegrationConfig> {
        self.configs.get(itype)
    }
}

impl Default for IntegrationPool {
    fn default() -> Self {
        IntegrationPool::new()
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestIntegration {
        itype: IntegrationType,
        state: std::sync::Mutex<ConnectionState>,
        cached_responses: std::sync::Mutex<Vec<IntegrationResponse>>,
    }

    impl TestIntegration {
        fn new(itype: IntegrationType) -> Self {
            TestIntegration {
                itype,
                state: std::sync::Mutex::new(ConnectionState::Disconnected),
                cached_responses: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl Integration for TestIntegration {
        type Error = IntegrationContractError;

        async fn connect(&mut self, _config: IntegrationConfig) -> Result<(), Self::Error> {
            *self.state.lock().unwrap() = ConnectionState::Connected;
            Ok(())
        }

        async fn query(&self, _request: IntegrationRequest) -> Result<IntegrationResponse, Self::Error> {
            Ok(IntegrationResponse {
                request_id: Uuid::new_v4(),
                status_code: 200,
                headers: HashMap::new(),
                body: b"{}".to_vec(),
                elapsed_ms: 50,
                retry_count: 0,
                cached: false,
                timestamp: chrono::Utc::now(),
            })
        }

        async fn validate_credentials(&self, _creds: Credentials) -> Result<bool, Self::Error> {
            Ok(true)
        }

        async fn disconnect(&mut self) -> Result<(), Self::Error> {
            *self.state.lock().unwrap() = ConnectionState::Disconnected;
            Ok(())
        }

        fn connection_state(&self) -> ConnectionState {
            self.state.lock().unwrap().clone()
        }

        async fn rate_limit_status(&self) -> Result<RateLimitStatus, Self::Error> {
            Ok(RateLimitStatus {
                limit: 100,
                remaining: 95,
                reset_at: chrono::Utc::now() + chrono::Duration::seconds(60),
                window_secs: 60,
            })
        }

        async fn health_status(&self) -> Result<IntegrationHealth, Self::Error> {
            Ok(IntegrationHealth::healthy(50))
        }

        fn capabilities(&self) -> IntegrationCapabilities {
            IntegrationCapabilities::default()
        }

        fn metrics(&self) -> IntegrationMetrics {
            IntegrationMetrics::default()
        }

        fn integration_type(&self) -> IntegrationType {
            self.itype.clone()
        }

        fn service_name(&self) -> &str {
            "test_service"
        }
    }

    #[tokio::test]
    async fn test_integration_connect_disconnect() {
        let mut integration = TestIntegration::new(IntegrationType::Shodan);
        assert_eq!(integration.connection_state(), ConnectionState::Disconnected);

        let config = IntegrationConfig::new(
            IntegrationType::Shodan,
            url::Url::parse("https://api.shodan.io").unwrap(),
            "test-key-12345",
        );
        integration.connect(config).await.unwrap();
        assert!(integration.is_connected());

        integration.disconnect().await.unwrap();
        assert!(!integration.is_connected());
    }

    #[tokio::test]
    async fn test_integration_query() {
        let integration = TestIntegration::new(IntegrationType::VirusTotal);
        let request = IntegrationRequest::new(HttpMethod::GET, "/api/v3/domains/example.com");
        let response = integration.query(request).await.unwrap();
        assert_eq!(response.status_code, 200);
    }

    #[tokio::test]
    async fn test_integration_check_rate_limit() {
        let integration = TestIntegration::new(IntegrationType::AlienVaultOTX);
        let can_query = integration.check_rate_limit().await.unwrap();
        assert!(can_query);
    }

    #[tokio::test]
    async fn test_integration_is_healthy() {
        let integration = TestIntegration::new(IntegrationType::SecurityTrails);
        assert!(integration.is_healthy().await);
    }

    #[test]
    fn test_integration_cache() {
        let cache = IntegrationCache::new(60, 10);

        // Cache miss
        assert!(cache.get("key1").is_none());

        // Cache put and hit
        let response = IntegrationResponse {
            request_id: Uuid::new_v4(),
            status_code: 200,
            headers: HashMap::new(),
            body: b"cached".to_vec(),
            elapsed_ms: 10,
            retry_count: 0,
            cached: true,
            timestamp: chrono::Utc::now(),
        };
        cache.put("key1", response);
        assert!(cache.get("key1").is_some());
        assert_eq!(cache.size(), 1);
        assert!(cache.hit_rate() > 0.0);
    }

    #[test]
    fn test_integration_cache_eviction() {
        let cache = IntegrationCache::new(60, 2);

        for i in 0..3 {
            let response = IntegrationResponse {
                request_id: Uuid::new_v4(),
                status_code: 200,
                headers: HashMap::new(),
                body: vec![i as u8],
                elapsed_ms: 10,
                retry_count: 0,
                cached: true,
                timestamp: chrono::Utc::now(),
            };
            cache.put(&format!("key{}", i), response);
        }

        assert!(cache.size() <= 2);
    }

    #[test]
    fn test_integration_cache_stats() {
        let cache = IntegrationCache::new(60, 10);
        let stats = cache.stats();
        assert_eq!(stats["size"], 0);
        assert_eq!(stats["max_entries"], 10);
        assert_eq!(stats["ttl_secs"], 60);
    }

    #[test]
    fn test_integration_pool() {
        let mut pool = IntegrationPool::new();
        let integration = TestIntegration::new(IntegrationType::Shodan);
        let config = IntegrationConfig::new(
            IntegrationType::Shodan,
            url::Url::parse("https://api.shodan.io").unwrap(),
            "test-key",
        );

        pool.add(Box::new(integration), config).unwrap();
        assert_eq!(pool.count(), 1);
        assert!(pool.get(&IntegrationType::Shodan).is_some());

        pool.remove(&IntegrationType::Shodan).unwrap();
        assert_eq!(pool.count(), 0);
        assert!(pool.is_empty());
    }

    #[test]
    fn test_integration_pool_duplicate() {
        let mut pool = IntegrationPool::new();
        let i1 = TestIntegration::new(IntegrationType::Censys);
        let config = IntegrationConfig::new(
            IntegrationType::Censys,
            url::Url::parse("https://search.censys.io").unwrap(),
            "test-key",
        );

        pool.add(Box::new(i1), config.clone()).unwrap();
        let i2 = TestIntegration::new(IntegrationType::Censys);
        let result = pool.add(Box::new(i2), config);
        assert!(result.is_err());
    }
}
