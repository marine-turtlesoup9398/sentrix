use crate::detector::LanguageDetector;
use regex::Regex;
use sentrix_core::Result;
use sentrix_ir::{
    ApiEndpointItem, ClassItem, FileItem, FunctionItem, Language, Parameter, SourceLocation,
    SymbolId,
};
use sha2::{Digest, Sha256};
use std::path::Path;

pub struct CodeExtractor;

impl CodeExtractor {
    pub fn parse_file<P: AsRef<Path>>(
        file_path: P,
        root_dir: P,
    ) -> Result<(FileItem, Vec<ApiEndpointItem>)> {
        let file_path = file_path.as_ref();
        let root_dir = root_dir.as_ref();

        let content = std::fs::read_to_string(file_path)?;
        let relative_path = file_path
            .strip_prefix(root_dir)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        let language = LanguageDetector::detect(file_path);
        let line_count = content.lines().count();

        let mut imports = Vec::new();
        let mut functions = Vec::new();
        let mut classes = Vec::new();
        let mut apis = Vec::new();

        Self::extract_imports(&content, &language, &mut imports);
        Self::extract_symbols(
            file_path,
            &content,
            &language,
            &mut functions,
            &mut classes,
            &mut apis,
        );

        let total_complexity = functions.iter().map(|f| f.cyclomatic_complexity).sum();

        let file_item = FileItem {
            path: file_path.to_path_buf(),
            relative_path,
            language,
            hash,
            size_bytes: content.len() as u64,
            line_count,
            imports,
            functions,
            classes,
            cyclomatic_complexity: total_complexity,
        };

        Ok((file_item, apis))
    }

    fn extract_imports(content: &str, lang: &Language, imports: &mut Vec<String>) {
        match lang {
            Language::Python => {
                let re_import =
                    Regex::new(r"(?m)^(?:from\s+([\w\.]+)\s+import|import\s+([\w\.]+))").unwrap();
                for cap in re_import.captures_iter(content) {
                    if let Some(m) = cap.get(1).or_else(|| cap.get(2)) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
            Language::JavaScript | Language::TypeScript | Language::Astro => {
                let re_import = Regex::new(
                    r#"(?m)(?:import\s+.*?from\s+['"]([^'"]+)['"]|require\(['"]([^'"]+)['"]\))"#,
                )
                .unwrap();
                for cap in re_import.captures_iter(content) {
                    if let Some(m) = cap.get(1).or_else(|| cap.get(2)) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
            Language::Html | Language::Liquid => {
                let re_script = Regex::new(r#"(?i)<script\s+[^>]*src=["']([^"']+)["']"#).unwrap();
                for cap in re_script.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
                let re_link = Regex::new(r#"(?i)<link\s+[^>]*href=["']([^"']+)["']"#).unwrap();
                for cap in re_link.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
                let re_liquid_inc =
                    Regex::new(r#"\{%\s*(?:include|render)\s+['"]([^'"]+)['"]"#).unwrap();
                for cap in re_liquid_inc.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
            Language::Css => {
                let re_import =
                    Regex::new(r#"(?m)@import\s+(?:url\()?['"]?([^'"\)]+)['"]?\)?;"#).unwrap();
                for cap in re_import.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
            Language::Go => {
                let re_import =
                    Regex::new(r#"(?m)^\s*import\s+(?:\(\s*([\s\S]*?)\)|"([^"]+)")"#).unwrap();
                let re_str = Regex::new(r#""([^"]+)""#).unwrap();
                for cap in re_import.captures_iter(content) {
                    if let Some(m) = cap.get(2) {
                        imports.push(m.as_str().to_string());
                    } else if let Some(m) = cap.get(1) {
                        for inner in re_str.captures_iter(m.as_str()) {
                            if let Some(imp) = inner.get(1) {
                                imports.push(imp.as_str().to_string());
                            }
                        }
                    }
                }
            }
            Language::Rust => {
                let re_import = Regex::new(r"(?m)^\s*use\s+([\w:]+)").unwrap();
                for cap in re_import.captures_iter(content) {
                    if let Some(m) = cap.get(1) {
                        imports.push(m.as_str().to_string());
                    }
                }
            }
            _ => {}
        }
    }

    fn extract_symbols(
        path: &Path,
        content: &str,
        lang: &Language,
        functions: &mut Vec<FunctionItem>,
        classes: &mut Vec<ClassItem>,
        apis: &mut Vec<ApiEndpointItem>,
    ) {
        let lines: Vec<&str> = content.lines().collect();

        match lang {
            Language::JavaScript | Language::TypeScript => {
                Self::extract_js_ts_symbols(path, content, &lines, lang, functions, classes, apis);
            }
            Language::Astro => {
                // Parse Astro frontmatter between --- and --- as TypeScript
                let re_frontmatter = Regex::new(r"(?s)^---\s*\n([\s\S]*?)\n---").unwrap();
                if let Some(cap) = re_frontmatter.captures(content) {
                    if let Some(fm_text) = cap.get(1) {
                        let fm_lines: Vec<&str> = fm_text.as_str().lines().collect();
                        Self::extract_js_ts_symbols(
                            path,
                            fm_text.as_str(),
                            &fm_lines,
                            lang,
                            functions,
                            classes,
                            apis,
                        );
                    }
                }
                // Parse embedded <script> tags in Astro template
                let re_script_block =
                    Regex::new(r"(?s)<script(?:\s+[^>]*)?>([\s\S]*?)</script>").unwrap();
                for cap in re_script_block.captures_iter(content) {
                    if let Some(script_body) = cap.get(1) {
                        let script_text = script_body.as_str();
                        let script_lines: Vec<&str> = script_text.lines().collect();
                        Self::extract_js_ts_symbols(
                            path,
                            script_text,
                            &script_lines,
                            lang,
                            functions,
                            classes,
                            apis,
                        );
                    }
                }
            }
            Language::Html | Language::Liquid => {
                let re_script_block =
                    Regex::new(r"(?s)<script(?:\s+[^>]*)?>([\s\S]*?)</script>").unwrap();
                for cap in re_script_block.captures_iter(content) {
                    if let Some(script_body) = cap.get(1) {
                        let script_text = script_body.as_str();
                        let script_lines: Vec<&str> = script_text.lines().collect();
                        Self::extract_js_ts_symbols(
                            path,
                            script_text,
                            &script_lines,
                            lang,
                            functions,
                            classes,
                            apis,
                        );
                    }
                }
            }
            Language::Python => {
                let re_def =
                    Regex::new(r"(?m)^\s*(async\s+)?def\s+([a-zA-Z0-9_]+)\s*\((.*?)\)").unwrap();
                let re_class =
                    Regex::new(r"(?m)^\s*class\s+([a-zA-Z0-9_]+)\s*(?:\((.*?)\))?:").unwrap();
                let re_api = Regex::new(
                    r#"(?m)@(app|router)\.(get|post|put|delete|patch)\s*\(\s*["']([^"']+)["']"#,
                )
                .unwrap();

                for (idx, line) in lines.iter().enumerate() {
                    if let Some(cap) = re_def.captures(line) {
                        let fn_name = cap.get(2).map_or("fn", |m| m.as_str());
                        let params_raw = cap.get(3).map_or("", |m| m.as_str());

                        let params: Vec<Parameter> = params_raw
                            .split(',')
                            .filter(|p| !p.trim().is_empty())
                            .map(|p| Parameter {
                                name: p.trim().split(':').next().unwrap_or(p).trim().to_string(),
                                param_type: p.split(':').nth(1).map(|t| t.trim().to_string()),
                            })
                            .collect();

                        let fqn = format!("{}::{}", path.to_string_lossy(), fn_name);
                        let calls = Self::extract_calls(content, fn_name);

                        let func = FunctionItem {
                            id: SymbolId::new(&fqn),
                            name: fn_name.to_string(),
                            language: lang.clone(),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            parameters: params,
                            return_type: None,
                            calls,
                            cyclomatic_complexity: Self::calc_complexity(line),
                            cognitive_complexity: 1,
                            is_api_handler: line.contains("@app.") || line.contains("@router."),
                            is_exported: !fn_name.starts_with('_'),
                            security_sensitive: fn_name.to_lowercase().contains("auth")
                                || fn_name.to_lowercase().contains("login")
                                || fn_name.to_lowercase().contains("token")
                                || fn_name.to_lowercase().contains("password")
                                || fn_name.to_lowercase().contains("exec"),
                            doc_comment: None,
                        };

                        functions.push(func);
                    }

                    if let Some(cap) = re_class.captures(line) {
                        let class_name = cap.get(1).map_or("Class", |m| m.as_str());
                        let fqn = format!("{}::{}", path.to_string_lossy(), class_name);
                        classes.push(ClassItem {
                            id: SymbolId::new(&fqn),
                            name: class_name.to_string(),
                            language: lang.clone(),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            extends: vec![],
                            implements: vec![],
                            methods: vec![],
                            is_exported: true,
                            doc_comment: None,
                        });
                    }

                    if let Some(cap) = re_api.captures(line) {
                        let method = cap.get(2).unwrap().as_str().to_uppercase();
                        let route_path = cap.get(3).unwrap().as_str().to_string();
                        apis.push(ApiEndpointItem {
                            id: format!("{}:{}", method, route_path),
                            http_method: method,
                            path_pattern: route_path,
                            handler_symbol: SymbolId::new(&format!(
                                "{}:L{}",
                                path.to_string_lossy(),
                                idx + 1
                            )),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            is_authenticated: line.contains("auth")
                                || line.contains("jwt")
                                || line.contains("bearer"),
                        });
                    }
                }
            }
            Language::Go => {
                let re_fn =
                    Regex::new(r"(?m)^\s*func\s+(?:\(.*?\)\s+)?([a-zA-Z0-9_]+)\s*\((.*?)\)")
                        .unwrap();
                let re_type =
                    Regex::new(r"(?m)^\s*type\s+([a-zA-Z0-9_]+)\s+(struct|interface)").unwrap();

                for (idx, line) in lines.iter().enumerate() {
                    if let Some(cap) = re_fn.captures(line) {
                        let fn_name = cap.get(1).unwrap().as_str();
                        let fqn = format!("{}::{}", path.to_string_lossy(), fn_name);
                        let is_exported = fn_name.chars().next().is_some_and(|c| c.is_uppercase());
                        functions.push(FunctionItem {
                            id: SymbolId::new(&fqn),
                            name: fn_name.to_string(),
                            language: lang.clone(),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            parameters: vec![],
                            return_type: None,
                            calls: Self::extract_calls(content, fn_name),
                            cyclomatic_complexity: Self::calc_complexity(line),
                            cognitive_complexity: 1,
                            is_api_handler: line.contains("http.ResponseWriter")
                                || line.contains("gin.Context")
                                || line.contains("echo.Context"),
                            is_exported,
                            security_sensitive: fn_name.to_lowercase().contains("auth")
                                || fn_name.to_lowercase().contains("exec"),
                            doc_comment: None,
                        });
                    }

                    if let Some(cap) = re_type.captures(line) {
                        let struct_name = cap.get(1).unwrap().as_str();
                        let fqn = format!("{}::{}", path.to_string_lossy(), struct_name);
                        let is_exported =
                            struct_name.chars().next().is_some_and(|c| c.is_uppercase());
                        classes.push(ClassItem {
                            id: SymbolId::new(&fqn),
                            name: struct_name.to_string(),
                            language: lang.clone(),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            extends: vec![],
                            implements: vec![],
                            methods: vec![],
                            is_exported,
                            doc_comment: None,
                        });
                    }
                }
            }
            Language::Rust => {
                let re_fn = Regex::new(r"(?m)^\s*(pub(?:\(.*?\))?\s+)?(?:async\s+)?fn\s+([a-zA-Z0-9_]+)\s*(?:<.*?>)?\s*\((.*?)\)").unwrap();
                let re_struct = Regex::new(
                    r"(?m)^\s*(pub(?:\(.*?\))?\s+)?(struct|enum|trait)\s+([a-zA-Z0-9_]+)",
                )
                .unwrap();

                for (idx, line) in lines.iter().enumerate() {
                    if let Some(cap) = re_fn.captures(line) {
                        let fn_name = cap.get(2).unwrap().as_str();
                        let is_pub = cap.get(1).is_some();
                        let fqn = format!("{}::{}", path.to_string_lossy(), fn_name);
                        functions.push(FunctionItem {
                            id: SymbolId::new(&fqn),
                            name: fn_name.to_string(),
                            language: lang.clone(),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            parameters: vec![],
                            return_type: None,
                            calls: Self::extract_calls(content, fn_name),
                            cyclomatic_complexity: Self::calc_complexity(line),
                            cognitive_complexity: 1,
                            is_api_handler: line.contains("State")
                                || line.contains("Axum")
                                || line.contains("Json"),
                            is_exported: is_pub,
                            security_sensitive: fn_name.to_lowercase().contains("auth")
                                || fn_name.to_lowercase().contains("unsafe"),
                            doc_comment: None,
                        });
                    }

                    if let Some(cap) = re_struct.captures(line) {
                        let name = cap.get(3).unwrap().as_str();
                        let is_pub = cap.get(1).is_some();
                        let fqn = format!("{}::{}", path.to_string_lossy(), name);
                        classes.push(ClassItem {
                            id: SymbolId::new(&fqn),
                            name: name.to_string(),
                            language: lang.clone(),
                            location: SourceLocation {
                                file_path: path.to_path_buf(),
                                start_line: idx + 1,
                                start_col: 1,
                                end_line: idx + 1,
                                end_col: line.len(),
                            },
                            extends: vec![],
                            implements: vec![],
                            methods: vec![],
                            is_exported: is_pub,
                            doc_comment: None,
                        });
                    }
                }
            }
            _ => {}
        }
    }

    fn extract_js_ts_symbols(
        path: &Path,
        content: &str,
        lines: &[&str],
        lang: &Language,
        functions: &mut Vec<FunctionItem>,
        classes: &mut Vec<ClassItem>,
        apis: &mut Vec<ApiEndpointItem>,
    ) {
        let re_fn =
            Regex::new(r"(?m)^\s*(export\s+)?(async\s+)?function\s+([a-zA-Z0-9_]+)\s*\((.*?)\)")
                .unwrap();
        let re_var_arrow = Regex::new(r"(?m)^\s*(export\s+)?(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*(async\s*)?(?:\((.*?)\)|([a-zA-Z0-9_]+))\s*=>").unwrap();
        let re_var_fn = Regex::new(r"(?m)^\s*(export\s+)?(?:const|let|var)\s+([a-zA-Z0-9_]+)\s*=\s*(async\s*)?function\s*\((.*?)\)").unwrap();
        let re_method = Regex::new(r"(?m)^\s*(async\s+)?([a-zA-Z0-9_]+)\s*\((.*?)\)\s*\{").unwrap();
        let re_class = Regex::new(r"(?m)^\s*(export\s+)?class\s+([a-zA-Z0-9_]+)").unwrap();
        let re_api =
            Regex::new(r#"(?m)(app|router)\.(get|post|put|delete|patch)\s*\(\s*["']([^"']+)["']"#)
                .unwrap();

        for (idx, line) in lines.iter().enumerate() {
            let mut fn_found = None;

            if let Some(cap) = re_fn.captures(line) {
                fn_found = Some((
                    cap.get(3).unwrap().as_str(),
                    cap.get(4).map_or("", |m| m.as_str()),
                    cap.get(1).is_some(),
                ));
            } else if let Some(cap) = re_var_arrow.captures(line) {
                let params = cap.get(4).or_else(|| cap.get(5)).map_or("", |m| m.as_str());
                fn_found = Some((cap.get(2).unwrap().as_str(), params, cap.get(1).is_some()));
            } else if let Some(cap) = re_var_fn.captures(line) {
                fn_found = Some((
                    cap.get(2).unwrap().as_str(),
                    cap.get(4).map_or("", |m| m.as_str()),
                    cap.get(1).is_some(),
                ));
            } else if let Some(cap) = re_method.captures(line) {
                let name = cap.get(2).unwrap().as_str();
                if !matches!(
                    name,
                    "if" | "for" | "while" | "switch" | "catch" | "function"
                ) {
                    fn_found = Some((name, cap.get(3).map_or("", |m| m.as_str()), false));
                }
            }

            if let Some((fn_name, params_raw, is_exp)) = fn_found {
                let fqn = format!("{}::{}", path.to_string_lossy(), fn_name);
                let calls = Self::extract_calls(content, fn_name);
                functions.push(FunctionItem {
                    id: SymbolId::new(&fqn),
                    name: fn_name.to_string(),
                    language: lang.clone(),
                    location: SourceLocation {
                        file_path: path.to_path_buf(),
                        start_line: idx + 1,
                        start_col: 1,
                        end_line: idx + 1,
                        end_col: line.len(),
                    },
                    parameters: params_raw
                        .split(',')
                        .filter(|s| !s.trim().is_empty())
                        .map(|s| Parameter {
                            name: s.trim().to_string(),
                            param_type: None,
                        })
                        .collect(),
                    return_type: None,
                    calls,
                    cyclomatic_complexity: Self::calc_complexity(line),
                    cognitive_complexity: 1,
                    is_api_handler: line.contains("req")
                        || line.contains("res")
                        || line.contains("next"),
                    is_exported: is_exp,
                    security_sensitive: fn_name.to_lowercase().contains("auth")
                        || fn_name.to_lowercase().contains("token")
                        || fn_name.to_lowercase().contains("session")
                        || fn_name.to_lowercase().contains("crypto")
                        || fn_name.to_lowercase().contains("eval"),
                    doc_comment: None,
                });
            }

            if let Some(cap) = re_class.captures(line) {
                let class_name = cap.get(2).unwrap().as_str();
                let fqn = format!("{}::{}", path.to_string_lossy(), class_name);
                classes.push(ClassItem {
                    id: SymbolId::new(&fqn),
                    name: class_name.to_string(),
                    language: lang.clone(),
                    location: SourceLocation {
                        file_path: path.to_path_buf(),
                        start_line: idx + 1,
                        start_col: 1,
                        end_line: idx + 1,
                        end_col: line.len(),
                    },
                    extends: vec![],
                    implements: vec![],
                    methods: vec![],
                    is_exported: cap.get(1).is_some(),
                    doc_comment: None,
                });
            }

            if let Some(cap) = re_api.captures(line) {
                let method = cap.get(2).unwrap().as_str().to_uppercase();
                let route_path = cap.get(3).unwrap().as_str().to_string();
                apis.push(ApiEndpointItem {
                    id: format!("{}:{}", method, route_path),
                    http_method: method,
                    path_pattern: route_path,
                    handler_symbol: SymbolId::new(&format!(
                        "{}:L{}",
                        path.to_string_lossy(),
                        idx + 1
                    )),
                    location: SourceLocation {
                        file_path: path.to_path_buf(),
                        start_line: idx + 1,
                        start_col: 1,
                        end_line: idx + 1,
                        end_col: line.len(),
                    },
                    is_authenticated: line.contains("passport")
                        || line.contains("jwt")
                        || line.contains("auth"),
                });
            }
        }
    }

    fn extract_calls(content: &str, current_fn: &str) -> Vec<String> {
        let mut calls = Vec::new();
        let re_call = Regex::new(r"([a-zA-Z0-9_]+)\s*\(").unwrap();
        for cap in re_call.captures_iter(content) {
            let called = cap.get(1).unwrap().as_str();
            if called != current_fn
                && !matches!(
                    called,
                    "if" | "while"
                        | "for"
                        | "switch"
                        | "match"
                        | "catch"
                        | "print"
                        | "println"
                        | "function"
                        | "return"
                )
                && !calls.contains(&called.to_string())
            {
                calls.push(called.to_string());
            }
        }
        calls
    }

    fn calc_complexity(line: &str) -> u32 {
        let mut score = 1;
        if line.contains("if") {
            score += 1;
        }
        if line.contains("else") {
            score += 1;
        }
        if line.contains("for") || line.contains("while") {
            score += 1;
        }
        if line.contains("&&") || line.contains("||") {
            score += 1;
        }
        score
    }
}
