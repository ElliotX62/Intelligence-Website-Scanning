// models/risk_scorer.rs
// IWS v1.0 - Risk Scorer
// Menghitung risk score berdasarkan CVSS v3.1 dan business context

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvssMetrics {
    pub attack_vector: AttackVector,
    pub attack_complexity: AttackComplexity,
    pub privileges_required: PrivilegesRequired,
    pub user_interaction: UserInteraction,
    pub scope: Scope,
    pub confidentiality: Impact,
    pub integrity: Impact,
    pub availability: Impact,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AttackVector { Network, Adjacent, Local, Physical }

impl AttackVector {
    pub fn weight(&self) -> f64 {
        match self { AttackVector::Network => 0.85, AttackVector::Adjacent => 0.62, AttackVector::Local => 0.55, AttackVector::Physical => 0.20 }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AttackComplexity { Low, High }

impl AttackComplexity {
    pub fn weight(&self) -> f64 { match self { AttackComplexity::Low => 0.77, AttackComplexity::High => 0.44 } }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PrivilegesRequired { None, Low, High }

impl PrivilegesRequired {
    pub fn weight(&self, scope: Scope) -> f64 {
        match (self, scope) {
            (PrivilegesRequired::None, _) => 0.85,
            (PrivilegesRequired::Low, Scope::Changed) => 0.68,
            (PrivilegesRequired::Low, Scope::Unchanged) => 0.62,
            (PrivilegesRequired::High, Scope::Changed) => 0.50,
            (PrivilegesRequired::High, Scope::Unchanged) => 0.27,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum UserInteraction { None, Required }

impl UserInteraction {
    pub fn weight(&self) -> f64 { match self { UserInteraction::None => 0.85, UserInteraction::Required => 0.62 } }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Scope { Unchanged, Changed }

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Impact { None, Low, High }

impl Impact {
    pub fn weight(&self) -> f64 { match self { Impact::None => 0.0, Impact::Low => 0.22, Impact::High => 0.56 } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub base_score: f64,
    pub temporal_score: f64,
    pub environmental_score: f64,
    pub business_score: f64,
    pub overall_score: f64,
    pub severity: String,
    pub priority: String,
    pub cvss_vector: String,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessContext {
    pub asset_value: f64,
    pub data_sensitivity: f64,
    pub regulatory_impact: f64,
    pub operational_impact: f64,
    pub reputational_impact: f64,
}

impl Default for BusinessContext {
    fn default() -> Self {
        BusinessContext {
            asset_value: 0.5, data_sensitivity: 0.5,
            regulatory_impact: 0.0, operational_impact: 0.5, reputational_impact: 0.3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalMetrics {
    pub exploit_code_maturity: f64,
    pub remediation_level: f64,
    pub report_confidence: f64,
}

impl Default for TemporalMetrics {
    fn default() -> Self {
        TemporalMetrics { exploit_code_maturity: 1.0, remediation_level: 1.0, report_confidence: 1.0 }
    }
}

pub struct RiskScorer;

impl RiskScorer {
    pub fn calculate_cvss(metrics: &CvssMetrics) -> RiskScore {
        let av = metrics.attack_vector.weight();
        let ac = metrics.attack_complexity.weight();
        let pr = metrics.privileges_required.weight(metrics.scope);
        let ui = metrics.user_interaction.weight();
        let c = metrics.confidentiality.weight();
        let i = metrics.integrity.weight();
        let a = metrics.availability.weight();

        let iss = 1.0 - ((1.0 - c) * (1.0 - i) * (1.0 - a));
        let impact = if metrics.scope == Scope::Unchanged { 6.42 * iss } else { 7.52 * (iss - 0.029) - 3.25 * (iss - 0.02).powf(15.0) };
        let exploitability = 8.22 * av * ac * pr * ui;
        let base = if impact <= 0.0 { 0.0 } else if metrics.scope == Scope::Unchanged {
            (impact + exploitability).min(10.0)
        } else {
            (1.08 * (impact + exploitability)).min(10.0)
        };

        let severity = if base >= 9.0 { "critical" } else if base >= 7.0 { "high" } else if base >= 4.0 { "medium" } else if base >= 0.1 { "low" } else { "info" };
        let priority = if base >= 9.0 { "P0" } else if base >= 7.0 { "P1" } else if base >= 4.0 { "P2" } else if base >= 0.1 { "P3" } else { "P4" };

        RiskScore {
            base_score: (base * 10.0).round() / 10.0,
            temporal_score: base,
            environmental_score: base,
            business_score: 0.0,
            overall_score: base,
            severity: severity.to_string(),
            priority: priority.to_string(),
            cvss_vector: format!("CVSS:3.1/AV:{}/AC:{}/PR:{}/UI:{}/S:{}/C:{}/I:{}/A:{}",
                Self::fmt_av(metrics.attack_vector), Self::fmt_ac(metrics.attack_complexity),
                Self::fmt_pr(metrics.privileges_required), Self::fmt_ui(metrics.user_interaction),
                Self::fmt_s(metrics.scope), Self::fmt_impact(metrics.confidentiality),
                Self::fmt_impact(metrics.integrity), Self::fmt_impact(metrics.availability)),
            recommendations: Self::generate_recommendations(metrics),
        }
    }

    pub fn with_business_context(base: &RiskScore, ctx: &BusinessContext) -> RiskScore {
        let biz_factor = (ctx.asset_value + ctx.data_sensitivity + ctx.regulatory_impact + ctx.operational_impact + ctx.reputational_impact) / 5.0;
        let business_score = (base.base_score * 0.6 + biz_factor * 4.0).min(10.0);
        let overall = (base.base_score * 0.5 + business_score * 0.5).min(10.0);

        RiskScore {
            base_score: base.base_score,
            temporal_score: base.temporal_score,
            environmental_score: base.environmental_score,
            business_score: (business_score * 10.0).round() / 10.0,
            overall_score: (overall * 10.0).round() / 10.0,
            severity: if overall >= 9.0 { "critical" } else if overall >= 7.0 { "high" } else if overall >= 4.0 { "medium" } else if overall >= 0.1 { "low" } else { "info" }.to_string(),
            priority: base.priority.clone(),
            cvss_vector: base.cvss_vector.clone(),
            recommendations: base.recommendations.clone(),
        }
    }

    pub fn with_temporal(base: &RiskScore, temporal: &TemporalMetrics) -> RiskScore {
        let temporal_factor = temporal.exploit_code_maturity * temporal.remediation_level * temporal.report_confidence;
        let temporal_score = (base.base_score * temporal_factor * 10.0).round() / 10.0;

        RiskScore {
            base_score: base.base_score,
            temporal_score: temporal_score.min(10.0),
            environmental_score: base.environmental_score,
            business_score: base.business_score,
            overall_score: (temporal_score * 0.7 + base.business_score * 0.3).min(10.0),
            severity: base.severity.clone(),
            priority: base.priority.clone(),
            cvss_vector: base.cvss_vector.clone(),
            recommendations: base.recommendations.clone(),
        }
    }

    fn generate_recommendations(m: &CvssMetrics) -> Vec<String> {
        let mut recs = Vec::new();
        if let Impact::High = m.confidentiality { recs.push("Encrypt sensitive data at rest and in transit".into()); }
        if let Impact::High = m.integrity { recs.push("Implement input validation and integrity checks".into()); }
        if let Impact::High = m.availability { recs.push("Deploy redundancy and DDoS protection".into()); }
        if m.privileges_required == PrivilegesRequired::None { recs.push("Implement proper authentication and authorization".into()); }
        if m.attack_vector == AttackVector::Network { recs.push("Restrict network access with firewall rules".into()); }
        recs
    }

    fn fmt_av(av: AttackVector) -> &'static str { match av { AttackVector::Network => "N", AttackVector::Adjacent => "A", AttackVector::Local => "L", AttackVector::Physical => "P" } }
    fn fmt_ac(ac: AttackComplexity) -> &'static str { match ac { AttackComplexity::Low => "L", AttackComplexity::High => "H" } }
    fn fmt_pr(pr: PrivilegesRequired) -> &'static str { match pr { PrivilegesRequired::None => "N", PrivilegesRequired::Low => "L", PrivilegesRequired::High => "H" } }
    fn fmt_ui(ui: UserInteraction) -> &'static str { match ui { UserInteraction::None => "N", UserInteraction::Required => "R" } }
    fn fmt_s(s: Scope) -> &'static str { match s { Scope::Unchanged => "U", Scope::Changed => "C" } }
    fn fmt_impact(i: Impact) -> &'static str { match i { Impact::None => "N", Impact::Low => "L", Impact::High => "H" } }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn critical_metrics() -> CvssMetrics {
        CvssMetrics {
            attack_vector: AttackVector::Network, attack_complexity: AttackComplexity::Low,
            privileges_required: PrivilegesRequired::None, user_interaction: UserInteraction::None,
            scope: Scope::Unchanged, confidentiality: Impact::High, integrity: Impact::High, availability: Impact::High,
        }
    }

    #[test]
    fn test_critical_score() {
        let score = RiskScorer::calculate_cvss(&critical_metrics());
        assert!(score.base_score >= 9.0);
        assert_eq!(score.severity, "critical");
        assert_eq!(score.priority, "P0");
        assert!(score.cvss_vector.contains("CVSS:3.1"));
    }

    #[test]
    fn test_business_context_increases_risk() {
        let base = RiskScorer::calculate_cvss(&critical_metrics());
        let ctx = BusinessContext {
            asset_value: 1.0, data_sensitivity: 1.0, regulatory_impact: 1.0,
            operational_impact: 1.0, reputational_impact: 1.0,
        };
        let biz_score = RiskScorer::with_business_context(&base, &ctx);
        assert!(biz_score.business_score >= base.base_score);
    }

    #[test]
    fn test_temporal_reduces_score() {
        let base = RiskScorer::calculate_cvss(&critical_metrics());
        let temporal = TemporalMetrics {
            exploit_code_maturity: 0.5, remediation_level: 0.8, report_confidence: 0.7,
        };
        let temp_score = RiskScorer::with_temporal(&base, &temporal);
        assert!(temp_score.temporal_score <= base.base_score);
    }

    #[test]
    fn test_recommendations_generated() {
        let score = RiskScorer::calculate_cvss(&critical_metrics());
        assert!(!score.recommendations.is_empty());
    }
}
