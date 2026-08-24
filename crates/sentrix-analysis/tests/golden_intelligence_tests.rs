use sentrix_analysis::{ArchitectureDriftEngine, DependencyIntelligenceEngine};
use sentrix_core::config::ArchitectureRule;
use sentrix_graph::{EdgeType, NodeType, SoftwareKnowledgeGraph};

#[test]
fn test_tarjan_cycle_detection() {
    let mut graph = SoftwareKnowledgeGraph::new();

    // Add cycle A -> B -> C -> A
    graph.add_node(
        "A".to_string(),
        "ModuleA".to_string(),
        NodeType::Module,
        None,
    );
    graph.add_node(
        "B".to_string(),
        "ModuleB".to_string(),
        NodeType::Module,
        None,
    );
    graph.add_node(
        "C".to_string(),
        "ModuleC".to_string(),
        NodeType::Module,
        None,
    );

    graph.add_edge("A", "B", EdgeType::DependsOn);
    graph.add_edge("B", "C", EdgeType::DependsOn);
    graph.add_edge("C", "A", EdgeType::DependsOn);

    let report = DependencyIntelligenceEngine::detect_circular_dependencies(&graph);
    assert_eq!(report.total_cycles_found, 1);
    assert_eq!(report.cycles[0].len(), 4);
    assert_eq!(report.cycles[0][0], "A");
    assert_eq!(report.cycles[0][3], "A");
}

#[test]
fn test_architecture_drift_layer_violation() {
    let mut graph = SoftwareKnowledgeGraph::new();

    graph.add_node(
        "UserController".to_string(),
        "UserController.ts".to_string(),
        NodeType::Function,
        Some("src/controllers/UserController.ts".to_string()),
    );
    graph.add_node(
        "UserRepository".to_string(),
        "UserRepository.ts".to_string(),
        NodeType::Function,
        Some("src/repositories/UserRepository.ts".to_string()),
    );

    graph.add_edge("UserController", "UserRepository", EdgeType::Calls);

    let rules = vec![ArchitectureRule {
        from: "controller".to_string(),
        to: "repository".to_string(),
        action: "deny".to_string(),
    }];

    let drift = ArchitectureDriftEngine::analyze_drift(&graph, &[], &rules);
    assert_eq!(drift.violations_count, 1);
    assert_eq!(drift.violations[0].source_component, "UserController.ts");
    assert_eq!(drift.violations[0].target_component, "UserRepository.ts");
    assert_eq!(drift.score, 90);
}
