use sentrix_ir::{FileItem, Language};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArchitecturePattern {
    Monolith,
    ModularMonolith,
    Microservices,
    LayeredArchitecture,
    MvcPattern,
    CleanHexagonal,
    FrontendBackendSeparated,
    StaticSite,
    FrontendApplication,
    LibraryPackage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureInsight {
    pub pattern: ArchitecturePattern,
    pub confidence: f32,
    pub evidence: Vec<String>,
    pub detected_layers: Vec<String>,
}

pub struct ArchitectureEngine;

impl ArchitectureEngine {
    pub fn discover(files: &[FileItem]) -> ArchitectureInsight {
        let mut dirs = HashSet::new();
        let mut has_frontend = false;
        let mut has_backend = false;
        let mut has_controllers = false;
        let mut has_models = false;
        let mut has_services = false;
        let mut has_docker_compose = false;
        let mut html_count = 0;
        let mut css_count = 0;
        let mut liquid_count = 0;
        let mut astro_count = 0;
        let mut js_ts_count = 0;

        let mut evidence = Vec::new();
        let mut layers = Vec::new();

        for file in files {
            let path_str = file.relative_path.to_lowercase();
            if let Some(parent) = file.path.parent() {
                dirs.insert(parent.to_string_lossy().to_string());
            }

            match file.language {
                Language::Html => html_count += 1,
                Language::Css => css_count += 1,
                Language::Liquid => liquid_count += 1,
                Language::Astro => astro_count += 1,
                Language::JavaScript | Language::TypeScript => js_ts_count += 1,
                _ => {}
            }

            // Exclude documentation files (.md, .markdown, docs/) from code layer detection
            let is_doc = path_str.ends_with(".md")
                || path_str.ends_with(".markdown")
                || path_str.starts_with("docs/")
                || path_str.contains("/docs/");
            if !is_doc {
                if path_str.contains("/controller")
                    || path_str.contains("/routes")
                    || path_str.contains("/api/")
                {
                    has_controllers = true;
                }
                if path_str.contains("/model")
                    || path_str.contains("/entity")
                    || path_str.contains("/schema")
                {
                    has_models = true;
                }
                if path_str.contains("/service") || path_str.contains("/usecase") {
                    has_services = true;
                }
                if path_str.contains("react")
                    || path_str.contains("vue")
                    || path_str.contains("frontend")
                    || path_str.contains("ui")
                    || path_str.contains("src/components")
                    || path_str.contains("src/pages")
                    || astro_count > 0
                {
                    has_frontend = true;
                }
                if path_str.contains("/backend/")
                    || path_str.contains("/server/")
                    || path_str.contains("crates/")
                    || path_str.contains("cmd/")
                {
                    has_backend = true;
                }
                if path_str.contains("docker-compose") {
                    has_docker_compose = true;
                }
            }
        }

        if has_controllers {
            layers.push("Controllers / Routes Layer".to_string());
        }
        if has_services {
            layers.push("Domain Services Layer".to_string());
        }
        if has_models {
            layers.push("Data Models / Database Layer".to_string());
        }
        if has_frontend || html_count > 0 || astro_count > 0 {
            layers.push("Frontend Presentation Layer".to_string());
        }

        // 1. Check for Static Site / Astro / Portfolio (HTML, Astro, CSS, Liquid, static JS/TS without backend server dirs)
        let is_static_web =
            (html_count > 0 || astro_count > 0 || liquid_count > 0 || css_count > 0)
                && !has_controllers
                && !has_backend
                && !has_services;
        if is_static_web {
            evidence.push(format!("Detected static web repository containing {} HTML, {} Astro, {} Liquid, {} CSS, and {} JS/TS files without backend server layers", html_count, astro_count, liquid_count, css_count, js_ts_count));
            return ArchitectureInsight {
                pattern: ArchitecturePattern::StaticSite,
                confidence: 0.95,
                evidence,
                detected_layers: layers,
            };
        }

        // 2. Check for Frontend Backend Separated
        if has_frontend && has_backend {
            evidence.push("Detected separate frontend and backend source trees".to_string());
            return ArchitectureInsight {
                pattern: ArchitecturePattern::FrontendBackendSeparated,
                confidence: 0.92,
                evidence,
                detected_layers: layers,
            };
        }

        // 3. Check for Layered Architecture
        if has_controllers && has_models && has_services {
            evidence.push("Detected explicit MVC / Layered architecture components (controllers, services, models)".to_string());
            return ArchitectureInsight {
                pattern: ArchitecturePattern::LayeredArchitecture,
                confidence: 0.88,
                evidence,
                detected_layers: layers,
            };
        }

        // 4. Check for Microservices
        if has_docker_compose {
            evidence.push("Detected multi-container orchestrations (docker-compose)".to_string());
            return ArchitectureInsight {
                pattern: ArchitecturePattern::Microservices,
                confidence: 0.80,
                evidence,
                detected_layers: layers,
            };
        }

        // 5. Check for Modular Monolith (multi-crate / multi-module backend)
        if dirs.len() > 10 && has_backend {
            evidence.push("Detected modular multi-directory backend workspace layout".to_string());
            ArchitectureInsight {
                pattern: ArchitecturePattern::ModularMonolith,
                confidence: 0.85,
                evidence,
                detected_layers: layers,
            }
        } else {
            evidence.push("Detected single package / library codebase".to_string());
            ArchitectureInsight {
                pattern: ArchitecturePattern::Monolith,
                confidence: 0.80,
                evidence,
                detected_layers: layers,
            }
        }
    }
}
