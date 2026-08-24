# SENTRIX v1.0.1 Release Notes

SENTRIX - Software Intelligence & Engineering Risk Explorer v1.0.1 is officially released.

---

## Key Maintenance & Engineering Fixes in v1.0.1
- Remote Git URL Analysis: Fixed remote HTTPS repository cloning so `sentrix analyze https://github.com/...` securely checks out remote repositories into isolated temporary storage, executes full AST static analysis, and cleans up temporary files.
- CLI Argument & Version Contract: Added `sentrix version` and `sentrix --version` returning build metadata, fast argument validation on missing or invalid target paths (Exit Code 2), and clean geometric ASCII terminal branding.
- Web Server & Graceful Shutdown: Added `/api/status` route and graceful `Ctrl+C` signal handling to `sentrix serve`.
- Quality Gates & Integration Test Suite: Added automated CLI integration test suite (`tests/cli_tests.rs`) verifying versioning, argument validation, JSON output, and `config validate`.

---

## Release Verification & Integrity
- Verified Platform: Linux x86_64 (`Ubuntu 22.04 / Linux 6.x`)
- Binary SHA-256 Checksum: `cd6449036827cfb2871ae2c2f6e322f837d14a80adc2651bfe28c66975ab8a54`
- Archive SHA-256 Checksum: `ddad542f3fa892d8c9075d57c0cc34c18ed79e613ec17777ef101f5f5b5e5839` (`sentrix-v1.0.1-x86_64-unknown-linux-gnu.tar.gz`)
