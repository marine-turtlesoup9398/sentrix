use sentrix_graph::{GraphQueryEngine, SoftwareKnowledgeGraph};
use sentrix_ir::{
    ApiEndpointItem, ConfidenceLevel, Evidence, EvidenceSourceType, EvidenceStrength, FileItem,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyBlastRadiusReport {
    pub target_package_or_module: String,
    pub direct_dependents_count: usize,
    pub transitive_dependents_count: usize,
    pub affected_apis_count: usize,
    pub blast_radius_level: String, // CRITICAL, HIGH, MEDIUM, LOW
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircularDependencyReport {
    pub total_cycles_found: usize,
    pub cycles: Vec<Vec<String>>,
    pub evidence: Vec<Evidence>,
}

pub struct DependencyIntelligenceEngine;

impl DependencyIntelligenceEngine {
    pub fn calculate_blast_radius(
        graph: &SoftwareKnowledgeGraph,
        _files: &[FileItem],
        _apis: &[ApiEndpointItem],
        target_name: &str,
    ) -> DependencyBlastRadiusReport {
        let direct = GraphQueryEngine::get_direct_dependents(graph, target_name);
        let transitive = GraphQueryEngine::get_transitive_downstream(graph, target_name);

        let direct_count = direct.len();
        let transitive_count = transitive.len();
        let affected_apis = transitive
            .iter()
            .filter(|n| n.node_type == sentrix_graph::NodeType::ApiEndpoint)
            .count();

        let level = if transitive_count > 20 || affected_apis > 3 {
            "CRITICAL"
        } else if transitive_count > 8 {
            "HIGH"
        } else if transitive_count > 2 {
            "MEDIUM"
        } else {
            "LOW"
        };

        let mut evidence = Vec::new();
        evidence.push(Evidence {
            id: format!("ev_blast_{}", target_name),
            source_type: EvidenceSourceType::Dependency,
            file_path: None,
            line: None,
            column: None,
            symbol: Some(target_name.to_string()),
            relationship: Some("DEPENDS_ON".to_string()),
            commit: None,
            description: format!("Dependency '{}' propagates blast radius across {} direct and {} transitive dependents", target_name, direct_count, transitive_count),
            strength: EvidenceStrength::DirectlyObserved,
            confidence: ConfidenceLevel::High,
            timestamp: chrono::Utc::now().to_rfc3339(),
        });

        DependencyBlastRadiusReport {
            target_package_or_module: target_name.to_string(),
            direct_dependents_count: direct_count,
            transitive_dependents_count: transitive_count,
            affected_apis_count: affected_apis,
            blast_radius_level: level.to_string(),
            evidence,
        }
    }

    pub fn detect_circular_dependencies(
        graph: &SoftwareKnowledgeGraph,
    ) -> CircularDependencyReport {
        let cycles = GraphQueryEngine::find_cycles(graph);
        let total = cycles.len();

        let mut evidence = Vec::new();
        for (idx, cycle) in cycles.iter().take(5).enumerate() {
            evidence.push(Evidence {
                id: format!("ev_cycle_{}", idx + 1),
                source_type: EvidenceSourceType::Dependency,
                file_path: None,
                line: None,
                column: None,
                symbol: None,
                relationship: Some("CIRCULAR_DEPENDENCY".to_string()),
                commit: None,
                description: format!(
                    "Circular dependency cycle #{}: {}",
                    idx + 1,
                    cycle.join(" -> ")
                ),
                strength: EvidenceStrength::DirectlyObserved,
                confidence: ConfidenceLevel::High,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
        }

        CircularDependencyReport {
            total_cycles_found: total,
            cycles,
            evidence,
        }
    }
}
