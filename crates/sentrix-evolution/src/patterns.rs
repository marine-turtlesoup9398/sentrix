use crate::git_history::CommitRecord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternSequence {
    pub sequence: Vec<String>,
    pub frequency: usize,
    pub confidence: f32,
}

pub struct PatternMiningEngine;

impl PatternMiningEngine {
    pub fn mine_patterns(
        records: &[CommitRecord],
        min_observations: usize,
    ) -> Vec<PatternSequence> {
        if records.len() < 2 {
            return Vec::new();
        }

        let mut seq_counts: HashMap<Vec<String>, usize> = HashMap::new();
        let mut window: Vec<String> = Vec::new();

        for r in records {
            if let Some(f) = r.files_changed.first() {
                window.push(f.clone());
                if window.len() > 3 {
                    window.remove(0);
                }

                if window.len() >= 2 {
                    *seq_counts.entry(window.clone()).or_insert(0) += 1;
                }
            }
        }

        let total_commits = records.len() as f32;
        let mut results = Vec::new();

        for (seq, freq) in seq_counts {
            if freq >= min_observations {
                let conf = (freq as f32 / total_commits).min(1.0);
                results.push(PatternSequence {
                    sequence: seq,
                    frequency: freq,
                    confidence: conf,
                });
            }
        }

        results.sort_by_key(|b| std::cmp::Reverse(b.frequency));
        results
    }
}
