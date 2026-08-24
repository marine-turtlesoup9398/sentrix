# SENTRIX Accuracy & Evidence Model

SENTRIX rejects black-box claims and arbitrary scores. Every insight produced by SENTRIX is backed by explicit code location evidence.

---

## 🎯 Grounding & Evidence Standard

### 1. Hotspot Risk Scoring Model
Hotspot scores combine measurable structural properties:
$$\text{Hotspot Score} = \text{Cyclomatic Complexity} + (15 \times \text{Security Functions}) + (2 \times \text{Commit Count}) + (5 \times \text{Bugfix Commits})$$
- **High Complexity**: Cyclomatic complexity > 10.
- **Security Sensitivity**: Functions handling auth, credentials, session, or crypto.
- **Git Churn & Bugfixes**: High commit density and bugfix commits.

### 2. Discovered Architecture Confidence
- **FrontendBackendSeparated**: Explicit detection of independent frontend UI and backend API directories.
- **Layered / MVC**: Presence of Controllers/Routes, Services, and Models.
- **Modular Monolith**: Workspace directory boundaries across crates or subpackages.

### 3. Security Findings Classification
- **Deterministic Findings**: Exact match against high-entropy secrets or unsafe process execution patterns (`exec()`, `os.system()`).
- **Data Flow Tracing**: Source-to-sink line-level evidence tracking.
