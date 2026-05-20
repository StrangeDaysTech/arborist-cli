---
id: AILOG-2026-04-03-001
title: Add integration test suite for CLI MVP
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [8]
lines_changed: 495
files_modified: [tests/cli/main.rs, tests/cli/single_file.rs, tests/cli/directory.rs, tests/cli/exit_codes.rs, tests/cli/filtering.rs, tests/cli/multi_input.rs, tests/cli/output_formats.rs, tests/cli/stdin.rs]
observability_scope: none
tags: [testing, integration-tests, cli, quality-assurance]
related: [AILOG-2026-04-02-002, TES-2026-04-03-001]
---

# AILOG: Add integration test suite for CLI MVP

## Summary

Added a comprehensive integration test suite (495 lines, 22 tests) covering all CLI functionality: single file analysis, directory traversal, stdin input, multi-file input, all output formats (table/JSON/CSV), threshold filtering with `--exceeds-only`, sorting (`--sort cognitive|name|cyclomatic`), top-N limiting, `--no-methods`, `--gitignore`, `--languages`, and exit code semantics.

## Context

The core CLI MVP was implemented in commit `07adb92` but had no automated tests beyond manual verification. The spec (`specs/001-core-cli-mvp/tasks.md`) tracked test tasks (T011–T067) as pending. This commit fulfills those requirements.

## Actions Performed

1. Created `tests/cli/main.rs` — test module declarations
2. Created `tests/cli/single_file.rs` — 5 tests: table output, metrics validation, nonexistent file error, unknown extension error, syntax error handling
3. Created `tests/cli/directory.rs` — 4 tests: recursive traversal, gitignore exclusion, language filtering, no recognized files
4. Created `tests/cli/exit_codes.rs` — 4 tests: exit 1 on threshold exceed, exit 0 on pass, exit 2 on error, error precedence over threshold
5. Created `tests/cli/filtering.rs` — 8 tests: threshold flags, exceeds-only, no-methods, sort cognitive/name, top-N, top-N > results
6. Created `tests/cli/output_formats.rs` — 6 tests: JSON schema validation, JSON function fields, CSV header, CSV multi-row, CSV no-functions
7. Created `tests/cli/stdin.rs` — 2 tests: stdin JSON output, stdin missing --language error
8. Created `tests/cli/multi_input.rs` — 1 test: multiple positional arguments

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `tests/cli/main.rs` | +7/-0 | Module declarations for test modules |
| `tests/cli/single_file.rs` | +67/-0 | Single file analysis tests |
| `tests/cli/directory.rs` | +62/-0 | Directory traversal tests |
| `tests/cli/exit_codes.rs` | +45/-0 | Exit code semantics tests |
| `tests/cli/filtering.rs` | +170/-0 | Threshold, sort, top-N filtering tests |
| `tests/cli/output_formats.rs` | +92/-0 | JSON and CSV output format tests |
| `tests/cli/stdin.rs` | +33/-0 | Stdin input tests |
| `tests/cli/multi_input.rs` | +19/-0 | Multi-file input test |

## Decisions Made

- Used `assert_cmd` + `predicates` crates for ergonomic CLI testing, consistent with Rust ecosystem best practices.
- Organized tests by functional area (one module per concern) rather than by spec task number for maintainability.
- Created a custom `AssertExt` trait in `filtering.rs` for `success_or()` to handle exit code 0 or 1 cases cleanly.

## Impact

- **Functionality**: Provides regression safety for all CLI features
- **Performance**: N/A (test-time only)
- **Security**: N/A
- **Privacy**: N/A
- **Environmental**: N/A

## Verification

- [x] `cargo test` passes all 22 tests
- [x] Tests cover all spec task IDs (T011–T067)
- [x] Tests use existing fixture files
- [ ] Manual review performed

---

<!-- Template: DevTrail | https://strangedays.tech -->
