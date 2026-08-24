# SENTRIX Schema Migration & Upgrade Guide

Guide for database schema versioning, cache invalidation, and migration safety in **SENTRIX**.

---

## 1. Local Database Storage Topology

SENTRIX uses a local SQLite database located at `.sentrix/cache.db` relative to the repository root directory. The database stores:
- `file_cache`: Processed SIR FileItems, line counts, language classifications, and SHA-256 content hashes.
- `graph_nodes` & `graph_edges`: Persistent Software Knowledge Graph representation.

---

## 2. Automatic Schema Migration & Version Invalidation

When upgrading SENTRIX to a newer version:
1. **Schema Check**: SENTRIX inspects the database schema version on connection.
2. **Safe Cache Invalidation**: If the SIR model version or Tree-sitter parser version changes, SENTRIX automatically invalidates incompatible `file_cache` rows without corrupting existing user state.
3. **Data Protection Invariant**: SENTRIX never silently destroys user configuration files (`sentrix.toml`) or overrides user security parameters.
