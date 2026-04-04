# Implementation Plan: Core CLI MVP

**Branch**: `001-core-cli-mvp` | **Date**: 2026-04-02 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-core-cli-mvp/spec.md`

## Summary

Build the arborist-cli binary crate — a thin CLI wrapper around `arborist-metrics`
that exposes cognitive complexity, cyclomatic complexity, and SLOC metrics through
a composable command-line interface. Supports single file, stdin, and directory
analysis with three output formats (table, JSON, CSV), threshold-based filtering,
sorting, and meaningful exit codes for CI integration.

## Technical Context

**Language/Version**: Rust, Edition 2024  
**Primary Dependencies**: `arborist-metrics` (analysis), `clap` (argument parsing), `serde`/`serde_json` (JSON output), `ignore` (directory traversal + gitignore), `thiserror` (error types)  
**Storage**: N/A — stateless CLI, no persistence  
**Testing**: `cargo test` + `assert_cmd` + `predicates` (integration), standard `#[test]` (unit)  
**Target Platform**: Cross-platform (Linux, macOS, Windows) — distributed via `cargo install`  
**Project Type**: CLI (binary crate)  
**Performance Goals**: 1,000-file project analyzed in <10 seconds on standard hardware  
**Constraints**: Minimal dependency footprint, zero mandatory configuration, sub-second single-file analysis  
**Scale/Scope**: Single-user CLI tool, projects up to ~100k LOC typical usage

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate Question | Status |
|-----------|--------------|--------|
| I. Zero Configuration | Does this feature require configuration to use? | ✅ PASS — zero config, all options via CLI flags with sensible defaults |
| II. Composable CLI | Does output remain parseable? No interactive prompts? | ✅ PASS — three structured formats, exit codes, TTY detection, no prompts |
| III. AI-Agent Friendly | Does JSON schema change or break? | ✅ PASS — JSON matches `FileReport` serialization from arborist-metrics |
| IV. Reference Implementation | Does this re-implement library logic? | ✅ PASS — CLI delegates all analysis to arborist-metrics, adds only CLI concerns |
| V. Fast & Minimal Overhead | Does this add unnecessary runtime dependencies? | ✅ PASS — 5 core dependencies, all justified (clap, serde, ignore, thiserror + table formatting TBD) |

**Rust Idioms gate**: `thiserror` for errors, `?` propagation, no `unwrap()`, clippy -D warnings, rustfmt enforced.  
**Testing gate**: Integration tests with `assert_cmd` mandatory for all CLI flags, unit tests where formatting logic warrants.

All gates pass. Proceeding to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-core-cli-mvp/
├── plan.md              # This file
├── research.md          # Phase 0: dependency research & decisions
├── data-model.md        # Phase 1: entity definitions
├── contracts/           # Phase 1: CLI interface contract
│   └── cli-interface.md
├── quickstart.md        # Phase 1: getting started guide
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2 (/speckit.tasks output)
```

### Source Code (repository root)

```text
src/
├── main.rs              # Entry point: clap parsing → run → ExitCode
├── cli.rs               # Clap struct definitions and argument validation
├── analysis.rs          # Orchestration: file/stdin/directory → arborist-metrics calls
├── output/
│   ├── mod.rs           # Output dispatcher (format selection)
│   ├── table.rs         # Human-readable table formatting
│   ├── json.rs          # JSON output (serde passthrough from FileReport)
│   └── csv.rs           # CSV output
├── traversal.rs         # Directory walking with ignore crate, language filtering
├── error.rs             # thiserror error types, exit code mapping
└── lib.rs               # Public API surface (for potential library consumers)

tests/
├── cli/                 # Integration tests (assert_cmd)
│   ├── single_file.rs
│   ├── stdin.rs
│   ├── directory.rs
│   ├── output_formats.rs
│   ├── exit_codes.rs
│   ├── filtering.rs     # --threshold, --exceeds-only, --sort, --top
│   └── multi_input.rs   # Multiple positional args
└── fixtures/            # Small source files for testing
    ├── simple.rs
    ├── complex.rs
    ├── simple.py
    ├── no_functions.rs
    ├── syntax_error.rs
    └── nested_project/
        ├── .gitignore
        ├── src/
        │   ├── main.rs
        │   └── lib.rs
        └── ignored/
            └── generated.rs
```

**Structure Decision**: Single binary crate with flat module layout. The `output/`
subdirectory groups the three formatters. No `models/` or `services/` directories —
domain types come from `arborist-metrics` and the CLI layer is thin enough to stay flat.

## Complexity Tracking

No constitution violations — table left empty intentionally.
