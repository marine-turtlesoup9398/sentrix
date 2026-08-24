# SENTRIX Release Engineering Checklist

24-point verification checklist for production releases of **SENTRIX — Software Intelligence & Engineering Risk Explorer**.

---

- [x] **1. Code Formatting**: `cargo fmt --all -- --check` passes with 0 errors.
- [x] **2. Compilation Check**: `cargo check --workspace` passes with 0 warnings.
- [x] **3. Workspace Tests**: `cargo test --workspace` passes 100% across all 14 crates.
- [x] **4. Clippy Lint Audit**: `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes with 0 warnings.
- [x] **5. Release Compilation**: `cargo build --release` produces binary `target/release/sentrix`.
- [x] **6. Frontend Production Build**: `cd frontend && npm run build` completes cleanly.
- [x] **7. SHA-256 Checksum**: Hash generated in `SHA256SUMS`.
- [x] **8. Configuration Validation**: `sentrix config validate` returns Exit Code 0.
- [x] **9. Exit Code Contract**: CLI returns exit codes 0 (success), 1 (drift violation), 2 (config error), 3 (runtime error).
- [x] **10. REST API Verification**: `/api/...` endpoints bind to `127.0.0.1:7070` by default.
- [x] **11. Static Isolation**: Zero code execution of target build scripts.
- [x] **12. Secret Redaction**: Credentials redacted before persistence and display.
- [x] **13. Prompt Injection Defense**: Untrusted text sanitized prior to AI prompt construction.
- [x] **14. SBOM Export**: `sentrix sbom` produces CycloneDX / SPDX JSON.
- [x] **15. License Compliance**: Dependencies audited and disclaimers included.
- [x] **16. SQLite Storage**: Schema versioning and safe cache invalidation verified.
- [x] **17. Incremental Analysis**: Partial re-analysis (1 ms) vs cache hit (0 ms) verified.
- [x] **18. Real Target Benchmark**: Tested against `pingsaketchoudhary.github.io` (105 files, 15,673 LOC, 302 nodes).
- [x] **19. Documentation Alignment**: `docs/` alignment with codebase implementation.
- [x] **20. Version Synchronization**: Version `1.0.0` consistent across all workspace manifests.
- [x] **21. Multi-Runner CI/CD**: `.github/workflows/release.yml` configured.
- [x] **22. Platform Matrix**: `reports/release/platform-matrix.json` published.
- [x] **23. Certification Artifact**: `reports/release/v1.0.0/release-certification.json` published.
- [x] **24. Launch Certification Report**: `docs/phase6-final-production-certification.md` published.
