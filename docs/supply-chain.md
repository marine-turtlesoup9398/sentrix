# SENTRIX Supply-Chain & Dependency Audit

Supply-chain security and SBOM specification for **SENTRIX - Software Intelligence & Engineering Risk Explorer**.

---

## 1. Dependency Inventory & Locking

- Workspace dependencies are audited and locked in `Cargo.lock`.
- Production release builds generate machine-readable SBOM JSON using CycloneDX v1.5 / SPDX formats via `sentrix sbom`.

---

## 2. Core Dependencies & License Classifications

| Dependency | Category | License | Audit Status |
|---|---|---|---|
| `tokio` | Async Runtime | MIT | Verified |
| `axum` / `tower-http` | REST Web Server | MIT | Verified |
| `rusqlite` | SQLite Database | MIT | Verified |
| `tree-sitter-*` | AST Parsers | MIT | Verified |
| `petgraph` | Knowledge Graph | MIT / Apache-2.0 | Verified |
| `git2` | Git Engine | MIT / Apache-2.0 | Verified |
| `serde` / `serde_json` | Serialization | MIT / Apache-2.0 | Verified |

---

## 3. License Compliance Disclaimer

License metadata extracted automatically from package manifests requires legal review prior to commercial distribution.
