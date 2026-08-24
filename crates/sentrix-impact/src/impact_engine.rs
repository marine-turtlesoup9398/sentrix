use sentrix_core::Result;
use sentrix_graph::{NodeType, SoftwareKnowledgeGraph};
use sentrix_ir::FileItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeImpactReport {
    pub target_revision_or_files: String,
    pub changed_files: Vec<String>,
    pub total_affected_nodes: usize,
    pub direct_dependents: Vec<String>,
    pub transitive_downstream: Vec<String>,
    pub affected_files: Vec<String>,
    pub affected_functions: Vec<String>,
    pub affected_apis: Vec<String>,
    pub affected_tests: Vec<String>,
    pub impact_risk: String, // CRITICAL, HIGH, MEDIUM, LOW
    pub critical_path: Vec<String>,
    pub evidence: Vec<String>,
}

pub struct ImpactEngine;

impl ImpactEngine {
    pub fn analyze_impact(
        graph: &SoftwareKnowledgeGraph,
        _files: &[FileItem],
        changed_files: &[String],
        target_name: &str,
    ) -> Result<ChangeImpactReport> {
        let affected_nodes = graph.get_impact_radius(changed_files);

        let mut direct_dependents = Vec::new();
        let mut transitive_downstream = Vec::new();
        let mut affected_files = Vec::new();
        let mut affected_functions = Vec::new();
        let mut affected_apis = Vec::new();
        let mut affected_tests = Vec::new();
        let mut critical_path = Vec::new();

        for (idx, node) in affected_nodes.iter().enumerate() {
            if idx <= 3 {
                direct_dependents.push(node.name.clone());
            } else {
                transitive_downstream.push(node.name.clone());
            }

            match node.node_type {
                NodeType::File => affected_files.push(node.name.clone()),
                NodeType::Function => {
                    if node.name.to_lowercase().contains("test") {
                        affected_tests.push(node.name.clone());
                    } else {
                        affected_functions.push(node.name.clone());
                    }
                }
                NodeType::ApiEndpoint => affected_apis.push(node.name.clone()),
                _ => {}
            }
        }

        // Build Critical Propagation Path
        for ch in changed_files {
            critical_path.push(format!("Changed File [{}]", ch));
            if !affected_functions.is_empty() {
                critical_path.push(format!("Direct Call Dependent [{}]", affected_functions[0]));
            }
            if !affected_apis.is_empty() {
                critical_path.push(format!("Exposed API Route [{}]", affected_apis[0]));
            }
            if !affected_tests.is_empty() {
                critical_path.push(format!("Impacted Test [{}]", affected_tests[0]));
            }
        }

        let total_count = affected_nodes.len();
        let risk = if total_count > 15 || !affected_apis.is_empty() {
            "HIGH"
        } else if total_count > 5 {
            "MEDIUM"
        } else {
            "LOW"
        };

        let mut evidence = Vec::new();
        evidence.push(format!(
            "Calculated impact radius for {} target item(s)",
            changed_files.len()
        ));
        evidence.push(format!(
            "Downstream ripple effects propagate across {} graph nodes",
            total_count
        ));
        if !affected_apis.is_empty() {
            evidence.push(format!(
                "Directly impacts {} external API endpoints",
                affected_apis.len()
            ));
        }
        if !affected_tests.is_empty() {
            evidence.push(format!(
                "Propagates into {} unit/integration tests",
                affected_tests.len()
            ));
        }

        Ok(ChangeImpactReport {
            target_revision_or_files: target_name.to_string(),
            changed_files: changed_files.to_vec(),
            total_affected_nodes: total_count,
            direct_dependents,
            transitive_downstream,
            affected_files,
            affected_functions,
            affected_apis,
            affected_tests,
            impact_risk: risk.to_string(),
            critical_path,
            evidence,
        })
    }
}
