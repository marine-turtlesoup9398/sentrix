use git2::{DiffOptions, Repository};
use sentrix_core::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileGitMetrics {
    pub relative_path: String,
    pub commit_count: u32,
    pub bugfix_count: u32,
    pub additions: u32,
    pub deletions: u32,
    pub churn_score: u32,
    pub unique_authors: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitDiffSummary {
    pub from_ref: String,
    pub to_ref: String,
    pub changed_files: Vec<String>,
    pub total_additions: u32,
    pub total_deletions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCommitInfo {
    pub hash: String,
    pub author: String,
    pub timestamp: String,
    pub message: String,
    pub files_changed: Vec<String>,
}

pub struct GitIntelligence {
    repo_path: std::path::PathBuf,
}

impl GitIntelligence {
    pub fn new<P: AsRef<Path>>(repo_path: P) -> Self {
        Self {
            repo_path: repo_path.as_ref().to_path_buf(),
        }
    }

    pub fn get_commit_history(&self, limit: usize) -> Result<Vec<RawCommitInfo>> {
        let repo = match Repository::open(&self.repo_path) {
            Ok(r) => r,
            Err(_) => return Ok(Vec::new()),
        };

        let mut revwalk = match repo.revwalk() {
            Ok(rw) => rw,
            Err(_) => return Ok(Vec::new()),
        };
        revwalk.push_head().ok();

        let mut commits = Vec::new();

        for oid in revwalk.take(limit).flatten() {
            if let Ok(commit) = repo.find_commit(oid) {
                let hash = commit.id().to_string();
                let author = commit.author().name().unwrap_or("Unknown").to_string();
                let message = commit.message().unwrap_or("").to_string();
                let time = commit.time();
                let dt = chrono::DateTime::from_timestamp(time.seconds(), 0)
                    .unwrap_or_else(chrono::Utc::now)
                    .to_rfc3339();

                let mut files = Vec::new();
                if let Ok(parent) = commit.parent(0) {
                    if let (Ok(tree), Ok(parent_tree)) = (commit.tree(), parent.tree()) {
                        let mut opts = DiffOptions::new();
                        if let Ok(diff) =
                            repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut opts))
                        {
                            diff.foreach(
                                &mut |delta, _| {
                                    if let Some(p) = delta.new_file().path() {
                                        files.push(p.to_string_lossy().to_string());
                                    }
                                    true
                                },
                                None,
                                None,
                                None,
                            )
                            .ok();
                        }
                    }
                }

                commits.push(RawCommitInfo {
                    hash,
                    author,
                    timestamp: dt,
                    message,
                    files_changed: files,
                });
            }
        }

        Ok(commits)
    }

    pub fn analyze_history(&self, depth: usize) -> Result<HashMap<String, FileGitMetrics>> {
        let repo = match Repository::open(&self.repo_path) {
            Ok(r) => r,
            Err(_) => return Ok(HashMap::new()),
        };

        let mut revwalk = match repo.revwalk() {
            Ok(rw) => rw,
            Err(_) => return Ok(HashMap::new()),
        };
        revwalk.push_head().ok();

        let mut metrics_map: HashMap<String, FileGitMetrics> = HashMap::new();
        let mut author_map: HashMap<String, std::collections::HashSet<String>> = HashMap::new();

        for oid in revwalk.take(depth).flatten() {
            if let Ok(commit) = repo.find_commit(oid) {
                let author_name = commit.author().name().unwrap_or("Unknown").to_string();
                let message = commit.message().unwrap_or("").to_lowercase();
                let is_bugfix = message.contains("fix")
                    || message.contains("bug")
                    || message.contains("patch")
                    || message.contains("resolve");

                if let Ok(parent) = commit.parent(0) {
                    if let (Ok(tree), Ok(parent_tree)) = (commit.tree(), parent.tree()) {
                        let mut opts = DiffOptions::new();
                        if let Ok(diff) =
                            repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut opts))
                        {
                            diff.foreach(
                                &mut |delta, _| {
                                    if let Some(new_file) = delta.new_file().path() {
                                        let path_str = new_file.to_string_lossy().to_string();
                                        let entry = metrics_map.entry(path_str.clone()).or_insert(
                                            FileGitMetrics {
                                                relative_path: path_str.clone(),
                                                commit_count: 0,
                                                bugfix_count: 0,
                                                additions: 0,
                                                deletions: 0,
                                                churn_score: 0,
                                                unique_authors: 0,
                                            },
                                        );

                                        entry.commit_count += 1;
                                        if is_bugfix {
                                            entry.bugfix_count += 1;
                                        }

                                        author_map
                                            .entry(path_str)
                                            .or_default()
                                            .insert(author_name.clone());
                                    }
                                    true
                                },
                                None,
                                None,
                                None,
                            )
                            .ok();
                        }
                    }
                }
            }
        }

        for (path, metrics) in metrics_map.iter_mut() {
            metrics.unique_authors = author_map.get(path).map_or(0, |s| s.len());
            metrics.churn_score = metrics.commit_count * 2 + metrics.bugfix_count * 5;
        }

        Ok(metrics_map)
    }

    pub fn get_diff_between_refs(&self, from_ref: &str, to_ref: &str) -> Result<GitDiffSummary> {
        let repo = Repository::open(&self.repo_path)
            .map_err(|e| sentrix_core::SentrixError::Git(e.to_string()))?;

        let obj_from = repo.revparse_single(from_ref).map_err(|e| {
            sentrix_core::SentrixError::Git(format!("Invalid from_ref '{}': {}", from_ref, e))
        })?;
        let obj_to = repo.revparse_single(to_ref).map_err(|e| {
            sentrix_core::SentrixError::Git(format!("Invalid to_ref '{}': {}", to_ref, e))
        })?;

        let tree_from = obj_from
            .peel_to_tree()
            .map_err(|e| sentrix_core::SentrixError::Git(e.to_string()))?;
        let tree_to = obj_to
            .peel_to_tree()
            .map_err(|e| sentrix_core::SentrixError::Git(e.to_string()))?;

        let mut opts = DiffOptions::new();
        let diff = repo
            .diff_tree_to_tree(Some(&tree_from), Some(&tree_to), Some(&mut opts))
            .map_err(|e| sentrix_core::SentrixError::Git(e.to_string()))?;

        let mut changed_files = Vec::new();
        diff.foreach(
            &mut |delta, _| {
                if let Some(p) = delta.new_file().path() {
                    changed_files.push(p.to_string_lossy().to_string());
                }
                true
            },
            None,
            None,
            None,
        )
        .ok();

        let stats = diff.stats().ok();
        let add = stats.as_ref().map_or(0, |s| s.insertions() as u32);
        let del = stats.as_ref().map_or(0, |s| s.deletions() as u32);

        Ok(GitDiffSummary {
            from_ref: from_ref.to_string(),
            to_ref: to_ref.to_string(),
            changed_files,
            total_additions: add,
            total_deletions: del,
        })
    }

    pub fn get_working_tree_changed_files(&self) -> Result<Vec<String>> {
        let repo = match Repository::open(&self.repo_path) {
            Ok(r) => r,
            Err(_) => return Ok(Vec::new()),
        };

        let mut status_opts = git2::StatusOptions::new();
        status_opts.include_untracked(true);

        let statuses = repo
            .statuses(Some(&mut status_opts))
            .map_err(|e| sentrix_core::SentrixError::Git(e.to_string()))?;

        let mut files = Vec::new();
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                files.push(path.to_string());
            }
        }

        Ok(files)
    }
}
