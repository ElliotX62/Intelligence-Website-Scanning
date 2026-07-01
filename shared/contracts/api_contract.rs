// shared/contracts/api_contract.rs
// IWS v1.0 - API Contract
// Mendefinisikan kontrak formal untuk semua API endpoints

use std::time::Duration;
use std::collections::HashMap;
use std::fmt;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// ============================================================
// API VERSION & FORMAT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ApiVersion {
    V1,
    V2,
    V3,
}

impl fmt::Display for ApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiVersion::V1 => write!(f, "v1"),
            ApiVersion::V2 => write!(f, "v2"),
            ApiVersion::V3 => write!(f, "v3"),
        }
    }
}

impl ApiVersion {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "v1" => Some(ApiVersion::V1),
            "v2" => Some(ApiVersion::V2),
            "v3" => Some(ApiVersion::V3),
            _ => None,
        }
    }

    pub fn latest() -> Self {
        ApiVersion::V1
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiFormat {
    Json,
    Yaml,
    MsgPack,
    Protobuf,
}

impl fmt::Display for ApiFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiFormat::Json => write!(f, "json"),
            ApiFormat::Yaml => write!(f, "yaml"),
            ApiFormat::MsgPack => write!(f, "msgpack"),
            ApiFormat::Protobuf => write!(f, "protobuf"),
        }
    }
}

impl Default for ApiFormat {
    fn default() -> Self {
        ApiFormat::Json
    }
}

// ============================================================
// AUTHENTICATION & AUTHORIZATION
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthMethod {
    None,
    ApiKey,
    BearerToken,
    Jwt,
    BasicAuth,
    OAuth2,
    Custom(String),
}

impl fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthMethod::None => write!(f, "none"),
            AuthMethod::ApiKey => write!(f, "api_key"),
            AuthMethod::BearerToken => write!(f, "bearer_token"),
            AuthMethod::Jwt => write!(f, "jwt"),
            AuthMethod::BasicAuth => write!(f, "basic_auth"),
            AuthMethod::OAuth2 => write!(f, "oauth2"),
            AuthMethod::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub method: AuthMethod,
    pub token: Option<String>,
    pub api_key: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub custom_fields: HashMap<String, String>,
}

impl AuthCredentials {
    pub fn api_key(key: &str) -> Self {
        AuthCredentials {
            method: AuthMethod::ApiKey,
            token: None,
            api_key: Some(key.to_string()),
            username: None,
            password: None,
            client_id: None,
            client_secret: None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn bearer_token(token: &str) -> Self {
        AuthCredentials {
            method: AuthMethod::BearerToken,
            token: Some(token.to_string()),
            api_key: None,
            username: None,
            password: None,
            client_id: None,
            client_secret: None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn jwt(token: &str) -> Self {
        AuthCredentials {
            method: AuthMethod::Jwt,
            token: Some(token.to_string()),
            api_key: None,
            username: None,
            password: None,
            client_id: None,
            client_secret: None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn basic_auth(username: &str, password: &str) -> Self {
        AuthCredentials {
            method: AuthMethod::BasicAuth,
            token: None,
            api_key: None,
            username: Some(username.to_string()),
            password: Some(password.to_string()),
            client_id: None,
            client_secret: None,
            custom_fields: HashMap::new(),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        match self.method {
            AuthMethod::None => true,
            AuthMethod::ApiKey => self.api_key.is_some(),
            AuthMethod::BearerToken | AuthMethod::Jwt => self.token.is_some(),
            AuthMethod::BasicAuth => self.username.is_some() && self.password.is_some(),
            AuthMethod::OAuth2 => self.client_id.is_some() && self.client_secret.is_some(),
            AuthMethod::Custom(_) => false,
        }
    }

    pub fn mask_sensitive(&self) -> Self {
        let mask = |s: &str| -> String {
            if s.len() <= 8 { "***".to_string() }
            else { format!("{}...{}", &s[..4], &s[s.len()-4..]) }
        };

        AuthCredentials {
            method: self.method.clone(),
            token: self.token.as_ref().map(|t| mask(t)),
            api_key: self.api_key.as_ref().map(|k| mask(k)),
            username: self.username.clone(),
            password: self.password.as_ref().map(|_| "***".to_string()),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.as_ref().map(|_| "***".to_string()),
            custom_fields: self.custom_fields.iter().map(|(k,_)| (k.clone(), "***".to_string())).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
    Guest,
    Custom(String),
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
            Role::Guest => write!(f, "guest"),
            Role::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

impl Role {
    pub fn can_scan(&self) -> bool {
        matches!(self, Role::Admin | Role::User)
    }

    pub fn can_configure(&self) -> bool {
        matches!(self, Role::Admin)
    }

    pub fn can_view_reports(&self) -> bool {
        true
    }

    pub fn rate_limit_per_hour(&self) -> u32 {
        match self {
            Role::Admin => 10000,
            Role::User => 1000,
            Role::Guest => 100,
            Role::Custom(_) => 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,
    pub action: Action,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Execute,
    Admin,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Create => write!(f, "create"),
            Action::Read => write!(f, "read"),
            Action::Update => write!(f, "update"),
            Action::Delete => write!(f, "delete"),
            Action::Execute => write!(f, "execute"),
            Action::Admin => write!(f, "admin"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub authenticated: bool,
    pub authorized: bool,
    pub user_id: Option<String>,
    pub role: Role,
    pub permissions: Vec<Permission>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub message: Option<String>,
}

impl AuthResult {
    pub fn success(user_id: &str, role: Role) -> Self {
        AuthResult {
            authenticated: true,
            authorized: true,
            user_id: Some(user_id.to_string()),
            role,
            permissions: vec![],
            token_expires_at: None,
            message: None,
        }
    }

    pub fn unauthorized(message: &str) -> Self {
        AuthResult {
            authenticated: false,
            authorized: false,
            user_id: None,
            role: Role::Guest,
            permissions: vec![],
            token_expires_at: None,
            message: Some(message.to_string()),
        }
    }

    pub fn forbidden(message: &str) -> Self {
        AuthResult {
            authenticated: true,
            authorized: false,
            user_id: None,
            role: Role::Guest,
            permissions: vec![],
            token_expires_at: None,
            message: Some(message.to_string()),
        }
    }
}

// ============================================================
// API REQUEST & RESPONSE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub request_id: Uuid,
    pub method: HttpMethod,
    pub path: String,
    pub version: ApiVersion,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub path_params: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub content_type: String,
    pub accept: ApiFormat,
    pub auth: Option<AuthCredentials>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl ApiRequest {
    pub fn new(method: HttpMethod, path: &str) -> Self {
        ApiRequest {
            request_id: Uuid::new_v4(),
            method,
            path: path.to_string(),
            version: ApiVersion::latest(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            path_params: HashMap::new(),
            body: None,
            content_type: "application/json".to_string(),
            accept: ApiFormat::Json,
            auth: None,
            client_ip: None,
            user_agent: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_auth(mut self, auth: AuthCredentials) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn with_json_body<T: Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(serde_json::to_vec(body)?);
        self.content_type = "application/json".to_string();
        Ok(self)
    }

    pub fn with_query_param(mut self, key: &str, value: &str) -> Self {
        self.query_params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_path_param(mut self, key: &str, value: &str) -> Self {
        self.path_params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn get_path_param(&self, key: &str) -> Option<&String> {
        self.path_params.get(key)
    }

    pub fn get_query_param(&self, key: &str) -> Option<&String> {
        self.query_params.get(key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T: Serialize + Clone> {
    pub status_code: u16,
    pub status_message: String,
    pub data: Option<T>,
    pub error: Option<ApiErrorResponse>,
    pub meta: Option<ResponseMetadata>,
    pub request_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub version: ApiVersion,
}

impl<T: Serialize + Clone> ApiResponse<T> {
    pub fn success(status_code: u16, data: T, request_id: Uuid) -> Self {
        ApiResponse {
            status_code,
            status_message: "OK".to_string(),
            data: Some(data),
            error: None,
            meta: None,
            request_id,
            timestamp: Utc::now(),
            version: ApiVersion::latest(),
        }
    }

    pub fn created(data: T, request_id: Uuid) -> Self {
        ApiResponse {
            status_code: 201,
            status_message: "Created".to_string(),
            data: Some(data),
            error: None,
            meta: None,
            request_id,
            timestamp: Utc::now(),
            version: ApiVersion::latest(),
        }
    }

    pub fn no_content(request_id: Uuid) -> Self {
        ApiResponse {
            status_code: 204,
            status_message: "No Content".to_string(),
            data: None,
            error: None,
            meta: None,
            request_id,
            timestamp: Utc::now(),
            version: ApiVersion::latest(),
        }
    }

    pub fn error(status_code: u16, error: ApiErrorResponse, request_id: Uuid) -> Self {
        ApiResponse {
            status_code,
            status_message: error.message.clone(),
            data: None,
            error: Some(error),
            meta: None,
            request_id,
            timestamp: Utc::now(),
            version: ApiVersion::latest(),
        }
    }

    pub fn with_meta(mut self, meta: ResponseMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    pub fn is_client_error(&self) -> bool {
        self.status_code >= 400 && self.status_code < 500
    }

    pub fn is_server_error(&self) -> bool {
        self.status_code >= 500
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub total_count: Option<u64>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub total_pages: Option<u32>,
    pub next_page_token: Option<String>,
    pub processing_time_ms: u64,
    pub rate_limit_remaining: Option<u64>,
    pub rate_limit_reset_secs: Option<u64>,
    pub server_version: String,
}

impl Default for ResponseMetadata {
    fn default() -> Self {
        ResponseMetadata {
            total_count: None,
            page: None,
            page_size: None,
            total_pages: None,
            next_page_token: None,
            processing_time_ms: 0,
            rate_limit_remaining: None,
            rate_limit_reset_secs: None,
            server_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

// ============================================================
// API ERROR
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub severity: String,
    pub details: Option<serde_json::Value>,
    pub help_url: Option<String>,
    pub retry_after_secs: Option<u64>,
}

impl ApiErrorResponse {
    pub fn new(code: &str, message: &str, severity: &str) -> Self {
        ApiErrorResponse {
            code: code.to_string(),
            message: message.to_string(),
            severity: severity.to_string(),
            details: None,
            help_url: None,
            retry_after_secs: None,
        }
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_retry_after(mut self, secs: u64) -> Self {
        self.retry_after_secs = Some(secs);
        self
    }
}

// Predefined API error codes
impl ApiErrorResponse {
    pub fn invalid_request(message: &str) -> Self {
        ApiErrorResponse::new("API_001", message, "high")
    }

    pub fn rate_limit_exceeded(retry_after_secs: u64) -> Self {
        ApiErrorResponse::new("API_002", "Rate limit exceeded", "medium")
            .with_retry_after(retry_after_secs)
    }

    pub fn internal_error(message: &str) -> Self {
        ApiErrorResponse::new("API_003", message, "critical")
    }

    pub fn not_found(resource: &str) -> Self {
        ApiErrorResponse::new("API_004", &format!("{} not found", resource), "medium")
    }

    pub fn invalid_token() -> Self {
        ApiErrorResponse::new("AUTH_001", "Invalid or expired token", "high")
    }

    pub fn token_expired() -> Self {
        ApiErrorResponse::new("AUTH_002", "Token has expired", "medium")
    }

    pub fn permission_denied(message: &str) -> Self {
        ApiErrorResponse::new("AUTH_003", message, "high")
    }

    pub fn bad_request(message: &str) -> Self {
        ApiErrorResponse::new("API_005", message, "high")
    }

    pub fn conflict(message: &str) -> Self {
        ApiErrorResponse::new("API_006", message, "medium")
    }

    pub fn service_unavailable(message: &str) -> Self {
        ApiErrorResponse::new("API_007", message, "critical")
    }

    pub fn unprocessable_entity(message: &str) -> Self {
        ApiErrorResponse::new("API_008", message, "high")
    }

    pub fn too_many_requests(retry_after_secs: u64) -> Self {
        ApiErrorResponse::new("API_009", "Too many requests", "medium")
            .with_retry_after(retry_after_secs)
    }
}

// ============================================================
// COMMON API PAYLOADS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanRequestPayload {
    pub url: String,
    pub profile: Option<String>,
    pub options: Option<ScanOptionsPayload>,
    pub tags: Option<Vec<String>>,
    pub priority: Option<String>,
    pub callback_url: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanOptionsPayload {
    pub timeout_secs: Option<u64>,
    pub max_pages: Option<usize>,
    pub follow_redirects: Option<bool>,
    pub respect_robots: Option<bool>,
    pub enable_js: Option<bool>,
    pub scan_depth: Option<u8>,
    pub threads: Option<usize>,
    pub delay_ms: Option<u64>,
    pub modules: Option<Vec<String>>,
    pub exclude_modules: Option<Vec<String>>,
    pub output_formats: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResponsePayload {
    pub scan_id: Uuid,
    pub status: String,
    pub message: String,
    pub status_url: String,
    pub report_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponsePayload {
    pub scan_id: Uuid,
    pub state: String,
    pub progress: f32,
    pub current_step: String,
    pub elapsed_time_secs: u64,
    pub estimated_time_secs: u64,
    pub pages_done: usize,
    pub pages_total: usize,
    pub findings_count: usize,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportRequestPayload {
    pub scan_id: Uuid,
    pub format: String,
    pub options: Option<ReportOptionsPayload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportOptionsPayload {
    pub include_executive_summary: bool,
    pub include_technical_details: bool,
    pub include_vulnerability_tracker: bool,
    pub include_timeline: bool,
    pub include_charts: bool,
    pub language: Option<String>,
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorRequestPayload {
    pub url: String,
    pub schedule: String,
    pub profile: Option<String>,
    pub alert_channels: Option<Vec<String>>,
    pub baseline_scan_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponsePayload {
    pub status: String,
    pub version: String,
    pub uptime_secs: u64,
    pub active_scans: usize,
    pub total_scans: u64,
    pub database_ok: bool,
    pub redis_ok: bool,
    pub storage_ok: bool,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponsePayload {
    pub error: ApiErrorResponse,
    pub request_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T: Serialize + Clone> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_previous: bool,
}

impl<T: Serialize + Clone> PaginatedResponse<T> {
    pub fn new(
        items: Vec<T>,
        total: u64,
        page: u32,
        page_size: u32,
    ) -> Self {
        let total_pages = if page_size == 0 {
            0
        } else {
            ((total as f64) / (page_size as f64)).ceil() as u32
        };
        PaginatedResponse {
            items,
            total,
            page,
            page_size,
            total_pages,
            has_next: page < total_pages,
            has_previous: page > 1,
        }
    }
}

// ============================================================
// API CONTRACT TRAIT
// ============================================================

#[async_trait]
pub trait ApiContract: Send + Sync {
    /// Handle incoming API request
    async fn handle_request(
        &self,
        req: ApiRequest,
    ) -> Result<ApiResponse<serde_json::Value>, ApiContractError>;

    /// Authenticate request
    async fn authenticate(
        &self,
        req: &ApiRequest,
    ) -> Result<AuthResult, ApiContractError>;

    /// Authorize request
    async fn authorize(
        &self,
        req: &ApiRequest,
        permission: &Permission,
    ) -> Result<AuthResult, ApiContractError>;

    /// Get supported API versions
    fn get_supported_versions(&self) -> Vec<ApiVersion>;

    /// Get API capabilities
    fn get_capabilities(&self) -> ApiCapabilities;

    /// Health check endpoint
    async fn health_check(&self) -> Result<HealthResponsePayload, ApiContractError>;

    /// Rate limit check
    async fn check_rate_limit(
        &self,
        client_id: &str,
        endpoint: &str,
    ) -> Result<bool, ApiContractError>;

    /// Validate request format
    fn validate_request(&self, req: &ApiRequest) -> Result<(), ApiContractError> {
        if req.path.is_empty() {
            return Err(ApiContractError::InvalidRequest(
                "path cannot be empty".to_string()
            ));
        }
        if req.path.len() > 2048 {
            return Err(ApiContractError::InvalidRequest(
                format!("path too long: {} chars (max 2048)", req.path.len())
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCapabilities {
    pub max_request_size_bytes: u64,
    pub max_response_size_bytes: u64,
    pub supported_methods: Vec<HttpMethod>,
    pub supported_auth_methods: Vec<AuthMethod>,
    pub rate_limit_per_ip: u32,
    pub rate_limit_per_user: u32,
    pub max_batch_size: usize,
    pub supports_websocket: bool,
    pub supports_streaming: bool,
    pub supports_pagination: bool,
    pub supports_filtering: bool,
    pub supports_sorting: bool,
    pub max_page_size: u32,
    pub api_version: String,
}

impl Default for ApiCapabilities {
    fn default() -> Self {
        ApiCapabilities {
            max_request_size_bytes: 10 * 1024 * 1024,
            max_response_size_bytes: 100 * 1024 * 1024,
            supported_methods: vec![
                HttpMethod::GET,
                HttpMethod::POST,
                HttpMethod::PUT,
                HttpMethod::DELETE,
                HttpMethod::PATCH,
                HttpMethod::HEAD,
                HttpMethod::OPTIONS,
            ],
            supported_auth_methods: vec![
                AuthMethod::ApiKey,
                AuthMethod::BearerToken,
                AuthMethod::Jwt,
            ],
            rate_limit_per_ip: 1000,
            rate_limit_per_user: 10000,
            max_batch_size: 100,
            supports_websocket: true,
            supports_streaming: true,
            supports_pagination: true,
            supports_filtering: true,
            supports_sorting: true,
            max_page_size: 100,
            api_version: "v1.0".to_string(),
        }
    }
}

// ============================================================
// API CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub cors_allowed_origins: Vec<String>,
    pub cors_allowed_methods: Vec<String>,
    pub cors_allowed_headers: Vec<String>,
    pub request_timeout_secs: u64,
    pub max_request_size_bytes: u64,
    pub max_concurrent_requests: usize,
    pub enable_websocket: bool,
    pub enable_request_logging: bool,
    pub enable_metrics: bool,
    pub enable_docs: bool,
    pub docs_path: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            cors_allowed_origins: vec!["*".to_string()],
            cors_allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "OPTIONS".to_string(),
            ],
            cors_allowed_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-API-Key".to_string(),
            ],
            request_timeout_secs: 30,
            max_request_size_bytes: 10 * 1024 * 1024,
            max_concurrent_requests: 100,
            enable_websocket: true,
            enable_request_logging: true,
            enable_metrics: true,
            enable_docs: true,
            docs_path: "/docs".to_string(),
        }
    }
}

impl ApiConfig {
    pub fn validate(&self) -> Result<(), ApiContractError> {
        if self.port == 0 {
            return Err(ApiContractError::InvalidConfiguration(
                "port cannot be 0".to_string()
            ));
        }
        if self.request_timeout_secs == 0 || self.request_timeout_secs > 300 {
            return Err(ApiContractError::InvalidConfiguration(
                format!("request_timeout_secs must be 1-300, got {}", self.request_timeout_secs)
            ));
        }
        if self.max_concurrent_requests == 0 || self.max_concurrent_requests > 1000 {
            return Err(ApiContractError::InvalidConfiguration(
                format!("max_concurrent_requests must be 1-1000, got {}", self.max_concurrent_requests)
            ));
        }
        Ok(())
    }

    pub fn socket_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

// ============================================================
// ERROR TYPES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiContractError {
    InvalidRequest(String),
    InvalidConfiguration(String),
    AuthenticationFailed(String),
    AuthorizationFailed(String),
    RateLimitExceeded(u64),
    NotFound(String),
    Conflict(String),
    ValidationFailed(String),
    InternalError(String),
    ServiceUnavailable(String),
    Timeout(String),
    PayloadTooLarge(u64),
    UnsupportedMediaType(String),
    MethodNotAllowed(String),
    UnsupportedVersion(ApiVersion),
}

impl fmt::Display for ApiContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiContractError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            ApiContractError::InvalidConfiguration(msg) => write!(f, "Invalid config: {}", msg),
            ApiContractError::AuthenticationFailed(msg) => write!(f, "Auth failed: {}", msg),
            ApiContractError::AuthorizationFailed(msg) => write!(f, "Authorization failed: {}", msg),
            ApiContractError::RateLimitExceeded(retry) => write!(f, "Rate limit exceeded, retry after {}s", retry),
            ApiContractError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiContractError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiContractError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            ApiContractError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            ApiContractError::ServiceUnavailable(msg) => write!(f, "Service unavailable: {}", msg),
            ApiContractError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            ApiContractError::PayloadTooLarge(size) => write!(f, "Payload too large: {} bytes", size),
            ApiContractError::UnsupportedMediaType(msg) => write!(f, "Unsupported media type: {}", msg),
            ApiContractError::MethodNotAllowed(msg) => write!(f, "Method not allowed: {}", msg),
            ApiContractError::UnsupportedVersion(v) => write!(f, "Unsupported API version: {}", v),
        }
    }
}

impl std::error::Error for ApiContractError {}

impl ApiContractError {
    pub fn code(&self) -> &str {
        match self {
            ApiContractError::InvalidRequest(_) => "API_001",
            ApiContractError::InvalidConfiguration(_) => "API_010",
            ApiContractError::AuthenticationFailed(_) => "AUTH_001",
            ApiContractError::AuthorizationFailed(_) => "AUTH_003",
            ApiContractError::RateLimitExceeded(_) => "API_002",
            ApiContractError::NotFound(_) => "API_004",
            ApiContractError::Conflict(_) => "API_006",
            ApiContractError::ValidationFailed(_) => "API_008",
            ApiContractError::InternalError(_) => "API_003",
            ApiContractError::ServiceUnavailable(_) => "API_007",
            ApiContractError::Timeout(_) => "API_011",
            ApiContractError::PayloadTooLarge(_) => "API_012",
            ApiContractError::UnsupportedMediaType(_) => "API_013",
            ApiContractError::MethodNotAllowed(_) => "API_014",
            ApiContractError::UnsupportedVersion(_) => "API_015",
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            ApiContractError::InvalidRequest(_) => 400,
            ApiContractError::InvalidConfiguration(_) => 500,
            ApiContractError::AuthenticationFailed(_) => 401,
            ApiContractError::AuthorizationFailed(_) => 403,
            ApiContractError::RateLimitExceeded(_) => 429,
            ApiContractError::NotFound(_) => 404,
            ApiContractError::Conflict(_) => 409,
            ApiContractError::ValidationFailed(_) => 422,
            ApiContractError::InternalError(_) => 500,
            ApiContractError::ServiceUnavailable(_) => 503,
            ApiContractError::Timeout(_) => 504,
            ApiContractError::PayloadTooLarge(_) => 413,
            ApiContractError::UnsupportedMediaType(_) => 415,
            ApiContractError::MethodNotAllowed(_) => 405,
            ApiContractError::UnsupportedVersion(_) => 400,
        }
    }

    pub fn to_error_response(&self) -> ApiErrorResponse {
        let msg = self.to_string();
        match self {
            ApiContractError::RateLimitExceeded(retry) => {
                ApiErrorResponse::rate_limit_exceeded(*retry)
            }
            ApiContractError::AuthenticationFailed(_) => {
                ApiErrorResponse::invalid_token()
            }
            ApiContractError::AuthorizationFailed(_) => {
                ApiErrorResponse::permission_denied(&msg)
            }
            ApiContractError::NotFound(_) => {
                ApiErrorResponse::not_found(&msg)
            }
            ApiContractError::InvalidRequest(_) => {
                ApiErrorResponse::invalid_request(&msg)
            }
            ApiContractError::InternalError(_) => {
                ApiErrorResponse::internal_error(&msg)
            }
            ApiContractError::ServiceUnavailable(_) => {
                ApiErrorResponse::service_unavailable(&msg)
            }
            _ => ApiErrorResponse::new(self.code(), &msg, "medium"),
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
    fn test_api_version_latest() {
        assert_eq!(ApiVersion::latest(), ApiVersion::V1);
    }

    #[test]
    fn test_api_version_from_str() {
        assert_eq!(ApiVersion::from_str("v1"), Some(ApiVersion::V1));
        assert_eq!(ApiVersion::from_str("V2"), Some(ApiVersion::V2));
        assert_eq!(ApiVersion::from_str("invalid"), None);
    }

    #[test]
    fn test_role_permissions() {
        assert!(Role::Admin.can_scan());
        assert!(Role::Admin.can_configure());
        assert!(Role::User.can_scan());
        assert!(!Role::User.can_configure());
        assert!(!Role::Guest.can_scan());
        assert!(Role::Guest.can_view_reports());
    }

    #[test]
    fn test_role_rate_limits() {
        assert_eq!(Role::Admin.rate_limit_per_hour(), 10000);
        assert_eq!(Role::User.rate_limit_per_hour(), 1000);
        assert_eq!(Role::Guest.rate_limit_per_hour(), 100);
    }

    #[test]
    fn test_auth_credentials_is_authenticated() {
        assert!(AuthCredentials::api_key("test123").is_authenticated());
        assert!(AuthCredentials::bearer_token("token123").is_authenticated());
        assert!(AuthCredentials::jwt("jwt123").is_authenticated());
        assert!(AuthCredentials::basic_auth("user", "pass").is_authenticated());
        assert!(!AuthCredentials {
            method: AuthMethod::ApiKey,
            token: None,
            api_key: None,
            username: None,
            password: None,
            client_id: None,
            client_secret: None,
            custom_fields: HashMap::new(),
        }.is_authenticated());
    }

    #[test]
    fn test_auth_credentials_masking() {
        let creds = AuthCredentials::api_key("my-secret-key-12345");
        let masked = creds.mask_sensitive();
        assert_ne!(masked.api_key, Some("my-secret-key-12345".to_string()));
        assert!(masked.api_key.unwrap().contains("..."));
    }

    #[test]
    fn test_auth_result_factories() {
        let success = AuthResult::success("user-1", Role::Admin);
        assert!(success.authenticated);
        assert!(success.authorized);
        assert_eq!(success.user_id, Some("user-1".to_string()));

        let unauth = AuthResult::unauthorized("no token");
        assert!(!unauth.authenticated);

        let forbidden = AuthResult::forbidden("no permission");
        assert!(forbidden.authenticated);
        assert!(!forbidden.authorized);
    }

    #[test]
    fn test_api_response_factories() {
        let request_id = Uuid::new_v4();
        let success = ApiResponse::success(200, serde_json::json!({"key": "value"}), request_id);
        assert!(success.is_success());
        assert_eq!(success.status_code, 200);

        let error = ApiResponse::error(
            404,
            ApiErrorResponse::not_found("scan"),
            request_id,
        );
        assert!(!error.is_success());
        assert!(error.is_client_error());

        let server_error = ApiResponse::error(
            500,
            ApiErrorResponse::internal_error("crash"),
            request_id,
        );
        assert!(server_error.is_server_error());
    }

    #[test]
    fn test_paginated_response() {
        let items = vec!["a", "b", "c"];
        let page = PaginatedResponse::new(items, 30, 1, 10);
        assert_eq!(page.total, 30);
        assert_eq!(page.total_pages, 3);
        assert!(page.has_next);
        assert!(!page.has_previous);

        let last_page = PaginatedResponse::new(vec!["x"], 21, 3, 10);
        assert!(!last_page.has_next);
        assert!(last_page.has_previous);
    }

    #[test]
    fn test_api_error_to_response() {
        let err = ApiContractError::RateLimitExceeded(60);
        let response = err.to_error_response();
        assert_eq!(response.code, "API_002");
        assert_eq!(response.retry_after_secs, Some(60));

        let err = ApiContractError::NotFound("scan-1".to_string());
        let response = err.to_error_response();
        assert_eq!(response.code, "API_004");
    }

    #[test]
    fn test_api_error_http_status() {
        assert_eq!(ApiContractError::InvalidRequest("".to_string()).http_status(), 400);
        assert_eq!(ApiContractError::AuthenticationFailed("".to_string()).http_status(), 401);
        assert_eq!(ApiContractError::AuthorizationFailed("".to_string()).http_status(), 403);
        assert_eq!(ApiContractError::NotFound("".to_string()).http_status(), 404);
        assert_eq!(ApiContractError::RateLimitExceeded(0).http_status(), 429);
        assert_eq!(ApiContractError::InternalError("".to_string()).http_status(), 500);
        assert_eq!(ApiContractError::ServiceUnavailable("".to_string()).http_status(), 503);
    }

    #[test]
    fn test_api_config_validation() {
        let mut config = ApiConfig::default();
        assert!(config.validate().is_ok());

        config.port = 0;
        assert!(config.validate().is_err());

        config.port = 8080;
        config.request_timeout_secs = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_api_request_builder() {
        let request = ApiRequest::new(HttpMethod::GET, "/api/v1/scans")
            .with_query_param("status", "active")
            .with_header("Accept", "application/json")
            .with_auth(AuthCredentials::bearer_token("token123"));

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.get_query_param("status"), Some(&"active".to_string()));
        assert!(request.auth.is_some());
    }

    #[test]
    fn test_predefined_api_errors() {
        let err = ApiErrorResponse::invalid_token();
        assert_eq!(err.code, "AUTH_001");

        let err = ApiErrorResponse::rate_limit_exceeded(30);
        assert_eq!(err.code, "API_002");
        assert_eq!(err.retry_after_secs, Some(30));

        let err = ApiErrorResponse::not_found("user");
        assert_eq!(err.code, "API_004");
    }
}
