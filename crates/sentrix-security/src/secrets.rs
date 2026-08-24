use regex::Regex;
use sentrix_ir::{FileItem, FindingSeverity, SecurityFindingItem, SourceLocation};

pub struct SecretScanner;

impl SecretScanner {
    pub fn scan(file: &FileItem, content: &str) -> Vec<SecurityFindingItem> {
        let mut findings = Vec::new();

        let patterns = [
            (Regex::new(r#"(?i)(api[_-]?key|secret|password|passwd|auth[_-]?token)\s*[:=]\s*["']([^"']{8,})["']"#).unwrap(), "Hardcoded Secret / Credentials", FindingSeverity::Critical),
            (Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(), "AWS Access Key ID", FindingSeverity::Critical),
            (Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(), "GitHub Personal Access Token", FindingSeverity::Critical),
            (Regex::new(r"-----BEGIN (RSA|EC|PRIVATE) KEY-----").unwrap(), "Unencrypted Private Key", FindingSeverity::Critical),
        ];

        for (idx, line) in content.lines().enumerate() {
            for (re, name, severity) in &patterns {
                if let Some(cap) = re.captures(line) {
                    let matched = cap.get(0).unwrap().as_str();
                    let redacted = if matched.len() > 10 {
                        format!("{}...", &matched[..10])
                    } else {
                        matched.to_string()
                    };

                    findings.push(SecurityFindingItem {
                        id: format!("SEC-SECRET-L{}", idx + 1),
                        title: name.to_string(),
                        description: format!(
                            "Potential hardcoded credential or token detected in source file: {}",
                            redacted
                        ),
                        severity: severity.clone(),
                        category: "Secrets Management".to_string(),
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
                }
            }
        }

        findings
    }
}
