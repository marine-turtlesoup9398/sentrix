# SENTRIX Product Claims & Evidence Matrix

Technical verification status for public capabilities of **SENTRIX - Software Intelligence & Engineering Risk Explorer v1.0.0**.

---

## 1. Product Capabilities Verification Table

| Claim / Capability | Technical Implementation Evidence | Verification Method | Status | Limitations |
|---|---|---|---|---|
| **Zero Target Code Execution** | Pure static analysis parser (`sentrix-parser`). | Security Architecture Audit | **VERIFIED** | Does not execute build scripts, Makefiles, or npm scripts. |
| **Multi-Language Parsing** | Tree-sitter extractors for 10 languages. | `parser_tests.rs` | **VERIFIED** | Supports Rust, JS/TS, Python, Go, Java, C/C++, HTML, CSS, Liquid, Astro. |
| **Tarjan Cycle Detection** | `sentrix-graph::query::detect_cycles` | `golden_intelligence_tests.rs` | **VERIFIED** | Detects strongly connected components in Knowledge Graph. |
| **Secret Redaction** | `SecretScanner::scan` regex pattern scanner. | `security_hardening_tests.rs` | **VERIFIED** | Redacts API keys, AWS credentials, and private keys. |
| **Prompt Injection Defense** | `PromptInjectionDefender::sanitize` | `security_hardening_tests.rs` | **VERIFIED** | Filters out untrusted prompt override directives. |
| **Predictive Change Risk** | `PredictiveRiskEngine::predict` | `evolution_tests.rs` | **VERIFIED** | Deterministic feature-weighted score (`LOW`, `MEDIUM`, `HIGH`). |
| **SPDX / CycloneDX SBOM** | `SbomGenerator::generate` | `security_hardening_tests.rs` | **VERIFIED** | Machine-readable SBOM JSON export. License audit disclaimer included. |
| **Linux x86_64 Support** | Production binary build `target/release/sentrix` | `cargo build --release` | **VERIFIED** | Compiled and tested on Linux 6.x x86_64. |
| **Windows / macOS Support** | GitHub Actions matrix setup | `.github/workflows/release.yml` | **BUILD NOT VERIFIED** | Binary compilation in non-Linux environments pending CI execution. |
