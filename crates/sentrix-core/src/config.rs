use crate::error::{Result, SentrixError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SentrixConfig {
    #[serde(default)]
    pub project: ProjectConfig,
    #[serde(default)]
    pub analysis: AnalysisConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub git: GitConfig,
    #[serde(default)]
    pub architecture: ArchitectureRulesConfig,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub server: ServerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub root_dir: PathBuf,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "SENTRIX Project".to_string(),
            root_dir: PathBuf::from("."),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub threads: usize,
    pub exclude_patterns: Vec<String>,
    pub max_file_size_mb: u64,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            threads: 4,
            exclude_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
            ],
            max_file_size_mb: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArchitectureRulesConfig {
    pub rules: Vec<ArchitectureRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureRule {
    pub from: String,
    pub to: String,
    pub action: String, // "allow" or "deny"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub scan_secrets: bool,
    pub scan_dataflow: bool,
    pub entropy_threshold: f32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            scan_secrets: true,
            scan_dataflow: true,
            entropy_threshold: 4.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    pub history_depth: usize,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self { history_depth: 500 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: "local".to_string(),
            api_key: None,
            model: "gpt-4o-mini".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 7070,
        }
    }
}

impl SentrixConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| SentrixError::Config(format!("Failed to read config file: {}", e)))?;

        let config: SentrixConfig = serde_json::from_str(&content)
            .map_err(|e| SentrixError::Config(format!("Failed to parse config JSON: {}", e)))?;

        Ok(config)
    }

    pub fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();

        if self.analysis.threads == 0 {
            errors.push("analysis.threads must be greater than 0".to_string());
        }
        if self.analysis.max_file_size_mb == 0 || self.analysis.max_file_size_mb > 500 {
            errors.push("analysis.max_file_size_mb must be between 1 and 500 MB".to_string());
        }
        if self.security.entropy_threshold <= 0.0 || self.security.entropy_threshold > 8.0 {
            errors.push("security.entropy_threshold must be between 0.0 and 8.0".to_string());
        }
        if self.server.port == 0 {
            errors.push("server.port must be a valid non-zero port number".to_string());
        }
        for (idx, r) in self.architecture.rules.iter().enumerate() {
            if r.action != "allow" && r.action != "deny" {
                errors.push(format!(
                    "architecture.rules[{}].action must be 'allow' or 'deny', found '{}'",
                    idx, r.action
                ));
            }
        }

        errors
    }
}
