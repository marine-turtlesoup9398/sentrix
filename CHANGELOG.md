# Changelog

All notable changes to **SENTRIX - Software Intelligence & Engineering Risk Explorer** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.0] - 2026-08-24

### Added
- **Software Intermediate Representation (SIR)** & multi-language Tree-sitter parsers (Rust, JS/TS, Python, Go, Java, C/C++, HTML, CSS, Liquid, Astro).
- **Software Knowledge Graph**: Petgraph `DiGraph` representation with Tarjan cycle detection, PageRank centrality, and shortest path evidence.
- **Deterministic Intelligence Engines**: Change Impact Engine, Dependency Intelligence Engine, Architecture Drift Engine, Repository Health Scorecard, and Engineering Risk Hotspots.
- **Evolution Engine (`sentrix-evolution`)**: Symbol history tracking, co-change pattern mining, test recommendation engine, contribution concentration metrics, and feature-weighted predictive change risk.
- **Security & Supply Chain**: Regex secret scanner, data flow taint tracker, SARIF v2.1.0 exporter, CycloneDX v1.5 SBOM generator, and prompt injection defender.
- **CLI & REST API**: Single binary CLI with `config validate`, `analyze`, `health`, `risk`, `drift`, `dependency`, `impact`, `history`, `evolution`, `predict`, `ownership`, `sbom`, and `serve` commands. Axum REST API server.
- **Web GUI Dashboard**: High-density React + TypeScript + Vite dashboard with Force Canvas Knowledge Graph visualizer and Evolution Workspace.
