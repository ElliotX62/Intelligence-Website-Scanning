//! # Scanner Contract
//!
//! Formal contracts for all scanner modules in the IWS system.
//! This module defines the `ScannerContract` trait that all scanner
//! implementations must adhere to, ensuring consistent behavior,
//! proper error handling, and predictable output across all scanners.
//!
//! ## Design by Contract
//!
//! This contract follows the Design by Contract (DbC) methodology:
//! - **Preconditions**: Conditions that must be true before method execution
//! - **Postconditions**: Conditions that must be true after method execution
//! - **Invariants**: Conditions that must remain true throughout execution

use std::fmt;
use std::error::Error;
use uuid::Uuid;
use url::Url;

// ---------------------------------------------------------------------------
// Scanner Error Types
// ---------------------------------------------------------------------------

/// Represents all possible errors that can occur during scanner operations.
///
/// Each variant corresponds to a specific failure mode in the scanning process,
/// allowing callers to handle different error scenarios appropriately.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScannerError {
    /// The provided URL is malformed, missing a scheme, or otherwise invalid.
    /// The contained string describes the specific validation failure.
    #[doc = "The provided URL is not valid for scanning."]
    InvalidUrl(String),

    /// The target domain could not be resolved to an IP address.
    /// This typically indicates the domain does not exist or DNS is unreachable.
    #[doc = "The target domain could not be resolved."]
    DomainNotFound(String),

    /// The connection to the target timed out before any data could be exchanged.
    /// This may indicate network issues, firewall blocks, or an unresponsive target.
    #[doc = "The connection to the target timed out."]
    ConnectionTimeout(String),

    /// The scan was cancelled by the user or system before completion.
    /// Partial results may be available depending on when cancellation occurred.
    #[doc = "The scan was cancelled before completion."]
    ScanCancelled(String),

    /// The scanner has been rate-limited by the target server.
    /// The contained string may include retry-after information if available.
    #[doc = "Rate limit has been exceeded for the target."]
    RateLimited(String),
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScannerError::InvalidUrl(msg) => {
                write!(f, "Invalid URL: {}", msg)
            }
            ScannerError::DomainNotFound(msg) => {
                write!(f, "Domain not found: {}", msg)
            }
            ScannerError::ConnectionTimeout(msg) => {
                write!(f, "Connection timeout: {}", msg)
            }
            ScannerError::ScanCancelled(msg) => {
                write!(f, "Scan cancelled: {}", msg)
            }
            ScannerError::RateLimited(msg) => {
                write!(f, "Rate limited: {}", msg)
            }
        }
    }
}

impl Error for ScannerError {}

// ---------------------------------------------------------------------------
// Forward Declarations for Types Used in the Contract
// ---------------------------------------------------------------------------

/// Represents the current state of an ongoing or completed scan.
///
/// The status is used by callers to determine whether a scan is
/// still in progress, has completed successfully, or has failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanStatus {
    /// The scan has not yet been started.
    Pending,
    /// The scan is currently in progress.
    Active,
    /// The scan has completed successfully.
    Completed,
    /// The scan has completed with errors.
    Failed,
    /// The scan was cancelled by the user.
    Cancelled,
    /// The scan is in an unknown or indeterminate state.
    Unknown,
}

impl fmt::Display for ScanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScanStatus::Pending => write!(f, "Pending"),
            ScanStatus::Active => write!(f, "Active"),
            ScanStatus::Completed => write!(f, "Completed"),
            ScanStatus::Failed => write!(f, "Failed"),
            ScanStatus::Cancelled => write!(f, "Cancelled"),
            ScanStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Represents a scanning profile that determines the behavior and intensity
/// of the scanning process.
#[derive(Debug, Clone)]
pub struct Profile {
    /// Number of parallel threads to use for scanning.
    /// Must be between 1 and 200 (inclusive).
    pub threads: u32,

    /// Timeout for individual requests in seconds.
    pub timeout_seconds: u64,

    /// Delay between consecutive requests in milliseconds.
    /// A value of 0 indicates no delay (aggressive mode).
    pub delay_ms: u64,

    /// Maximum number of pages to crawl and scan.
    pub max_pages: u32,

    /// Whether to follow HTTP redirects during crawling.
    pub follow_redirects: bool,

    /// Whether to respect robots.txt directives.
    pub respect_robots: bool,
}

/// Represents the complete result of a scanning operation.
///
/// This structure contains all metadata about the scan as well as
/// the results from individual scanning modules.
#[derive(Debug, Clone)]
pub struct ScanResult {
    /// Unique identifier for this scan.
    pub scan_id: Uuid,

    /// The target URL that was scanned.
    pub url: Url,

    /// The profile used for this scan.
    pub profile: Profile,

    /// Timestamp when the scan started.
    pub start_time: chrono::DateTime<chrono::Utc>,

    /// Timestamp when the scan ended.
    /// Only valid when status is Completed, Failed, or Cancelled.
    pub end_time: chrono::DateTime<chrono::Utc>,

    /// Current status of the scan.
    pub status: ScanStatus,

    /// Overall progress of the scan as a percentage (0-100).
    pub progress: f32,

    /// Results from each scanning module that was executed.
    /// This vector is populated progressively as modules complete.
    pub modules_results: Vec<ModuleResult>,

    /// Any errors encountered during scanning, even if the scan
    /// partially succeeded.
    pub errors: Vec<ScannerError>,
}

/// Represents the result from a single scanning module.
#[derive(Debug, Clone)]
pub struct ModuleResult {
    /// Name of the module that produced this result.
    pub module_name: String,

    /// Whether the module completed successfully.
    pub success: bool,

    /// The raw output data from the module, if any.
    pub data: Option<Vec<u8>>,

    /// Duration of module execution in milliseconds.
    pub duration_ms: u64,

    /// Any error message from the module, if it failed.
    pub error: Option<String>,
}

// ---------------------------------------------------------------------------
// Scanner Contract Trait
// ---------------------------------------------------------------------------

/// The `ScannerContract` trait defines the formal contract that all scanner
/// implementations must fulfill.
///
/// ## Design by Contract
///
/// ### Preconditions
/// - The target URL must not be empty.
/// - The target URL must use either HTTP or HTTPS scheme.
/// - The scanning profile must specify between 1 and 200 threads.
///
/// ### Postconditions
/// - The scan start time must be before or equal to the end time.
/// - At least one module result must be present in the scan result.
/// - The scan status must not be `Unknown` upon completion.
///
/// ### Invariants
/// - Each scan is identified by a unique `scan_id`.
/// - `start_time` must always be less than `end_time` (once the scan completes).
/// - Progress value must always be between 0.0 and 100.0 (inclusive).
///
/// ## Error Handling
///
/// All methods return `Result<T, ScannerError>` to ensure that errors are
/// handled explicitly by callers. Common error variants include:
/// - `InvalidUrl`: The provided URL is not valid
/// - `DomainNotFound`: The target domain could not be resolved
/// - `ConnectionTimeout`: The connection to the target timed out
/// - `ScanCancelled`: The scan was cancelled before completion
/// - `RateLimited`: The scanner has been rate-limited
pub trait ScannerContract {
    /// Scan a website at the given URL using the default scanning profile.
    ///
    /// This is a convenience method that internally calls
    /// `scan_with_profile` using the system's default profile.
    ///
    /// # Preconditions
    /// - `!url.as_str().is_empty()`: The URL must not be empty.
    /// - `url.scheme() == "http" || url.scheme() == "https"`: The scheme must be HTTP or HTTPS.
    ///
    /// # Postconditions
    /// - `result.start_time <= result.end_time`: Start time must not be after end time.
    /// - `result.modules_results.len() > 0`: At least one module must produce a result.
    /// - `result.status != ScanStatus::Unknown`: The final status must be known.
    ///
    /// # Errors
    /// Returns `ScannerError::InvalidUrl` if the URL fails validation.
    /// Returns `ScannerError::DomainNotFound` if DNS resolution fails.
    /// Returns `ScannerError::ConnectionTimeout` if the target is unreachable.
    #[doc = "Scan a website using the default scanning profile."]
    fn scan_website(&self, url: Url) -> Result<ScanResult, ScannerError> {
        // Precondition: URL must not be empty
        debug_assert!(
            !url.as_str().is_empty(),
            "Precondition failed: URL must not be empty"
        );
        // Precondition: URL scheme must be HTTP or HTTPS
        debug_assert!(
            url.scheme() == "http" || url.scheme() == "https",
            "Precondition failed: URL scheme must be 'http' or 'https', got '{}'",
            url.scheme()
        );

        // Delegate to the profile-aware implementation.
        // The default profile is defined by the concrete implementation.
        let result = self.scan_with_profile(url.clone(), Profile::default())?;

        // Postcondition: Start time must be before or equal to end time
        assert!(
            result.start_time <= result.end_time,
            "Postcondition failed: start_time ({:?}) must be <= end_time ({:?})",
            result.start_time,
            result.end_time
        );
        // Postcondition: At least one module result must be present
        assert!(
            !result.modules_results.is_empty(),
            "Postcondition failed: modules_results must contain at least one entry"
        );
        // Postcondition: Status must not be Unknown
        assert!(
            result.status != ScanStatus::Unknown,
            "Postcondition failed: scan status must not be Unknown"
        );

        Ok(result)
    }

    /// Scan a website at the given URL using the specified scanning profile.
    ///
    /// The profile controls scanning intensity, parallelism, timeout behavior,
    /// and other operational parameters.
    ///
    /// # Preconditions
    /// - `!url.as_str().is_empty()`: The URL must not be empty.
    /// - `url.scheme() == "http" || url.scheme() == "https"`: The scheme must be HTTP or HTTPS.
    /// - `profile.threads > 0 && profile.threads <= 200`: Thread count must be valid.
    ///
    /// # Postconditions
    /// - `result.start_time <= result.end_time`: Start time must not be after end time.
    /// - `result.modules_results.len() > 0`: At least one module must produce a result.
    /// - `result.status != ScanStatus::Unknown`: The final status must be known.
    ///
    /// # Errors
    /// Returns `ScannerError::InvalidUrl` if the URL fails validation.
    /// Returns `ScannerError::DomainNotFound` if DNS resolution fails.
    /// Returns `ScannerError::ConnectionTimeout` if the target is unreachable.
    /// Returns `ScannerError::RateLimited` if the target enforces rate limiting.
    #[doc = "Scan a website using the specified scanning profile."]
    fn scan_with_profile(
        &self,
        url: Url,
        profile: Profile,
    ) -> Result<ScanResult, ScannerError> {
        // Precondition: URL must not be empty
        debug_assert!(
            !url.as_str().is_empty(),
            "Precondition failed: URL must not be empty"
        );
        // Precondition: URL scheme must be HTTP or HTTPS
        debug_assert!(
            url.scheme() == "http" || url.scheme() == "https",
            "Precondition failed: URL scheme must be 'http' or 'https', got '{}'",
            url.scheme()
        );
        // Precondition: Thread count must be valid (1-200)
        debug_assert!(
            profile.threads > 0 && profile.threads <= 200,
            "Precondition failed: profile.threads must be between 1 and 200, got {}",
            profile.threads
        );

        // The actual implementation is provided by concrete types.
        // This default implementation exists only to document the contract;
        // it will be overridden by implementors.
        unimplemented!(
            "scan_with_profile must be implemented by the concrete scanner type"
        )
    }

    /// Cancel an ongoing scan identified by its unique scan ID.
    ///
    /// If the scan has already completed or does not exist, an appropriate
    /// error is returned.
    ///
    /// # Preconditions
    /// - `scan_id` must reference a valid, active scan.
    ///
    /// # Postconditions
    /// - If successful, the scan status transitions to `Cancelled`.
    /// - Resources associated with the scan are released.
    ///
    /// # Errors
    /// Returns `ScannerError::ScanCancelled` if the scan was already cancelled
    /// or does not exist.
    #[doc = "Cancel an ongoing scan by its unique identifier."]
    fn cancel_scan(&self, scan_id: Uuid) -> Result<(), ScannerError> {
        // Precondition: scan_id must be a valid UUID (non-nil)
        debug_assert!(
            !scan_id.is_nil(),
            "Precondition failed: scan_id must not be nil"
        );

        // The actual implementation is provided by concrete types.
        unimplemented!(
            "cancel_scan must be implemented by the concrete scanner type"
        )
    }

    /// Retrieve the current status of a scan identified by its unique scan ID.
    ///
    /// # Preconditions
    /// - `scan_id` must reference a valid scan that was previously initiated.
    ///
    /// # Postconditions
    /// - The returned `ScanStatus` accurately reflects the current scan state.
    /// - The status must not be `Unknown` if the scan ID is valid.
    ///
    /// # Errors
    /// Returns an error variant if the scan ID is not found.
    #[doc = "Get the current status of a scan by its unique identifier."]
    fn get_scan_status(&self, scan_id: Uuid) -> Result<ScanStatus, ScannerError> {
        // Precondition: scan_id must be a valid UUID (non-nil)
        debug_assert!(
            !scan_id.is_nil(),
            "Precondition failed: scan_id must not be nil"
        );

        // The actual implementation is provided by concrete types.
        unimplemented!(
            "get_scan_status must be implemented by the concrete scanner type"
        )
    }
}

// ---------------------------------------------------------------------------
// Default Profile Implementation
// ---------------------------------------------------------------------------

impl Default for Profile {
    /// Returns a moderate default scanning profile.
    ///
    /// - threads: 50 (balanced concurrency)
    /// - timeout_seconds: 15 (reasonable response wait)
    /// - delay_ms: 100 (light throttling to avoid overwhelming targets)
    /// - max_pages: 500 (comprehensive but bounded)
    /// - follow_redirects: true (standard crawling behavior)
    /// - respect_robots: true (polite scanning by default)
    fn default() -> Self {
        Profile {
            threads: 50,
            timeout_seconds: 15,
            delay_ms: 100,
            max_pages: 500,
            follow_redirects: true,
            respect_robots: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Unit Tests for Contract Validation
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// A minimal concrete implementation of `ScannerContract` used solely
    /// for testing the contract's default method logic.
    struct TestScanner;

    impl ScannerContract for TestScanner {
        fn scan_with_profile(
            &self,
            url: Url,
            profile: Profile,
        ) -> Result<ScanResult, ScannerError> {
            // Return a valid ScanResult that satisfies all postconditions
            let now = chrono::Utc::now();
            Ok(ScanResult {
                scan_id: Uuid::new_v4(),
                url,
                profile,
                start_time: now,
                end_time: now,
                status: ScanStatus::Completed,
                progress: 100.0,
                modules_results: vec![ModuleResult {
                    module_name: "test_module".to_string(),
                    success: true,
                    data: None,
                    duration_ms: 10,
                    error: None,
                }],
                errors: Vec::new(),
            })
        }

        fn cancel_scan(&self, _scan_id: Uuid) -> Result<(), ScannerError> {
            Ok(())
        }

        fn get_scan_status(&self, _scan_id: Uuid) -> Result<ScanStatus, ScannerError> {
            Ok(ScanStatus::Completed)
        }
    }

    // -----------------------------------------------------------------------
    // ScannerError Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_scanner_error_display() {
        assert_eq!(
            ScannerError::InvalidUrl("missing scheme".to_string()).to_string(),
            "Invalid URL: missing scheme"
        );
        assert_eq!(
            ScannerError::DomainNotFound("example.invalid".to_string()).to_string(),
            "Domain not found: example.invalid"
        );
        assert_eq!(
            ScannerError::ConnectionTimeout("10.0.0.1:443".to_string()).to_string(),
            "Connection timeout: 10.0.0.1:443"
        );
        assert_eq!(
            ScannerError::ScanCancelled("user requested".to_string()).to_string(),
            "Scan cancelled: user requested"
        );
        assert_eq!(
            ScannerError::RateLimited("retry after 60s".to_string()).to_string(),
            "Rate limited: retry after 60s"
        );
    }

    #[test]
    fn test_scanner_error_implements_error_trait() {
        let err = ScannerError::InvalidUrl("test".to_string());
        // Verify that ScannerError implements the std::error::Error trait
        let _: &dyn Error = &err;
    }

    #[test]
    fn test_scanner_error_equality() {
        assert_eq!(
            ScannerError::InvalidUrl("x".to_string()),
            ScannerError::InvalidUrl("x".to_string())
        );
        assert_ne!(
            ScannerError::InvalidUrl("x".to_string()),
            ScannerError::DomainNotFound("x".to_string())
        );
    }

    // -----------------------------------------------------------------------
    // ScanStatus Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_scan_status_display() {
        assert_eq!(ScanStatus::Pending.to_string(), "Pending");
        assert_eq!(ScanStatus::Active.to_string(), "Active");
        assert_eq!(ScanStatus::Completed.to_string(), "Completed");
        assert_eq!(ScanStatus::Failed.to_string(), "Failed");
        assert_eq!(ScanStatus::Cancelled.to_string(), "Cancelled");
        assert_eq!(ScanStatus::Unknown.to_string(), "Unknown");
    }

    // -----------------------------------------------------------------------
    // Profile Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_profile_default_values() {
        let profile = Profile::default();
        assert_eq!(profile.threads, 50);
        assert_eq!(profile.timeout_seconds, 15);
        assert_eq!(profile.delay_ms, 100);
        assert_eq!(profile.max_pages, 500);
        assert!(profile.follow_redirects);
        assert!(profile.respect_robots);
    }

    // -----------------------------------------------------------------------
    // ScannerContract Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_scan_website_success() {
        let scanner = TestScanner;
        let url = Url::parse("https://example.com").unwrap();
        let result = scanner.scan_website(url);
        assert!(result.is_ok());
        let scan_result = result.unwrap();
        assert_eq!(scan_result.status, ScanStatus::Completed);
        assert!(!scan_result.modules_results.is_empty());
        assert!(scan_result.progress >= 0.0 && scan_result.progress <= 100.0);
    }

    #[test]
    #[should_panic(expected = "Precondition failed: URL must not be empty")]
    fn test_scan_website_empty_url_panics_in_debug() {
        let scanner = TestScanner;
        // An empty URL should trigger the debug_assert! in debug builds
        let url = Url::parse("https://").unwrap(); // empty host
        let _ = scanner.scan_website(url);
    }

    #[test]
    fn test_scan_with_profile_success() {
        let scanner = TestScanner;
        let url = Url::parse("https://example.com").unwrap();
        let profile = Profile {
            threads: 10,
            ..Profile::default()
        };
        let result = scanner.scan_with_profile(url, profile);
        assert!(result.is_ok());
        let scan_result = result.unwrap();
        assert!(scan_result.progress >= 0.0 && scan_result.progress <= 100.0);
    }

    #[test]
    #[should_panic(expected = "Precondition failed: profile.threads must be between 1 and 200")]
    fn test_scan_with_profile_zero_threads_panics_in_debug() {
        let scanner = TestScanner;
        let url = Url::parse("https://example.com").unwrap();
        let profile = Profile {
            threads: 0,
            ..Profile::default()
        };
        let _ = scanner.scan_with_profile(url, profile);
    }

    #[test]
    #[should_panic(expected = "Precondition failed: profile.threads must be between 1 and 200")]
    fn test_scan_with_profile_excessive_threads_panics_in_debug() {
        let scanner = TestScanner;
        let url = Url::parse("https://example.com").unwrap();
        let profile = Profile {
            threads: 201,
            ..Profile::default()
        };
        let _ = scanner.scan_with_profile(url, profile);
    }

    // -----------------------------------------------------------------------
    // Invariants Tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_scan_result_progress_range_invariant() {
        let now = chrono::Utc::now();
        let result = ScanResult {
            scan_id: Uuid::new_v4(),
            url: Url::parse("https://example.com").unwrap(),
            profile: Profile::default(),
            start_time: now,
            end_time: now,
            status: ScanStatus::Completed,
            progress: 50.0,
            modules_results: vec![ModuleResult {
                module_name: "test".to_string(),
                success: true,
                data: None,
                duration_ms: 5,
                error: None,
            }],
            errors: Vec::new(),
        };
        // Invariant: progress must be between 0 and 100
        assert!(result.progress >= 0.0 && result.progress <= 100.0);
    }

    #[test]
    fn test_scan_result_time_invariant() {
        let start = chrono::Utc::now();
        let end = start + chrono::Duration::seconds(10);
        let result = ScanResult {
            scan_id: Uuid::new_v4(),
            url: Url::parse("https://example.com").unwrap(),
            profile: Profile::default(),
            start_time: start,
            end_time: end,
            status: ScanStatus::Completed,
            progress: 100.0,
            modules_results: vec![ModuleResult {
                module_name: "test".to_string(),
                success: true,
                data: None,
                duration_ms: 10000,
                error: None,
            }],
            errors: Vec::new(),
        };
        // Invariant: start_time must be <= end_time
        assert!(result.start_time <= result.end_time);
    }

    #[test]
    fn test_scan_result_unique_id_invariant() {
        let result1 = ScanResult {
            scan_id: Uuid::new_v4(),
            url: Url::parse("https://example.com").unwrap(),
            profile: Profile::default(),
            start_time: chrono::Utc::now(),
            end_time: chrono::Utc::now(),
            status: ScanStatus::Completed,
            progress: 100.0,
            modules_results: vec![ModuleResult {
                module_name: "test".to_string(),
                success: true,
                data: None,
                duration_ms: 10,
                error: None,
            }],
            errors: Vec::new(),
        };
        let result2 = ScanResult {
            scan_id: Uuid::new_v4(),
            url: Url::parse("https://example.org").unwrap(),
            profile: Profile::default(),
            start_time: chrono::Utc::now(),
            end_time: chrono::Utc::now(),
            status: ScanStatus::Completed,
            progress: 100.0,
            modules_results: vec![ModuleResult {
                module_name: "test".to_string(),
                success: true,
                data: None,
                duration_ms: 10,
                error: None,
            }],
            errors: Vec::new(),
        };
        // Invariant: each scan must have a unique ID
        assert_ne!(result1.scan_id, result2.scan_id);
    }
}
