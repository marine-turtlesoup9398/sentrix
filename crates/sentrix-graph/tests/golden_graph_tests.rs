use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_ir::{FileItem, FunctionItem, Language, SourceLocation, SymbolId};

#[test]
fn test_golden_relationship_propagation() {
    let mut graph = SoftwareKnowledgeGraph::new();

    let file_item = FileItem {
        path: "src/auth.rs".into(),
        relative_path: "src/auth.rs".into(),
        language: Language::Rust,
        hash: "abc".into(),
        size_bytes: 100,
        line_count: 10,
        imports: vec!["std::sync".into()],
        functions: vec![FunctionItem {
            id: SymbolId::new("src/auth.rs::login"),
            name: "login".into(),
            language: Language::Rust,
            location: SourceLocation {
                file_path: "src/auth.rs".into(),
                start_line: 1,
                start_col: 1,
                end_line: 5,
                end_col: 10,
            },
            parameters: vec![],
            return_type: None,
            calls: vec!["verify_password".into()],
            cyclomatic_complexity: 2,
            cognitive_complexity: 1,
            is_api_handler: true,
            is_exported: true,
            security_sensitive: true,
            doc_comment: None,
        }],
        classes: vec![],
        cyclomatic_complexity: 2,
    };

    graph.build_from_sir(&[file_item], &[], &[], &[]);

    assert!(graph.node_map.contains_key("file:src/auth.rs"));
    assert!(graph.node_map.contains_key("func:src/auth.rs::login"));
    assert!(graph.node_map.contains_key("import:std::sync"));
    assert!(graph.node_map.contains_key("func:verify_password"));
}
