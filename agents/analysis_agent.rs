// agents/analysis_agent.rs
// IWS v1.0 - Analysis Agent
// Menjalankan pipeline analisis pada data yang dikumpulkan

use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::agents::base_agent::{Agent, AgentState, AgentMessage, AgentStatus, AgentConfig};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub analysis_id: Uuid,
    pub scan_id: Uuid,
    pub status: String,
    pub findings: Vec<Finding>,
    pub risk_score: f64,
    pub confidence: f64,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub id: Uuid,
    pub finding_type: String,
    pub title: String,
    pub description: String,
    pub severity: String,
    pub confidence: f64,
    pub source: String,
}

pub struct AnalysisAgent {
    config: AgentConfig,
    state: AgentState,
    messages: Vec<AgentMessage>,
    results: HashMap<Uuid, AnalysisResult>,
    current_analysis: Option<AnalysisResult>,
}

impl AnalysisAgent {
    pub fn new(config: AgentConfig) -> Self {
        AnalysisAgent {
            config,
            state: AgentState::Uninitialized,
            messages: Vec::new(),
            results: HashMap::new(),
            current_analysis: None,
        }
    }

    pub async fn analyze_scan(&mut self, scan_id: Uuid, scan_data: &[u8]) -> Result<AnalysisResult, String> {
        let analysis_id = Uuid::new_v4();
        let mut result = AnalysisResult {
            analysis_id,
            scan_id,
            status: "analyzing".to_string(),
            findings: Vec::new(),
            risk_score: 0.0,
            confidence: 0.0,
            started_at: Utc::now(),
            completed_at: None,
        };

        // Stage 1: Preprocessing
        self.process_stage("preprocessing", &mut result).await?;

        // Stage 2: Pattern Detection
        let patterns = self.detect_patterns(scan_data).await?;
        result.findings.extend(patterns);

        // Stage 3: Vulnerability Matching
        let vulns = self.match_vulnerabilities(scan_data).await?;
        result.findings.extend(vulns);

        // Stage 4: Risk Scoring
        result.risk_score = self.calculate_risk(&result.findings).await?;

        // Stage 5: Confidence Scoring
        result.confidence = self.calculate_confidence(&result.findings);

        result.status = "completed".to_string();
        result.completed_at = Some(Utc::now());

        self.results.insert(analysis_id, result.clone());
        self.current_analysis = Some(result.clone());

        Ok(result)
    }

    async fn process_stage(&mut self, stage: &str, result: &mut AnalysisResult) -> Result<(), String> {
        result.status = format!("processing_{}", stage);
        Ok(())
    }

    async fn detect_patterns(&self, data: &[u8]) -> Result<Vec<Finding>, String> {
        let mut findings = Vec::new();
        let text = String::from_utf8_lossy(data);

        // Deteksi XSS patterns
        if text.contains("<script>") || text.contains("javascript:") {
            findings.push(Finding {
                id: Uuid::new_v4(),
                finding_type: "xss".into(),
                title: "Potential XSS vulnerability".into(),
                description: "Script injection pattern detected".into(),
                severity: "high".into(),
                confidence: 0.8,
                source: "pattern_detection".into(),
            });
        }

        // Deteksi SQL injection patterns
        if text.contains("SELECT") && text.contains("FROM") && text.contains("'") {
            findings.push(Finding {
                id: Uuid::new_v4(),
                finding_type: "sql_injection".into(),
                title: "Potential SQL injection".into(),
                description: "SQL query pattern detected".into(),
                severity: "critical".into(),
                confidence: 0.7,
                source: "pattern_detection".into(),
            });
        }

        Ok(findings)
    }

    async fn match_vulnerabilities(&self, data: &[u8]) -> Result<Vec<Finding>, String> {
        let mut findings = Vec::new();
        let text = String::from_utf8_lossy(data);

        // Cek header security
        if !text.contains("Strict-Transport-Security") {
            findings.push(Finding {
                id: Uuid::new_v4(),
                finding_type: "missing_header".into(),
                title: "Missing HSTS header".into(),
                description: "HTTP Strict Transport Security header not found".into(),
                severity: "medium".into(),
                confidence: 0.95,
                source: "vulnerability_matching".into(),
            });
        }

        if !text.contains("Content-Security-Policy") {
            findings.push(Finding {
                id: Uuid::new_v4(),
                finding_type: "missing_header".into(),
                title: "Missing CSP header".into(),
                description: "Content Security Policy header not found".into(),
                severity: "medium".into(),
                confidence: 0.95,
                source: "vulnerability_matching".into(),
            });
        }

        Ok(findings)
    }

    async fn calculate_risk(&self, findings: &[Finding]) -> Result<f64, String> {
        if findings.is_empty() { return Ok(0.0); }

        let total: f64 = findings.iter()
            .map(|f| match f.severity.as_str() {
                "critical" => 10.0,
                "high" => 7.5,
                "medium" => 5.0,
                "low" => 2.5,
                _ => 0.0,
            })
            .sum();

        Ok((total / findings.len() as f64 * 10.0).round() / 10.0)
    }

    fn calculate_confidence(&self, findings: &[Finding]) -> f64 {
        if findings.is_empty() { return 100.0; }
        let total: f64 = findings.iter().map(|f| f.confidence).sum();
        total / findings.len() as f64
    }

    pub fn get_current_analysis(&self) -> Option<&AnalysisResult> {
        self.current_analysis.as_ref()
    }

    pub fn get_result(&self, analysis_id: &Uuid) -> Option<&AnalysisResult> {
        self.results.get(analysis_id)
    }

    pub fn get_stats(&self) -> AnalysisStats {
        AnalysisStats {
            total_analyses: self.results.len(),
            completed: self.results.values().filter(|r| r.status == "completed").count(),
            failed: self.results.values().filter(|r| r.status == "failed").count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisStats {
    pub total_analyses: usize,
    pub completed: usize,
    pub failed: usize,
}

#[async_trait]
impl Agent for AnalysisAgent {
    async fn init(&mut self) -> Result<(), String> { self.state = AgentState::Initialized; Ok(()) }
    async fn run(&mut self) -> Result<(), String> { self.state = AgentState::Running; Ok(()) }
    async fn pause(&mut self) -> Result<(), String> { self.state = AgentState::Paused; Ok(()) }
    async fn resume(&mut self) -> Result<(), String> { self.state = AgentState::Running; Ok(()) }
    async fn shutdown(&mut self) -> Result<(), String> { self.state = AgentState::Shutdown; Ok(()) }
    fn get_state(&self) -> AgentState { self.state.clone() }
    fn get_id(&self) -> Uuid { self.config.agent_id }
    fn get_name(&self) -> &str { &self.config.agent_name }
    fn get_type(&self) -> &str { "analysis" }
    async fn send_message(&self, _msg: AgentMessage) -> Result<(), String> { Ok(()) }
    async fn receive_message(&mut self) -> Option<AgentMessage> { self.messages.pop() }
    fn get_status(&self) -> AgentStatus {
        AgentStatus {
            agent_id: self.config.agent_id,
            state: self.state.clone(),
            uptime_secs: 0, messages_processed: 0,
            tasks_completed: self.results.len() as u64,
            tasks_failed: 0, last_heartbeat: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analyze_scan() {
        let config = AgentConfig { agent_name: "analysis-test".into(), ..Default::default() };
        let mut agent = AnalysisAgent::new(config);
        agent.init().await.unwrap();

        let scan_data = b"HTTP/1.1 200 OK\nServer: nginx\n<script>alert(1)</script>\nSELECT * FROM users WHERE '1'='1'";
        let result = agent.analyze_scan(Uuid::new_v4(), scan_data).await.unwrap();

        assert_eq!(result.status, "completed");
        assert!(result.findings.len() >= 2);
        assert!(result.risk_score > 0.0);
    }

    #[test]
    fn test_risk_calculation() {
        let config = AgentConfig { agent_name: "risk-test".into(), ..Default::default() };
        let agent = AnalysisAgent::new(config);
        let findings = vec![
            Finding {
                id: Uuid::new_v4(), finding_type: "xss".into(),
                title: "XSS".into(), description: "".into(),
                severity: "critical".into(), confidence: 0.9, source: "test".into(),
            },
        ];
        let rt = tokio::runtime::Runtime::new().unwrap();
        let risk = rt.block_on(agent.calculate_risk(&findings)).unwrap();
        assert!(risk > 7.0);
    }
}
