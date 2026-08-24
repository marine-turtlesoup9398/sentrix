# SENTRIX - Software Intelligence & Engineering Risk Explorer

[![CI](https://github.com/pingsaketchoudhary/sentrix/actions/workflows/ci.yml/badge.svg)](https://github.com/pingsaketchoudhary/sentrix/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/pingsaketchoudhary/sentrix)](https://github.com/pingsaketchoudhary/sentrix/releases/tag/v1.0.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

SENTRIX is a local-first static software analysis and engineering intelligence tool. It parses source code repositories into a Software Intermediate Representation (SIR) and builds an in-memory Software Knowledge Graph. Analysis engines operate on this representation to analyze system relationships, execution paths, architecture boundaries, dependency blast radius, historical change evolution, and security risk.

---

## Overview

Software repositories contain complex structural and historical relationships that are difficult to evaluate from individual source files. SENTRIX analyzes source code and Git commit logs to produce structured software intelligence without executing target repository code.

The core design principle is static analysis isolation: target repositories are treated as untrusted data inputs. SENTRIX does not execute build scripts, package manifests, Makefiles, setup scripts, or binaries from target repositories.

---

## Core Capabilities

- Static Repository Analysis: Multi-language AST parsing using Tree-sitter for 10 languages and markup formats.
- Software Knowledge Graph: In-memory directed graph using Petgraph with Tarjan strongly connected component cycle detection and PageRank centrality.
- Architecture Intelligence: Automated pattern discovery (ModularMonolith, FrontendBackendSeparated, Microservices, StaticSite) and rule-based architecture drift detection.
- Change Impact Analysis: Call-graph ripple calculation determining direct and transitive downstream impact of file or symbol changes.
- Git Evolution Engine: Historical commit log mining for symbol evolution tracking, co-change pattern mining, and contribution concentration metrics.
- Predictive Change Risk: Feature-weighted deterministic risk evaluation (LOW, MEDIUM, HIGH) combining structural impact radius, security sensitivity, historical churn, and co-change propagation.
- Test Recommendation Engine: Call-graph and co-change analysis linking changed components to relevant test suites.
- Security and Secret Scanning: Pattern scanner for hardcoded credentials, API keys, and private keys, paired with intra-file data-flow taint analysis.
- Prompt Injection Defense: Sanitizes untrusted repository source code and commit messages before passing context to optional AI models.
- Software Bill of Materials (SBOM): CycloneDX v1.5 and SPDX machine-readable SBOM JSON generation.
- Grounded AI Layer: Optional AI provider integration (local, openai, anthropic). AI operates strictly as an explanation layer with an insufficient evidence fallback.
- User Interfaces: Single binary CLI with structured exit codes, Axum REST API server (127.0.0.1:7070), and React/TypeScript web dashboard.

---

## Processing Pipeline

```
Repository Source Files and Git Logs
                 |
   Tree-sitter AST and Pattern Parsers
                 |
Software Intermediate Representation (SIR)
                 |
  Software Knowledge Graph (Petgraph DiGraph)
   * Tarjan Cycle Detection and PageRank
   * Architecture Drift Detection
   * Change Impact Blast Radius
   * Evolution and Co-Change Mining
   * Secret and Data-Flow Scanner
                 |
  CLI  |  Axum REST API  |  React Web GUI
                 |
  Optional AI Explanation Layer (Local / Cloud)
```

Deterministic analysis engines are authoritative. The optional AI layer translates retrieved evidence into natural language and returns an explicit insufficient evidence fallback when grounding criteria are not met.

---

## Supported Languages

| Language or Format | Parsing Strategy | Extracted Symbols and Features |
|---|---|---|
| Rust | Tree-sitter AST | Functions, Structs, Enums, Traits, Use imports, API routes |
| JavaScript / TypeScript | Tree-sitter AST | Functions, Classes, Arrow functions, ESM/CJS imports, Web endpoints |
| Python | Tree-sitter AST | Functions, Async defs, Classes, Imports, FastAPI/Flask routes |
| Go | Tree-sitter AST | Functions, Methods, Structs, Interfaces, Package imports |
| Java | Pattern Extractor | Classes, Methods, Package imports, Spring routes |
| C / C++ | Pattern Extractor | Functions, Structs, Includes |
| HTML / CSS | Pattern Extractor | Script src links, Stylesheet hrefs, @import statements |
| Liquid / Astro | Pattern Extractor | Component inclusions, Frontmatter scripts |

---

## Security Model

1. Target Repositories as Untrusted Data: SENTRIX does not execute target build scripts, npm scripts, Makefiles, setup.py, Cargo build.rs, Dockerfiles, or target binaries.
2. Local-First Operation: All SIR state, graph models, and database files remain on local disk in `.sentrix/cache.db`. No automated telemetry or phone-home mechanisms exist.
3. Local REST API Boundary: The Axum REST server binds to `127.0.0.1:7070` by default. Request payload limits and CORS restrictions protect API routes.
4. Secret Redaction: Credentials and private keys are redacted before persistence and display.
5. Prompt Injection Defense: Untrusted repository content is filtered to neutralize prompt override commands before transport to optional AI providers.

---

## Installation

### Linux x86_64 Prebuilt Binary

Download the official release archive from GitHub:

```bash
# Download release archive
curl -LO https://github.com/pingsaketchoudhary/sentrix/releases/download/v1.0.1/sentrix-v1.0.1-x86_64-unknown-linux-gnu.tar.gz

# Extract binary
tar -xzf sentrix-v1.0.1-x86_64-unknown-linux-gnu.tar.gz

# Install binary to /usr/local/bin
sudo install -m 0755 sentrix /usr/local/bin/sentrix

# Verify installation
sentrix --version
```

### Source Build

Build from source using Cargo:

```bash
# Clone repository
git clone https://github.com/pingsaketchoudhary/sentrix.git
cd sentrix

# Build release binary
cargo build --release

# Install binary
sudo install -m 0755 target/release/sentrix /usr/local/bin/sentrix
```

---

## Quick Start

### Scan Repository

Run a static analysis scan on a local repository:

```bash
sentrix analyze /path/to/repository
```

Run scan with JSON output:

```bash
sentrix --json analyze /path/to/repository
```

### Validate Configuration

Validate `sentrix.toml` configuration:

```bash
sentrix config validate
```

### Risk and Hotspots

Display engineering risk hotspots:

```bash
sentrix risk /path/to/repository
```

### Architecture Drift

Detect architecture rule violations:

```bash
sentrix drift /path/to/repository
```

### Change Impact and Test Recommendations

Compute blast radius and recommended test files for a component:

```bash
sentrix impact src/services/auth_service.ts
```

### Evolution and Symbol History

Inspect Git churn and symbol evolution:

```bash
sentrix history --symbol AuthService /path/to/repository
sentrix evolution /path/to/repository
```

### Software Bill of Materials (SBOM)

Generate CycloneDX or SPDX SBOM JSON:

```bash
sentrix sbom /path/to/repository > sbom.json
```

### Web GUI Dashboard

Start local web server and launch dashboard:

```bash
sentrix serve --port 7070
```

Access the interface at `http://127.0.0.1:7070`.

---

## CLI Command Summary and Exit Codes

| Command | Description |
|---|---|
| `sentrix analyze [path]` | Scan repository code, dependencies, and security findings |
| `sentrix config validate [path]` | Validate sentrix.toml configuration file bounds |
| `sentrix health [path]` | Show repository health scorecard across 6 categories |
| `sentrix risk [path]` | Show engineering risk hotspots and complexity |
| `sentrix drift [path]` | Detect architecture rule violations and drift |
| `sentrix dependency [path]` | Detect circular dependencies and cycles |
| `sentrix impact [target]` | Calculate change impact radius and recommend tests |
| `sentrix history [symbol]` | Query historical symbol evolution from Git log |
| `sentrix evolution [path]` | Mine repository historical change patterns and co-changes |
| `sentrix predict [target]` | Estimate feature-weighted predictive change risk |
| `sentrix ownership [target]` | Show contribution concentration metrics |
| `sentrix sbom [path]` | Export SPDX and CycloneDX SBOM JSON |
| `sentrix serve [path]` | Launch local web GUI server |

### Exit Code Contract
- `0`: Successful execution.
- `1`: Finding, policy violation, or drift alert condition.
- `2`: Command usage or configuration validation error.
- `3`: Runtime or infrastructure failure.

---

## Configuration (`sentrix.toml`)

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

## REST API Reference

The REST API server binds to `127.0.0.1:7070` by default.

- `GET /api/status`: Analysis status and health metrics.
- `GET /api/graph`: Software Knowledge Graph nodes and edges.
- `GET /api/architecture`: System architecture pattern classification.
- `GET /api/hotspots`: Risk hotspots and complexity breakdown.
- `GET /api/findings`: Security findings and data-flow taint results.
- `POST /api/impact`: Compute change blast radius for a target component.
- `POST /api/evolution/predict`: Compute predictive change risk.
- `POST /api/evolution/recommend-tests`: Call-graph backed test recommendations.
- `POST /api/evolution/ownership`: Contributor concentration indicators.

---

## Local Storage and Cache Semantics

SENTRIX stores state in `.sentrix/cache.db` relative to the repository root directory.
- Cache Invalidation: Schema version updates or parser changes trigger automatic cache row invalidation.
- Git History Requirements: Evolution analysis requires Git commit logs. Repositories with fewer than 2 commits return an insufficient historical evidence message.

---

## Platform Support Matrix

| Target Platform | Support Status | Notes |
|---|---|---|
| Linux x86_64 | VERIFIED | Tested on Ubuntu 22.04 / Linux 6.x x86_64 |
| Linux ARM64 | BUILD NOT VERIFIED | Configured in GitHub release workflow |
| macOS ARM64 | BUILD NOT VERIFIED | Configured in GitHub release workflow |
| macOS x86_64 | BUILD NOT VERIFIED | Configured in GitHub release workflow |
| Windows x86_64 | BUILD NOT VERIFIED | Configured in GitHub release workflow |

---

## Reference Benchmark

Recorded baseline measurement on real target repository `pingsaketchoudhary.github.io` (Release profile on Linux x86_64):

- Files Analyzed: 105
- Lines of Code: 15,673
- Functions Extracted: 23
- Knowledge Graph: 302 nodes, 477 edges
- Initial Analysis Duration: 199 ms
- Incremental Re-analysis Duration: 1 ms
- Cache Hit Duration: 0 ms
- Architecture Pattern: StaticSite
- Security Findings: 0
- Risk Hotspots: 3

This recorded benchmark serves as a single target reference measurement and is not a universal performance guarantee.

---

## Release Verification and Checksums

- Release Version: 1.0.1
- GitHub Release: https://github.com/pingsaketchoudhary/sentrix/releases/tag/v1.0.1
- Release Archive: `sentrix-v1.0.1-x86_64-unknown-linux-gnu.tar.gz`
- Archive SHA-256 Checksum: `ddad542f3fa892d8c9075d57c0cc34c18ed79e613ec17777ef101f5f5b5e5839`
- Binary SHA-256 Checksum: `cd6449036827cfb2871ae2c2f6e322f837d14a80adc2651bfe28c66975ab8a54`

---

## Project Structure

```
.
|-- .github/workflows/   # CI and release workflow definitions
|-- crates/              # 14 modular Rust workspace crates
|   |-- sentrix-analysis # Architecture, health, hotspots, and drift engines
|   |-- sentrix-api      # Axum REST API server
|   |-- sentrix-cli      # Single binary CLI entrypoint
|   |-- sentrix-core     # Error definitions, telemetry, and config validation
|   |-- sentrix-evolution# Git churn, symbol history, and predictive risk
|   |-- sentrix-git      # Git log parser
|   |-- sentrix-graph    # Petgraph Knowledge Graph and Tarjan engine
|   |-- sentrix-impact   # Change blast radius and ripple analysis
|   |-- sentrix-ir       # Software Intermediate Representation
|   |-- sentrix-parser   # Tree-sitter multi-language parser engine
|   |-- sentrix-search   # Grounded search and query intent engine
|   |-- sentrix-security # Secret scanner, SBOM, and prompt injection defense
|   `-- sentrix-storage  # Local SQLite database persistence
|-- frontend/            # React, TypeScript, and Vite GUI dashboard
|-- docs/                # Architecture, security, and API documentation
|-- reports/             # Benchmark reports and release certification JSON
|-- tests/               # Fixtures and end-to-end acceptance tests
|-- Cargo.toml           # Workspace manifest
`-- LICENSE              # MIT License
```

---

## Development and Testing

Run local quality checks:

```bash
# Code formatting check
cargo fmt --all -- --check

# Compilation check
cargo check --workspace

# Workspace unit and integration tests
cargo test --workspace

# Clippy lints audit
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Frontend production build
cd frontend && npm run build
```

---

## Contributing

Contributions are welcome. Please ensure pull requests pass formatting (`cargo fmt`), clippy (`cargo clippy`), unit tests (`cargo test`), and frontend production builds (`npm run build`) prior to submission.

---

## Security Reporting

To report a potential security vulnerability, please submit a report via GitHub security advisories or open a private issue. Target repositories are treated as untrusted data inputs.

---

## License

SENTRIX is licensed under the [MIT License](LICENSE).
