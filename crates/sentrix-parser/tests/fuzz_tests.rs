use sentrix_parser::CodeExtractor;

#[test]
fn test_fuzz_malformed_code() {
    let temp_dir = std::env::temp_dir().join("sentrix_fuzz");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let malformed_py = temp_dir.join("broken.py");
    std::fs::write(
        &malformed_py,
        "def (((( broken_syntax ::: === \n def login(",
    )
    .unwrap();

    let res = CodeExtractor::parse_file(&malformed_py, &temp_dir);
    assert!(
        res.is_ok(),
        "Parser should degrade gracefully on malformed Python code"
    );

    let malformed_ts = temp_dir.join("broken.ts");
    std::fs::write(&malformed_ts, "const x = ((( ; function ; export").unwrap();

    let res_ts = CodeExtractor::parse_file(&malformed_ts, &temp_dir);
    assert!(
        res_ts.is_ok(),
        "Parser should degrade gracefully on malformed TypeScript code"
    );
}
