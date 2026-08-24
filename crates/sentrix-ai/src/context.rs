use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_ir::{FileItem, SecurityFindingItem};

pub struct ContextBuilder;

impl ContextBuilder {
    pub fn build_evidence_prompt(
        question: &str,
        files: &[FileItem],
        findings: &[SecurityFindingItem],
        graph: &SoftwareKnowledgeGraph,
    ) -> String {
        let mut prompt = String::new();
        prompt.push_str("SENTRIX SOFTWARE KNOWLEDGE GRAPH EVIDENCE:\n");
        prompt.push_str("=========================================\n\n");

        prompt.push_str(&format!(
            "Project Overview: {} source files, {} graph nodes, {} edges\n\n",
            files.len(),
            graph.node_count(),
            graph.edge_count()
        ));

        prompt.push_str("Top Source Files:\n");
        for file in files.iter().take(10) {
            prompt.push_str(&format!(
                "- {}: {} lines, complexity {}\n",
                file.relative_path, file.line_count, file.cyclomatic_complexity
            ));
        }

        if !findings.is_empty() {
            prompt.push_str("\nSecurity Findings:\n");
            for f in findings.iter().take(5) {
                prompt.push_str(&format!(
                    "- [{:?}] {}: {}\n",
                    f.severity, f.title, f.evidence
                ));
            }
        }

        prompt.push_str("\nUser Question:\n");
        prompt.push_str(question);
        prompt.push_str("\n\nPlease provide a clear, technical explanation strictly grounded in the evidence above.");
        prompt
    }
}
