use sentrix_ir::Language;
use sentrix_parser::{CodeExtractor, LanguageDetector};
use std::path::PathBuf;

#[test]
fn test_language_detection() {
    assert_eq!(LanguageDetector::detect("main.py"), Language::Python);
    assert_eq!(LanguageDetector::detect("index.ts"), Language::TypeScript);
    assert_eq!(LanguageDetector::detect("server.go"), Language::Go);
    assert_eq!(LanguageDetector::detect("lib.rs"), Language::Rust);
}

#[test]
fn test_python_parsing() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir.join("../../tests/fixtures/python_app");
    let file = root.join("main.py");
    let (file_item, apis) =
        CodeExtractor::parse_file(&file, &root).expect("Parsing python fixture should succeed");

    assert_eq!(file_item.language, Language::Python);
    assert!(file_item.functions.iter().any(|f| f.name == "get_users"));
    assert!(file_item.functions.iter().any(|f| f.name == "login"));
    assert_eq!(apis.len(), 2);
}
