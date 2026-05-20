---
id: AILOG-2026-04-03-005
title: Fix binary name from arborist-cli to arborist
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [8]
lines_changed: 26
files_modified: [Cargo.toml, src/update.rs, tests/cli/directory.rs, tests/cli/exit_codes.rs, tests/cli/filtering.rs, tests/cli/multi_input.rs, tests/cli/output_formats.rs, tests/cli/single_file.rs, tests/cli/stdin.rs]
observability_scope: none
tags: [bugfix, binary-name, packaging, release]
related: [AILOG-2026-04-03-004]
---

# AILOG: Fix binary name from arborist-cli to arborist

## Summary

Added `[[bin]] name = "arborist"` to Cargo.toml so the installed binary is `arborist` instead of `arborist-cli` (the package name default). Updated `self_update` bin_name and all test references to match. Released as v0.1.1.

## Context

After publishing v0.1.0, users installing via `cargo binstall arborist-cli` got a binary named `arborist-cli`, but the CLI was designed to be invoked as `arborist` (matching the clap `name` field). Without the `[[bin]]` section, Cargo defaults the binary name to the package name.

## Actions Performed

1. Added `[[bin]] name = "arborist" path = "src/main.rs"` to Cargo.toml
2. Updated `src/update.rs`: `bin_name("arborist-cli")` → `bin_name("arborist")`
3. Updated 7 test files: `cargo_bin("arborist-cli")` → `cargo_bin("arborist")`
4. Bumped version to 0.1.1

## Impact

- **Functionality**: Binary now installs as `arborist` across all installation methods
- **Performance**: N/A
- **Security**: N/A

## Verification

- [x] `arborist --version` prints `arborist 0.1.1`
- [x] All 29 tests pass
- [x] Published to crates.io and GitHub Releases as v0.1.1

---

<!-- Template: DevTrail | https://strangedays.tech -->
