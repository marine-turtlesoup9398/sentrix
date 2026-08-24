use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_ir::ConfidenceLevel;
use sentrix_search::{QueryIntent, QueryIntentEngine};

#[test]
fn test_query_intent_classification() {
    let intent1 = QueryIntentEngine::classify_intent("what calls AuthService.login?");
    assert_eq!(
        intent1,
        QueryIntent::CallerQuery("AuthService.login".to_string())
    );

    let intent2 = QueryIntentEngine::classify_intent("what depends on payment?");
    assert_eq!(intent2, QueryIntent::DependencyQuery("payment".to_string()));

    let intent3 = QueryIntentEngine::classify_intent("show security findings");
    assert_eq!(intent3, QueryIntent::SecurityQuery);
}

#[test]
fn test_insufficient_evidence_fallback() {
    let graph = SoftwareKnowledgeGraph::new();
    let res =
        QueryIntentEngine::execute_grounded_query("what calls non_existent_symbol()", &[], &graph);

    assert_eq!(res.confidence, ConfidenceLevel::Low);
    assert!(res.answer_summary.contains("Insufficient evidence"));
}
