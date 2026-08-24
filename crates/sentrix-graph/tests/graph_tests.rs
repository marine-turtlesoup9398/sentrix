use sentrix_graph::{EdgeType, NodeType, SoftwareKnowledgeGraph};

#[test]
fn test_graph_node_and_edge_addition() {
    let mut graph = SoftwareKnowledgeGraph::new();

    let _f1 = graph.add_node(
        "file:main.rs".to_string(),
        "main.rs".to_string(),
        NodeType::File,
        Some("src/main.rs".to_string()),
    );
    let _f2 = graph.add_node(
        "func:parse".to_string(),
        "parse()".to_string(),
        NodeType::Function,
        Some("src/main.rs".to_string()),
    );

    graph.add_edge("file:main.rs", "func:parse", EdgeType::Contains);

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn test_impact_radius() {
    let mut graph = SoftwareKnowledgeGraph::new();
    graph.add_node(
        "file:src/auth.rs".to_string(),
        "src/auth.rs".to_string(),
        NodeType::File,
        Some("src/auth.rs".to_string()),
    );
    graph.add_node(
        "func:login".to_string(),
        "login()".to_string(),
        NodeType::Function,
        Some("src/auth.rs".to_string()),
    );
    graph.add_edge("file:src/auth.rs", "func:login", EdgeType::Contains);

    let impact = graph.get_impact_radius(&["src/auth.rs".to_string()]);
    assert_eq!(impact.len(), 2);
}
