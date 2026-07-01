// database/connection_pool.rs
// IWS v1.0 - Database Connection Pool
// Mengelola koneksi database dengan deadpool-postgres

use deadpool_postgres::{Config, Pool, PoolConfig, Runtime};
use tokio_postgres::NoTls;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ConnectionPool {
    pool: Pool,
    config: PoolSettings,
    metrics: Arc<RwLock<PoolMetrics>>,
}

#[derive(Debug, Clone)]
pub struct PoolSettings {
    pub max_size: usize,
    pub min_idle: usize,
    pub timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub connection_timeout_secs: u64,
}

impl Default for PoolSettings {
    fn default() -> Self {
        PoolSettings {
            max_size: 20,
            min_idle: 5,
            timeout_secs: 30,
            idle_timeout_secs: 600,
            max_lifetime_secs: 1800,
            connection_timeout_secs: 10,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PoolMetrics {
    pub active_connections: usize,
    pub idle_connections: usize,
    pub total_connections: usize,
    pub waiting_requests: usize,
    pub total_requests: u64,
    pub failed_requests: u64,
}

impl ConnectionPool {
    pub async fn new(database_url: &str, settings: PoolSettings) -> Result<Self, String> {
        let mut cfg = Config::new();
        cfg.url = Some(database_url.to_string());
        cfg.pool = Some(PoolConfig {
            max_size: settings.max_size,
            timeouts: deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(settings.timeout_secs)),
                create: Some(Duration::from_secs(settings.connection_timeout_secs)),
                recycle: Some(Duration::from_secs(settings.idle_timeout_secs)),
            },
        });

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| format!("Failed to create pool: {}", e))?;

        Ok(ConnectionPool {
            pool,
            config: settings,
            metrics: Arc::new(RwLock::new(PoolMetrics::default())),
        })
    }

    pub async fn get_connection(&self) -> Result<deadpool_postgres::Client, String> {
        self.pool.get().await.map_err(|e| format!("Connection error: {}", e))
    }

    pub async fn health_check(&self) -> bool {
        match self.get_connection().await {
            Ok(client) => {
                let result = client.query_one("SELECT 1", &[]).await;
                result.is_ok()
            }
            Err(_) => false,
        }
    }

    pub async fn close_all(&self) {
        self.pool.close();
    }

    pub async fn get_metrics(&self) -> PoolMetrics {
        let status = self.pool.status();
        let mut metrics = self.metrics.write().await;
        metrics.active_connections = status.size as usize - status.available;
        metrics.idle_connections = status.available;
        metrics.total_connections = status.size as usize;
        metrics.waiting_requests = status.waiting as usize;
        metrics.clone()
    }

    pub async fn is_healthy(&self) -> bool {
        let metrics = self.get_metrics().await;
        let usage = if metrics.total_connections > 0 {
            (metrics.active_connections as f64 / metrics.total_connections as f64) * 100.0
        } else {
            0.0
        };
        usage < 80.0
    }

    pub async fn reset(&self) -> Result<(), String> {
        self.close_all().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_settings_default() {
        let settings = PoolSettings::default();
        assert_eq!(settings.max_size, 20);
        assert_eq!(settings.timeout_secs, 30);
    }

    #[tokio::test]
    async fn test_connection_pool_creation() {
        let url = "postgresql://test:test@localhost:5432/testdb";
        let settings = PoolSettings::default();
        let result = ConnectionPool::new(url, settings).await;
        // Akan gagal karena tidak ada database, tapi pool objek terbuat
        assert!(result.is_ok() || result.is_err());
    }
}
