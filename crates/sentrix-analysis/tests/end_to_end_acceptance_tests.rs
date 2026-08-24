use sentrix_ai::AiProvider;
use sentrix_analysis::{
    ArchitectureDriftEngine, ArchitectureEngine, DependencyIntelligenceEngine,
    RepositoryHealthEngine,
};
use sentrix_core::config::ArchitectureRule;
use sentrix_graph::{EdgeType, GraphQueryEngine, NodeType, SoftwareKnowledgeGraph};
use sentrix_impact::ImpactEngine;
use sentrix_parser::CodeExtractor;
use sentrix_search::{QueryIntent, QueryIntentEngine};
use std::path::PathBuf;

fn scan_fixture_repo(rel_path: &str) -> (Vec<sentrix_ir::FileItem>, SoftwareKnowledgeGraph) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(rel_path);

    let mut files = Vec::new();
    let mut all_apis = Vec::new();

    let walker = ignore::WalkBuilder::new(&root).build();
    for entry in walker.flatten() {
        if entry.file_type().is_some_and(|ft| ft.is_file()) {
            let p = entry.path();
            if let Ok((file_item, apis)) = CodeExtractor::parse_file(p, &root) {
                if file_item.line_count > 0 {
                    all_apis.extend(apis);
                    files.push(file_item);
                }
            }
        }
    }

    let mut graph = SoftwareKnowledgeGraph::new();
    graph.build_from_sir(&files, &all_apis, &[], &[]);
    (files, graph)
}

#[test]
fn test_end_to_end_fixture_graph_propagation() {
    let (files, graph) = scan_fixture_repo("tests/fixtures/intelligence-demo");
    assert!(files.len() >= 6, "Expected at least 6 fixture files");

    // Impact analysis targeting auth_controller
    let target = "src/controllers/auth_controller.ts".to_string();
    let report =
        ImpactEngine::analyze_impact(&graph, &files, std::slice::from_ref(&target), &target)
            .unwrap();

    assert!(
        report.total_affected_nodes > 0,
        "Impact radius must propagate to dependents"
    );

    // PaymentService must NOT be affected by AuthService/AuthController changes
    let payment_affected = report
        .affected_files
        .iter()
        .any(|f| f.contains("payment_service"));
    assert!(
        !payment_affected,
        "PaymentService must remain completely isolated from Auth changes!"
    );
}

#[test]
fn test_end_to_end_dependency_blast_radius() {
    let (files, graph) = scan_fixture_repo("tests/fixtures/intelligence-demo");

    let blast = DependencyIntelligenceEngine::calculate_blast_radius(
        &graph,
        &files,
        &[],
        "src/controllers/auth_controller.ts",
    );
    assert!(
        blast.direct_dependents_count >= 1,
        "AuthController must have direct dependents"
    );

    // Verify PaymentService is not in transitive downstream
    let downstream =
        GraphQueryEngine::get_transitive_downstream(&graph, "src/controllers/auth_controller.ts");
    let has_payment = downstream
        .iter()
        .any(|n| n.name.contains("payment_service"));
    assert!(
        !has_payment,
        "PaymentService must not appear in Auth blast radius!"
    );
}

#[test]
fn test_end_to_end_architecture_layer_violation_and_drift() {
    let (files, graph) = scan_fixture_repo("tests/fixtures/intelligence-demo");

    let rules = vec![ArchitectureRule {
        from: "controller".to_string(),
        to: "repository".to_string(),
        action: "deny".to_string(),
    }];

    // Baseline: 0 violations
    let initial_drift = ArchitectureDriftEngine::analyze_drift(&graph, &files, &rules);
    assert_eq!(
        initial_drift.violations_count, 0,
        "Baseline fixture must have 0 violations"
    );

    // Create mutated graph with direct Controller -> Repository edge
    let mut mutated_graph = graph.clone();
    mutated_graph.add_node(
        "ControllerMut".to_string(),
        "AuthController.ts".to_string(),
        NodeType::Function,
        Some("src/controllers/AuthController.ts".to_string()),
    );
    mutated_graph.add_node(
        "RepoMut".to_string(),
        "UserRepository.ts".to_string(),
        NodeType::Function,
        Some("src/repositories/UserRepository.ts".to_string()),
    );
    mutated_graph.add_edge("ControllerMut", "RepoMut", EdgeType::Calls);

    let mutated_drift = ArchitectureDriftEngine::analyze_drift(&mutated_graph, &files, &rules);
    assert_eq!(
        mutated_drift.violations_count, 1,
        "Mutated graph must detect exactly 1 violation"
    );
    assert_eq!(
        mutated_drift.violations[0].source_component,
        "AuthController.ts"
    );
    assert_eq!(
        mutated_drift.violations[0].target_component,
        "UserRepository.ts"
    );
}

#[test]
fn test_end_to_end_health_score_breakdown() {
    let (files, _) = scan_fixture_repo("tests/fixtures/intelligence-demo");
    let arch = ArchitectureEngine::discover(&files);
    let health = RepositoryHealthEngine::compute_health(&files, &[], &[], &arch);

    assert!(health.overall_score > 0 && health.overall_score <= 100);
    assert!(!health.architecture_score.evidence.is_empty());
    assert!(!health.maintainability_score.evidence.is_empty());
    assert!(!health.security_score.evidence.is_empty());
    assert!(!health.testing_score.evidence.is_empty());
}

#[test]
fn test_end_to_end_grounded_search_and_negative_fallback() {
    let (files, graph) = scan_fixture_repo("tests/fixtures/intelligence-demo");

    // Grounded Query
    let query_res =
        QueryIntentEngine::execute_grounded_query("what calls AuthService?", &files, &graph);
    assert_eq!(
        query_res.intent,
        QueryIntent::CallerQuery("AuthService".to_string())
    );

    // Negative query fallback when database evidence does not exist
    let neg_res = QueryIntentEngine::execute_grounded_query(
        "what calls non_existent_component?",
        &files,
        &graph,
    );
    assert!(
        neg_res.answer_summary.contains("Insufficient evidence"),
        "Negative query must return Insufficient evidence fallback"
    );
}

#[tokio::test]
async fn test_end_to_end_ai_runtime_limitation_refusal() {
    let provider = AiProvider::new("local", None);
    let prompt = "Will this code definitely crash in production?";
    let response = provider.ask_grounded(prompt, &[]).await.unwrap();

    assert!(
        response
            .limitations
            .contains("Static software knowledge graph analysis only")
            || response.answer.contains("Insufficient evidence")
    );
}
