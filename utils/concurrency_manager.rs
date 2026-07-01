// utils/concurrency_manager.rs
// IWS v1.0 - Concurrency Manager
// Mengelola konkurensi dengan async/await, thread pooling, dan semaphore

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::{Semaphore, RwLock};
use tokio::task::JoinHandle;
use std::future::Future;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ConcurrencyManager {
    semaphore: Arc<Semaphore>,
    task_counter: Arc<AtomicUsize>,
    max_tasks: usize,
    shutdown_flag: Arc<AtomicUsize>,
    stats: Arc<RwLock<ConcurrencyStats>>,
}

#[derive(Debug, Clone, Default)]
pub struct ConcurrencyStats {
    pub tasks_spawned: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub tasks_rejected: u64,
    pub peak_concurrent: usize,
}

impl ConcurrencyManager {
    pub fn new(max_concurrent: usize, max_queued: usize) -> Self {
        ConcurrencyManager {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            task_counter: Arc::new(AtomicUsize::new(0)),
            max_tasks: max_concurrent + max_queued,
            shutdown_flag: Arc::new(AtomicUsize::new(0)),
            stats: Arc::new(RwLock::new(ConcurrencyStats::default())),
        }
    }

    pub async fn spawn<F, T>(&self, future: F) -> Result<JoinHandle<T>, String>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        if self.shutdown_flag.load(Ordering::Relaxed) == 1 {
            return Err("ConcurrencyManager is shutting down".to_string());
        }

        let current = self.task_counter.load(Ordering::Relaxed);
        if current >= self.max_tasks {
            self.update_stats_rejected().await;
            return Err("Task queue full".to_string());
        }

        self.task_counter.fetch_add(1, Ordering::Relaxed);
        self.update_peak(current + 1).await;
        self.update_stats_spawned().await;

        let permit = self.semaphore.clone().acquire_owned().await
            .map_err(|_| "Semaphore closed".to_string())?;

        let counter = self.task_counter.clone();
        let stats = self.stats.clone();
        let shutdown = self.shutdown_flag.clone();

        let handle = tokio::spawn(async move {
            let result = future.await;
            drop(permit);
            counter.fetch_sub(1, Ordering::Relaxed);

            // Update stats
            if let Ok(mut s) = stats.try_write() {
                s.tasks_completed += 1;
            }

            result
        });

        Ok(handle)
    }

    pub async fn spawn_blocking<F, T>(&self, f: F) -> Result<JoinHandle<T>, String>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        if self.shutdown_flag.load(Ordering::Relaxed) == 1 {
            return Err("ConcurrencyManager is shutting down".to_string());
        }

        let current = self.task_counter.load(Ordering::Relaxed);
        if current >= self.max_tasks {
            return Err("Task queue full".to_string());
        }

        self.task_counter.fetch_add(1, Ordering::Relaxed);
        self.update_stats_spawned().await;

        let _permit = self.semaphore.clone().acquire_owned().await
            .map_err(|_| "Semaphore closed".to_string())?;

        let counter = self.task_counter.clone();
        let stats = self.stats.clone();

        let handle = tokio::task::spawn_blocking(move || {
            let result = f();
            counter.fetch_sub(1, Ordering::Relaxed);
            // Note: permit auto-drops when _permit goes out of scope
            result
        });

        Ok(handle)
    }

    pub async fn active_tasks(&self) -> usize {
        self.task_counter.load(Ordering::Relaxed)
    }

    pub async fn available_permits(&self) -> usize {
        self.semaphore.available_permits()
    }

    pub async fn is_at_capacity(&self) -> bool {
        self.task_counter.load(Ordering::Relaxed) >= self.max_tasks
    }

    pub async fn shutdown(&self, timeout: Duration) {
        self.shutdown_flag.store(1, Ordering::Relaxed);
        self.semaphore.close();

        let start = std::time::Instant::now();
        while self.task_counter.load(Ordering::Relaxed) > 0 {
            if start.elapsed() > timeout {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }

    pub async fn get_stats(&self) -> ConcurrencyStats {
        self.stats.read().await.clone()
    }

    async fn update_stats_spawned(&self) {
        if let Ok(mut s) = self.stats.try_write() {
            s.tasks_spawned += 1;
        }
    }

    async fn update_stats_rejected(&self) {
        if let Ok(mut s) = self.stats.try_write() {
            s.tasks_rejected += 1;
        }
    }

    async fn update_peak(&self, current: usize) {
        if let Ok(mut s) = self.stats.try_write() {
            if current > s.peak_concurrent {
                s.peak_concurrent = current;
            }
        }
    }
}

impl Default for ConcurrencyManager {
    fn default() -> Self {
        ConcurrencyManager::new(100, 500)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_spawn_and_complete() {
        let cm = ConcurrencyManager::new(10, 100);
        let handle = cm.spawn(async { 42 }).await.unwrap();
        let result = handle.await.unwrap();
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_concurrent_limit() {
        let cm = ConcurrencyManager::new(2, 10);
        // Spawn 2 tasks → harusnya ok
        let h1 = cm.spawn(async { tokio::time::sleep(Duration::from_millis(100)).await; 1 }).await.unwrap();
        let h2 = cm.spawn(async { tokio::time::sleep(Duration::from_millis(100)).await; 2 }).await.unwrap();
        assert!(cm.active_tasks().await >= 2);
        let (r1, r2) = tokio::join!(h1, h2);
        assert_eq!(r1.unwrap(), 1);
        assert_eq!(r2.unwrap(), 2);
    }

    #[tokio::test]
    async fn test_spawn_blocking() {
        let cm = ConcurrencyManager::new(5, 50);
        let handle = cm.spawn_blocking(|| "blocking result".to_string()).await.unwrap();
        let result = handle.await.unwrap();
        assert_eq!(result, "blocking result");
    }

    #[tokio::test]
    async fn test_shutdown() {
        let cm = ConcurrencyManager::new(10, 50);
        cm.shutdown(Duration::from_secs(1)).await;
        let result = cm.spawn(async { 1 }).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stats() {
        let cm = ConcurrencyManager::new(10, 100);
        for _ in 0..5 {
            cm.spawn(async { 1 }).await.unwrap();
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
        let stats = cm.get_stats().await;
        assert!(stats.tasks_spawned >= 5);
    }
}
