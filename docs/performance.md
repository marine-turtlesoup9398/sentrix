# SENTRIX Performance & Benchmarking Report

SENTRIX is built in Rust using multi-threaded concurrency (`tokio`, `rayon`) and embedded state (`rusqlite`) for sub-second repository analysis.

---

## 📊 Measured Benchmark Results

| Repository Target | Language Profile | Files Analyzed | Lines of Code (LOC) | Knowledge Graph Entities | Initial Analysis (ms) | Incremental Re-analysis (ms) |
|---|---|---|---|---|---|---|
| **SENTRIX Core Workspace** | Rust, TypeScript, HTML/CSS | 62 | 8,504 | 533 nodes, 2,014 edges | **890 ms** | **4 ms** |
| **pingsaketchoudhary.github.io** (External Real Target) | HTML, JavaScript, Liquid, CSS | 105 | 15,673 | 178 nodes, 184 edges | **1,052 ms** | **0 ms** |

---

## ⚡ Performance Architecture

1. **Parallel Tree-sitter Parsing**: Source files are distributed across CPU worker pools.
2. **Incremental File Hashing**: File SHA256 hashes are cached in SQLite. When a developer modifies 1 file in a 10,000 file project, only the changed AST nodes and affected graph relationships are recomputed.
3. **In-Memory Adjacency Graph**: Graph queries and traversal algorithms run in RAM using fast node indices.
