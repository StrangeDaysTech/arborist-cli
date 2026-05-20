---
id: AILOG-2026-04-03-003
title: Add cargo-dist release infrastructure
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [8]
lines_changed: 383
files_modified: [Cargo.toml, dist-workspace.toml, .github/workflows/release.yml, RELEASING.md]
observability_scope: none
tags: [release, cargo-dist, github-actions, cross-platform, binaries, infrastructure]
related: [AILOG-2026-04-03-002, AIDEC-2026-04-03-001]
---

# AILOG: Add cargo-dist release infrastructure

## Summary

Configured cargo-dist v0.31.0 for automated cross-platform binary builds. Tag-triggered GitHub Actions workflow builds release binaries for 5 targets (Linux x86_64/aarch64, macOS Intel/Apple Silicon, Windows x86_64) with shell and PowerShell installers. Added `RELEASING.md` documenting the release procedure.

## Context

Users needed pre-compiled binaries to install without a Rust toolchain. cargo-dist was chosen as the standard Rust ecosystem tool for this purpose, generating optimized builds with LTO and platform-specific installers.

## Actions Performed

1. Ran `dist init --yes --ci=github --installer=shell --installer=powershell`
2. Generated `dist-workspace.toml` with 5 target triples and installer config
3. Generated `.github/workflows/release.yml` — multi-stage pipeline: plan → build-local → build-global → host → announce
4. Added `[profile.dist]` to Cargo.toml (inherits release, LTO thin)
5. Created `RELEASING.md` with step-by-step release procedure

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `Cargo.toml` | +5/-0 | Added `[profile.dist]` with LTO |
| `dist-workspace.toml` | +17/-0 | cargo-dist configuration (targets, installers, CI) |
| `.github/workflows/release.yml` | +296/-0 | Tag-triggered release workflow |
| `RELEASING.md` | +65/-0 | Release procedure documentation |

## Decisions Made

- Selected cargo-dist over manual CI workflows for its opinionated, maintained approach and automatic installer generation.
- Used `install-path = "CARGO_HOME"` so binaries install alongside cargo-managed tools.
- Set `install-updater = false` because we implement self-update in-app (see AILOG-2026-04-03-004).

## Impact

- **Functionality**: Enables binary distribution for non-Rust users
- **Performance**: Release binaries use LTO for smaller, faster output
- **Security**: N/A
- **Privacy**: N/A

## Verification

- [x] `dist plan` generates valid manifest
- [x] All tests pass
- [x] v0.1.0 and v0.1.1 releases built successfully via this workflow

---

<!-- Template: DevTrail | https://strangedays.tech -->
