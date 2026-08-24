use crate::symbol::SymbolId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    Python,
    JavaScript,
    TypeScript,
    Go,
    Rust,
    Java,
    Cpp,
    C,
    CSharp,
    Ruby,
    Php,
    Kotlin,
    Swift,
    Html,
    Css,
    Liquid,
    Astro,
    Unknown(String),
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "py" | "pyw" => Language::Python,
            "js" | "jsx" | "mjs" | "cjs" => Language::JavaScript,
            "ts" | "tsx" | "mts" | "cts" => Language::TypeScript,
            "astro" => Language::Astro,
            "go" => Language::Go,
            "rs" => Language::Rust,
            "java" => Language::Java,
            "cpp" | "cxx" | "cc" | "hpp" | "h" => Language::Cpp,
            "c" => Language::C,
            "cs" => Language::CSharp,
            "rb" => Language::Ruby,
            "php" => Language::Php,
            "kt" | "kts" => Language::Kotlin,
            "swift" => Language::Swift,
            "html" | "htm" => Language::Html,
            "css" | "scss" | "sass" | "less" => Language::Css,
            "liquid" => Language::Liquid,
            other => Language::Unknown(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::Astro => "astro",
            Language::Go => "go",
            Language::Rust => "rust",
            Language::Java => "java",
            Language::Cpp => "cpp",
            Language::C => "c",
            Language::CSharp => "csharp",
            Language::Ruby => "ruby",
            Language::Php => "php",
            Language::Kotlin => "kotlin",
            Language::Swift => "swift",
            Language::Html => "html",
            Language::Css => "css",
            Language::Liquid => "liquid",
            Language::Unknown(_) => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file_path: PathBuf,
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionItem {
    pub id: SymbolId,
    pub name: String,
    pub language: Language,
    pub location: SourceLocation,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<String>,
    pub calls: Vec<String>,
    pub cyclomatic_complexity: u32,
    pub cognitive_complexity: u32,
    pub is_api_handler: bool,
    pub is_exported: bool,
    pub security_sensitive: bool,
    pub doc_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassItem {
    pub id: SymbolId,
    pub name: String,
    pub language: Language,
    pub location: SourceLocation,
    pub extends: Vec<String>,
    pub implements: Vec<String>,
    pub methods: Vec<FunctionItem>,
    pub is_exported: bool,
    pub doc_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleItem {
    pub id: SymbolId,
    pub name: String,
    pub file_path: PathBuf,
    pub language: Language,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub functions: Vec<FunctionItem>,
    pub classes: Vec<ClassItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub path: PathBuf,
    pub relative_path: String,
    pub language: Language,
    pub hash: String,
    pub size_bytes: u64,
    pub line_count: usize,
    pub imports: Vec<String>,
    pub functions: Vec<FunctionItem>,
    pub classes: Vec<ClassItem>,
    pub cyclomatic_complexity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEndpointItem {
    pub id: String,
    pub http_method: String,
    pub path_pattern: String,
    pub handler_symbol: SymbolId,
    pub location: SourceLocation,
    pub is_authenticated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyItem {
    pub name: String,
    pub version: String,
    pub ecosystem: String,
    pub is_direct: bool,
    pub manifest_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlowItem {
    pub id: String,
    pub source_symbol: SymbolId,
    pub sink_symbol: SymbolId,
    pub flow_path: Vec<SourceLocation>,
    pub description: String,
    pub confidence_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FindingSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityFindingItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub severity: FindingSeverity,
    pub category: String,
    pub location: SourceLocation,
    pub evidence: String,
    pub deterministic: bool,
}

// --- PHASE 3: EVIDENCE & CONFIDENCE MODEL ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceSourceType {
    DirectSource,
    Ast,
    Import,
    CallGraph,
    Dependency,
    GitHistory,
    SecurityFlow,
    Architecture,
    Test,
    Configuration,
    Inference,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceStrength {
    DirectlyObserved,
    Inferred,
    Heuristic,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConfidenceLevel {
    High,
    Medium,
    Low,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: String,
    pub source_type: EvidenceSourceType,
    pub file_path: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub symbol: Option<String>,
    pub relationship: Option<String>,
    pub commit: Option<String>,
    pub description: String,
    pub strength: EvidenceStrength,
    pub confidence: ConfidenceLevel,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SemanticChangeType {
    FunctionModified,
    SignatureChanged,
    CallGraphChanged,
    DependencyChanged,
    ApiChanged,
    SecurityBehaviorChanged,
    ArchitectureChanged,
}
