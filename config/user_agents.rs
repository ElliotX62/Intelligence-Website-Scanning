// config/user_agents.rs
// IWS v1.0 - User Agents Configuration
// Database User-Agent strings untuk rotasi saat scanning

use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;

// ============================================================
// USER AGENTS STRUCT
// ============================================================

#[derive(Debug, Clone)]
pub struct UserAgents {
    agents: Vec<String>,
    current_index: AtomicUsize,
}

impl UserAgents {
    /// Load User-Agent list dari file, fallback ke built-in list
    pub fn load(path: Option<&Path>) -> Self {
        let agents = match path {
            Some(p) if p.exists() => {
                fs::read_to_string(p)
                    .map(|content| {
                        content
                            .lines()
                            .map(|l| l.trim().to_string())
                            .filter(|l| !l.is_empty() && !l.starts_with('#'))
                            .collect()
                    })
                    .unwrap_or_else(|_| UserAgents::built_in())
            }
            _ => UserAgents::built_in(),
        };

        UserAgents {
            agents,
            current_index: AtomicUsize::new(0),
        }
    }

    /// Built-in User-Agent list (Chrome, Firefox, Safari, Edge, Opera, Mobile)
    fn built_in() -> Vec<String> {
        vec![
            // Chrome 120-124 (Windows, Mac, Linux)
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36".to_string(),
            // Firefox 120-123 (Windows, Mac, Linux)
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:123.0) Gecko/20100101 Firefox/123.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:122.0) Gecko/20100101 Firefox/122.0".to_string(),
            "Mozilla/5.0 (X11; Linux i686; rv:123.0) Gecko/20100101 Firefox/123.0".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0".to_string(),
            // Safari 17.x (Mac, iOS)
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15".to_string(),
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            "Mozilla/5.0 (iPad; CPU OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Mobile/15E148 Safari/604.1".to_string(),
            // Edge 120-123 (Windows)
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.0.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.0.0".to_string(),
            // Opera 106-107 (Windows, Mac)
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 OPR/107.0.0.0".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 OPR/106.0.0.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 OPR/106.0.0.0".to_string(),
            // Mobile User-Agents (Android, iOS)
            "Mozilla/5.0 (Linux; Android 14; Pixel 8 Pro) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.6367.82 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; Android 13; SM-S908B) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.6312.80 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (Linux; Android 14; Pixel 7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.6261.119 Mobile Safari/537.36".to_string(),
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_2 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/124.0.6367.82 Mobile/15E148 Safari/604.1".to_string(),
            // Generic / Bots (untuk variasi)
            "Mozilla/5.0 (compatible; IWS-Scanner/1.0; +https://iws.local/bot)".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:120.0) Gecko/20100101 Firefox/120.0".to_string(),
        ]
    }

    /// Dapatkan random User-Agent
    pub fn get_random_agent(&self) -> String {
        if self.agents.is_empty() {
            return "Mozilla/5.0 (compatible; IWS/1.0)".to_string();
        }
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.agents.len());
        self.agents[idx].clone()
    }

    /// Dapatkan User-Agent berikutnya (round-robin)
    pub fn get_next_agent(&self) -> String {
        if self.agents.is_empty() {
            return "Mozilla/5.0 (compatible; IWS/1.0)".to_string();
        }
        let idx = self.current_index.fetch_add(1, Ordering::Relaxed) % self.agents.len();
        self.agents[idx].clone()
    }

    /// Dapatkan User-Agent spesifik platform
    pub fn get_agent_by_platform(&self, platform: &str) -> String {
        let filtered: Vec<&String> = self.agents
            .iter()
            .filter(|ua| {
                let ua_lower = ua.to_lowercase();
                match platform.to_lowercase().as_str() {
                    "windows" => ua_lower.contains("windows nt"),
                    "mac" | "macos" => ua_lower.contains("macintosh") || ua_lower.contains("mac os x"),
                    "linux" => ua_lower.contains("linux") && !ua_lower.contains("android"),
                    "android" => ua_lower.contains("android"),
                    "ios" | "iphone" | "ipad" => ua_lower.contains("iphone") || ua_lower.contains("ipad"),
                    _ => true,
                }
            })
            .collect();

        if filtered.is_empty() {
            return self.get_random_agent();
        }

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..filtered.len());
        filtered[idx].clone()
    }

    /// Dapatkan User-Agent spesifik browser
    pub fn get_agent_by_browser(&self, browser: &str) -> String {
        let filtered: Vec<&String> = self.agents
            .iter()
            .filter(|ua| {
                let ua_lower = ua.to_lowercase();
                match browser.to_lowercase().as_str() {
                    "chrome" => ua_lower.contains("chrome") && !ua_lower.contains("edg") && !ua_lower.contains("opr"),
                    "firefox" => ua_lower.contains("firefox"),
                    "safari" => ua_lower.contains("safari") && !ua_lower.contains("chrome") && !ua_lower.contains("crios"),
                    "edge" => ua_lower.contains("edg"),
                    "opera" => ua_lower.contains("opr"),
                    _ => true,
                }
            })
            .collect();

        if filtered.is_empty() {
            return self.get_random_agent();
        }

        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..filtered.len());
        filtered[idx].clone()
    }

    /// Refresh User-Agent list dari file
    pub fn refresh(&mut self, path: Option<&Path>) {
        let new_agents = match path {
            Some(p) if p.exists() => {
                fs::read_to_string(p)
                    .map(|content| {
                        content
                            .lines()
                            .map(|l| l.trim().to_string())
                            .filter(|l| !l.is_empty() && !l.starts_with('#'))
                            .collect()
                    })
                    .unwrap_or_else(|_| UserAgents::built_in())
            }
            _ => UserAgents::built_in(),
        };
        self.agents = new_agents;
        self.current_index.store(0, Ordering::Relaxed);
    }

    /// Jumlah User-Agent yang tersedia
    pub fn count(&self) -> usize {
        self.agents.len()
    }

    /// Cek apakah list kosong
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Dapatkan semua agents
    pub fn all(&self) -> &[String] {
        &self.agents
    }

    /// Tambah custom User-Agent
    pub fn add_custom(&mut self, user_agent: &str) {
        if !user_agent.is_empty() && !self.agents.contains(&user_agent.to_string()) {
            self.agents.push(user_agent.to_string());
        }
    }

    /// Hapus User-Agent
    pub fn remove(&mut self, user_agent: &str) -> bool {
        if let Some(pos) = self.agents.iter().position(|ua| ua == user_agent) {
            self.agents.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for UserAgents {
    fn default() -> Self {
        UserAgents::load(None)
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_built_in() {
        let agents = UserAgents::load(None);
        assert!(!agents.is_empty());
        assert!(agents.count() >= 20);
    }

    #[test]
    fn test_get_random_agent() {
        let agents = UserAgents::default();
        let ua1 = agents.get_random_agent();
        let ua2 = agents.get_random_agent();
        assert!(!ua1.is_empty());
        assert!(!ua2.is_empty());
        // Mungkin sama (random), tapi tidak boleh kosong
    }

    #[test]
    fn test_get_next_agent_rotation() {
        let agents = UserAgents::default();
        let ua1 = agents.get_next_agent();
        let ua2 = agents.get_next_agent();
        // Dalam round-robin, ua1 dan ua2 mungkin berbeda jika agents.len() > 1
        assert!(!ua1.is_empty());
        assert!(!ua2.is_empty());
    }

    #[test]
    fn test_get_agent_by_platform_windows() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_platform("windows");
        assert!(ua.to_lowercase().contains("windows"));
    }

    #[test]
    fn test_get_agent_by_platform_mac() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_platform("mac");
        assert!(ua.to_lowercase().contains("macintosh") || ua.to_lowercase().contains("mac os x"));
    }

    #[test]
    fn test_get_agent_by_platform_linux() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_platform("linux");
        assert!(ua.to_lowercase().contains("linux"));
        assert!(!ua.to_lowercase().contains("android"));
    }

    #[test]
    fn test_get_agent_by_platform_android() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_platform("android");
        assert!(ua.to_lowercase().contains("android"));
    }

    #[test]
    fn test_get_agent_by_browser_chrome() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_browser("chrome");
        assert!(ua.to_lowercase().contains("chrome"));
        assert!(!ua.to_lowercase().contains("edg"));
        assert!(!ua.to_lowercase().contains("opr"));
    }

    #[test]
    fn test_get_agent_by_browser_firefox() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_browser("firefox");
        assert!(ua.to_lowercase().contains("firefox"));
    }

    #[test]
    fn test_get_agent_by_browser_edge() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_browser("edge");
        assert!(ua.to_lowercase().contains("edg"));
    }

    #[test]
    fn test_get_agent_by_browser_safari() {
        let agents = UserAgents::default();
        let ua = agents.get_agent_by_browser("safari");
        assert!(ua.to_lowercase().contains("safari"));
        // Safari UA tidak boleh mengandung Chrome (kecuali CriOS)
        assert!(!ua.to_lowercase().contains("chrome") || ua.to_lowercase().contains("crios"));
    }

    #[test]
    fn test_add_custom_agent() {
        let mut agents = UserAgents::default();
        let initial = agents.count();
        agents.add_custom("MyCustomAgent/1.0");
        assert_eq!(agents.count(), initial + 1);
    }

    #[test]
    fn test_add_duplicate_agent() {
        let mut agents = UserAgents::default();
        let first = agents.get_random_agent();
        let initial = agents.count();
        agents.add_custom(&first);
        assert_eq!(agents.count(), initial); // Tidak bertambah (duplicate)
    }

    #[test]
    fn test_remove_agent() {
        let mut agents = UserAgents::default();
        let first = agents.get_random_agent();
        let initial = agents.count();
        assert!(agents.remove(&first));
        assert_eq!(agents.count(), initial - 1);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut agents = UserAgents::default();
        assert!(!agents.remove("NonexistentAgent/1.0"));
    }

    #[test]
    fn test_refresh() {
        let mut agents = UserAgents::default();
        let initial = agents.count();
        agents.refresh(None);
        assert_eq!(agents.count(), initial); // Refresh dari built-in
    }

    #[test]
    fn test_count_and_is_empty() {
        let agents = UserAgents::default();
        assert!(agents.count() > 0);
        assert!(!agents.is_empty());
    }

    #[test]
    fn test_all_agents() {
        let agents = UserAgents::default();
        let all = agents.all();
        assert_eq!(all.len(), agents.count());
    }

    #[test]
    fn test_round_robin_wraps() {
        let agents = UserAgents::default();
        let total = agents.count();
        for _ in 0..total + 5 {
            agents.get_next_agent();
        }
        // Should not panic, index wraps around
    }
}
