pub mod arch_history;
pub mod co_change;
pub mod git_history;
pub mod ownership;
pub mod patterns;
pub mod predictive;
pub mod similar;
pub mod symbol_history;
pub mod test_recommendation;

pub use arch_history::{ArchitectureHistoryEngine, ArchitectureSnapshot};
pub use co_change::{CoChangeEngine, CoChangePair};
pub use git_history::{CommitRecord, EvolutionGitExtractor, EvolutionSummary};
pub use ownership::{ComponentOwnership, ContributorShare, OwnershipEngine};
pub use patterns::{PatternMiningEngine, PatternSequence};
pub use predictive::{FeatureContribution, PredictiveRiskEngine, PredictiveRiskReport, RiskLevel};
pub use similar::{SimilarChangeEngine, SimilarCommitResult};
pub use symbol_history::{SymbolHistoryEngine, SymbolHistoryReport};
pub use test_recommendation::{TestPriority, TestRecommendation, TestRecommendationEngine};
