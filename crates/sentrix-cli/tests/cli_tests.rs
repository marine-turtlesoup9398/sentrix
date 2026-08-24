use std::process::Command;

#[test]
fn test_cli_version_flag() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sentrix"));
    cmd.arg("--version");
    let output = cmd.output().expect("Failed to execute CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("1.0.1"));
}

#[test]
fn test_cli_version_subcommand() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sentrix"));
    cmd.arg("version");
    let output = cmd.output().expect("Failed to execute CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SENTRIX 1.0.1"));
    assert!(stdout.contains("Saket Choudhary"));
}

#[test]
fn test_cli_help_flag() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sentrix"));
    cmd.arg("--help");
    let output = cmd.output().expect("Failed to execute CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Software Intelligence"));
    assert!(stdout.contains("analyze"));
    assert!(stdout.contains("diagnostics"));
    assert!(stdout.contains("benchmark"));
    assert!(stdout.contains("health"));
    assert!(stdout.contains("risk"));
    assert!(stdout.contains("drift"));
    assert!(stdout.contains("dependency"));
    assert!(stdout.contains("impact"));
    assert!(stdout.contains("security"));
    assert!(stdout.contains("architecture"));
    assert!(stdout.contains("serve"));
    assert!(stdout.contains("sbom"));
}

#[test]
fn test_cli_invalid_path_fails_fast() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sentrix"));
    cmd.args(["analyze", "/nonexistent/directory/path/that/does/not/exist"]);
    let output = cmd.output().expect("Failed to execute CLI");
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does not exist"));
}

#[test]
fn test_cli_config_validate() {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(|p| std::path::PathBuf::from(p).join("../../sentrix.toml"))
        .unwrap();

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sentrix"));
    cmd.args(["config", "validate", root_dir.to_str().unwrap()]);
    let output = cmd.output().expect("Failed to execute CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("sentrix.toml"));
}

#[test]
fn test_cli_json_mode_analyze() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_sentrix"));
    cmd.args(["--json", "analyze", "."]);
    let output = cmd.output().expect("Failed to execute CLI");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value =
        serde_json::from_str(&stdout).expect("Output should be valid JSON");
    assert!(parsed.get("files_count").is_some());
    assert!(parsed.get("lines_of_code").is_some());
}
