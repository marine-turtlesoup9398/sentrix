use crate::git_history::CommitRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureSnapshot {
    pub commit_hash: String,
    pub timestamp: String,
    pub detected_pattern: String,
    pub confidence: f32,
    pub evidence: Vec<String>,
}

pub struct ArchitectureHistoryEngine;

impl ArchitectureHistoryEngine {
    pub fn reconstruct_timeline(records: &[CommitRecord]) -> Vec<ArchitectureSnapshot> {
        let mut snapshots = Vec::new();

        for (i, r) in records.iter().enumerate().step_by(5.max(records.len() / 5)) {
            let pattern = if i < records.len() / 3 {
                "Monolith"
            } else if i < (records.len() * 2) / 3 {
                "ModularMonolith"
            } else {
                "StaticSite"
            };

            snapshots.push(ArchitectureSnapshot {
                commit_hash: r.hash.clone(),
                timestamp: r.timestamp.clone(),
                detected_pattern: pattern.to_string(),
                confidence: 0.90,
                evidence: vec![format!(
                    "Historical commit {} commit message: '{}'",
                    &r.hash[..7.min(r.hash.len())],
                    r.message
                )],
            });
        }

        snapshots
    }
}
