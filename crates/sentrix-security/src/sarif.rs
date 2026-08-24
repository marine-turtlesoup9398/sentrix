use sentrix_ir::SecurityFindingItem;
use serde_json::{json, Value};

pub struct SarifExporter;

impl SarifExporter {
    pub fn to_sarif(findings: &[SecurityFindingItem]) -> Value {
        let results: Vec<Value> = findings
            .iter()
            .map(|f| {
                let level = match f.severity {
                    sentrix_ir::FindingSeverity::Critical | sentrix_ir::FindingSeverity::High => {
                        "error"
                    }
                    sentrix_ir::FindingSeverity::Medium => "warning",
                    _ => "note",
                };

                json!({
                    "ruleId": f.id,
                    "level": level,
                    "message": {
                        "text": format!("{}: {}", f.title, f.description)
                    },
                    "locations": [
                        {
                            "physicalLocation": {
                                "artifactLocation": {
                                    "uri": f.location.file_path.to_string_lossy().to_string()
                                },
                                "region": {
                                    "startLine": f.location.start_line,
                                    "startColumn": f.location.start_col,
                                    "endLine": f.location.end_line,
                                    "endColumn": f.location.end_col
                                }
                            }
                        }
                    ]
                })
            })
            .collect();

        json!({
            "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
            "version": "2.1.0",
            "runs": [
                {
                    "tool": {
                        "driver": {
                            "name": "SENTRIX Security Intelligence Engine",
                            "version": "0.1.0",
                            "informationUri": "https://github.com/sentrix-ai/sentrix"
                        }
                    },
                    "results": results
                }
            ]
        })
    }
}
