use crate::git_history::CommitRecord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorShare {
    pub author_name: String,
    pub commit_count: usize,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentOwnership {
    pub component: String,
    pub total_commits: usize,
    pub contributors: Vec<ContributorShare>,
    pub contribution_concentration: String, // "HIGH", "MEDIUM", "DISTRIBUTED"
    pub bus_factor_indicator: usize,
    pub interpretation: String,
}

pub struct OwnershipEngine;

impl OwnershipEngine {
    pub fn analyze_ownership(component: &str, records: &[CommitRecord]) -> ComponentOwnership {
        let comp_low = component.to_lowercase();
        let mut author_counts: HashMap<String, usize> = HashMap::new();
        let mut total = 0;

        for r in records {
            let matches = r
                .files_changed
                .iter()
                .any(|f| f.to_lowercase().contains(&comp_low))
                || r.message.to_lowercase().contains(&comp_low);
            if matches {
                *author_counts.entry(r.author.clone()).or_insert(0) += 1;
                total += 1;
            }
        }

        if total == 0 {
            return ComponentOwnership {
                component: component.to_string(),
                total_commits: 0,
                contributors: Vec::new(),
                contribution_concentration: "UNKNOWN".to_string(),
                bus_factor_indicator: 0,
                interpretation: "No historical commit evidence found for this component."
                    .to_string(),
            };
        }

        let mut shares = Vec::new();
        for (author, count) in author_counts {
            let pct = (count as f32 / total as f32) * 100.0;
            shares.push(ContributorShare {
                author_name: author,
                commit_count: count,
                percentage: pct,
            });
        }
        shares.sort_by_key(|b| std::cmp::Reverse(b.commit_count));

        let top_share = shares.first().map_or(0.0, |s| s.percentage);
        let conc = if top_share >= 60.0 {
            "HIGH"
        } else if top_share >= 35.0 {
            "MEDIUM"
        } else {
            "DISTRIBUTED"
        };

        let bus_factor = shares
            .iter()
            .filter(|s| s.percentage >= 20.0)
            .count()
            .max(1);

        ComponentOwnership {
            component: component.to_string(),
            total_commits: total,
            contributors: shares,
            contribution_concentration: conc.to_string(),
            bus_factor_indicator: bus_factor,
            interpretation: format!("Historical changes are concentrated among {} main contributor(s). Top contributor accounts for {:.1}% of historical commits.", bus_factor, top_share),
        }
    }
}
