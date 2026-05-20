---
id: SBOM-2026-04-03-001
title: Addition of self_update dependency and transitive dependencies
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: [information_security]
iso_42001_clause: [8]
tags: [dependencies, sbom, self-update, security]
related: [AILOG-2026-04-03-004]
---

# SBOM: Addition of self_update dependency

## Summary

Added `self_update v0.43.1` as a direct dependency for the `arborist update` subcommand. This introduces a significant number of transitive dependencies (~197 new packages in Cargo.lock), primarily from the reqwest HTTP client and TLS stack.

## Direct Dependency Added

| Crate | Version | License | Purpose |
|-------|---------|---------|---------|
| `self_update` | 0.43.1 | MIT | Self-update from GitHub Releases |

### Features Enabled

- `archive-tar` — extract tar archives (Linux/macOS releases)
- `archive-zip` — extract zip archives (Windows releases)
- `compression-flate2` — decompress gzip/deflate

## Key Transitive Dependencies

| Crate | Version | Purpose | Risk Notes |
|-------|---------|---------|------------|
| `reqwest` | 0.12.28 | HTTP client for GitHub API | Well-maintained, widely used |
| `native-tls` | 0.2.18 | TLS via OS library | Uses OpenSSL on Linux, Secure Transport on macOS, SChannel on Windows |
| `tokio` | 1.51.0 | Async runtime (reqwest dependency) | Adds binary size but not used directly |
| `self-replace` | 1.5.0 | Binary self-replacement | Core update mechanism |
| `tar` | 0.4.45 | Tar archive extraction | — |
| `zip` | 6.0.0 | Zip archive extraction | — |
| `indicatif` | 0.18.4 | Progress bar display | — |
| `semver` | 1.0.28 | Version comparison | — |

## Impact Assessment

- **Binary size**: Increases from ~5MB to ~8MB (debug), ~3MB to ~5MB (release with LTO) due to TLS and HTTP stack
- **Compile time**: Increases by ~30s (first build) due to openssl-sys and reqwest
- **Attack surface**: reqwest + native-tls are well-audited, widely-used crates. The update mechanism only contacts `api.github.com` over HTTPS.
- **Supply chain**: All dependencies are from crates.io. No git dependencies.

## Mitigation

- TLS is handled by the OS native library (not rustls), reducing the cryptographic code compiled into the binary
- Updates only download from the project's own GitHub Releases URL
- Binary replacement uses `self-replace` which handles atomic replacement safely

---

<!-- Template: DevTrail | https://strangedays.tech -->
