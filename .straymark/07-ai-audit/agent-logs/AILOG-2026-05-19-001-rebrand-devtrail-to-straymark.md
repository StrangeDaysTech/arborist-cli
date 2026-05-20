---
id: AILOG-2026-05-19-001
title: Rebrand DevTrail → StrayMark and migrate framework to fw-4.17.0
status: accepted
created: 2026-05-19
agent: claude-code-v4.7
confidence: high
review_required: true
risk_level: medium
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [7, 8]
lines_changed: 0
files_modified:
  - .devtrail/ (renamed → .straymark/)
  - CLAUDE.md
  - .cursorrules
  - Cargo.toml
  - GEMINI.md
  - .github/copilot-instructions.md
  - .specify/memory/constitution.md
  - .specify/starter/IDEA.md
  - DEVTRAIL.md (deleted; STRAYMARK.md installed by framework)
  - .claude/skills/devtrail-* (7 deleted)
  - .gemini/skills/devtrail-* (7 deleted)
  - .agent/workflows/devtrail-*.md (7 deleted)
  - .cursor/rules/devtrail.md (deleted)
observability_scope: none
tags: [rebrand, straymark, framework-update, governance, tooling]
related:
  - AILOG-2026-04-02-001-add-git-operations-rules-claude-md.md
---

# AILOG: Rebrand DevTrail → StrayMark and migrate framework to fw-4.17.0

## Summary

Upstream DevTrail tool was renamed to **StrayMark**. This change migrates the arborist-cli repo from the DevTrail framework (fw-4.1.0, CLI v3.15.0 as `devtrail`) to the StrayMark framework (fw-4.17.0, CLI v3.15.0 as `straymark`): the `.devtrail/` directory was renamed to `.straymark/`, the framework was repaired and updated, orphan `devtrail-*` files were removed, and project-level references in directive files (`CLAUDE.md`, `.cursorrules`, `Cargo.toml`, `.specify/*`) were updated. No code in `src/` was touched.

## Context

The `straymark` CLI is now installed at `~/.cargo/bin/straymark` (v3.15.0). Running `straymark status` against the repo reported `StrayMark is not installed` because the tool looks for `.straymark/`, not `.devtrail/`. Beyond the simple rename, the new framework version (fw-4.17.0) introduces new concepts not present in fw-4.1.0: **Charters** (bounded units of work with telemetry), GB 45438 AI content labeling, multi-batch ledgers, external audit pipelines, China regulatory scope (opt-in), and additional `.straymark/00-governance/` files (CHARTER-CHAIN-EVOLUTION, EMERGENT-OBSERVATION-DESIGN, FOLLOW-UPS-BACKLOG-PATTERN, SPECKIT-CHARTER-BRIDGE, CHINA-REGULATORY-FRAMEWORK, etc.). The operator chose to bundle the rebrand and the framework upgrade in a single PR rather than land them sequentially.

Before starting, two untracked artifacts (`dist-manifest.yml` in repo root and `dist-templates/`) were identified as residue from a prior DevTrail experiment and discarded per operator decision.

## Actions Performed

1. Created branch `feat/straymark-rebrand` from `main`.
2. Deleted untracked residuals `dist-manifest.yml` (root) and `dist-templates/`.
3. `git stash`'d 16 locally-modified framework files; later inspected the stash, confirmed the only substantive change (cognitive-complexity heuristic in `devtrail-status` skills) was already incorporated upstream in fw-4.17.0, and dropped the stash.
4. `git mv .devtrail .straymark` — preserves git history for all 81+ framework files.
5. Ran `straymark repair` — framework auto-detected the rename, downloaded fw-4.17.0, restored 117 files including `STRAYMARK.md`, schemas, charter templates, China-regulatory artifacts, zh-CN translations, `AGENTS.md`, and rebrandend skills/workflows (`.claude/skills/straymark-*`, `.gemini/skills/straymark-*`, `.agent/workflows/straymark-*`, `.cursor/rules/straymark.md`).
6. Ran `straymark update-framework` — updated 190 files from fw-4.1.0 to fw-4.17.0; the new `<!-- straymark:begin --> ... <!-- straymark:end -->` directive block was injected into `CLAUDE.md` and `.cursorrules`.
7. Deleted orphan files: `DEVTRAIL.md`, 7× `.claude/skills/devtrail-*/`, 7× `.gemini/skills/devtrail-*/`, 7× `.agent/workflows/devtrail-*.md`, `.cursor/rules/devtrail.md`.
8. Manually rewrote `CLAUDE.md` and `.cursorrules` to remove the legacy `<!-- devtrail:begin --> ... <!-- devtrail:end -->` block left behind by the framework update and replace the project's header strings (`DevTrail` → `StrayMark`, `.devtrail/` → `.straymark/`, `devtrail analyze` → `straymark analyze`).
9. Updated `Cargo.toml` `exclude` to point at `.straymark/`.
10. Deleted stale `GEMINI.md` and `.github/copilot-instructions.md` (still carrying the legacy block) and re-ran `straymark repair`, which regenerated them from the new manifest.
11. Updated project-level references in `.specify/memory/constitution.md` and `.specify/starter/IDEA.md` (kept one explicit "formerly DevTrail" mention as historical context).
12. Validated: `cargo check` (clean), `cargo test` (29/29 pass), `straymark validate` (0 errors, 1 preexisting unrelated warning), `straymark status` (framework fw-4.17.0, CLI cli-3.15.0, 17/17 structure items OK).
13. Updated Claude's auto-memory (`MEMORY.md`, renamed `feedback_devtrail_complexity.md` → `feedback_straymark_complexity.md`, edited `feedback_git_workflow.md`, added `project_rebrand_straymark.md`).

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `.devtrail/` → `.straymark/` | n/a | Directory rename (`git mv`); preserves history. |
| `CLAUDE.md` | rewritten | Removed legacy block, `DevTrail`→`StrayMark`, path updates. |
| `.cursorrules` | rewritten | Same as above. |
| `Cargo.toml` | 1 line | `exclude` path. |
| `Cargo.lock` | 0 | No dependency change. |
| `STRAYMARK.md` | created | Installed by framework (replaces `DEVTRAIL.md`). |
| `AGENTS.md` | created | New open-standard directive file introduced in fw-4.17.0. |
| `.claude/skills/straymark-*` | 11 created | New skills (adr, aidec, ailog, audit-execute, audit-prompt, audit-review, charter-new, mcard, new, sec, status). |
| `.gemini/skills/straymark-*` | 11 created | Same. |
| `.agent/workflows/straymark-*.md` | 11 created | Same. |
| `.cursor/rules/straymark.md` | created | New directive injection target. |
| `.straymark/schemas/*` | 3 created | Charter telemetry/audit schemas. |
| `.straymark/00-governance/*` (new) | 12 created | Charter, China, observation-design files. |
| `.devtrail/**` orphans | 7+7+7+5 deleted | DEVTRAIL.md + devtrail-* skills/workflows/rules. |
| `.specify/memory/constitution.md` | 2 lines | `DevTrail`→`StrayMark`. |
| `.specify/starter/IDEA.md` | 3 lines | Same + "formerly DevTrail" historical clarifier. |

## Decisions & Tradeoffs

- **Bundled rebrand + framework update in one PR.** Alternative: split into rebrand-only commit followed by update-framework commit. Chose the bundle because (a) `straymark repair` and `update-framework` together regenerate the new content in one pass, (b) the operator already confirmed scope. Commit history within the branch is still segmented by phase.
- **Did not edit historical AILOGs/TES/AIDEC/SBOM** under `.straymark/07-ai-audit/` and `.straymark/04-testing/` even though they mention "DevTrail". These are point-in-time records and should remain faithful to the naming at the time of writing.
- **Discarded stash entirely** after confirming its substantive content (cognitive-complexity heuristic) is already upstream in fw-4.17.0. Avoided manual conflict resolution against renamed paths.
- **Kept one "formerly DevTrail" mention** in `.specify/starter/IDEA.md` to preserve the project's origin story.

## Verification

- `cargo check` — clean.
- `cargo test` — 29 passed, 0 failed.
- `straymark status` — Framework fw-4.17.0, CLI cli-3.15.0, all 17 structure items OK.
- `straymark validate` — 0 errors, 1 preexisting warning unrelated to rebrand (`AILOG-2026-04-04-001-about-subcommand.md` has no traceability links — predates this AILOG).
- `grep -r "devtrail\|DevTrail"` over tracked non-historical files — only the intentional "formerly DevTrail" clarifier in `.specify/starter/IDEA.md` remains.

## Risks & Follow-ups

- **risk_level: medium** because the change touches every directive file an AI agent reads when entering this repo. A mis-edit could cause future agents to misroute documents or look for nonexistent paths. Mitigated by `straymark validate` and the cargo test suite, but a human review is recommended (`review_required: true`).
- New Charter machinery (`.straymark/charters/`, `straymark charter new`) is now available but not adopted by any in-flight work. Future sessions can opt in.
- China regulatory scope is **not** enabled (`.straymark/config.yml` `regional_scope` unset). The China-specific templates and governance files are present but inert.
- No follow-up issues required; rebrand is self-contained.
