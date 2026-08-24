pub mod architecture;
pub mod benchmark;
pub mod complexity;
pub mod dependency_intel;
pub mod drift;
pub mod health;
pub mod hotspots;

pub use architecture::{ArchitectureEngine, ArchitectureInsight, ArchitecturePattern};
pub use benchmark::{BenchmarkEngine, BenchmarkReport};
pub use complexity::{ComplexityEngine, ComplexityMetricsSummary};
pub use dependency_intel::{
    CircularDependencyReport, DependencyBlastRadiusReport, DependencyIntelligenceEngine,
};
pub use drift::{ArchitectureDriftEngine, ArchitectureDriftReport, ArchitectureViolation};
pub use health::{CategoryHealthScore, RepositoryHealthEngine, RepositoryHealthReport};
pub use hotspots::{HotspotEngine, HotspotItem, RiskLevel};
