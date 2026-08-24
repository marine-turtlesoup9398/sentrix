# SENTRIX Configuration Guide

Complete configuration reference for **SENTRIX — Software Intelligence & Engineering Risk Explorer**.

---

## 1. Overview

SENTRIX is configured via a local `sentrix.toml` or `sentrix.json` file in the repository root directory. Environment variables and CLI flags take precedence over configuration defaults.

---

## 2. Configuration Schema & Key Reference

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
provider = "local" # local, openai, anthropic
model = "gpt-4o-mini"
api_key = "ENV_AI_API_KEY"

[server]
host = "127.0.0.1"
port = 7070
```

---

## 3. Configuration Validation Subcommand

Validate configuration files using:

```bash
sentrix config validate [sentrix.toml]
```

### Exit Codes:
- `0`: Configuration is valid.
- `2`: Configuration schema validation failed (returns exact field name and valid range).
