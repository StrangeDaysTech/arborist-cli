---
id: TES-2026-04-03-001
title: CLI integration test coverage for MVP
status: accepted
created: 2026-04-03
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [8]
test_type: integration
test_framework: assert_cmd + predicates
total_tests: 22
tags: [testing, integration, cli, coverage, mvp]
related: [AILOG-2026-04-03-001, AILOG-2026-04-02-002]
---

# TES: CLI Integration Test Coverage for MVP

## Summary

Documents the integration test suite for `arborist-cli` MVP. 22 tests across 7 modules cover all CLI features defined in the spec.

## Test Matrix

| Module | Tests | Coverage Area |
|--------|-------|---------------|
| `single_file` | 5 | Single file analysis, error handling, partial parsing |
| `directory` | 4 | Recursive traversal, gitignore, language filter, empty results |
| `exit_codes` | 4 | Exit 0/1/2 semantics, error precedence |
| `filtering` | 8 | `--threshold`, `--exceeds-only`, `--no-methods`, `--sort`, `--top` |
| `output_formats` | 6 | JSON schema/fields, CSV header/rows/empty |
| `stdin` | 2 | Stdin with `--language`, missing language error |
| `multi_input` | 1 | Multiple positional file arguments |

## Spec Task Mapping

| Spec Task | Test Function | Status |
|-----------|---------------|--------|
| T011 | `single_file::simple_file_table_output` | Pass |
| T012 | `single_file::complex_file_metrics` | Pass |
| T013 | `single_file::nonexistent_file_error` | Pass |
| T014 | `single_file::unknown_extension_error` | Pass |
| T014b | `single_file::syntax_error_partial_parsing` | Pass |
| T020 | `output_formats::json_output_valid_schema` | Pass |
| T021 | `stdin::stdin_json_output` | Pass |
| T022 | `stdin::stdin_no_language_error` | Pass |
| T023 | `output_formats::json_output_function_fields` | Pass |
| T027 | `directory::directory_recursive_analysis` | Pass |
| T028 | `directory::directory_gitignore_excludes` | Pass |
| T029 | `directory::directory_language_filter` | Pass |
| T030 | `directory::directory_no_recognized_files` | Pass |
| T031 | `multi_input::multi_file_input` | Pass |
| T036 | `filtering::threshold_flags_exceeding` | Pass |
| T037 | `exit_codes::exit_code_1_threshold_exceeded` | Pass |
| T038 | `exit_codes::exit_code_0_nothing_exceeds` | Pass |
| T039 | `filtering::threshold_exceeds_only` | Pass |
| T040 | `filtering::exceeds_only_without_threshold` | Pass |
| T041 | `exit_codes::exit_code_error_precedence_over_threshold` | Pass |
| T041b | `filtering::no_methods_flag` | Pass |
| T047 | `filtering::sort_cognitive_descending` | Pass |
| T048 | `filtering::sort_name_ascending` | Pass |
| T049 | `filtering::top_n_limits_results` | Pass |
| T050 | `filtering::top_n_greater_than_results` | Pass |
| T056 | `output_formats::csv_output_header` | Pass |
| T057 | `output_formats::csv_directory_multiple_rows` | Pass |
| T058 | `output_formats::csv_no_functions_header_only` | Pass |
| T067 | `exit_codes::exit_code_error_precedence_over_threshold` | Pass |

## Test Infrastructure

- **Framework**: `assert_cmd` (CLI binary invocation) + `predicates` (output assertions)
- **Fixtures**: `tests/fixtures/` — `simple.rs`, `complex.rs`, `no_functions.rs`, `syntax_error.rs`, `simple.py`, `nested_project/`
- **Run command**: `cargo test`

## Gaps and Future Work

- No performance/benchmark tests yet
- No fuzz testing for malformed input files
- No test for `--format table` column alignment edge cases
- Stdin with large files not tested

---

<!-- Template: DevTrail | https://strangedays.tech -->
