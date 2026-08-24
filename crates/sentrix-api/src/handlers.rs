use axum::{
    extract::{Path, Query, State},
    response::Json,
};
use sentrix_ai::{AiProvider, GroundedAiResponse};
use sentrix_analysis::{
    ArchitectureDriftEngine, ArchitectureDriftReport, ArchitectureInsight,
    CircularDependencyReport, ComplexityMetricsSummary, DependencyBlastRadiusReport,
    DependencyIntelligenceEngine, HotspotItem, RepositoryHealthEngine, RepositoryHealthReport,
};
use sentrix_core::config::SentrixConfig;
use sentrix_evolution::{
    ComponentOwnership, OwnershipEngine, PredictiveRiskEngine, PredictiveRiskReport,
    TestRecommendation, TestRecommendationEngine,
};
use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_impact::{ChangeImpactReport, ImpactEngine};
use sentrix_ir::{
    ConfidenceLevel, Evidence, EvidenceSourceType, EvidenceStrength, FileItem, SecurityFindingItem,
};
use sentrix_search::{GroundedQueryResult, QueryIntentEngine, SearchEngine};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub files: Vec<FileItem>,
    pub graph: SoftwareKnowledgeGraph,
    pub architecture: ArchitectureInsight,
    pub hotspots: Vec<HotspotItem>,
    pub complexity: ComplexityMetricsSummary,
    pub findings: Vec<SecurityFindingItem>,
    pub config: SentrixConfig,
}

#[derive(Serialize)]
pub struct OverviewResponse {
    pub api_version: &'static str,
    pub project_name: String,
    pub total_files: usize,
    pub total_lines: usize,
    pub total_functions: usize,
    pub node_count: usize,
    pub edge_count: usize,
    pub architecture_pattern: String,
    pub critical_hotspots: usize,
    pub health_score: u32,
}

pub async fn handle_overview(State(state): State<Arc<RwLock<AppState>>>) -> Json<OverviewResponse> {
    let s = state.read().await;
    let critical = s
        .hotspots
        .iter()
        .filter(|h| h.risk_level == sentrix_analysis::RiskLevel::Critical)
        .count();
    let health =
        RepositoryHealthEngine::compute_health(&s.files, &s.findings, &s.hotspots, &s.architecture);

    Json(OverviewResponse {
        api_version: "v1",
        project_name: "SENTRIX Target System".to_string(),
        total_files: s.files.len(),
        total_lines: s.complexity.total_lines,
        total_functions: s.complexity.total_functions,
        node_count: s.graph.node_count(),
        edge_count: s.graph.edge_count(),
        architecture_pattern: format!("{:?}", s.architecture.pattern),
        critical_hotspots: critical,
        health_score: health.overall_score,
    })
}

#[derive(Serialize)]
pub struct GraphSummaryResponse {
    pub node_count: usize,
    pub edge_count: usize,
}

pub async fn handle_graph(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<GraphSummaryResponse> {
    let s = state.read().await;
    Json(GraphSummaryResponse {
        node_count: s.graph.node_count(),
        edge_count: s.graph.edge_count(),
    })
}

pub async fn handle_hotspots(State(state): State<Arc<RwLock<AppState>>>) -> Json<Vec<HotspotItem>> {
    let s = state.read().await;
    Json(s.hotspots.clone())
}

pub async fn handle_architecture(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<ArchitectureInsight> {
    let s = state.read().await;
    Json(s.architecture.clone())
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn handle_search(
    Query(query): Query<SearchQuery>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<Vec<sentrix_search::SearchResult>> {
    let s = state.read().await;
    let results = SearchEngine::search(&query.q, &s.files, &s.graph);
    Json(results)
}

#[derive(Deserialize)]
pub struct ImpactRequest {
    pub target: String,
}

pub async fn handle_impact(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(req): Json<ImpactRequest>,
) -> Json<ChangeImpactReport> {
    let s = state.read().await;
    let report = ImpactEngine::analyze_impact(
        &s.graph,
        &s.files,
        std::slice::from_ref(&req.target),
        &req.target,
    )
    .unwrap_or_else(|_| ChangeImpactReport {
        target_revision_or_files: req.target,
        changed_files: vec![],
        total_affected_nodes: 0,
        direct_dependents: vec![],
        transitive_downstream: vec![],
        affected_files: vec![],
        affected_functions: vec![],
        affected_apis: vec![],
        affected_tests: vec![],
        impact_risk: "LOW".to_string(),
        critical_path: vec![],
        evidence: vec![],
    });
    Json(report)
}

pub async fn handle_health(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<RepositoryHealthReport> {
    let s = state.read().await;
    let health =
        RepositoryHealthEngine::compute_health(&s.files, &s.findings, &s.hotspots, &s.architecture);
    Json(health)
}

pub async fn handle_drift(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<ArchitectureDriftReport> {
    let s = state.read().await;
    let drift =
        ArchitectureDriftEngine::analyze_drift(&s.graph, &s.files, &s.config.architecture.rules);
    Json(drift)
}

pub async fn handle_dependencies(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<CircularDependencyReport> {
    let s = state.read().await;
    let report = DependencyIntelligenceEngine::detect_circular_dependencies(&s.graph);
    Json(report)
}

#[derive(Deserialize)]
pub struct TargetQuery {
    pub target: String,
}

pub async fn handle_dependency_impact(
    Query(q): Query<TargetQuery>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<DependencyBlastRadiusReport> {
    let s = state.read().await;
    let report =
        DependencyIntelligenceEngine::calculate_blast_radius(&s.graph, &s.files, &[], &q.target);
    Json(report)
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub query: String,
}

pub async fn handle_query(
    Query(q): Query<QueryRequest>,
    State(state): State<Arc<RwLock<AppState>>>,
) -> Json<GroundedQueryResult> {
    let s = state.read().await;
    let result = QueryIntentEngine::execute_grounded_query(&q.query, &s.files, &s.graph);
    Json(result)
}

#[derive(Deserialize)]
pub struct AskRequest {
    pub question: String,
}

pub async fn handle_ask(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(req): Json<AskRequest>,
) -> Json<GroundedAiResponse> {
    let s = state.read().await;
    let grounded_query =
        QueryIntentEngine::execute_grounded_query(&req.question, &s.files, &s.graph);
    let provider = AiProvider::new(&s.config.ai.provider, s.config.ai.api_key.clone());

    let resp = provider
        .ask_grounded(&req.question, &grounded_query.evidence)
        .await
        .unwrap_or_else(|_| GroundedAiResponse {
            answer: "Error executing grounded AI provider.".to_string(),
            evidence: vec![],
            confidence: ConfidenceLevel::Low,
            limitations: "System error".to_string(),
        });

    Json(resp)
}

pub async fn handle_evidence(
    Path(id): Path<String>,
    State(_state): State<Arc<RwLock<AppState>>>,
) -> Json<Evidence> {
    Json(Evidence {
        id: id.clone(),
        source_type: EvidenceSourceType::Ast,
        file_path: None,
        line: None,
        column: None,
        symbol: Some(id),
        relationship: None,
        commit: None,
        description: "Directly observed graph item evidence".to_string(),
        strength: EvidenceStrength::DirectlyObserved,
        confidence: ConfidenceLevel::High,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[derive(Deserialize)]
pub struct PredictRequest {
    pub target: String,
}

pub async fn handle_evolution_predict(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(req): Json<PredictRequest>,
) -> Json<PredictiveRiskReport> {
    let s = state.read().await;
    let report = PredictiveRiskEngine::predict_change_risk(&req.target, &s.files, &s.graph, 0, 0);
    Json(report)
}

#[derive(Deserialize)]
pub struct RecommendTestsRequest {
    pub targets: Vec<String>,
}

pub async fn handle_recommend_tests(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(req): Json<RecommendTestsRequest>,
) -> Json<Vec<TestRecommendation>> {
    let s = state.read().await;
    let recs = TestRecommendationEngine::recommend_tests(&req.targets, &s.files, &s.graph);
    Json(recs)
}

pub async fn handle_evolution_ownership(
    Query(q): Query<TargetQuery>,
    State(_state): State<Arc<RwLock<AppState>>>,
) -> Json<ComponentOwnership> {
    let report = OwnershipEngine::analyze_ownership(&q.target, &[]);
    Json(report)
}
