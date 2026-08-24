# SENTRIX Software Reasoning & Intelligence Engine Architecture

SENTRIX Phase 3 elevates the platform from static code scanning into an evidence-grounded **Software Reasoning Engine**.

---

## 🧠 Core Intelligence Principles

1. **Evidence Grounding Standard**:
   - Every inference or risk score is backed by explicit line-level AST, graph, or Git evidence.
   - Categories: `DirectlyObserved`, `Inferred`, `Heuristic`, `Unknown`.
   - Confidence levels: `High`, `Medium`, `Low`, `Unknown`.
   - Fallback: If no evidence exists, SENTRIX returns **"Insufficient evidence"** instead of guessing.

2. **Graph Query Abstraction (`GraphQueryEngine`)**:
   - Direct & reverse call graph traversals.
   - Shortest path node evidence computation.
   - Tarjan / DFS Cycle Detection (`A -> B -> C -> A`) for circular dependencies.
   - PageRank and Degree Centrality scoring for architectural hotspots.

3. **Architecture Intelligence & Drift Detection**:
   - Configurable rules in `sentrix.toml` (`[architecture.rules]`).
   - Detects illegal layer dependencies (e.g. `Controller` bypassing `Service` to call `Repository` directly).
   - Tracks drift score and violation counts over Git history.

4. **Change Impact & Blast Radius**:
   - Converts textual Git diffs into semantic changes (`FunctionModified`, `SignatureChanged`, `CallGraphChanged`, `DependencyChanged`, `ApiChanged`, `SecurityBehaviorChanged`, `ArchitectureChanged`).
   - Computes transitive ripple effects, affected APIs, affected unit/integration tests, and critical propagation paths.

5. **Repository Health Engine**:
   - Scores repository overall health (0-100) across 6 categories:
     - Architecture
     - Maintainability
     - Security
     - Dependencies
     - Testing
     - Change Risk

6. **Grounded AI Reasoning**:
   - Translates natural language questions into structured `QueryIntent`.
   - Retrieves graph evidence before invoking AI model.
   - Enforces grounding contract with clear limitations ("Static analysis only; no runtime execution performed").
