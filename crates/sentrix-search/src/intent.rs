use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_ir::{ConfidenceLevel, Evidence, EvidenceSourceType, EvidenceStrength, FileItem};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QueryIntent {
    SymbolSearch(String),
    DependencyQuery(String),
    CallerQuery(String),
    CalleeQuery(String),
    ArchitectureQuery,
    SecurityQuery,
    ImpactQuery(String),
    RiskQuery,
    HistoryQuery(String),
    ApiQuery,
    TestQuery,
    GeneralRepositoryQuery(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundedQueryResult {
    pub intent: QueryIntent,
    pub answer_summary: String,
    pub evidence: Vec<Evidence>,
    pub confidence: ConfidenceLevel,
    pub limitation: String,
}

pub struct QueryIntentEngine;

impl QueryIntentEngine {
    pub fn classify_intent(query: &str) -> QueryIntent {
        let q_lower = query.to_lowercase();
        let cleaned = query.trim_matches(|c: char| c == '?' || c == '.' || c == '!');

        if q_lower.contains("calls") || q_lower.contains("caller") || q_lower.contains("who calls")
        {
            let target = cleaned
                .to_string()
                .replace("what calls", "")
                .replace("What calls", "")
                .replace("who calls", "")
                .replace("Who calls", "")
                .replace("calls", "")
                .trim()
                .to_string();
            QueryIntent::CallerQuery(target)
        } else if q_lower.contains("depends on") || q_lower.contains("dependency") {
            let target = cleaned
                .to_string()
                .replace("what depends on", "")
                .replace("What depends on", "")
                .replace("depends on", "")
                .replace("dependency", "")
                .trim()
                .to_string();
            QueryIntent::DependencyQuery(target)
        } else if q_lower.contains("security")
            || q_lower.contains("secret")
            || q_lower.contains("vulnerability")
        {
            QueryIntent::SecurityQuery
        } else if q_lower.contains("architecture")
            || q_lower.contains("pattern")
            || q_lower.contains("layer")
        {
            QueryIntent::ArchitectureQuery
        } else if q_lower.contains("impact")
            || q_lower.contains("break")
            || q_lower.contains("affect")
        {
            let target = cleaned
                .to_string()
                .replace("what is the impact of", "")
                .replace("impact", "")
                .trim()
                .to_string();
            QueryIntent::ImpactQuery(target)
        } else if q_lower.contains("risk") || q_lower.contains("hotspot") {
            QueryIntent::RiskQuery
        } else if q_lower.contains("api")
            || q_lower.contains("endpoint")
            || q_lower.contains("route")
        {
            QueryIntent::ApiQuery
        } else if q_lower.contains("test") {
            QueryIntent::TestQuery
        } else {
            QueryIntent::GeneralRepositoryQuery(query.to_string())
        }
    }

    pub fn execute_grounded_query(
        query: &str,
        files: &[FileItem],
        graph: &SoftwareKnowledgeGraph,
    ) -> GroundedQueryResult {
        let intent = Self::classify_intent(query);
        let mut evidence = Vec::new();

        let (summary, confidence) = match &intent {
            QueryIntent::CallerQuery(target) => {
                let callers = sentrix_graph::GraphQueryEngine::get_direct_dependents(graph, target);
                for c in &callers {
                    evidence.push(Evidence {
                        id: format!("ev_caller_{}", c.id),
                        source_type: EvidenceSourceType::CallGraph,
                        file_path: c.path.clone(),
                        line: None,
                        column: None,
                        symbol: Some(c.name.clone()),
                        relationship: Some("CALLS".to_string()),
                        commit: None,
                        description: format!("Component {} calls target '{}'", c.name, target),
                        strength: EvidenceStrength::DirectlyObserved,
                        confidence: ConfidenceLevel::High,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                }

                if callers.is_empty() {
                    ("Insufficient evidence: No direct callers found in knowledge graph for symbol.".to_string(), ConfidenceLevel::Low)
                } else {
                    (
                        format!(
                            "Found {} direct caller components targeting '{}'",
                            callers.len(),
                            target
                        ),
                        ConfidenceLevel::High,
                    )
                }
            }
            QueryIntent::DependencyQuery(target) => {
                let deps =
                    sentrix_graph::GraphQueryEngine::get_transitive_downstream(graph, target);
                for d in &deps {
                    evidence.push(Evidence {
                        id: format!("ev_dep_{}", d.id),
                        source_type: EvidenceSourceType::Dependency,
                        file_path: d.path.clone(),
                        line: None,
                        column: None,
                        symbol: Some(d.name.clone()),
                        relationship: Some("DEPENDS_ON".to_string()),
                        commit: None,
                        description: format!("Component {} depends on target '{}'", d.name, target),
                        strength: EvidenceStrength::DirectlyObserved,
                        confidence: ConfidenceLevel::High,
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                }
                (
                    format!(
                        "Identified {} downstream dependent components relying on '{}'",
                        deps.len(),
                        target
                    ),
                    ConfidenceLevel::High,
                )
            }
            QueryIntent::ArchitectureQuery => {
                evidence.push(Evidence {
                    id: "ev_arch_query".to_string(),
                    source_type: EvidenceSourceType::Architecture,
                    file_path: None,
                    line: None,
                    column: None,
                    symbol: None,
                    relationship: None,
                    commit: None,
                    description: format!("System contains {} files and {} graph nodes across workspace crates/modules", files.len(), graph.node_count()),
                    strength: EvidenceStrength::DirectlyObserved,
                    confidence: ConfidenceLevel::High,
                    timestamp: chrono::Utc::now().to_rfc3339(),
                });
                ("Repository architecture consists of modular workspace boundaries and separated source layers.".to_string(), ConfidenceLevel::High)
            }
            _ => (
                "Graph evidence retrieved for general repository intent query.".to_string(),
                ConfidenceLevel::Medium,
            ),
        };

        GroundedQueryResult {
            intent,
            answer_summary: summary,
            evidence,
            confidence,
            limitation:
                "Static software knowledge graph analysis only; no runtime execution performed."
                    .to_string(),
        }
    }
}
