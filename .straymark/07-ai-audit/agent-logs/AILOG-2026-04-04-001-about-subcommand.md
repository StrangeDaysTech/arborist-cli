---
id: AILOG-2026-04-04-001
title: "Add `about` subcommand"
type: AILOG
status: accepted
created: 2026-04-04
agent: claude-code-v1
confidence: high
risk_level: low
review_required: false
tags: [cli, feature]
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
files_changed:
  - src/about.rs
  - src/cli.rs
  - src/main.rs
  - src/lib.rs
---

## Summary

Added `arborist about` subcommand that displays project metadata (version, description, author, license, repository, website). Follows the same format as `devtrail about`.

## Changes

- **src/about.rs** (new): Module with `print()` function that outputs project info using `env!()` macros to read `CARGO_PKG_VERSION` and `CARGO_PKG_DESCRIPTION` at compile time.
- **src/cli.rs**: Added `About` variant to the `Command` enum.
- **src/main.rs**: Added match arm for `Command::About` calling `about::print()`.
- **src/lib.rs**: Registered `pub mod about`.

## Complexity Analysis

Ran `arborist` on all changed files:

| Function | Cognitive | Cyclomatic | SLOC |
|----------|-----------|------------|------|
| `about::print` | 0 | 1 | 13 |
| `main` | 3 | 6 | 17 |

All functions remain at low complexity. No documentation escalation needed.

## Rationale

Provides a quick way for users to see project metadata without checking Cargo.toml or the repository directly.
