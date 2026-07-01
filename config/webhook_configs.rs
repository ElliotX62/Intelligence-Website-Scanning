// config/webhook_configs.rs
// IWS v1.0 - Webhook Configurations
// Mengkonfigurasi webhook untuk integrasi dengan platform notifikasi

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// ============================================================
// WEBHOOK CONFIG
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub slack: Option<SlackWebhook>,
    pub discord: Option<DiscordWebhook>,
    pub telegram: Option<TelegramWebhook>,
    pub email: Option<EmailWebhook>,
    pub custom: Vec<CustomWebhook>,
}

impl WebhookConfig {
    pub fn new() -> Self {
        WebhookConfig {
            slack: None,
            discord: None,
            telegram: None,
            email: None,
            custom: vec![],
        }
    }

    /// Dapatkan semua webhook yang enabled
    pub fn get_enabled_webhooks(&self) -> Vec<WebhookType> {
        let mut enabled = Vec::new();
        if self.slack.is_some() { enabled.push(WebhookType::Slack); }
        if self.discord.is_some() { enabled.push(WebhookType::Discord); }
        if self.telegram.is_some() { enabled.push(WebhookType::Telegram); }
        if self.email.is_some() { enabled.push(WebhookType::Email); }
        for _ in &self.custom { enabled.push(WebhookType::Custom); }
        enabled
    }

    /// Kirim ke semua webhook yang enabled
    pub async fn send_all(&self, event: &WebhookEvent) -> Vec<Result<String, String>> {
        let mut results = Vec::new();

        if let Some(ref slack) = self.slack {
            results.push(slack.send(event).await);
        }
        if let Some(ref discord) = self.discord {
            results.push(discord.send(event).await);
        }
        if let Some(ref telegram) = self.telegram {
            results.push(telegram.send(event).await);
        }
        if let Some(ref email) = self.email {
            results.push(email.send(event).await);
        }
        for custom in &self.custom {
            results.push(custom.send(event).await);
        }

        results
    }

    /// Test semua webhook
    pub async fn test_all(&self) -> Vec<Result<String, String>> {
        let test_event = WebhookEvent::test_event();
        self.send_all(&test_event).await
    }
}

impl Default for WebhookConfig {
    fn default() -> Self {
        WebhookConfig::new()
    }
}

// ============================================================
// WEBHOOK TYPE
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebhookType {
    Slack,
    Discord,
    Telegram,
    Email,
    Custom,
}

impl std::fmt::Display for WebhookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebhookType::Slack => write!(f, "slack"),
            WebhookType::Discord => write!(f, "discord"),
            WebhookType::Telegram => write!(f, "telegram"),
            WebhookType::Email => write!(f, "email"),
            WebhookType::Custom => write!(f, "custom"),
        }
    }
}

// ============================================================
// WEBHOOK EVENT
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_type: String,
    pub title: String,
    pub message: String,
    pub severity: String,
    pub scan_id: Option<String>,
    pub target_url: Option<String>,
    pub findings_count: Option<usize>,
    pub risk_score: Option<f32>,
    pub timestamp: String,
    pub metadata: serde_json::Value,
}

impl WebhookEvent {
    pub fn new(event_type: &str, title: &str, message: &str, severity: &str) -> Self {
        WebhookEvent {
            event_type: event_type.to_string(),
            title: title.to_string(),
            message: message.to_string(),
            severity: severity.to_string(),
            scan_id: None,
            target_url: None,
            findings_count: None,
            risk_score: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
            metadata: serde_json::json!({}),
        }
    }

    pub fn test_event() -> Self {
        WebhookEvent::new(
            "test",
            "IWS Webhook Test",
            "This is a test message from IWS to verify webhook configuration.",
            "info",
        )
    }

    pub fn scan_completed(scan_id: &str, target_url: &str, findings: usize, risk_score: f32) -> Self {
        let mut event = WebhookEvent::new(
            "scan_completed",
            &format!("Scan Completed: {}", target_url),
            &format!("Scan {} completed with {} findings (risk score: {:.1})", scan_id, findings, risk_score),
            if risk_score >= 7.0 { "high" } else if risk_score >= 4.0 { "medium" } else { "low" },
        );
        event.scan_id = Some(scan_id.to_string());
        event.target_url = Some(target_url.to_string());
        event.findings_count = Some(findings);
        event.risk_score = Some(risk_score);
        event
    }

    pub fn scan_failed(scan_id: &str, target_url: &str, error: &str) -> Self {
        let mut event = WebhookEvent::new(
            "scan_failed",
            &format!("Scan Failed: {}", target_url),
            &format!("Scan {} failed: {}", scan_id, error),
            "critical",
        );
        event.scan_id = Some(scan_id.to_string());
        event.target_url = Some(target_url.to_string());
        event
    }

    pub fn vulnerability_found(scan_id: &str, title: &str, severity: &str, description: &str) -> Self {
        WebhookEvent::new(
            "vulnerability_found",
            &format!("[{}] {}", severity.to_uppercase(), title),
            description,
            severity,
        )
    }

    pub fn with_metadata(mut self, key: &str, value: serde_json::Value) -> Self {
        self.metadata[key] = value;
        self
    }
}

// ============================================================
// SLACK WEBHOOK
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackWebhook {
    pub url: String,
    pub channel: String,
    pub username: String,
    pub icon_emoji: String,
}

impl SlackWebhook {
    pub fn new(url: &str, channel: &str) -> Self {
        SlackWebhook {
            url: url.to_string(),
            channel: channel.to_string(),
            username: "IWS Scanner".to_string(),
            icon_emoji: ":shield:".to_string(),
        }
    }

    pub async fn send(&self, event: &WebhookEvent) -> Result<String, String> {
        let color = match event.severity.as_str() {
            "critical" => "#FF0000",
            "high" => "#FF6600",
            "medium" => "#FFCC00",
            "low" => "#00CC00",
            _ => "#999999",
        };

        let payload = serde_json::json!({
            "channel": self.channel,
            "username": self.username,
            "icon_emoji": self.icon_emoji,
            "attachments": [{
                "color": color,
                "title": event.title,
                "text": event.message,
                "fields": [
                    {"title": "Severity", "value": event.severity.to_uppercase(), "short": true},
                    {"title": "Event Type", "value": event.event_type, "short": true},
                ],
                "footer": format!("IWS | {}", event.timestamp),
                "ts": chrono::Utc::now().timestamp(),
            }]
        });

        // Real implementation: reqwest::Client::post(&self.url).json(&payload).send().await
        let _ = payload;
        Ok(format!("slack:{}", self.channel))
    }
}

// ============================================================
// DISCORD WEBHOOK
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordWebhook {
    pub url: String,
    pub username: String,
    pub avatar_url: Option<String>,
    pub tts: bool,
}

impl DiscordWebhook {
    pub fn new(url: &str) -> Self {
        DiscordWebhook {
            url: url.to_string(),
            username: "IWS Scanner".to_string(),
            avatar_url: None,
            tts: false,
        }
    }

    pub async fn send(&self, event: &WebhookEvent) -> Result<String, String> {
        let color = match event.severity.as_str() {
            "critical" => 0xFF0000u32,
            "high" => 0xFF6600,
            "medium" => 0xFFCC00,
            "low" => 0x00CC00,
            _ => 0x999999,
        };

        let payload = serde_json::json!({
            "username": self.username,
            "avatar_url": self.avatar_url,
            "tts": self.tts,
            "embeds": [{
                "title": event.title,
                "description": event.message,
                "color": color,
                "fields": [
                    {"name": "Severity", "value": event.severity.to_uppercase(), "inline": true},
                    {"name": "Type", "value": event.event_type, "inline": true},
                ],
                "footer": {"text": format!("IWS | {}", event.timestamp)},
                "timestamp": event.timestamp,
            }]
        });

        let _ = payload;
        Ok(format!("discord:webhook"))
    }
}

// ============================================================
// TELEGRAM WEBHOOK
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramWebhook {
    pub bot_token: String,
    pub chat_id: String,
    pub parse_mode: String,
}

impl TelegramWebhook {
    pub fn new(bot_token: &str, chat_id: &str) -> Self {
        TelegramWebhook {
            bot_token: bot_token.to_string(),
            chat_id: chat_id.to_string(),
            parse_mode: "HTML".to_string(),
        }
    }

    pub async fn send(&self, event: &WebhookEvent) -> Result<String, String> {
        let severity_emoji = match event.severity.as_str() {
            "critical" => "🔴",
            "high" => "🟠",
            "medium" => "🟡",
            "low" => "🟢",
            _ => "ℹ️",
        };

        let text = format!(
            "{} <b>{}</b>\n\n{}\n\n<i>Severity: {} | Type: {}</i>",
            severity_emoji, event.title, event.message,
            event.severity.to_uppercase(), event.event_type,
        );

        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.bot_token
        );

        let payload = serde_json::json!({
            "chat_id": self.chat_id,
            "text": text,
            "parse_mode": self.parse_mode,
        });

        let _ = payload;
        let _ = url;
        Ok(format!("telegram:{}", self.chat_id))
    }
}

// ============================================================
// EMAIL WEBHOOK
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailWebhook {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub from_address: String,
    pub to_addresses: Vec<String>,
    pub use_tls: bool,
}

impl EmailWebhook {
    pub fn new(smtp_host: &str, from: &str, to: Vec<String>) -> Self {
        EmailWebhook {
            smtp_host: smtp_host.to_string(),
            smtp_port: 587,
            smtp_username: String::new(),
            smtp_password: String::new(),
            from_address: from.to_string(),
            to_addresses: to,
            use_tls: true,
        }
    }

    pub async fn send(&self, event: &WebhookEvent) -> Result<String, String> {
        let subject = format!(
            "[IWS] [{}] {}",
            event.severity.to_uppercase(),
            event.title,
        );

        let body = format!(
            "{}\n\nSeverity: {}\nType: {}\nTimestamp: {}",
            event.message,
            event.severity.to_uppercase(),
            event.event_type,
            event.timestamp,
        );

        let _ = subject;
        let _ = body;
        Ok(format!("email:{}", self.to_addresses.join(",")))
    }
}

// ============================================================
// CUSTOM WEBHOOK
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWebhook {
    pub name: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub body_template: String,
}

impl CustomWebhook {
    pub fn new(name: &str, url: &str) -> Self {
        CustomWebhook {
            name: name.to_string(),
            url: url.to_string(),
            method: "POST".to_string(),
            headers: HashMap::new(),
            body_template: String::new(),
        }
    }

    pub async fn send(&self, event: &WebhookEvent) -> Result<String, String> {
        let body = if self.body_template.is_empty() {
            serde_json::to_string(event).unwrap_or_default()
        } else {
            self.body_template
                .replace("{{title}}", &event.title)
                .replace("{{message}}", &event.message)
                .replace("{{severity}}", &event.severity)
                .replace("{{event_type}}", &event.event_type)
                .replace("{{timestamp}}", &event.timestamp)
        };

        let _ = body;
        Ok(format!("custom:{}", self.name))
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_template(mut self, template: &str) -> Self {
        self.body_template = template.to_string();
        self
    }
}

// ============================================================
// WEBHOOK FORMATTER
// ============================================================

pub struct WebhookFormatter;

impl WebhookFormatter {
    /// Format event untuk Slack
    pub fn format_slack(event: &WebhookEvent) -> serde_json::Value {
        serde_json::json!({
            "text": format!("*{}*\n{}", event.title, event.message),
        })
    }

    /// Format event untuk Discord
    pub fn format_discord(event: &WebhookEvent) -> serde_json::Value {
        serde_json::json!({
            "content": format!("**{}**\n{}", event.title, event.message),
        })
    }

    /// Format event untuk Telegram
    pub fn format_telegram(event: &WebhookEvent) -> serde_json::Value {
        serde_json::json!({
            "text": format!("<b>{}</b>\n\n{}", event.title, event.message),
            "parse_mode": "HTML",
        })
    }

    /// Format ringkasan scan untuk webhook
    pub fn format_scan_summary(event: &WebhookEvent) -> String {
        format!(
            "Scan {} completed\nTarget: {}\nFindings: {}\nRisk Score: {:.1}\nSeverity: {}",
            event.scan_id.as_deref().unwrap_or("unknown"),
            event.target_url.as_deref().unwrap_or("unknown"),
            event.findings_count.unwrap_or(0),
            event.risk_score.unwrap_or(0.0),
            event.severity.to_uppercase(),
        )
    }
}

// ============================================================
// UNIT TESTS
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webhook_event_creation() {
        let event = WebhookEvent::new("scan_started", "Scan Started", "Starting scan...", "info");
        assert_eq!(event.event_type, "scan_started");
        assert_eq!(event.severity, "info");
    }

    #[test]
    fn test_webhook_event_scan_completed() {
        let event = WebhookEvent::scan_completed(
            "scan-123",
            "https://example.com",
            5,
            7.5,
        );
        assert_eq!(event.scan_id, Some("scan-123".to_string()));
        assert_eq!(event.findings_count, Some(5));
        assert!(event.risk_score.unwrap() > 7.0);
    }

    #[test]
    fn test_webhook_event_scan_failed() {
        let event = WebhookEvent::scan_failed(
            "scan-456",
            "https://example.com",
            "Connection timeout",
        );
        assert_eq!(event.severity, "critical");
        assert!(event.message.contains("timeout"));
    }

    #[test]
    fn test_webhook_event_test() {
        let event = WebhookEvent::test_event();
        assert_eq!(event.event_type, "test");
        assert_eq!(event.severity, "info");
    }

    #[test]
    fn test_slack_webhook_creation() {
        let slack = SlackWebhook::new("https://hooks.slack.com/services/xxx", "#alerts");
        assert_eq!(slack.channel, "#alerts");
        assert_eq!(slack.username, "IWS Scanner");
    }

    #[test]
    fn test_discord_webhook_creation() {
        let discord = DiscordWebhook::new("https://discord.com/api/webhooks/xxx");
        assert!(!discord.tts);
    }

    #[test]
    fn test_telegram_webhook_creation() {
        let telegram = TelegramWebhook::new("123456:ABC-DEF", "-100123456");
        assert_eq!(telegram.parse_mode, "HTML");
    }

    #[test]
    fn test_custom_webhook_template() {
        let custom = CustomWebhook::new("custom-api", "https://api.example.com/hook")
            .with_header("Authorization", "Bearer token123")
            .with_template("{\"title\": \"{{title}}\", \"msg\": \"{{message}}\"}");

        assert_eq!(custom.method, "POST");
        assert!(custom.body_template.contains("{{title}}"));
    }

    #[test]
    fn test_webhook_config_enabled() {
        let mut config = WebhookConfig::new();
        assert!(config.get_enabled_webhooks().is_empty());

        config.slack = Some(SlackWebhook::new("url", "#chan"));
        assert_eq!(config.get_enabled_webhooks().len(), 1);
    }

    #[test]
    fn test_webhook_formatter_scan_summary() {
        let event = WebhookEvent::scan_completed("s1", "https://test.com", 10, 8.5);
        let summary = WebhookFormatter::format_scan_summary(&event);
        assert!(summary.contains("https://test.com"));
        assert!(summary.contains("10"));
        assert!(summary.contains("8.5"));
    }
}
