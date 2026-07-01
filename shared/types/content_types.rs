// shared/types/content_types.rs
// IWS v1.0 - Content Types
// Mendefinisikan tipe data untuk website content analysis

use std::fmt;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use url::Url;
use super::common_types::{Severity, Confidence};

// ============================================================
// HTML PARSING
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedHtml {
    pub url: String,
    pub title: Option<String>,
    pub headings: Vec<Heading>,
    pub forms: Vec<HtmlForm>,
    pub scripts: Vec<HtmlScript>,
    pub images: Vec<HtmlImage>,
    pub links: Vec<HtmlLink>,
    pub meta_tags: Vec<MetaTag>,
    pub comments: Vec<String>,
    pub text_content: String,
    pub text_length: usize,
    pub dom_depth: usize,
    pub total_elements: usize,
    pub parse_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl ParsedHtml {
    pub fn new(url: &str) -> Self {
        ParsedHtml {
            url: url.to_string(),
            title: None,
            headings: vec![],
            forms: vec![],
            scripts: vec![],
            images: vec![],
            links: vec![],
            meta_tags: vec![],
            comments: vec![],
            text_content: String::new(),
            text_length: 0,
            dom_depth: 0,
            total_elements: 0,
            parse_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn internal_links(&self) -> Vec<&HtmlLink> {
        self.links.iter().filter(|l| l.is_internal).collect()
    }

    pub fn external_links(&self) -> Vec<&HtmlLink> {
        self.links.iter().filter(|l| !l.is_internal).collect()
    }

    pub fn forms_with_method(&self, method: &str) -> Vec<&HtmlForm> {
        self.forms
            .iter()
            .filter(|f| f.method.to_lowercase() == method.to_lowercase())
            .collect()
    }

    pub fn login_forms(&self) -> Vec<&HtmlForm> {
        self.forms
            .iter()
            .filter(|f| f.is_login_form())
            .collect()
    }

    pub fn images_without_alt(&self) -> Vec<&HtmlImage> {
        self.images.iter().filter(|i| i.alt.is_none()).collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heading {
    pub level: u8,
    pub text: String,
    pub id: Option<String>,
    pub class: Option<String>,
}

impl Heading {
    pub fn new(level: u8, text: &str) -> Self {
        Heading {
            level: level.min(6).max(1),
            text: text.to_string(),
            id: None,
            class: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlForm {
    pub action: Option<String>,
    pub method: String,
    pub inputs: Vec<FormInput>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub class: Option<String>,
    pub has_file_upload: bool,
    pub has_csrf_token: bool,
    pub csrf_token_field: Option<String>,
}

impl HtmlForm {
    pub fn new(method: &str) -> Self {
        HtmlForm {
            action: None,
            method: method.to_uppercase(),
            inputs: vec![],
            id: None,
            name: None,
            class: None,
            has_file_upload: false,
            has_csrf_token: false,
            csrf_token_field: None,
        }
    }

    pub fn is_login_form(&self) -> bool {
        let action_lower = self.action.as_deref().unwrap_or("").to_lowercase();
        let has_password = self.inputs.iter().any(|i| i.input_type == "password");
        let login_urls = ["login", "signin", "auth", "logon"];
        has_password
            && (login_urls.iter().any(|u| action_lower.contains(u))
                || self.inputs.iter().any(|i| {
                    i.name.as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains("password")
                }))
    }

    pub fn input_names(&self) -> Vec<String> {
        self.inputs
            .iter()
            .filter_map(|i| i.name.clone())
            .collect()
    }

    pub fn detect_csrf_token(&mut self) {
        let csrf_names = [
            "csrf_token", "_token", "csrfmiddlewaretoken",
            "authenticity_token", "csrf-token", "xsrf-token",
            "_csrf", "csrf", "__RequestVerificationToken",
        ];

        for input in &self.inputs {
            if let Some(ref name) = input.name {
                if csrf_names.contains(&name.to_lowercase().as_str()) {
                    self.has_csrf_token = true;
                    self.csrf_token_field = Some(name.clone());
                    return;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormInput {
    pub name: Option<String>,
    pub input_type: String,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub required: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub autocomplete: Option<String>,
    pub maxlength: Option<usize>,
    pub pattern: Option<String>,
}

impl FormInput {
    pub fn new(input_type: &str) -> Self {
        FormInput {
            name: None,
            input_type: input_type.to_string(),
            value: None,
            placeholder: None,
            required: false,
            disabled: false,
            readonly: false,
            autocomplete: None,
            maxlength: None,
            pattern: None,
        }
    }

    pub fn is_sensitive_field(&self) -> bool {
        matches!(
            self.input_type.as_str(),
            "password" | "hidden" | "tel" | "email" | "ssn" | "credit-card"
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlScript {
    pub src: Option<String>,
    pub script_type: String,
    pub content: Option<String>,
    pub is_async: bool,
    pub is_defer: bool,
    pub integrity: Option<String>,
    pub crossorigin: Option<String>,
    pub is_external: bool,
    pub size_bytes: usize,
}

impl HtmlScript {
    pub fn new() -> Self {
        HtmlScript {
            src: None,
            script_type: "text/javascript".to_string(),
            content: None,
            is_async: false,
            is_defer: false,
            integrity: None,
            crossorigin: None,
            is_external: false,
            size_bytes: 0,
        }
    }

    pub fn has_integrity_check(&self) -> bool {
        self.integrity.is_some()
    }

    pub fn is_potentially_dangerous(&self) -> bool {
        if let Some(ref content) = self.content {
            content.contains("eval(")
                || content.contains("document.write(")
                || content.contains("innerHTML")
                || content.contains("Function(")
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlImage {
    pub src: String,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub loading: Option<String>,
    pub is_external: bool,
}

impl HtmlImage {
    pub fn new(src: &str) -> Self {
        HtmlImage {
            src: src.to_string(),
            alt: None,
            title: None,
            width: None,
            height: None,
            loading: None,
            is_external: src.starts_with("http"),
        }
    }

    pub fn has_alt_text(&self) -> bool {
        self.alt.is_some() && !self.alt.as_deref().unwrap_or("").is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HtmlLink {
    pub href: String,
    pub text: Option<String>,
    pub rel: Option<String>,
    pub target: Option<String>,
    pub is_internal: bool,
    pub is_nofollow: bool,
    pub is_noopener: bool,
    pub status_code: Option<u16>,
    pub is_broken: bool,
}

impl HtmlLink {
    pub fn new(href: &str, is_internal: bool) -> Self {
        HtmlLink {
            href: href.to_string(),
            text: None,
            rel: None,
            target: None,
            is_internal,
            is_nofollow: false,
            is_noopener: false,
            status_code: None,
            is_broken: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaTag {
    pub name: Option<String>,
    pub property: Option<String>,
    pub content: Option<String>,
    pub charset: Option<String>,
    pub http_equiv: Option<String>,
}

impl MetaTag {
    pub fn new() -> Self {
        MetaTag {
            name: None,
            property: None,
            content: None,
            charset: None,
            http_equiv: None,
        }
    }

    pub fn is_og_tag(&self) -> bool {
        self.property
            .as_deref()
            .map(|p| p.starts_with("og:"))
            .unwrap_or(false)
    }

    pub fn is_twitter_tag(&self) -> bool {
        self.name
            .as_deref()
            .map(|n| n.starts_with("twitter:"))
            .unwrap_or(false)
    }
}

// ============================================================
// JAVASCRIPT ANALYSIS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsAnalysis {
    pub url: String,
    pub framework_detected: Vec<Framework>,
    pub api_keys: Vec<ApiKey>,
    pub dangerous_functions: Vec<DangerousFunction>,
    pub library_versions: Vec<LibraryVersion>,
    pub obfuscation_detected: bool,
    pub obfuscation_score: f32,
    pub total_functions: usize,
    pub total_lines: usize,
    pub total_size_bytes: usize,
    pub analysis_time_ms: u64,
    pub errors: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

impl JsAnalysis {
    pub fn new(url: &str) -> Self {
        JsAnalysis {
            url: url.to_string(),
            framework_detected: vec![],
            api_keys: vec![],
            dangerous_functions: vec![],
            library_versions: vec![],
            obfuscation_detected: false,
            obfuscation_score: 0.0,
            total_functions: 0,
            total_lines: 0,
            total_size_bytes: 0,
            analysis_time_ms: 0,
            errors: vec![],
            timestamp: Utc::now(),
        }
    }

    pub fn has_exposed_keys(&self) -> bool {
        !self.api_keys.is_empty()
    }

    pub fn has_dangerous_code(&self) -> bool {
        !self.dangerous_functions.is_empty()
    }

    pub fn security_risk_level(&self) -> Severity {
        let mut risk_score = 0u8;
        if self.has_exposed_keys() { risk_score += 3; }
        if self.has_dangerous_code() { risk_score += 2; }
        if self.obfuscation_detected { risk_score += 1; }

        match risk_score {
            0 => Severity::Info,
            1..=2 => Severity::Low,
            3..=4 => Severity::Medium,
            5 => Severity::High,
            _ => Severity::Critical,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Framework {
    pub name: String,
    pub version: Option<String>,
    pub confidence: Confidence,
    pub detection_method: String,
}

impl Framework {
    pub fn new(name: &str) -> Self {
        Framework {
            name: name.to_string(),
            version: None,
            confidence: Confidence::Medium,
            detection_method: "unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub key_type: String,
    pub value_masked: String,
    pub source_file: String,
    pub line: u32,
    pub column: u32,
    pub context: String,
    pub is_valid_format: bool,
}

impl ApiKey {
    pub fn new(key_type: &str, value: &str, source: &str, line: u32, column: u32) -> Self {
        let value_masked = if value.len() > 12 {
            format!("{}...{}", &value[..6], &value[value.len()-6..])
        } else {
            "***".to_string()
        };

        ApiKey {
            key_type: key_type.to_string(),
            value_masked,
            source_file: source.to_string(),
            line,
            column,
            context: String::new(),
            is_valid_format: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DangerousFunction {
    pub function_name: String,
    pub line: u32,
    pub column: u32,
    pub context: String,
    pub risk: Severity,
}

impl DangerousFunction {
    pub fn new(function_name: &str, line: u32, column: u32) -> Self {
        let risk = match function_name {
            "eval" => Severity::Critical,
            "Function" => Severity::High,
            "document.write" => Severity::High,
            "innerHTML" => Severity::Medium,
            "setTimeout" => Severity::Medium,
            "setInterval" => Severity::Medium,
            _ => Severity::Low,
        };

        DangerousFunction {
            function_name: function_name.to_string(),
            line,
            column,
            context: String::new(),
            risk,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryVersion {
    pub name: String,
    pub version: String,
    pub is_outdated: bool,
    pub latest_version: Option<String>,
    pub known_vulnerabilities: usize,
}

impl LibraryVersion {
    pub fn new(name: &str, version: &str) -> Self {
        LibraryVersion {
            name: name.to_string(),
            version: version.to_string(),
            is_outdated: false,
            latest_version: None,
            known_vulnerabilities: 0,
        }
    }
}

// ============================================================
// CSS ANALYSIS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssInfo {
    pub url: String,
    pub total_rules: usize,
    pub total_selectors: usize,
    pub frameworks: Vec<Framework>,
    pub unused_percentage: f32,
    pub variables: Vec<CssVariable>,
    pub imports: Vec<String>,
    pub fonts: Vec<String>,
    pub media_queries: Vec<String>,
    pub keyframes: Vec<String>,
    pub total_size_bytes: usize,
    pub analysis_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl CssInfo {
    pub fn new(url: &str) -> Self {
        CssInfo {
            url: url.to_string(),
            total_rules: 0,
            total_selectors: 0,
            frameworks: vec![],
            unused_percentage: 0.0,
            variables: vec![],
            imports: vec![],
            fonts: vec![],
            media_queries: vec![],
            keyframes: vec![],
            total_size_bytes: 0,
            analysis_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn has_framework(&self, name: &str) -> bool {
        self.frameworks
            .iter()
            .any(|f| f.name.to_lowercase() == name.to_lowercase())
    }

    pub fn efficiency_score(&self) -> f32 {
        100.0 - self.unused_percentage
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssVariable {
    pub name: String,
    pub value: String,
    pub scope: String,
}

impl CssVariable {
    pub fn new(name: &str, value: &str) -> Self {
        CssVariable {
            name: name.to_string(),
            value: value.to_string(),
            scope: "root".to_string(),
        }
    }
}

// ============================================================
// METADATA
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaData {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub og_tags: HashMap<String, String>,
    pub twitter_cards: HashMap<String, String>,
    pub canonical_url: Option<String>,
    pub robots: Option<String>,
    pub sitemap_url: Option<String>,
    pub rss_feed_url: Option<String>,
    pub author: Option<String>,
    pub generator: Option<String>,
    pub theme_color: Option<String>,
    pub viewport: Option<String>,
    pub language: Option<String>,
    pub charset: Option<String>,
    pub favicon_url: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl MetaData {
    pub fn new(url: &str) -> Self {
        MetaData {
            url: url.to_string(),
            title: None,
            description: None,
            keywords: vec![],
            og_tags: HashMap::new(),
            twitter_cards: HashMap::new(),
            canonical_url: None,
            robots: None,
            sitemap_url: None,
            rss_feed_url: None,
            author: None,
            generator: None,
            theme_color: None,
            viewport: None,
            language: None,
            charset: None,
            favicon_url: None,
            timestamp: Utc::now(),
        }
    }

    pub fn has_og_tags(&self) -> bool {
        !self.og_tags.is_empty()
    }

    pub fn has_twitter_cards(&self) -> bool {
        !self.twitter_cards.is_empty()
    }

    pub fn social_media_score(&self) -> f32 {
        let mut score = 0f32;
        let required_og = ["og:title", "og:description", "og:image", "og:url"];
        for tag in &required_og {
            if self.og_tags.contains_key(*tag) { score += 25.0; }
        }
        score
    }

    pub fn seo_summary(&self) -> String {
        let mut summary = String::new();
        if let Some(ref title) = self.title {
            summary.push_str(&format!("Title: {} ({} chars)\n", title, title.len()));
        } else {
            summary.push_str("Title: MISSING\n");
        }
        if let Some(ref desc) = self.description {
            summary.push_str(&format!("Description: {} ({} chars)\n", desc, desc.len()));
        } else {
            summary.push_str("Description: MISSING\n");
        }
        summary.push_str(&format!("Keywords: {}\n", self.keywords.len()));
        summary.push_str(&format!("OG Tags: {}\n", self.og_tags.len()));
        summary.push_str(&format!("Twitter Cards: {}\n", self.twitter_cards.len()));
        summary
    }
}

// ============================================================
// LINK GRAPH
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkGraph {
    pub root_url: String,
    pub internal_links: Vec<HtmlLink>,
    pub external_links: Vec<HtmlLink>,
    pub broken_links: Vec<BrokenLink>,
    pub redirect_chains: Vec<RedirectChain>,
    pub total_pages_crawled: usize,
    pub total_links_found: usize,
    pub max_depth_reached: usize,
    pub crawl_time_ms: u64,
    pub timestamp: DateTime<Utc>,
}

impl LinkGraph {
    pub fn new(root_url: &str) -> Self {
        LinkGraph {
            root_url: root_url.to_string(),
            internal_links: vec![],
            external_links: vec![],
            broken_links: vec![],
            redirect_chains: vec![],
            total_pages_crawled: 0,
            total_links_found: 0,
            max_depth_reached: 0,
            crawl_time_ms: 0,
            timestamp: Utc::now(),
        }
    }

    pub fn internal_count(&self) -> usize {
        self.internal_links.len()
    }

    pub fn external_count(&self) -> usize {
        self.external_links.len()
    }

    pub fn broken_count(&self) -> usize {
        self.broken_links.len()
    }

    pub fn broken_percentage(&self) -> f32 {
        if self.total_links_found == 0 {
            return 0.0;
        }
        (self.broken_links.len() as f32 / self.total_links_found as f32) * 100.0
    }

    pub fn health_score(&self) -> f32 {
        let broken_penalty = (self.broken_percentage() * 2.0).min(50.0);
        100.0 - broken_penalty
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenLink {
    pub url: String,
    pub status_code: Option<u16>,
    pub error_message: Option<String>,
    pub found_on_page: String,
    pub timestamp: DateTime<Utc>,
}

impl BrokenLink {
    pub fn new(url: &str, found_on: &str) -> Self {
        BrokenLink {
            url: url.to_string(),
            status_code: None,
            error_message: None,
            found_on_page: found_on.to_string(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectChain {
    pub source_url: String,
    pub chain: Vec<RedirectHop>,
    pub final_url: String,
    pub total_hops: usize,
    pub is_loop: bool,
    pub timestamp: DateTime<Utc>,
}

impl RedirectChain {
    pub fn new(source_url: &str) -> Self {
        RedirectChain {
            source_url: source_url.to_string(),
            chain: vec![],
            final_url: source_url.to_string(),
            total_hops: 0,
            is_loop: false,
            timestamp: Utc::now(),
        }
    }

    pub fn add_hop(&mut self, url: &str, status_code: u16) {
        self.chain.push(RedirectHop {
            url: url.to_string(),
            status_code,
        });
        self.final_url = url.to_string();
        self.total_hops = self.chain.len();

        // Deteksi loop: URL sudah pernah muncul di chain
        let urls: Vec<&str> = self.chain.iter().map(|h| h.url.as_str()).collect();
        if urls.len() > 1 {
            let last = urls.last().unwrap();
            if urls[..urls.len()-1].contains(last) {
                self.is_loop = true;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectHop {
    pub url: String,
    pub status_code: u16,
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_form_is_login() {
        let mut form = HtmlForm::new("POST");
        form.action = Some("/login".to_string());
        form.inputs.push({
            let mut input = FormInput::new("text");
            input.name = Some("username".to_string());
            input
        });
        form.inputs.push({
            let mut input = FormInput::new("password");
            input.name = Some("password".to_string());
            input
        });
        assert!(form.is_login_form());
    }

    #[test]
    fn test_html_form_detect_csrf() {
        let mut form = HtmlForm::new("POST");
        let mut input = FormInput::new("hidden");
        input.name = Some("csrf_token".to_string());
        input.value = Some("abc123".to_string());
        form.inputs.push(input);
        form.detect_csrf_token();
        assert!(form.has_csrf_token);
        assert_eq!(form.csrf_token_field, Some("csrf_token".to_string()));
    }

    #[test]
    fn test_form_input_sensitive() {
        let input = FormInput::new("password");
        assert!(input.is_sensitive_field());
        let input = FormInput::new("text");
        assert!(!input.is_sensitive_field());
    }

    #[test]
    fn test_html_script_dangerous() {
        let mut script = HtmlScript::new();
        script.content = Some("eval(userInput)".to_string());
        assert!(script.is_potentially_dangerous());

        let mut safe_script = HtmlScript::new();
        safe_script.content = Some("console.log('hello')".to_string());
        assert!(!safe_script.is_potentially_dangerous());
    }

    #[test]
    fn test_parsed_html_internal_links() {
        let mut html = ParsedHtml::new("https://example.com");
        html.links.push(HtmlLink::new("/about", true));
        html.links.push(HtmlLink::new("https://external.com", false));
        assert_eq!(html.internal_links().len(), 1);
        assert_eq!(html.external_links().len(), 1);
    }

    #[test]
    fn test_js_analysis_risk_level() {
        let mut js = JsAnalysis::new("app.js");
        assert_eq!(js.security_risk_level(), Severity::Info);

        js.api_keys.push(ApiKey::new("AWS", "AKIA...", "app.js", 10, 5));
        js.dangerous_functions.push(DangerousFunction::new("eval", 20, 3));
        assert_eq!(js.security_risk_level(), Severity::High);
    }

    #[test]
    fn test_meta_data_social_score() {
        let mut meta = MetaData::new("https://example.com");
        assert_eq!(meta.social_media_score(), 0.0);

        meta.og_tags.insert("og:title".to_string(), "Title".to_string());
        meta.og_tags.insert("og:description".to_string(), "Desc".to_string());
        meta.og_tags.insert("og:image".to_string(), "img.jpg".to_string());
        meta.og_tags.insert("og:url".to_string(), "https://example.com".to_string());
        assert_eq!(meta.social_media_score(), 100.0);
    }

    #[test]
    fn test_link_graph_health_score() {
        let mut graph = LinkGraph::new("https://example.com");
        graph.total_links_found = 100;
        graph.broken_links.push(BrokenLink::new("/404", "/home"));
        graph.broken_links.push(BrokenLink::new("/gone", "/about"));

        assert_eq!(graph.broken_percentage(), 2.0);
        assert!((graph.health_score() - 96.0).abs() < 0.1);
    }

    #[test]
    fn test_redirect_chain_loop_detection() {
        let mut chain = RedirectChain::new("/start");
        chain.add_hop("/page1", 302);
        chain.add_hop("/page2", 302);
        chain.add_hop("/page1", 302); // Loop back
        assert!(chain.is_loop);
        assert_eq!(chain.total_hops, 3);
    }

    #[test]
    fn test_css_info_efficiency() {
        let mut css = CssInfo::new("style.css");
        css.unused_percentage = 30.0;
        assert!((css.efficiency_score() - 70.0).abs() < 0.1);
    }

    #[test]
    fn test_meta_tag_types() {
        let mut tag = MetaTag::new();
        tag.property = Some("og:title".to_string());
        assert!(tag.is_og_tag());
        assert!(!tag.is_twitter_tag());

        let mut tag = MetaTag::new();
        tag.name = Some("twitter:card".to_string());
        assert!(!tag.is_og_tag());
        assert!(tag.is_twitter_tag());
    }
}
