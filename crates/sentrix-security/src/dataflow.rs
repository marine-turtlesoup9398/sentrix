use regex::Regex;
use sentrix_ir::{
    DataFlowItem, FileItem, FindingSeverity, SecurityFindingItem, SourceLocation, SymbolId,
};

pub struct DataFlowEngine;

impl DataFlowEngine {
    pub fn analyze_file(
        file: &FileItem,
        content: &str,
    ) -> (Vec<SecurityFindingItem>, Vec<DataFlowItem>) {
        let mut findings = Vec::new();
        let mut flows = Vec::new();

        let sink_patterns = [
            (
                Regex::new(r"(?i)(os\.system|subprocess\.Popen|exec\(|eval\(|child_process\.exec)")
                    .unwrap(),
                "Dangerous Process Execution Sink",
                FindingSeverity::High,
            ),
            (
                Regex::new(r#"(?i)(SELECT|INSERT|UPDATE|DELETE).*?\+\s*[a-zA-Z0-9_]+"#).unwrap(),
                "Potential SQL String Concatenation Injection",
                FindingSeverity::High,
            ),
            (
                Regex::new(r"(?i)(pickle\.loads|unserialize|yaml\.load\()").unwrap(),
                "Unsafe Deserialization Sink",
                FindingSeverity::High,
            ),
        ];

        for (idx, line) in content.lines().enumerate() {
            for (re, title, severity) in &sink_patterns {
                if line.contains("def ") || line.contains("function ") || line.contains("fn ") {
                    continue;
                }
                if let Some(cap) = re.captures(line) {
                    let matched = cap.get(0).unwrap().as_str();

                    findings.push(SecurityFindingItem {
                        id: format!("SEC-FLOW-L{}", idx + 1),
                        title: title.to_string(),
                        description: format!(
                            "Data flows into potentially unsafe sink: {}",
                            matched
                        ),
                        severity: severity.clone(),
                        category: "Data Flow & Injection".to_string(),
                        location: SourceLocation {
                            file_path: file.path.clone(),
                            start_line: idx + 1,
                            start_col: 1,
                            end_line: idx + 1,
                            end_col: line.len(),
                        },
                        evidence: line.trim().to_string(),
                        deterministic: true,
                    });

                    flows.push(DataFlowItem {
                        id: format!("FLOW-{}", idx + 1),
                        source_symbol: SymbolId::new(&format!("{}:UserInput", file.relative_path)),
                        sink_symbol: SymbolId::new(&format!(
                            "{}:SinkL{}",
                            file.relative_path,
                            idx + 1
                        )),
                        flow_path: vec![SourceLocation {
                            file_path: file.path.clone(),
                            start_line: idx + 1,
                            start_col: 1,
                            end_line: idx + 1,
                            end_col: line.len(),
                        }],
                        description: format!("User input flows into sink: {}", matched),
                        confidence_score: 0.85,
                    });
                }
            }
        }

        (findings, flows)
    }
}
