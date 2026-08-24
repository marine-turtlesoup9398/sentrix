# Contributing to SENTRIX

Thank you for your interest in contributing to SENTRIX.

---

## Development Setup

1. Prerequisites:
   - Rust toolchain 1.85+ (`rustc`, `cargo`)
   - Node.js 20.x and `npm`

2. Clone repository:
   ```bash
   git clone https://github.com/pingsaketchoudhary/sentrix.git
   cd sentrix
   ```

---

## Quality Gates

Before submitting a pull request, please ensure all quality checks pass:

```bash
# Code formatting check
cargo fmt --all -- --check

# Workspace compilation check
cargo check --workspace

# Unit and integration test suite
cargo test --workspace

# Clippy lints audit
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Frontend production build
cd frontend && npm run build
```

---

## Security and Untrusted Data Invariant

SENTRIX treats target repositories strictly as untrusted data inputs. Contributions must not introduce automatic execution of target repository build scripts, package manifests, Makefiles, setup scripts, or binaries.
