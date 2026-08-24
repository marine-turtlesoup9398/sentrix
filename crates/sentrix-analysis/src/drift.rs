use petgraph::visit::EdgeRef;
use sentrix_core::config::ArchitectureRule;
use sentrix_graph::{EdgeType, SoftwareKnowledgeGraph};
use sentrix_ir::{ConfidenceLevel, Evidence, EvidenceSourceType, EvidenceStrength, FileItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureViolation {
    pub rule_name: String,
    pub source_component: String,
    pub target_component: String,
    pub violation_type: String,
    pub evidence: Evidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureDriftReport {
    pub violations_count: usize,
    pub violations: Vec<ArchitectureViolation>,
    pub score: u32,
    pub drift_trend: String,
}

pub struct ArchitectureDriftEngine;

impl ArchitectureDriftEngine {
    pub fn analyze_drift(
        graph: &SoftwareKnowledgeGraph,
        _files: &[FileItem],
        rules: &[ArchitectureRule],
    ) -> ArchitectureDriftReport {
        let mut violations = Vec::new();

        for edge_ref in graph.graph.edge_references() {
            let src_node = &graph.graph[edge_ref.source()];
            let tgt_node = &graph.graph[edge_ref.target()];
            let edge_data = edge_ref.weight();

            if matches!(
                edge_data.edge_type,
                EdgeType::Calls | EdgeType::Imports | EdgeType::DependsOn
            ) {
                let src_path = src_node
                    .path
                    .as_deref()
                    .unwrap_or(&src_node.name)
                    .to_lowercase();
                let tgt_path = tgt_node
                    .path
                    .as_deref()
                    .unwrap_or(&tgt_node.name)
                    .to_lowercase();

                for rule in rules {
                    if rule.action == "deny" {
                        let from_matches = src_path.contains(&rule.from.to_lowercase());
                        let to_matches = tgt_path.contains(&rule.to.to_lowercase());

                        if from_matches && to_matches {
                            let ev = Evidence {
                                id: format!("ev_drift_{}_{}", src_node.id, tgt_node.id),
                                source_type: EvidenceSourceType::Architecture,
                                file_path: src_node.path.clone(),
                                line: None,
                                column: None,
                                symbol: Some(src_node.name.clone()),
                                relationship: Some(format!("{:?}", edge_data.edge_type)),
                                commit: None,
                                description: format!("Layer violation: '{}' component ({}) directly accesses '{}' ({})", rule.from, src_node.name, rule.to, tgt_node.name),
                                strength: EvidenceStrength::DirectlyObserved,
                                confidence: ConfidenceLevel::High,
                                timestamp: chrono::Utc::now().to_rfc3339(),
                            };

                            violations.push(ArchitectureViolation {
                                rule_name: format!("DENY {} -> {}", rule.from, rule.to),
                                source_component: src_node.name.clone(),
                                target_component: tgt_node.name.clone(),
                                violation_type: "Illegal Layer Dependency".to_string(),
                                evidence: ev,
                            });
                        }
                    }
                }
            }
        }

        let count = violations.len();
        let score = if count * 10 >= 100 {
            0
        } else {
            100 - (count as u32 * 10)
        };
        let trend = if count > 2 {
            "INCREASING_RISK"
        } else {
            "STABLE"
        };

        ArchitectureDriftReport {
            violations_count: count,
            violations,
            score,
            drift_trend: trend.to_string(),
        }
    }
}
