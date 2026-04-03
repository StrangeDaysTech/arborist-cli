# Research: Core CLI MVP

**Phase**: 0 — Outline & Research  
**Date**: 2026-04-02

## R-001: Table Formatting Crate

**Decision**: Use `comfy-table` for human-readable table output.

**Rationale**: `comfy-table` provides dynamic column sizing, TTY-aware
formatting (auto-disables styling when piped), and UTF-8 border support
with minimal API surface. It's the most popular Rust table formatting
crate with active maintenance.

**Alternatives considered**:
- `tabled` — more features but heavier API, macro-driven approach adds
  compile time for features we don't need
- `prettytable-rs` — unmaintained since 2021
- Manual formatting with `format!` — viable for simple cases but
  fragile for dynamic column widths across varying function name lengths

## R-002: Directory Traversal with `ignore` Crate

**Decision**: Use the `ignore` crate (from BurntSushi/ripgrep) for
directory traversal.

**Rationale**: `ignore` natively supports `.gitignore` patterns,
provides parallel directory walking, and handles symlinks safely.
It's battle-tested in ripgrep and aligns with the `--gitignore` flag
requirement.

**Alternatives considered**:
- `walkdir` — simpler API but no gitignore support (would need
  additional crate like `gitignore` for that)
- `glob` — pattern matching only, not recursive traversal
- Manual `std::fs::read_dir` — too low-level, would re-implement
  what `ignore` already does

## R-003: Color and TTY Detection

**Decision**: Use `comfy-table`'s built-in TTY awareness for table
formatting. For threshold flagging, use ANSI escape codes directly
(no color crate needed for the MVP's limited color needs).

**Rationale**: The only color usage is highlighting threshold-exceeding
functions in table output. A full color crate adds a dependency for
2-3 ANSI codes. `comfy-table` already handles style suppression when
piped.

**Alternatives considered**:
- `colored` — popular but overkill for 2-3 escape sequences
- `owo-colors` — zero-cost abstractions but still an extra dependency
- Revisit if color needs grow beyond threshold flagging post-MVP

## R-004: CSV Output Strategy

**Decision**: Use the `csv` crate for CSV output.

**Rationale**: CSV has edge cases (quoting, escaping, commas in values)
that are easy to get wrong manually. The `csv` crate handles RFC 4180
compliance and integrates with serde for zero-effort serialization.
Small dependency footprint.

**Alternatives considered**:
- Manual `format!` with comma joining — fragile, breaks on values
  containing commas or quotes
- `serde_csv` — doesn't exist as a standalone; the `csv` crate IS
  the serde-integrated solution

## R-005: Error Type Design

**Decision**: Single `ArboristError` enum with `thiserror` derive,
mapping to exit codes at the `main()` boundary.

**Rationale**: Constitution mandates `thiserror`, `?` propagation,
and `ExitCode` return from main. A single error enum keeps the thin
CLI layer simple — no error hierarchy needed when there's one binary
with ~5 failure modes.

**Variants**:
- `Io(#[from] std::io::Error)` — file not found, permission denied
- `Analysis(String)` — arborist-metrics errors (unrecognized language,
  parse failures)
- `NoLanguage` — stdin without `--language`
- `InvalidArgument(String)` — bad flag combinations

**Exit code mapping**:
- All `ArboristError` variants → exit code 2
- Threshold exceeded (not an error) → exit code 1
- Success → exit code 0

## R-006: arborist-metrics API Surface (0.1.x)

**Decision**: Wrap the full public API.

**Key functions to expose via CLI**:
- `analyze_file(path)` → `arborist <file>`
- `analyze_file_with_config(path, config)` → `arborist <file> --threshold N`
- `analyze_source(source, language)` → `echo code | arborist --language rust`
- `analyze_source_with_config(source, lang, config)` → combined flags
- `AnalysisConfig { cognitive_threshold, include_methods }` → `--threshold`, `--no-methods`
- `FileReport` serialization → `--format json` passthrough

**Note**: The CLI adds directory traversal (not in library), multiple
positional arguments, output formatting, sorting, and top-N filtering
as CLI-native features.

## R-007: Clap Configuration Strategy

**Decision**: Use clap derive macros with a single `Args` struct.

**Rationale**: The CLI has a flat command structure (no subcommands).
All flags operate on the same analysis flow. A single derive struct
maps cleanly to `AnalysisConfig` + output options + traversal options.

**Structure**:
- Positional: `paths: Vec<PathBuf>` (files and/or directories)
- `--format`: enum {table, json, csv}
- `--language`: Option<String> (required for stdin)
- `--threshold`: Option<u64>
- `--exceeds-only`: bool
- `--sort`: Option<enum {cognitive, cyclomatic, sloc, name}>
- `--top`: Option<usize>
- `--languages`: Option<Vec<String>> (directory filter)
- `--gitignore`: bool
- `--no-methods`: bool

## All NEEDS CLARIFICATION: Resolved

No unknowns remain. All technical decisions are documented above.
