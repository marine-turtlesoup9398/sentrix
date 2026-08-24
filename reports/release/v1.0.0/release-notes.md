# SENTRIX v1.0.0 Release Notes

SENTRIX - Software Intelligence & Engineering Risk Explorer v1.0.0 is officially released.

---

## Capabilities Delivered
- **Multi-Language AST Parsing**: Static analysis across Rust, TypeScript, JavaScript, Python, Go, Java, C/C++, HTML, CSS, Liquid, and Astro. Zero target repository code execution.
- **Software Knowledge Graph**: In-memory Petgraph `DiGraph` with Tarjan cycle detection, PageRank centrality, and shortest-path evidence chains.
- **Architecture & Drift Engine**: Automated architecture pattern discovery (`ModularMonolith`, `FrontendBackendSeparated`, `Microservices`, `StaticSite`) and rule-based drift detection.
- **Change Impact & Test Recommendations**: Call-graph ripple calculation and co-change backed test suite recommendations.
- **Evolution Engine**: Symbol evolution tracking, co-change pattern mining, and contribution concentration metrics.
- **Security Surface**: Secret scanner, intra-file data-flow taint tracker, prompt injection defender, SARIF v2.1.0 exporter, and CycloneDX v1.5 / SPDX SBOM generator.
- **Local-First & Grounded AI**: Axum REST API server (`127.0.0.1:7070`), React GUI visualizer, CLI, and optional AI provider abstraction with `"Insufficient evidence"` fallback.

---

## Release Verification & Integrity
- **Verified Platform**: Linux x86_64 (`Ubuntu 22.04 / Linux 6.x`)
- **Binary SHA-256 Checksum**: `f79aac3b2670718d76d8e80716898e3ca2a79e866603d86d51a85d3a9d84913c`
- **Archive SHA-256 Checksum**: `ffd797de677ac624c278470cd44676714c642da804f243d571b725930a8a6b37` (`sentrix-v1.0.0-x86_64-unknown-linux-gnu.tar.gz`)
