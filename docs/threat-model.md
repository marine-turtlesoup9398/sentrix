# SENTRIX Threat Model

Threat model analysis using STRIDE methodology for **SENTRIX — Software Intelligence & Engineering Risk Explorer**.

---

| Threat Category | Potential Risk Vector | SENTRIX Countermeasure & Security Mitigation |
|---|---|---|
| **Spoofing** | Malicious Git commit author metadata. | Contributor shares are explicitly labeled as *"Historical Contribution Concentration"* based on Git commit headers, not verified developer identities. |
| **Tampering** | Corrupted SQLite database or cache invalidation bypass. | Database schema versioning and safe cache invalidation force cold re-analysis on schema or version mismatch. |
| **Repudiation** | Unverified evidence claims or hallucinated risk scores. | All findings require line-level `Evidence` provenance chains (`DirectlyObserved` AST/graph items). |
| **Information Disclosure** | Secret leakage in logs, SARIF exports, or AI context payloads. | Secret redaction filters mask credentials before persistence and external LLM API transport. |
| **Denial of Service** | Oversized files, deep AST trees, or infinite circular graph traversals. | Configurable file size limits (`max_file_size_mb = 10`), bounded petgraph traversals, and `LIMIT_EXCEEDED` error handling. |
| **Elevation of Privilege** | Prompt injection embedded in repository code causing AI tool execution. | AI provider has zero system execution capabilities. Prompt injection defender neutralizes directive overrides. |
