use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_impact::ImpactEngine;
use sentrix_ir::FileItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TestPriority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRecommendation {
    pub test_file: String,
    pub test_name: Option<String>,
    pub priority: TestPriority,
    pub reason: String,
    pub call_graph_connected: bool,
    pub co_change_connected: bool,
}

pub struct TestRecommendationEngine;

impl TestRecommendationEngine {
    pub fn recommend_tests(
        changed_components: &[String],
        files: &[FileItem],
        graph: &SoftwareKnowledgeGraph,
    ) -> Vec<TestRecommendation> {
        let mut recommendations = Vec::new();

        for target in changed_components {
            if let Ok(impact) =
                ImpactEngine::analyze_impact(graph, files, std::slice::from_ref(target), target)
            {
                for file_path in &impact.affected_files {
                    let path_low = file_path.to_lowercase();
                    let is_test = path_low.contains("test")
                        || path_low.contains("spec")
                        || path_low.starts_with("tests/");

                    if is_test {
                        let priority = if !impact.direct_dependents.is_empty() {
                            TestPriority::High
                        } else {
                            TestPriority::Medium
                        };

                        recommendations.push(TestRecommendation {
                            test_file: file_path.clone(),
                            test_name: None,
                            priority,
                            reason: format!("Test is connected to changed component '{}' through call/import graph", target),
                            call_graph_connected: true,
                            co_change_connected: false,
                        });
                    }
                }
            }
        }

        // Deduplicate recommendations by test_file
        let mut seen = std::collections::HashSet::new();
        let mut dedup = Vec::new();
        for r in recommendations {
            if seen.insert(r.test_file.clone()) {
                dedup.push(r);
            }
        }

        dedup
    }
}
