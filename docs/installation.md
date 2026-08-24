# SENTRIX Installation & Uninstallation Guide

Installation, upgrade, and uninstallation guide for **SENTRIX - Software Intelligence & Engineering Risk Explorer**.

---

## 1. Local Binary Installation (Linux x86_64)

```bash
# Build production binary from source
cargo build --release

# Copy executable to standard user binary directory
mkdir -p ~/.local/bin
cp target/release/sentrix ~/.local/bin/

# Ensure ~/.local/bin is in your PATH
export PATH="$HOME/.local/bin:$PATH"

# Verify installation
sentrix --help
```

---

## 2. Uninstallation Procedure

To cleanly remove SENTRIX binary, local configuration, and cache state:

```bash
# Remove binary
rm -f ~/.local/bin/sentrix

# Remove local application data directory
rm -rf ~/.sentrix

# Remove local configuration files from repositories if desired
rm -f sentrix.toml
```
