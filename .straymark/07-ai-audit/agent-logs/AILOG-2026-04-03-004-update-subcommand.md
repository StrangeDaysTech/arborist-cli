---
id: AILOG-2026-04-03-004
title: Add arborist update subcommand for self-updating
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [7, 8]
lines_changed: 225
files_modified: [Cargo.toml, src/cli.rs, src/main.rs, src/lib.rs, src/analysis.rs, src/update.rs, src/output/mod.rs, src/output/table.rs]
observability_scope: none
tags: [self-update, cli, subcommand, github-releases, feature]
related: [AILOG-2026-04-03-003, AIDEC-2026-04-03-001]
---

# AILOG: Add arborist update subcommand for self-updating

## Summary

Added `arborist update` subcommand that self-updates the binary from GitHub Releases using the `self_update` crate, and `arborist update --check` to check for new versions without installing. Restructured the CLI from flat `CliArgs` to `Cli` + `Command` + `AnalyzeArgs` with optional subcommands while maintaining full backward compatibility.

## Context

Users who install via shell/PowerShell installers or direct binary download needed a way to update without manual steps. The `self_update` crate was chosen over `axoupdater` for its flexibility across install methods (see AIDEC-2026-04-03-001). The CLI restructure was necessary to support subcommands alongside the existing flat argument interface.

## Actions Performed

1. Restructured `src/cli.rs`: `CliArgs` → `Cli` (Parser) + `Command` (Subcommand) + `AnalyzeArgs` (Args)
2. Created `src/update.rs` (105 lines): GitHub Releases self-update with install method detection
3. Updated `src/main.rs`: dispatch on `cli.command` variant
4. Renamed all `CliArgs` references to `AnalyzeArgs` across `lib.rs`, `analysis.rs`, `output/mod.rs`, `output/table.rs`
5. Added `self_update` dependency to `Cargo.toml`

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `src/update.rs` | +105/-0 | New self-update module |
| `src/cli.rs` | +21/-2 | CLI restructure with optional subcommands |
| `src/main.rs` | +11/-8 | Command dispatch logic |
| `src/lib.rs` | +3/-2 | CliArgs → AnalyzeArgs, add update module |
| `src/analysis.rs` | +4/-4 | CliArgs → AnalyzeArgs |
| `src/output/mod.rs` | +2/-2 | CliArgs → AnalyzeArgs |
| `src/output/table.rs` | +3/-3 | CliArgs → AnalyzeArgs |
| `Cargo.toml` | +1/-0 | Added self_update dependency |

## Decisions Made

- Install method detection via heuristic: if binary path contains `.cargo/bin`, suggest `cargo install` instead of self-replace
- `--check` flag for non-destructive version checking
- `no_confirm(true)` for non-interactive update (suitable for CI/automation)

## Impact

- **Functionality**: Users can self-update with a single command
- **Performance**: `self_update` adds ~2MB to binary size due to reqwest/TLS
- **Security**: Updates are downloaded over HTTPS from GitHub Releases
- **Privacy**: N/A (only contacts GitHub API for release info)

## Verification

- [x] `cargo test` — 29 tests pass (backward compatible)
- [x] `arborist update --check` runs correctly (404 expected before first release)
- [x] `arborist tests/fixtures/complex.rs` works as before

---

<!-- Template: DevTrail | https://strangedays.tech -->
