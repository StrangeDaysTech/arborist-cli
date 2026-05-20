---
id: AIDEC-2026-04-03-001
title: CLI restructure to subcommands and self-update crate selection
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [7]
tags: [architecture, cli, self-update, decision]
related: [AILOG-2026-04-03-004]
---

# AIDEC: CLI restructure to subcommands and self-update crate selection

## Decision

Restructure the CLI from flat `CliArgs` to optional subcommands (`Cli` + `Command` + `AnalyzeArgs`) and use `self_update` crate for self-updating from GitHub Releases.

## Context

Adding an `arborist update` command required introducing subcommands to a CLI that previously used only flat positional arguments and flags. Additionally, two crates were available for self-updating: `axoupdater` (cargo-dist companion) and `self_update` (general-purpose).

## Alternatives Considered

### CLI Structure

| Alternative | Pros | Cons |
|-------------|------|------|
| **Optional subcommands (chosen)** | Full backward compatibility, `arborist src/` still works, clean separation of concerns | Potential ambiguity if a file is literally named "update" |
| Mandatory subcommands (`arborist analyze src/`) | Cleaner architecture, no ambiguity | Breaking change, all existing usage and docs would need updating |
| Flag-based (`arborist --update`) | No subcommand parsing issues | Conflates update with analysis flags, unclear UX |

### Self-Update Crate

| Alternative | Pros | Cons |
|-------------|------|------|
| **self_update (chosen)** | Works regardless of install method, queries GitHub API directly, mature (0.43.x) | Pulls in reqwest/TLS, adds ~2MB to binary |
| axoupdater | Tight cargo-dist integration, lighter | Only works with cargo-dist install receipts, fails for cargo-install users |
| Custom implementation | No extra deps | Significant effort for HTTP, archive extraction, binary replacement |

## Rationale

- **Optional subcommands**: Backward compatibility was non-negotiable. The positional args ambiguity is minimal (a file named "update" can be addressed as `./update`).
- **self_update**: Users install via multiple channels (cargo install, cargo binstall, shell installer, direct download). `axoupdater` only works for the shell installer path, while `self_update` works for all GitHub Release downloads. The binary size increase is acceptable for a developer tool.

## Consequences

- The `CliArgs` type was renamed to `AnalyzeArgs` across 6 files — a one-time refactor
- `self_update` adds reqwest as a transitive dependency, increasing compile time and binary size
- Future subcommands (e.g., `arborist init`, `arborist config`) can be added trivially to the `Command` enum

---

<!-- Template: DevTrail | https://strangedays.tech -->
