# SENTRIX — Software Intelligence & Engineering Risk Explorer

SENTRIX is a local-first software intelligence and engineering risk analysis platform designed to analyze software repositories into structured, queryable knowledge graph models. It combines static AST parsing, graph dependency traversal, architecture drift detection, historical Git evolution analysis, secret scanning, and evidence-grounded reasoning.

---

## Problem Statement

A software repository is a living system of dependencies, execution paths, architectural boundaries, historical churn, and engineering risk. Standard static scanners often present file-level metrics without analyzing system-wide relationships, component blast radius, or historical change co-occurrences.

SENTRIX parses multi-language codebases into a Software Intermediate Representation (SIR) and builds an in-memory Software Knowledge Graph. This graph enables deterministic intelligence calculations, including call-graph ripple analysis, architecture rule enforcement, contribution concentration tracking, and call-graph backed test recommendations.

---

## Core Capabilities

- **Static Repository Analysis**: Pure static AST parsing across 10 programming languages and markup formats. Zero target repository code execution.
- **Software Knowledge Graph**: Petgraph `DiGraph` model with Tarjan strongly connected component cycle detection, PageRank centrality, and shortest-path evidence chains.
- **Architecture Intelligence**: Automated pattern detection (`ModularMonolith`, `FrontendBackendSeparated`, `Microservices`, `StaticSite`) and rule-based architecture drift enforcement.
- **Dependency Blast Radius**: Call-graph ripple calculation determining direct and transitive downstream impact of proposed file or symbol changes.
- **Git Churn & Evolution Engine**: Historical commit log mining for symbol evolution histories, co-change pattern pairs, and contributor concentration metrics.
- **Predictive Change Risk**: Feature-weighted deterministic risk evaluation (`LOW`, `MEDIUM`, `HIGH`) combining structural impact radius, security sensitivity, historical churn, and co-change propagation.
- **Test Recommendation Engine**: Call-graph and co-change analysis to recommend relevant test files to re-run following target code modifications.
- **Security & Secret Scanning**: Regex-based entropy and pattern scanner for credentials, tokens, and private keys, paired with intra-file data-flow taint tracking.
- **Prompt Injection Defense**: Sanitizes untrusted repository source code and commit messages before feeding context into optional AI models.
- **Software Bill of Materials (SBOM)**: CycloneDX v1.5 / SPDX machine-readable SBOM JSON export (`sentrix sbom`).
- **Grounded AI Layer**: Optional AI provider abstraction (`local`, `openai`, `anthropic`). AI acts solely as an evidence explainer layer with `"Insufficient evidence"` fallback when grounding requirements are unmet.
- **Interfaces**: Single binary CLI with structured exit codes, Axum REST API server (`127.0.0.1:7070`), and React/TypeScript web dashboard.

---

## Security Model & Invariants

1. **Target Repositories are Untrusted Input**: SENTRIX never executes target repository build scripts, package scripts, Makefiles, setup scripts, Dockerfiles, or arbitrary binaries. All analysis is strictly static.
2. **Local-First Architecture**: SIR data, Knowledge Graph nodes, and local state are stored in `.sentrix/cache.db`. No automated telemetry or phone-home mechanisms exist.
3. **Deterministic Authority**: Deterministic engines remain authoritative over analysis results. The optional AI layer cannot override evidence, execute commands, or modify files.
4. **Local API Network Boundary**: The Axum REST server binds to `127.0.0.1:7070` by default.

---

## Supported Languages

| Language / Format | Parsing Strategy | Extracted Symbols & Features |
|---|---|---|
| **Rust** | Tree-sitter AST | Functions, Structs, Enums, Traits, Use imports, API routes |
| **TypeScript / JavaScript** | Tree-sitter AST | Functions, Classes, Arrow functions, ESM/CJS imports, Express/Fastify endpoints |
| **Python** | Tree-sitter AST | Functions, Async defs, Classes, Imports, FastAPI/Flask routes |
| **Go** | Tree-sitter AST | Functions, Methods, Structs, Interfaces, Package imports |
| **Java** | Static Pattern Extractor | Classes, Methods, Package imports, Spring routes |
| **C / C++** | Static Pattern Extractor | Functions, Structs, Includes |
| **HTML / CSS** | Static Pattern Extractor | Script src links, Stylesheet hrefs, @import statements |
| **Liquid / Astro** | Static Pattern Extractor | Component inclusions, Frontmatter scripts |

---

## System Architecture

```
Target Repository Files & Git History
                 ↓
   Tree-sitter AST & Pattern Extractors
                 ↓
Software Intermediate Representation (SIR)
                 ↓
  Software Knowledge Graph (Petgraph DiGraph)
   ├── Tarjan Cycle Engine & PageRank Centrality
   ├── Architecture Drift Engine
   ├── Change Impact Blast Radius Calculator
   ├── Evolution & Co-Change Mining Engine
   └── Secret & Data-Flow Security Scanner
                 ↓
  CLI  |  Axum REST API  |  React Web Dashboard
```

---

## Installation

### Linux x86_64 Standalone Binary

```bash
# Build release binary from source
cargo build --release

# Copy executable to standard user binary directory
mkdir -p ~/.local/bin
cp target/release/sentrix ~/.local/bin/

# Ensure ~/.local/bin is in your PATH
export PATH="$HOME/.local/bin:$PATH"

# Verify installation
sentrix --version
```

### Build Requirements
- Rust toolchain `1.85+` (`cargo`, `rustc`)
- Node.js `20.x` and `npm` (for Web GUI dashboard build)

---

## Quickstart & Usage

### 1. Repository Analysis
Run a complete static scan of a local repository:

```bash
sentrix analyze /path/to/repository
```

Output as JSON:
```bash
sentrix --json analyze /path/to/repository
```

### 2. Configuration Validation
Validate local `sentrix.toml` configuration:

```bash
sentrix config validate
```

### 3. Engineering Risk & Hotspots
Display risk hotspots and maintainability metrics:

```bash
sentrix risk /path/to/repository
```

### 4. Architecture Drift Detection
Check layer rule violations against `sentrix.toml` rules:

```bash
sentrix drift /path/to/repository
```

### 5. Change Impact Blast Radius & Test Recommendations
Calculate affected components and recommended tests for a file or revision range:

```bash
sentrix impact src/services/auth_service.ts
```

### 6. Symbol History & Co-Change Mining
Inspect historical Git commit churn and symbol evolution:

```bash
sentrix history --symbol AuthService /path/to/repository
sentrix evolution /path/to/repository
```

### 7. Software Bill of Materials (SBOM) Export
Generate SPDX / CycloneDX SBOM JSON:

```bash
sentrix sbom /path/to/repository > sbom.json
```

### 8. Web GUI Dashboard
Launch the interactive local web dashboard:

```bash
sentrix serve --port 7070
```

Access the dashboard at `http://127.0.0.1:7070`.

---

## REST API Reference

The Axum REST server binds to `127.0.0.1:7070` by default.

- `GET /api/status`: Analysis status and health scorecard metrics.
- `GET /api/graph`: Software Knowledge Graph nodes and edges.
- `GET /api/architecture`: System architecture pattern classification and confidence.
- `GET /api/hotspots`: Risk hotspots and complexity breakdown.
- `GET /api/findings`: Secret findings and data-flow taint analysis findings.
- `POST /api/impact`: Compute change blast radius for a target component.
- `POST /api/evolution/predict`: Feature-weighted predictive change risk evaluation.
- `POST /api/evolution/recommend-tests`: Call-graph backed test recommendation engine.
- `POST /api/evolution/ownership`: Contributor concentration and bus-factor indicators.

---

## Configuration Reference (`sentrix.toml`)

```toml
[project]
name = "My Application"
root_dir = "."

[analysis]
threads = 4
max_file_size_mb = 10
exclude_patterns = ["target", "node_modules", ".git", "dist"]

[security]
scan_secrets = true
scan_dataflow = true
entropy_threshold = 4.5

[git]
history_depth = 500

[architecture]
rules = [
  { from = "controller", to = "service", action = "allow" },
  { from = "service", to = "repository", action = "allow" },
  { from = "controller", to = "repository", action = "deny" }
]

[ai]
enabled = false
provider = "local"
model = "gpt-4o-mini"

[server]
host = "127.0.0.1"
port = 7070
```

---

## Local Storage & Cache Semantics

SENTRIX maintains state in `.sentrix/cache.db` relative to the target repository root directory.
- **Cache Invalidation**: On schema version changes or parser updates, incompatible file cache rows are safely invalidated.
- **Git History Requirements**: Evolution commands rely on Git commit logs. Repositories with fewer than 2 commits return `"Insufficient historical evidence"`.

---

## Platform Support Matrix

| Target Platform | Support Status | Notes |
|---|---|---|
| **Linux x86_64** | **VERIFIED** | Tested on Ubuntu 22.04 / Linux 6.x x86_64 |
| **Linux ARM64** | **BUILD NOT VERIFIED** | Configured in GitHub release workflow |
| **macOS ARM64** | **BUILD NOT VERIFIED** | Configured in GitHub release workflow |
| **macOS x86_64** | **BUILD NOT VERIFIED** | Configured in GitHub release workflow |
| **Windows x86_64** | **BUILD NOT VERIFIED** | Configured in GitHub release workflow |

---

## Benchmark Metrics

Recorded baseline measurement on real target repository `pingsaketchoudhary.github.io` (Release profile on Linux x86_64):

- **Files Analyzed**: 105
- **Lines of Code**: 15,673
- **Functions Extracted**: 23
- **Knowledge Graph**: 302 nodes, 477 edges
- **Initial Analysis Duration**: 199 ms
- **Incremental Re-analysis Duration**: 1 ms
- **Cache Hit Duration**: 0 ms
- **Architecture Pattern**: `StaticSite`
- **Security Findings**: 0
- **Risk Hotspots**: 3

*Note: This is a recorded baseline benchmark for reference and does not constitute a universal performance guarantee.*

---

## Release Verification & Integrity

- **Release Version**: `1.0.0`
- **Linux x86_64 Release Binary SHA-256**: `f79aac3b2670718d76d8e80716898e3ca2a79e866603d86d51a85d3a9d84913c`
- **Checksum Verification**: `sha256sum -c SHA256SUMS`

---

## Project Structure

```
.
├── .github/workflows/   # CI quality & release workflows
├── crates/              # 14 modular Rust workspace crates
│   ├── sentrix-analysis # Architecture, health, hotspots & drift engines
│   ├── sentrix-api      # Axum REST API server
│   ├── sentrix-cli      # Single binary CLI entrypoint
│   ├── sentrix-core     # Errors, telemetry & config validation
│   ├── sentrix-evolution# Git churn, symbol history & predictive risk
│   ├── sentrix-git      # Git log parser
│   ├── sentrix-graph    # Petgraph Knowledge Graph & Tarjan engine
│   ├── sentrix-impact   # Change blast radius & ripple analysis
│   ├── sentrix-ir       # Software Intermediate Representation
│   ├── sentrix-parser   # Tree-sitter multi-language parser engine
│   ├── sentrix-search   # Grounded search & query intent engine
│   ├── sentrix-security # Secret scanner, SBOM & prompt injection defense
│   └── sentrix-storage  # Local SQLite database persistence
├── frontend/            # React + TypeScript + Vite GUI dashboard
├── docs/                # Architecture, security & API documentation
├── reports/             # Benchmark validation reports & certification JSON
├── tests/               # Fixtures & end-to-end acceptance tests
├── Cargo.toml           # Workspace manifest
└── LICENSE              # MIT License
```

---

## Development & Testing

Run quality gates locally:

```bash
# Check code formatting
cargo fmt --all -- --check

# Check workspace compilation
cargo check --workspace

# Run full workspace unit & integration test suite
cargo test --workspace

# Run Clippy lints
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Build frontend production bundle
cd frontend && npm run build
```

---

## Contributing

Contributions are welcome. Please ensure all pull requests pass formatting (`cargo fmt`), clippy (`cargo clippy`), unit tests (`cargo test`), and frontend production builds prior to submission.

---

## Security Reporting

If you discover a security vulnerability, please report it via GitHub security advisories or open a private disclosure issue. Target repositories are treated as untrusted data; please include test fixtures reproducing the vulnerability safely.

---

## License

SENTRIX is licensed under the [MIT License](LICENSE).
