use sentrix_ir::Language;
use std::path::Path;

pub struct LanguageDetector;

impl LanguageDetector {
    pub fn detect<P: AsRef<Path>>(path: P) -> Language {
        let path = path.as_ref();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            Language::from_extension(ext)
        } else {
            // Content or filename check fallback
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                match file_name {
                    "Dockerfile" => Language::Unknown("docker".to_string()),
                    "Makefile" => Language::Unknown("make".to_string()),
                    "Cargo.toml" => Language::Rust,
                    "go.mod" => Language::Go,
                    "package.json" => Language::JavaScript,
                    "pyproject.toml" | "requirements.txt" => Language::Python,
                    _ => Language::Unknown("text".to_string()),
                }
            } else {
                Language::Unknown("binary".to_string())
            }
        }
    }
}
