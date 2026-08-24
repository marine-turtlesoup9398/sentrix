use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_ir::FileItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub category: String, // File, Function, Class, API, Finding
    pub location: String,
    pub score: u32,
}

pub struct SearchEngine;

impl SearchEngine {
    pub fn search(
        query: &str,
        files: &[FileItem],
        _graph: &SoftwareKnowledgeGraph,
    ) -> Vec<SearchResult> {
        let q = query.to_lowercase();
        let mut results = Vec::new();

        for file in files {
            // Check File name
            if file.relative_path.to_lowercase().contains(&q) {
                results.push(SearchResult {
                    id: format!("file:{}", file.relative_path),
                    title: file.relative_path.clone(),
                    category: "File".to_string(),
                    location: file.relative_path.clone(),
                    score: 100,
                });
            }

            // Check Functions
            for func in &file.functions {
                if func.name.to_lowercase().contains(&q) {
                    let score = if func.name.to_lowercase() == q {
                        120
                    } else {
                        80
                    };
                    results.push(SearchResult {
                        id: format!("func:{}", func.id),
                        title: func.name.clone(),
                        category: "Function".to_string(),
                        location: format!("{}:L{}", file.relative_path, func.location.start_line),
                        score,
                    });
                }
            }

            // Check Classes
            for class in &file.classes {
                if class.name.to_lowercase().contains(&q) {
                    results.push(SearchResult {
                        id: format!("class:{}", class.id),
                        title: class.name.clone(),
                        category: "Class".to_string(),
                        location: format!("{}:L{}", file.relative_path, class.location.start_line),
                        score: 90,
                    });
                }
            }
        }

        results.sort_by_key(|b| std::cmp::Reverse(b.score));
        results
    }
}
