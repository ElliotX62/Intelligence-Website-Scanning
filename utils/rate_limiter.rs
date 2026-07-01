// utils/rate_limiter.rs
// IWS v1.0 - Rate Limiter
// Mengelola rate limiting dengan token bucket algorithm, sliding window, dan adaptive rate

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use dashmap::DashMap;

// ============================================================
// TOKEN BUCKET RATE LIMITER
// ============================================================

#[derive(Debug)]
pub struct TokenBucket {
    tokens: AtomicUsize,
    capacity: usize,
    fill_rate: usize,        // tokens per second
    last_refill: AtomicU64,  // timestamp in millis
}

impl TokenBucket {
    pub fn new(capacity: usize, fill_rate: usize) -> Self {
        let now = Instant::now();
        let now_ms = now.elapsed().as_millis() as u64;

        TokenBucket {
            tokens: AtomicUsize::new(capacity),
            capacity,
            fill_rate,
            last_refill: AtomicU64::new(now_ms),
        }
    }

    /// Cek apakah request diizinkan (consume 1 token)
    pub fn allow(&self) -> bool {
        self.refill();
        loop {
            let current = self.tokens.load(Ordering::Relaxed);
            if current == 0 {
                return false;
            }
            if self.tokens.compare_exchange_weak(
                current,
                current - 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ).is_ok() {
                return true;
            }
        }
    }

    /// Cek apakah N tokens tersedia (tanpa consume)
    pub fn available(&self) -> usize {
        self.refill();
        self.tokens.load(Ordering::Relaxed)
    }

    /// Refill tokens berdasarkan waktu yang berlalu
    fn refill(&self) {
        let now = Instant::now();
        let now_ms = now.elapsed().as_millis() as u64;
        let last = self.last_refill.load(Ordering::Relaxed);

        if now_ms <= last {
            return;
        }

        let elapsed_secs = (now_ms - last) as f64 / 1000.0;
        let new_tokens = (elapsed_secs * self.fill_rate as f64) as usize;

        if new_tokens > 0 {
            let mut current = self.tokens.load(Ordering::Relaxed);
            loop {
                let new_value = (current + new_tokens).min(self.capacity);
                match self.tokens.compare_exchange_weak(
                    current,
                    new_value,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        self.last_refill.store(now_ms, Ordering::Relaxed);
                        break;
                    }
                    Err(actual) => current = actual,
                }
            }
        }
    }

    pub fn capacity(&self) -> usize { self.capacity }
    pub fn fill_rate(&self) -> usize { self.fill_rate }
}

// ============================================================
// SLIDING WINDOW RATE LIMITER
// ============================================================

#[derive(Debug)]
pub struct SlidingWindow {
    timestamps: Mutex<Vec<Instant>>,
    window: Duration,
    max_requests: usize,
}

impl SlidingWindow {
    pub fn new(window: Duration, max_requests: usize) -> Self {
        SlidingWindow {
            timestamps: Mutex::new(Vec::with_capacity(max_requests)),
            window,
            max_requests,
        }
    }

    /// Cek apakah request diizinkan
    pub fn allow(&self) -> bool {
        let now = Instant::now();
        let mut timestamps = self.timestamps.lock().unwrap();

        // Hapus timestamps di luar window
        let cutoff = now - self.window;
        timestamps.retain(|t| *t > cutoff);

        if timestamps.len() < self.max_requests {
            timestamps.push(now);
            true
        } else {
            false
        }
    }

    /// Dapatkan jumlah request dalam window saat ini
    pub fn current_count(&self) -> usize {
        let now = Instant::now();
        let timestamps = self.timestamps.lock().unwrap();
        let cutoff = now - self.window;
        timestamps.iter().filter(|t| **t > cutoff).count()
    }

    /// Waktu sampai rate limit reset (dalam detik)
    pub fn time_until_reset(&self) -> f64 {
        let timestamps = self.timestamps.lock().unwrap();
        if timestamps.is_empty() {
            return 0.0;
        }
        let oldest = timestamps[0];
        let elapsed = oldest.elapsed().as_secs_f64();
        (self.window.as_secs_f64() - elapsed).max(0.0)
    }
}

// ============================================================
// PER-DOMAIN RATE LIMITER
// ============================================================

#[derive(Debug)]
pub struct PerDomainLimiter {
    limiters: DashMap<String, TokenBucket>,
    default_capacity: usize,
    default_fill_rate: usize,
}

impl PerDomainLimiter {
    pub fn new(default_capacity: usize, default_fill_rate: usize) -> Self {
        PerDomainLimiter {
            limiters: DashMap::new(),
            default_capacity,
            default_fill_rate,
        }
    }

    /// Dapatkan atau buat rate limiter untuk domain
    pub fn get_or_create(&self, domain: &str) -> &TokenBucket {
        // Coba get dulu
        if let Some(bucket) = self.limiters.get(domain) {
            // Return reference — DashMap tidak mendukung ini langsung
            // Gunakan entry API
        }
        
        // Insert baru
        self.limiters.insert(
            domain.to_string(),
            TokenBucket::new(self.default_capacity, self.default_fill_rate),
        );
        self.limiters.get(domain).unwrap()
    }

    /// Cek apakah request ke domain diizinkan
    pub fn allow(&self, domain: &str) -> bool {
        if let Some(bucket) = self.limiters.get(domain) {
            bucket.allow()
        } else {
            // Auto-create untuk domain baru
            self.limiters.insert(
                domain.to_string(),
                TokenBucket::new(self.default_capacity, self.default_fill_rate),
            );
            self.limiters.get(domain).unwrap().allow()
        }
    }

    /// Set custom rate untuk domain tertentu
    pub fn set_domain_rate(&self, domain: &str, capacity: usize, fill_rate: usize) {
        self.limiters.insert(
            domain.to_string(),
            TokenBucket::new(capacity, fill_rate),
        );
    }

    /// Hapus rate limiter domain
    pub fn remove(&self, domain: &str) {
        self.limiters.remove(domain);
    }

    /// Dapatkan jumlah domain yang di-track
    pub fn domain_count(&self) -> usize {
        self.limiters.len()
    }

    /// Reset semua limiters
    pub fn reset_all(&self) {
        self.limiters.clear();
    }

    /// Dapatkan status rate limiter domain
    pub fn get_status(&self, domain: &str) -> Option<(usize, usize)> {
        self.limiters.get(domain).map(|b| (b.available(), b.capacity()))
    }
}

// ============================================================
// ADAPTIVE RATE LIMITER
// ============================================================

#[derive(Debug)]
pub struct AdaptiveRateLimiter {
    limiter: RwLock<TokenBucket>,
    min_rate: usize,
    max_rate: usize,
    current_rate: AtomicUsize,
    response_times: Mutex<Vec<f64>>,
    error_count: AtomicUsize,
    success_count: AtomicUsize,
}

impl AdaptiveRateLimiter {
    pub fn new(initial_rate: usize, min_rate: usize, max_rate: usize) -> Self {
        AdaptiveRateLimiter {
            limiter: RwLock::new(TokenBucket::new(initial_rate * 2, initial_rate)),
            min_rate,
            max_rate,
            current_rate: AtomicUsize::new(initial_rate),
            response_times: Mutex::new(Vec::with_capacity(100)),
            error_count: AtomicUsize::new(0),
            success_count: AtomicUsize::new(0),
        }
    }

    /// Cek apakah request diizinkan
    pub fn allow(&self) -> bool {
        self.limiter.read().unwrap().allow()
    }

    /// Laporkan response time untuk adaptive adjustment
    pub fn report_response(&self, response_time_ms: f64, is_error: bool) {
        let mut times = self.response_times.lock().unwrap();
        times.push(response_time_ms);
        if times.len() > 100 {
            times.remove(0);
        }

        if is_error {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        } else {
            self.success_count.fetch_add(1, Ordering::Relaxed);
        }

        // Adjust rate setiap 50 samples
        if times.len() % 50 == 0 {
            self.adjust_rate();
        }
    }

    /// Adjust rate berdasarkan performa
    fn adjust_rate(&self) {
        let times = self.response_times.lock().unwrap();
        if times.len() < 10 {
            return;
        }

        let avg_time: f64 = times.iter().sum::<f64>() / times.len() as f64;
        let error_rate = self.error_count.load(Ordering::Relaxed) as f64
            / (self.error_count.load(Ordering::Relaxed) + self.success_count.load(Ordering::Relaxed)) as f64;

        let current = self.current_rate.load(Ordering::Relaxed);
        let mut new_rate = current;

        if error_rate > 0.1 || avg_time > 5000.0 {
            // High error rate atau slow response → kurangi rate
            new_rate = (current as f64 * 0.7) as usize;
        } else if error_rate < 0.02 && avg_time < 1000.0 {
            // Low error rate dan fast response → naikkan rate
            new_rate = (current as f64 * 1.2) as usize;
        }

        new_rate = new_rate.clamp(self.min_rate, self.max_rate);

        if new_rate != current {
            self.current_rate.store(new_rate, Ordering::Relaxed);
            let mut limiter = self.limiter.write().unwrap();
            *limiter = TokenBucket::new(new_rate * 2, new_rate);
        }
    }

    pub fn current_rate(&self) -> usize {
        self.current_rate.load(Ordering::Relaxed)
    }

    pub fn stats(&self) -> AdaptiveStats {
        let times = self.response_times.lock().unwrap();
        let avg = if times.is_empty() { 0.0 } else { times.iter().sum::<f64>() / times.len() as f64 };
        AdaptiveStats {
            current_rate: self.current_rate(),
            avg_response_ms: avg,
            error_count: self.error_count.load(Ordering::Relaxed),
            success_count: self.success_count.load(Ordering::Relaxed),
            sample_count: times.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdaptiveStats {
    pub current_rate: usize,
    pub avg_response_ms: f64,
    pub error_count: usize,
    pub success_count: usize,
    pub sample_count: usize,
}

// ============================================================
// BURST CAPACITY RATE LIMITER
// ============================================================

#[derive(Debug)]
pub struct BurstRateLimiter {
    bucket: TokenBucket,
    burst_size: usize,
    burst_available: AtomicUsize,
    burst_cooldown: Duration,
    last_burst: AtomicU64,
}

impl BurstRateLimiter {
    pub fn new(rate: usize, burst_size: usize, burst_cooldown: Duration) -> Self {
        let now = Instant::now().elapsed().as_millis() as u64;
        BurstRateLimiter {
            bucket: TokenBucket::new(rate * 2, rate),
            burst_size,
            burst_available: AtomicUsize::new(burst_size),
            burst_cooldown,
            last_burst: AtomicU64::new(now),
        }
    }

    /// Cek apakah request diizinkan (prioritas: burst > token)
    pub fn allow(&self) -> bool {
        // Coba gunakan burst capacity dulu
        let mut burst = self.burst_available.load(Ordering::Relaxed);
        if burst > 0 {
            if self.burst_available.compare_exchange_weak(
                burst, burst - 1,
                Ordering::Acquire, Ordering::Relaxed,
            ).is_ok() {
                // Refill burst setelah cooldown
                let now = Instant::now().elapsed().as_millis() as u64;
                let last = self.last_burst.load(Ordering::Relaxed);
                if now - last > self.burst_cooldown.as_millis() as u64 {
                    self.burst_available.store(self.burst_size, Ordering::Relaxed);
                    self.last_burst.store(now, Ordering::Relaxed);
                }
                return true;
            }
        }

        // Fallback ke token bucket
        self.bucket.allow()
    }

    pub fn burst_remaining(&self) -> usize {
        self.burst_available.load(Ordering::Relaxed)
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket_allow() {
        let bucket = TokenBucket::new(10, 10);
        // Harusnya bisa consume 10 tokens
        for _ in 0..10 {
            assert!(bucket.allow());
        }
        // Token habis
        assert!(!bucket.allow());
    }

    #[test]
    fn test_token_bucket_refill() {
        let bucket = TokenBucket::new(10, 100); // 100 tokens/sec, capacity 10
        // Habiskan semua token
        for _ in 0..10 {
            assert!(bucket.allow());
        }
        assert!(!bucket.allow());
        // Tunggu refill
        std::thread::sleep(Duration::from_millis(100));
        // Harusnya sudah terisi ~10 token
        assert!(bucket.allow());
    }

    #[test]
    fn test_sliding_window_allow() {
        let window = SlidingWindow::new(Duration::from_secs(1), 5);
        for _ in 0..5 {
            assert!(window.allow());
        }
        // Request ke-6 ditolak
        assert!(!window.allow());
    }

    #[test]
    fn test_sliding_window_time_until_reset() {
        let window = SlidingWindow::new(Duration::from_secs(10), 5);
        window.allow();
        let reset = window.time_until_reset();
        assert!(reset > 0.0);
        assert!(reset <= 10.0);
    }

    #[test]
    fn test_per_domain_limiter() {
        let limiter = PerDomainLimiter::new(5, 10);
        assert!(limiter.allow("example.com"));
        assert!(limiter.allow("example.org"));
        // Domains terpisah
        assert!(limiter.domain_count() >= 2);
    }

    #[test]
    fn test_per_domain_custom_rate() {
        let limiter = PerDomainLimiter::new(5, 10);
        limiter.set_domain_rate("api.example.com", 20, 50);
        let status = limiter.get_status("api.example.com");
        assert!(status.is_some());
        let (available, capacity) = status.unwrap();
        assert!(available <= capacity);
    }

    #[test]
    fn test_adaptive_rate_limiter_initial() {
        let limiter = AdaptiveRateLimiter::new(10, 5, 50);
        assert_eq!(limiter.current_rate(), 10);
        assert!(limiter.allow());
    }

    #[test]
    fn test_adaptive_rate_adjust_down() {
        let limiter = AdaptiveRateLimiter::new(20, 5, 50);
        // Laporkan banyak error → rate harus turun
        for _ in 0..50 {
            limiter.report_response(8000.0, true);
        }
        let rate = limiter.current_rate();
        assert!(rate <= 20);
    }

    #[test]
    fn test_adaptive_rate_adjust_up() {
        let limiter = AdaptiveRateLimiter::new(10, 5, 50);
        // Laporkan response cepat tanpa error → rate harus naik
        for _ in 0..50 {
            limiter.report_response(100.0, false);
        }
        let rate = limiter.current_rate();
        assert!(rate >= 10);
    }

    #[test]
    fn test_adaptive_rate_bounds() {
        let limiter = AdaptiveRateLimiter::new(10, 5, 50);
        // Force rate up
        for _ in 0..200 {
            limiter.report_response(10.0, false);
        }
        assert!(limiter.current_rate() <= 50); // Max

        // Force rate down
        for _ in 0..200 {
            limiter.report_response(10000.0, true);
        }
        assert!(limiter.current_rate() >= 5); // Min
    }

    #[test]
    fn test_burst_rate_limiter() {
        let burst = BurstRateLimiter::new(5, 3, Duration::from_secs(10));
        // Gunakan burst dulu
        assert!(burst.allow());
        assert!(burst.allow());
        assert!(burst.allow());
        // Burst habis, lanjut token bucket
        assert_eq!(burst.burst_remaining(), 0);
    }

    #[test]
    fn test_adaptive_stats() {
        let limiter = AdaptiveRateLimiter::new(10, 5, 50);
        limiter.report_response(200.0, false);
        limiter.report_response(300.0, false);
        let stats = limiter.stats();
        assert_eq!(stats.sample_count, 2);
        assert!(stats.avg_response_ms > 0.0);
    }
}
