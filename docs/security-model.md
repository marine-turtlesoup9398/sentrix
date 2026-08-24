# SENTRIX Security Model

Comprehensive security model specification for **SENTRIX - Software Intelligence & Engineering Risk Explorer**.

---

## 1. Core Security Philosophy & Threat Boundary

SENTRIX operates as a local-first static software analysis engine. Its primary security guarantee is **Static Analysis Isolation**:

1. **Untrusted Data Processing**: All repository source code, AST trees, Git commit messages, author metadata, and configuration files are treated strictly as untrusted data inputs.
2. **Zero Code Execution**: SENTRIX never executes target repository build scripts (`npm run build`, `make`, `python setup.py`, `Cargo.toml build.rs`) or binary files.
3. **Local Storage Boundary**: Analysis results, Knowledge Graphs, and SIR state are stored in local SQLite databases (`.sentrix/`). No automated phone-home telemetries are transmitted.

---

## 2. Security Subsystems & Controls

- **Secret Redaction**: Regex-based entropy and pattern detection scans for API tokens, private keys, and AWS credentials, redacting raw secret values prior to persistence and visual display.
- **Prompt Injection Defense**: Untrusted source files and commit messages passed into the optional AI provider are sanitized via `PromptInjectionDefender` to neutralize prompt override commands.
- **CORS & Network Bound**: The local Axum REST server binds to `127.0.0.1:7070` with explicit route handlers and payload limits.
- **Path Traversal Protection**: File reading routines restrict path resolution relative to the target repository root directory.
