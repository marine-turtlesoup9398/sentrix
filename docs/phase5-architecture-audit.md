# SENTRIX Phase 5 — Production Architecture Audit

Systematic architectural audit of **SENTRIX — Software Intelligence & Engineering Risk Explorer**.

---

## 1. Workspace Topology & Crate Graph

SENTRIX is structured as a 14-crate local-first Rust workspace:

```
[sentrix-cli] (Binary)
   ├── [sentrix-api] (Axum REST Server)
   ├── [sentrix-evolution] (Git Churn, Co-Change, Predictive Risk, Ownership)
   ├── [sentrix-search] (Query Intent, Semantic Grounding)
   ├── [sentrix-impact] (Change Ripple Radius)
   ├── [sentrix-analysis] (Architecture, Hotspots, Health, Drift)
   ├── [sentrix-security] (Secrets, DataFlow, SARIF, SBOM, PromptInjection)
   ├── [sentrix-ai] (Pluggable AI Provider Abstraction)
   ├── [sentrix-graph] (Knowledge Graph DiGraph & Tarjan Cycle Engine)
   ├── [sentrix-parser] (Tree-sitter Extractor Engine)
   ├── [sentrix-git] (Git History Analyzer)
   ├── [sentrix-storage] (SQLite Persistence)
   ├── [sentrix-ir] (Software Intermediate Representation)
   └── [sentrix-core] (Config, Error, Telemetry)
```

---

## 2. Core Subsystem Boundaries

| Subsystem | Primary Crate | Responsibilities & Invariants |
|---|---|---|
| **Core & Configuration** | `sentrix-core` | Defines `SentrixError`, `SentrixConfig`, and logging initialization. |
| **Intermediate Representation** | `sentrix-ir` | Defines `FileItem`, `SymbolItem`, `ApiEndpointItem`, `SecurityFindingItem`, `Evidence`, `ConfidenceLevel`. |
| **Parser Engine** | `sentrix-parser` | Pure static AST analysis across Rust, JS/TS, Python, Go, Java, C/C++, HTML, CSS, Liquid, and Astro. Zero script execution. |
| **Graph Model** | `sentrix-graph` | Petgraph `DiGraph` representation of repository entities (`NodeType`) and relationships (`EdgeType`). Tarjan cycle detection & PageRank centrality. |
| **Security Surface** | `sentrix-security` | Regex secret scanner, intra-file data-flow taint tracker, SARIF v2.1.0 exporter, SPDX/CycloneDX SBOM exporter, and prompt injection defender. |
| **Evolution Engine** | `sentrix-evolution` | Git log parsing, symbol evolution history, co-change mining, predictive change risk calculation, and test recommendation. |
| **API & CLI** | `sentrix-api`, `sentrix-cli` | Axum REST web server (`/api/v1/...`) and single-binary command-line interface. |
| **Frontend Dashboard** | `frontend/` | React + TypeScript + Vite GUI with Force Canvas Knowledge Graph visualizer and Evolution Workspace. |

---

## 3. Data Flow & Security Isolation Model

1. **Static Analysis Input**: Source code files and Git logs are read as read-only strings.
2. **Zero Code Execution**: Target repositories are treated as untrusted data. No Makefiles, npm scripts, or build scripts are ever executed.
3. **Local State**: SQLite databases and memory graphs remain strictly on local disk (`.sentrix/`).
4. **Prompt Injection Defense**: Repository text passed to optional AI models is filtered to neutralize prompt override commands.
