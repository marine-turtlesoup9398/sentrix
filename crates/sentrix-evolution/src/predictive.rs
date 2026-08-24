use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_impact::ImpactEngine;
use sentrix_ir::{ConfidenceLevel, Evidence, EvidenceSourceType, EvidenceStrength, FileItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureContribution {
    pub feature_name: String,
    pub weight: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveRiskReport {
    pub target_component: String,
    pub predicted_risk: RiskLevel,
    pub confidence: ConfidenceLevel,
    pub total_risk_score: f32,
    pub feature_contributions: Vec<FeatureContribution>,
    pub direct_impact_count: usize,
    pub transitive_impact_count: usize,
    pub historical_bugfix_associations: usize,
    pub co_changed_files_count: usize,
    pub evidence: Vec<Evidence>,
    pub limitations: String,
}

pub struct PredictiveRiskEngine;

impl PredictiveRiskEngine {
    pub fn predict_change_risk(
        target: &str,
        files: &[FileItem],
        graph: &SoftwareKnowledgeGraph,
        historical_bugfixes: usize,
        co_changed_count: usize,
    ) -> PredictiveRiskReport {
        let mut features = Vec::new();
        let mut evidence = Vec::new();
        let mut score = 0.0;

        // 1. Structural Impact
        let mut direct_cnt = 0;
        let mut trans_cnt = 0;
        if let Ok(impact) =
            ImpactEngine::analyze_impact(graph, files, &[target.to_string()], target)
        {
            direct_cnt = impact.direct_dependents.len();
            trans_cnt = impact.transitive_downstream.len();
            let impact_w = ((direct_cnt * 3 + trans_cnt) as f32 * 2.5).min(35.0);
            score += impact_w;

            features.push(FeatureContribution {
                feature_name: "Structural Impact Radius".to_string(),
                weight: impact_w,
                description: format!(
                    "Directly affects {} components, transitively affects {} components",
                    direct_cnt, trans_cnt
                ),
            });

            evidence.push(Evidence {
                id: format!("ev_risk_impact_{}", target),
                source_type: EvidenceSourceType::CallGraph,
                file_path: Some(target.to_string()),
                line: None,
                column: None,
                symbol: Some(target.to_string()),
                relationship: Some("AFFECTS".to_string()),
                commit: None,
                description: format!(
                    "Structural graph ripple affects {} downstream entities",
                    impact.total_affected_nodes
                ),
                strength: EvidenceStrength::DirectlyObserved,
                confidence: ConfidenceLevel::High,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }

        // 2. Historical Churn & Bugfixes
        let bugfix_w = (historical_bugfixes as f32 * 5.0).min(30.0);
        score += bugfix_w;
        features.push(FeatureContribution {
            feature_name: "Historical Bugfix Frequency".to_string(),
            weight: bugfix_w,
            description: format!(
                "Associated with {} historical bugfix/revert commits",
                historical_bugfixes
            ),
        });

        // 3. Co-Change Propagation Risk
        let co_change_w = (co_changed_count as f32 * 2.0).min(20.0);
        score += co_change_w;
        features.push(FeatureContribution {
            feature_name: "Historical Co-Change Risk".to_string(),
            weight: co_change_w,
            description: format!(
                "Historically co-changed with {} other repository files",
                co_changed_count
            ),
        });

        // 4. Security Sensitivity Check
        let is_sec = target.contains("auth")
            || target.contains("security")
            || target.contains("crypto")
            || target.contains("payment")
            || target.contains("user");
        let sec_w = if is_sec { 15.0 } else { 0.0 };
        score += sec_w;
        features.push(FeatureContribution {
            feature_name: "Security Sensitivity".to_string(),
            weight: sec_w,
            description: if is_sec {
                "Component handles authentication, payments, or security boundaries"
            } else {
                "Standard component"
            }
            .to_string(),
        });

        let level = if score >= 50.0 {
            RiskLevel::High
        } else if score >= 25.0 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        PredictiveRiskReport {
            target_component: target.to_string(),
            predicted_risk: level,
            confidence: ConfidenceLevel::Medium,
            total_risk_score: score,
            feature_contributions: features,
            direct_impact_count: direct_cnt,
            transitive_impact_count: trans_cnt,
            historical_bugfix_associations: historical_bugfixes,
            co_changed_files_count: co_changed_count,
            evidence,
            limitations: "Predictive risk represents historical and structural heuristics. It does not establish definite runtime behavior or production failure certainty.".to_string(),
        }
    }
}
