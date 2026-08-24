use crate::git_history::CommitRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolHistoryReport {
    pub symbol_name: String,
    pub file_path: Option<String>,
    pub first_seen_commit: Option<String>,
    pub last_changed_commit: Option<String>,
    pub total_commits: usize,
    pub unique_authors_count: usize,
    pub bugfix_associated_commits: usize,
    pub revert_associated_commits: usize,
    pub co_changed_symbols: Vec<String>,
    pub evidence_provenance: Vec<String>,
}

pub struct SymbolHistoryEngine;

impl SymbolHistoryEngine {
    pub fn query_symbol_history(
        symbol: &str,
        file_path: Option<&str>,
        records: &[CommitRecord],
    ) -> Option<SymbolHistoryReport> {
        if records.is_empty() {
            return None;
        }

        let mut matching_commits = Vec::new();
        let mut authors = std::collections::HashSet::new();
        let mut bugfix_count = 0;
        let mut revert_count = 0;
        let mut co_changed_files = std::collections::HashSet::new();

        let sym_lower = symbol.to_lowercase();
        let file_filter = file_path.map(|p| p.to_lowercase());

        for rec in records {
            let matches_symbol = rec.message.to_lowercase().contains(&sym_lower);
            let matches_file = file_filter.as_ref().is_some_and(|fp| {
                rec.files_changed
                    .iter()
                    .any(|fc| fc.to_lowercase().contains(fp))
            });

            if matches_symbol || matches_file {
                matching_commits.push(rec);
                authors.insert(rec.author.clone());
                if rec.is_bugfix {
                    bugfix_count += 1;
                }
                if rec.is_revert {
                    revert_count += 1;
                }

                for f in &rec.files_changed {
                    co_changed_files.insert(f.clone());
                }
            }
        }

        if matching_commits.is_empty() {
            return None;
        }

        let first = matching_commits.last().unwrap().hash.clone();
        let last = matching_commits.first().unwrap().hash.clone();

        let mut prov = Vec::new();
        for mc in &matching_commits {
            prov.push(format!(
                "Commit {} by {}: '{}'",
                &mc.hash[..7.min(mc.hash.len())],
                mc.author,
                mc.message
            ));
        }

        Some(SymbolHistoryReport {
            symbol_name: symbol.to_string(),
            file_path: file_path.map(|s| s.to_string()),
            first_seen_commit: Some(first),
            last_changed_commit: Some(last),
            total_commits: matching_commits.len(),
            unique_authors_count: authors.len(),
            bugfix_associated_commits: bugfix_count,
            revert_associated_commits: revert_count,
            co_changed_symbols: co_changed_files.into_iter().collect(),
            evidence_provenance: prov,
        })
    }
}
