use crate::git_history::CommitRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarCommitResult {
    pub commit_hash: String,
    pub author: String,
    pub message: String,
    pub similarity_score: f32,
    pub matching_files: Vec<String>,
    pub timestamp: String,
}

pub struct SimilarChangeEngine;

impl SimilarChangeEngine {
    pub fn find_similar_commits(
        diff_files: &[String],
        records: &[CommitRecord],
        top_n: usize,
    ) -> Vec<SimilarCommitResult> {
        if diff_files.is_empty() || records.is_empty() {
            return Vec::new();
        }

        let diff_set: std::collections::HashSet<String> =
            diff_files.iter().map(|f| f.to_lowercase()).collect();
        let mut results = Vec::new();

        for r in records {
            let mut matches = Vec::new();
            for f in &r.files_changed {
                if diff_set.contains(&f.to_lowercase()) {
                    matches.push(f.clone());
                }
            }

            if !matches.is_empty() {
                let sim = (matches.len() as f32 / diff_set.len().max(r.files_changed.len()) as f32)
                    .min(1.0);
                results.push(SimilarCommitResult {
                    commit_hash: r.hash.clone(),
                    author: r.author.clone(),
                    message: r.message.clone(),
                    similarity_score: sim,
                    matching_files: matches,
                    timestamp: r.timestamp.clone(),
                });
            }
        }

        results.sort_by(|a, b| {
            b.similarity_score
                .partial_cmp(&a.similarity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results.truncate(top_n);
        results
    }
}
