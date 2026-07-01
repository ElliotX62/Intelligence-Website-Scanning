// utils/request_handler.rs
// IWS v1.0 - Request Handler
// Mengelola semua HTTP request dengan session pooling, retry mechanism, dan timeout

use reqwest::{
    Client, ClientBuilder, Response, RequestBuilder, StatusCode,
    header::{HeaderMap, HeaderValue, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE, ACCEPT_ENCODING, CONTENT_TYPE},
    redirect, cookie,
};
use std::time::Duration;
use std::collections::HashMap;
use tokio::time::timeout;
use rand::Rng;

// ============================================================
// REQUEST CONFIG
// ============================================================

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub timeout_secs: u64,
    pub connect_timeout_secs: u64,
    pub read_timeout_secs: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub max_redirects: usize,
    pub user_agent: String,
    pub headers: HashMap<String, String>,
    pub proxy_url: Option<String>,
    pub verify_ssl: bool,
    pub enable_compression: bool,
    pub enable_cookies: bool,
}

impl Default for RequestConfig {
    fn default() -> Self {
        RequestConfig {
            timeout_secs: 30,
            connect_timeout_secs: 10,
            read_timeout_secs: 20,
            max_retries: 3,
            retry_delay_ms: 1000,
            max_redirects: 10,
            user_agent: "Mozilla/5.0 (compatible; IWS/1.0)".to_string(),
            headers: HashMap::new(),
            proxy_url: None,
            verify_ssl: true,
            enable_compression: true,
            enable_cookies: true,
        }
    }
}

// ============================================================
// REQUEST HANDLER
// ============================================================

#[derive(Debug, Clone)]
pub struct RequestHandler {
    client: Client,
    config: RequestConfig,
    cookie_store: cookie::Jar,
}

impl RequestHandler {
    /// Buat RequestHandler baru
    pub fn new(config: RequestConfig) -> Result<Self, String> {
        let mut builder = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout_secs))
            .connect_timeout(Duration::from_secs(config.connect_timeout_secs))
            .read_timeout(Duration::from_secs(config.read_timeout_secs))
            .redirect(redirect::Policy::limited(config.max_redirects))
            .danger_accept_invalid_certs(!config.verify_ssl)
            .user_agent(&config.user_agent);

        // Proxy
        if let Some(ref proxy_url) = config.proxy_url {
            if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
                builder = builder.proxy(proxy);
            }
        }

        // Compression
        if config.enable_compression {
            builder = builder.gzip(true).brotli(true);
        }

        let client = builder
            .build()
            .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

        Ok(RequestHandler {
            client,
            config,
            cookie_store: cookie::Jar::default(),
        })
    }

    /// Buat default handler
    pub fn default_handler() -> Result<Self, String> {
        RequestHandler::new(RequestConfig::default())
    }

    /// Kirim HTTP request dengan retry dan exponential backoff
    pub async fn send_request(
        &self,
        method: &str,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
    ) -> Result<RequestResult, String> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            match self.execute_request(method, url, headers.clone(), body.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) if attempt < self.config.max_retries && Self::is_retryable_error(&e) => {
                    let backoff = self.config.retry_delay_ms * 2u64.pow(attempt);
                    let jitter = rand::thread_rng().gen_range(0..=backoff / 4);
                    tokio::time::sleep(Duration::from_millis(backoff + jitter)).await;
                    last_error = Some(e);
                }
                Err(e) => return Err(e),
            }
        }

        Err(last_error.unwrap_or_else(|| "Request failed with no error".to_string()))
    }

    /// Eksekusi single HTTP request
    async fn execute_request(
        &self,
        method: &str,
        url: &str,
        headers: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
    ) -> Result<RequestResult, String> {
        let start = std::time::Instant::now();

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            "PATCH" => self.client.patch(url),
            "HEAD" => self.client.head(url),
            "OPTIONS" => self.client.request(reqwest::Method::OPTIONS, url),
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
        };

        // Custom headers
        if let Some(hdrs) = headers {
            for (key, value) in &hdrs {
                request = request.header(key.as_str(), value.as_str());
            }
        }

        // Body
        if let Some(data) = body {
            request = request.body(data);
        }

        // Timeout wrapper
        match timeout(Duration::from_secs(self.config.timeout_secs), request.send()).await {
            Ok(Ok(response)) => {
                let elapsed = start.elapsed();
                let status = response.status().as_u16();
                let resp_headers: HashMap<String, String> = response
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();
                let content_length = response.content_length();
                let body_bytes = response.bytes().await.map_err(|e| format!("Read body: {}", e))?;

                Ok(RequestResult {
                    url: url.to_string(),
                    method: method.to_string(),
                    status_code: status,
                    headers: resp_headers,
                    body: body_bytes.to_vec(),
                    elapsed_ms: elapsed.as_millis() as u64,
                    content_length,
                    retry_count: 0,
                    cached: false,
                })
            }
            Ok(Err(e)) => {
                Err(format!("Request failed: {}", e))
            }
            Err(_) => {
                Err(format!("Request timed out after {}s", self.config.timeout_secs))
            }
        }
    }

    /// GET request
    pub async fn get(&self, url: &str) -> Result<RequestResult, String> {
        self.send_request("GET", url, None, None).await
    }

    /// POST request dengan JSON body
    pub async fn post_json(&self, url: &str, json: &serde_json::Value) -> Result<RequestResult, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let body = serde_json::to_vec(json).map_err(|e| format!("JSON serialize: {}", e))?;
        self.send_request("POST", url, Some(headers), Some(body)).await
    }

    /// POST request dengan form data
    pub async fn post_form(&self, url: &str, form: &HashMap<String, String>) -> Result<RequestResult, String> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());
        let body: String = form.iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&");
        self.send_request("POST", url, Some(headers), Some(body.into_bytes())).await
    }

    /// Handle redirects manually
    pub async fn follow_redirects(&self, url: &str, max_hops: usize) -> Result<Vec<RequestResult>, String> {
        let mut results = Vec::new();
        let mut current_url = url.to_string();
        let mut hops = 0;

        while hops < max_hops {
            let result = self.send_request("GET", &current_url, None, None).await?;
            let status = result.status_code;

            results.push(result.clone());
            hops += 1;

            if status == 301 || status == 302 || status == 303 || status == 307 || status == 308 {
                if let Some(location) = result.headers.get("location") {
                    current_url = location.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok(results)
    }

    /// Cek apakah error bisa di-retry
    fn is_retryable_error(error: &str) -> bool {
        let retryable = [
            "timed out", "connection refused", "connection reset",
            "broken pipe", "dns", "tls", "ssl",
            "500", "502", "503", "504", "429",
        ];
        let lower = error.to_lowercase();
        retryable.iter().any(|&r| lower.contains(r))
    }

    /// Update User-Agent
    pub fn set_user_agent(&mut self, user_agent: &str) {
        self.config.user_agent = user_agent.to_string();
        // Rebuild client diperlukan untuk perubahan User-Agent
    }

    /// Dapatkan cookie store
    pub fn get_cookies(&self, url: &str) -> Vec<String> {
        if let Ok(parsed) = url::Url::parse(url) {
            self.cookie_store.cookies(&parsed)
                .map(|c| c.to_string())
                .unwrap_or_default()
        } else {
            vec![]
        }
    }

    /// Clear cookies
    pub fn clear_cookies(&self) {
        // Cookie jar tidak mendukung clear, perlu recreate
    }

    /// Health check — test koneksi ke URL
    pub async fn health_check(&self, url: &str) -> Result<bool, String> {
        match self.send_request("HEAD", url, None, None).await {
            Ok(result) => Ok(result.status_code < 500),
            Err(_) => Ok(false),
        }
    }

    /// Download file
    pub async fn download_file(&self, url: &str, output_path: &std::path::Path) -> Result<u64, String> {
        let result = self.get(url).await?;
        std::fs::write(output_path, &result.body)
            .map_err(|e| format!("Cannot write file: {}", e))?;
        Ok(result.body.len() as u64)
    }
}

// ============================================================
// REQUEST RESULT
// ============================================================

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub url: String,
    pub method: String,
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub elapsed_ms: u64,
    pub content_length: Option<u64>,
    pub retry_count: u32,
    pub cached: bool,
}

impl RequestResult {
    /// Cek apakah response sukses (2xx)
    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    /// Cek apakah response redirect (3xx)
    pub fn is_redirect(&self) -> bool {
        self.status_code >= 300 && self.status_code < 400
    }

    /// Cek apakah response client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }

    /// Cek apakah response server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status_code >= 500
    }

    /// Dapatkan header spesifik
    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }

    /// Parse body sebagai JSON
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, String> {
        serde_json::from_slice(&self.body)
            .map_err(|e| format!("JSON parse error: {}", e))
    }

    /// Parse body sebagai string
    pub fn text(&self) -> Result<String, String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| format!("UTF-8 decode error: {}", e))
    }

    /// Dapatkan content type
    pub fn content_type(&self) -> Option<&String> {
        self.headers.get("content-type")
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_config_default() {
        let config = RequestConfig::default();
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.max_retries, 3);
        assert!(config.verify_ssl);
    }

    #[test]
    fn test_request_handler_creation() {
        let handler = RequestHandler::default_handler();
        assert!(handler.is_ok());
    }

    #[test]
    fn test_is_retryable_error() {
        assert!(RequestHandler::is_retryable_error("Connection timed out"));
        assert!(RequestHandler::is_retryable_error("503 Service Unavailable"));
        assert!(RequestHandler::is_retryable_error("429 Too Many Requests"));
        assert!(!RequestHandler::is_retryable_error("404 Not Found"));
        assert!(!RequestHandler::is_retryable_error("Invalid URL"));
    }

    #[test]
    fn test_request_result_success() {
        let result = RequestResult {
            url: "https://example.com".to_string(),
            method: "GET".to_string(),
            status_code: 200,
            headers: HashMap::new(),
            body: b"OK".to_vec(),
            elapsed_ms: 50,
            content_length: Some(2),
            retry_count: 0,
            cached: false,
        };
        assert!(result.is_success());
        assert!(!result.is_redirect());
        assert!(!result.is_client_error());
        assert!(!result.is_server_error());
    }

    #[test]
    fn test_request_result_error_types() {
        let redirect = RequestResult {
            url: "".into(), method: "".into(), status_code: 302,
            headers: HashMap::new(), body: vec![], elapsed_ms: 0,
            content_length: None, retry_count: 0, cached: false,
        };
        assert!(redirect.is_redirect());

        let client_err = RequestResult {
            url: "".into(), method: "".into(), status_code: 404,
            headers: HashMap::new(), body: vec![], elapsed_ms: 0,
            content_length: None, retry_count: 0, cached: false,
        };
        assert!(client_err.is_client_error());

        let server_err = RequestResult {
            url: "".into(), method: "".into(), status_code: 500,
            headers: HashMap::new(), body: vec![], elapsed_ms: 0,
            content_length: None, retry_count: 0, cached: false,
        };
        assert!(server_err.is_server_error());
    }

    #[test]
    fn test_request_result_json() {
        let result = RequestResult {
            url: "".into(), method: "".into(), status_code: 200,
            headers: HashMap::new(),
            body: br#"{"key": "value", "num": 42}"#.to_vec(),
            elapsed_ms: 0, content_length: None, retry_count: 0, cached: false,
        };
        let parsed: serde_json::Value = result.json().unwrap();
        assert_eq!(parsed["key"], "value");
        assert_eq!(parsed["num"], 42);
    }

    #[test]
    fn test_request_result_text() {
        let result = RequestResult {
            url: "".into(), method: "".into(), status_code: 200,
            headers: HashMap::new(),
            body: b"Hello World".to_vec(),
            elapsed_ms: 0, content_length: None, retry_count: 0, cached: false,
        };
        assert_eq!(result.text().unwrap(), "Hello World");
    }
}
