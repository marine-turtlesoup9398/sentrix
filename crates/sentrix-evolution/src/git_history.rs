use sentrix_git::GitIntelligence;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitRecord {
    pub hash: String,
    pub author: String,
    pub timestamp: String,
    pub message: String,
    pub files_changed: Vec<String>,
    pub is_bugfix: bool,
    pub is_revert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionSummary {
    pub total_commits: usize,
    pub total_authors: usize,
    pub files_tracked: usize,
    pub bugfix_commits_count: usize,
    pub revert_commits_count: usize,
    pub top_changed_files: Vec<(String, usize)>,
}

pub struct EvolutionGitExtractor;

impl EvolutionGitExtractor {
    pub fn extract_commit_records(repo_path: &str, limit: usize) -> Vec<CommitRecord> {
        let git = GitIntelligence::new(repo_path);
        let raw_commits = git.get_commit_history(limit).unwrap_or_default();
        let bugfix_re =
            regex::Regex::new(r"(?i)\b(fix|bug|hotfix|patch|issue|regression)\b").unwrap();
        let revert_re = regex::Regex::new(r"(?i)\b(revert|undo)\b").unwrap();

        let mut records = Vec::new();
        for c in raw_commits {
            let msg = c.message.trim().to_string();
            let is_bugfix = bugfix_re.is_match(&msg);
            let is_revert = revert_re.is_match(&msg);

            records.push(CommitRecord {
                hash: c.hash,
                author: c.author,
                timestamp: c.timestamp,
                message: msg,
                files_changed: c.files_changed,
                is_bugfix,
                is_revert,
            });
        }

        records
    }

    pub fn summarize(records: &[CommitRecord]) -> EvolutionSummary {
        let mut authors = HashSet::new();
        let mut files_map = HashMap::new();
        let mut bugfix_cnt = 0;
        let mut revert_cnt = 0;

        for r in records {
            authors.insert(r.author.clone());
            if r.is_bugfix {
                bugfix_cnt += 1;
            }
            if r.is_revert {
                revert_cnt += 1;
            }

            for f in &r.files_changed {
                *files_map.entry(f.clone()).or_insert(0) += 1;
            }
        }

        let mut top_files: Vec<(String, usize)> = files_map.into_iter().collect();
        top_files.sort_by_key(|b| std::cmp::Reverse(b.1));
        top_files.truncate(10);

        EvolutionSummary {
            total_commits: records.len(),
            total_authors: authors.len(),
            files_tracked: top_files.len(),
            bugfix_commits_count: bugfix_cnt,
            revert_commits_count: revert_cnt,
            top_changed_files: top_files,
        }
    }
}
