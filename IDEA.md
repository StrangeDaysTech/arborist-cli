# arborist-cli — Project Idea

## What is this?

A command-line interface for [arborist-metrics](https://crates.io/crates/arborist-metrics),
the multi-language code complexity library powered by tree-sitter.

`arborist-cli` gives developers and AI agents instant access to cognitive complexity,
cyclomatic complexity, and SLOC metrics — no Rust code required, no build integration
needed. It also serves as a **reference implementation** showcasing the full surface area
of the `arborist-metrics` API.

## Why?

When we integrated `arborist-metrics` into DevTrail, we realized that the library's
value isn't limited to Rust projects embedding it as a dependency. There are two
underserved audiences:

1. **AI agents** that need to evaluate code complexity as part of a larger workflow
   (e.g., deciding whether a change needs documentation, review, or refactoring) but
   shouldn't need to build a custom Rust tool to do so.

2. **Developers** who want quick, one-off complexity checks during code review, PR
   triage, or exploratory analysis — without committing to a full CI pipeline
   integration.

The DevTrail CLI (`devtrail`) already has complexity awareness, but its scope is
documentation governance — it's overkill when you just want to ask "how complex is
this file?" or "which functions in this directory exceed threshold X?".

## Capabilities (based on arborist-metrics 0.1.x API)

The CLI should expose every capability of the library through a natural command-line
interface:

### Core analysis

| Library API | CLI equivalent |
|------------|----------------|
| `analyze_file(path)` | `arborist <file>` |
| `analyze_file_with_config(path, config)` | `arborist <file> --threshold 15 --no-methods` |
| `analyze_source(source, language)` | `echo "code" \| arborist --language rust` |
| `analyze_source_with_config(source, lang, config)` | `echo "code" \| arborist --language rust --threshold 10` |

### Multi-file analysis (CLI-native, not in library)

The library operates on single files/strings. The CLI should add directory traversal:

- `arborist src/` — recursively analyze all recognized files
- `arborist src/ --languages rust,python` — filter by language
- `arborist . --gitignore` — respect `.gitignore` patterns

### Output formats

| Format | Flag | Use case |
|--------|------|----------|
| Human-readable table | (default) | Terminal usage |
| JSON | `--format json` | AI agents, piping to `jq` |
| CSV | `--format csv` | Spreadsheets, data analysis |

The JSON output should match the library's `FileReport` serialization format for
consistency.

### Filtering and thresholds

- `--threshold <N>` — set cognitive complexity threshold, flag exceeding functions
- `--sort <metric>` — sort output by `cognitive`, `cyclomatic`, `sloc`, or `name`
- `--top <N>` — show only the N most complex functions
- `--exceeds-only` — only show functions that exceed the threshold

### Configuration

The library exposes `AnalysisConfig` with:
- `cognitive_threshold: Option<u64>` → `--threshold`
- `include_methods: bool` → `--no-methods` to exclude

### Exit codes

For CI/scripting integration:
- `0` — success, no functions exceed threshold (or no threshold set)
- `1` — success, but one or more functions exceed threshold
- `2` — error (file not found, unrecognized language, etc.)

## Design principles

1. **Zero configuration** — works out of the box on any codebase with recognized files
2. **Composable** — plays well with Unix pipes, `jq`, `grep`, `xargs`
3. **AI-agent friendly** — JSON output is structured, predictable, and matches the
   library's serialization format
4. **Reference implementation** — demonstrates every public API in `arborist-metrics`,
   serving as living documentation of what the library can do
5. **Fast** — tree-sitter parsing is already fast; the CLI should add minimal overhead

## What this is NOT

- Not a linter (doesn't enforce rules, just reports metrics)
- Not a CI gate (use the library directly for that, or check exit codes)
- Not a replacement for DevTrail (DevTrail uses complexity as *input* to documentation
  decisions; this CLI just reports the raw metrics)

## Relationship to arborist-metrics

```
arborist-metrics (library crate, crates.io)
    ↑ depends on
arborist-cli (binary crate, this project)
```

The CLI imports `arborist-metrics` with `features = ["all"]` and adds:
- `clap` for argument parsing
- `serde_json` for JSON output
- Directory traversal (walkdir or ignore crate)
- Terminal formatting (colored output, tables)

The library repo (`arborist`) maintains its **no-cli constitution** — all CLI
concerns live here.

## Example sessions

```bash
# Quick check on a single file
$ arborist src/main.rs
src/main.rs (Rust) — 3 functions, 45 SLOC

  Function          Lines    Cognitive  Cyclomatic  SLOC
  main              1-20     3          4           18
  process_input     22-38    8          6           15
  validate          40-48    2          3           8

# Find the most complex functions in a project
$ arborist src/ --top 5 --sort cognitive
  Function                    File                    Cognitive
  parse_expression            src/parser.rs           24
  resolve_dependencies        src/resolver.rs         19
  analyze_scope               src/analyzer.rs         15
  transform_ast               src/transform.rs        12
  validate_config             src/config.rs           11

# AI agent usage — structured JSON for programmatic consumption
$ arborist src/auth/ --format json --threshold 10
[
  {
    "path": "src/auth/login.rs",
    "language": "Rust",
    "functions": [...],
    "file_cognitive": 23,
    "file_cyclomatic": 18,
    "file_sloc": 89
  }
]

# Pipe source code directly
$ cat src/main.rs | arborist --language rust --format json

# CI gate — exit code 1 if any function exceeds threshold
$ arborist src/ --threshold 15 --exceeds-only && echo "OK" || echo "COMPLEX"
```

## Open questions

- Should we support a config file (`.arborist.toml`) for default threshold/language settings?
- Should `--watch` mode be in scope for live development feedback?
- Should we add a `--diff` mode that only analyzes files changed in a git diff?
- What's the minimum Rust edition / MSRV? (likely same as arborist-metrics: edition 2024)

---

*This document captures the initial idea and scope. Implementation planning will
follow using SpecKit once the idea is reviewed and refined.*
