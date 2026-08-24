use sentrix_git::FileGitMetrics;
use sentrix_ir::FileItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HotspotItem {
    pub relative_path: String,
    pub risk_level: RiskLevel,
    pub hotspot_score: u32,
    pub complexity: u32,
    pub commit_count: u32,
    pub bugfix_count: u32,
    pub security_sensitive_functions: usize,
    pub reasons: Vec<String>,
}

pub struct HotspotEngine;

impl HotspotEngine {
    pub fn compute_hotspots(
        files: &[FileItem],
        git_metrics: &HashMap<String, FileGitMetrics>,
    ) -> Vec<HotspotItem> {
        let mut hotspots = Vec::new();

        for file in files {
            let mut score = 0u32;
            let mut reasons = Vec::new();

            // 1. Complexity
            score += file.cyclomatic_complexity;
            if file.cyclomatic_complexity > 10 {
                reasons.push(format!(
                    "High cyclomatic complexity ({})",
                    file.cyclomatic_complexity
                ));
            }

            // 2. Security sensitive functions
            let sec_count = file
                .functions
                .iter()
                .filter(|f| f.security_sensitive)
                .count();
            if sec_count > 0 {
                score += sec_count as u32 * 15;
                reasons.push(format!(
                    "Contains {} security-sensitive functions",
                    sec_count
                ));
            }

            // 3. Git churn & bugfixes
            let git = git_metrics.get(&file.relative_path);
            let mut commits = 0;
            let mut bugfixes = 0;

            if let Some(gm) = git {
                commits = gm.commit_count;
                bugfixes = gm.bugfix_count;
                score += gm.churn_score;

                if gm.commit_count > 5 {
                    reasons.push(format!(
                        "High change frequency ({} commits)",
                        gm.commit_count
                    ));
                }
                if gm.bugfix_count > 1 {
                    reasons.push(format!(
                        "Historical bugfix hotspot ({} fix commits)",
                        gm.bugfix_count
                    ));
                }
            }

            // 4. File size / line count
            if file.line_count > 300 {
                score += 10;
                reasons.push(format!("Large file size ({} lines)", file.line_count));
            }

            let risk_level = if score >= 40 {
                RiskLevel::Critical
            } else if score >= 25 {
                RiskLevel::High
            } else if score >= 12 {
                RiskLevel::Medium
            } else {
                RiskLevel::Low
            };

            if score > 5 {
                hotspots.push(HotspotItem {
                    relative_path: file.relative_path.clone(),
                    risk_level,
                    hotspot_score: score,
                    complexity: file.cyclomatic_complexity,
                    commit_count: commits,
                    bugfix_count: bugfixes,
                    security_sensitive_functions: sec_count,
                    reasons,
                });
            }
        }

        hotspots.sort_by_key(|b| std::cmp::Reverse(b.hotspot_score));
        hotspots
    }
}
