# SENTRIX Release Process & Metadata

Release procedures and verification instructions for **SENTRIX v1.0.0**.

---

## 1. Release Verification Checklist

- [x] Workspace cargo tests pass (`cargo test --workspace`).
- [x] Workspace compiler checks pass (`cargo check --workspace`).
- [x] Frontend production Vite bundle builds cleanly (`npm run build`).
- [x] Single binary release target built (`target/release/sentrix`).
- [x] Checksum hash file generated (`SHA256SUMS`).

---

## 2. Checksum Verification

```bash
# Verify binary integrity against official release checksums
sha256sum -c SHA256SUMS
```

---

## 3. Platform Support Matrix

| Platform | Target Triple | Support Status | Verification Environment |
|---|---|---|---|
| Linux x86_64 | `x86_64-unknown-linux-gnu` | **VERIFIED** | Ubuntu 22.04 / Linux 6.x x86_64 |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | **BUILD NOT VERIFIED** | Not executed in current environment |
| macOS ARM64 | `aarch64-apple-darwin` | **BUILD NOT VERIFIED** | Not executed in current environment |
| macOS x86_64 | `x86_64-apple-darwin` | **BUILD NOT VERIFIED** | Not executed in current environment |
| Windows x86_64 | `x86_64-pc-windows-msvc` | **BUILD NOT VERIFIED** | Not executed in current environment |
