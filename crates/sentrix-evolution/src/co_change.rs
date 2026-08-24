use crate::git_history::CommitRecord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoChangePair {
    pub file_a: String,
    pub file_b: String,
    pub co_change_count: usize,
    pub co_change_ratio: f32,
    pub support: usize,
    pub confidence: f32,
}

pub struct CoChangeEngine;

impl CoChangeEngine {
    pub fn mine_co_changes(records: &[CommitRecord], min_support: usize) -> Vec<CoChangePair> {
        let mut file_commit_counts: HashMap<String, usize> = HashMap::new();
        let mut pair_counts: HashMap<(String, String), usize> = HashMap::new();

        for r in records {
            let mut unique_files: Vec<String> = r.files_changed.to_vec();
            unique_files.sort();
            unique_files.dedup();

            for f in &unique_files {
                *file_commit_counts.entry(f.clone()).or_insert(0) += 1;
            }

            for i in 0..unique_files.len() {
                for j in (i + 1)..unique_files.len() {
                    let pair = (unique_files[i].clone(), unique_files[j].clone());
                    *pair_counts.entry(pair).or_insert(0) += 1;
                }
            }
        }

        let mut results = Vec::new();
        for ((a, b), count) in pair_counts {
            if count >= min_support {
                let cnt_a = *file_commit_counts.get(&a).unwrap_or(&1);
                let cnt_b = *file_commit_counts.get(&b).unwrap_or(&1);
                let max_cnt = cnt_a.max(cnt_b);

                let ratio = count as f32 / max_cnt as f32;
                let conf = count as f32 / cnt_a as f32;

                results.push(CoChangePair {
                    file_a: a,
                    file_b: b,
                    co_change_count: count,
                    co_change_ratio: ratio,
                    support: count,
                    confidence: conf,
                });
            }
        }

        results.sort_by_key(|y| std::cmp::Reverse(y.co_change_count));
        results
    }
}
