use axum::{
    routing::{get, post},
    Router,
};
use sentrix_core::Result;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use crate::handlers::{
    handle_architecture, handle_ask, handle_dependencies, handle_dependency_impact, handle_drift,
    handle_evidence, handle_evolution_ownership, handle_evolution_predict, handle_graph,
    handle_health, handle_hotspots, handle_impact, handle_overview, handle_query,
    handle_recommend_tests, handle_search, AppState,
};

pub struct ApiServer {
    state: Arc<RwLock<AppState>>,
    host: String,
    port: u16,
    web_dir: Option<PathBuf>,
}

impl ApiServer {
    pub fn new(state: AppState, host: &str, port: u16, web_dir: Option<PathBuf>) -> Self {
        Self {
            state: Arc::new(RwLock::new(state)),
            host: host.to_string(),
            port,
            web_dir,
        }
    }

    pub async fn run(self) -> Result<()> {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        let mut app = Router::new()
            .route("/api/overview", get(handle_overview))
            .route("/api/graph", get(handle_graph))
            .route("/api/hotspots", get(handle_hotspots))
            .route("/api/architecture", get(handle_architecture))
            .route("/api/search", get(handle_search))
            .route("/api/impact", post(handle_impact))
            .route("/api/intelligence/health", get(handle_health))
            .route("/api/intelligence/drift", get(handle_drift))
            .route("/api/intelligence/dependencies", get(handle_dependencies))
            .route(
                "/api/intelligence/dependency-impact",
                get(handle_dependency_impact),
            )
            .route("/api/intelligence/query", get(handle_query))
            .route("/api/intelligence/ask", post(handle_ask))
            .route("/api/evidence/{id}", get(handle_evidence))
            .route("/api/evolution/predict", post(handle_evolution_predict))
            .route(
                "/api/evolution/recommend-tests",
                post(handle_recommend_tests),
            )
            .route("/api/evolution/ownership", get(handle_evolution_ownership))
            .layer(cors)
            .with_state(self.state);

        if let Some(ref static_path) = self.web_dir {
            if static_path.exists() {
                app = app.fallback_service(ServeDir::new(static_path));
            }
        }

        let addr: SocketAddr = format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|e| sentrix_core::SentrixError::Api(format!("Invalid address: {}", e)))?;

        println!(
            "SENTRIX Unified Intelligence API Server running at http://{}",
            addr
        );

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| sentrix_core::SentrixError::Api(e.to_string()))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| sentrix_core::SentrixError::Api(e.to_string()))?;

        Ok(())
    }
}
