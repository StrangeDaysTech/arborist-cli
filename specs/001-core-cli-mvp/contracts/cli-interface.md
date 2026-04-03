# CLI Interface Contract: arborist-cli

**Phase**: 1 — Design & Contracts  
**Date**: 2026-04-02

## Command Signature

```text
arborist [OPTIONS] [PATHS...]
```

If no PATHS and stdin is a pipe, reads from stdin.
If no PATHS and stdin is not a pipe, prints help and exits.

## Positional Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| PATHS | file or directory paths | no | One or more files/directories to analyze. Mixed allowed. |

## Options

| Flag | Type | Default | Description |
|------|------|---------|-------------|
| `--format <FORMAT>` | table / json / csv | table | Output format |
| `--language <LANG>` | string | auto-detect | Required for stdin; language name (e.g., rust, python) |
| `--threshold <N>` | integer | none | Cognitive complexity threshold |
| `--exceeds-only` | boolean | false | Show only functions exceeding threshold |
| `--sort <METRIC>` | cognitive / cyclomatic / sloc / name | none | Sort results by metric (descending for numeric, ascending for name) |
| `--top <N>` | integer | none | Show only top N results |
| `--languages <LIST>` | comma-separated | none | Filter directory traversal by language |
| `--gitignore` | boolean | false | Respect .gitignore patterns |
| `--no-methods` | boolean | false | Exclude method-level analysis |
| `-h, --help` | | | Print help |
| `-V, --version` | | | Print version |

## Output Formats

### Table (default, TTY)

```text
src/main.rs (Rust) — 3 functions, 45 SLOC

  Function          Lines    Cognitive  Cyclomatic  SLOC
  main              1-20     3          4           18
  process_input     22-38    8 ⚠       6           15
  validate          40-48    2          3           8
```

- `⚠` marker on functions exceeding `--threshold`
- File header with path, language, function count, total SLOC
- No colors or decorative characters when stdout is not a TTY

### Table with --sort/--top (flat mode)

```text
  Function                    File                    Cognitive
  parse_expression            src/parser.rs           24
  resolve_dependencies        src/resolver.rs         19
  analyze_scope               src/analyzer.rs         15
```

- Flat function list across all files
- File column added for cross-file context
- Only the sorted metric column shown (plus function and file)
- When `--threshold` is active in flat mode, the `⚠` marker appears on
  functions exceeding the threshold, regardless of which metric is sorted
- `--exceeds-only` also applies in flat mode, filtering the ranked list

### JSON

```json
[
  {
    "path": "src/main.rs",
    "language": "Rust",
    "functions": [
      {
        "name": "main",
        "line_start": 1,
        "line_end": 20,
        "cognitive": 3,
        "cyclomatic": 4,
        "sloc": 18
      }
    ],
    "file_cognitive": 13,
    "file_cyclomatic": 13,
    "file_sloc": 41
  }
]
```

- Always an array of FileReport objects (even for single file)
- Schema matches `arborist-metrics` `FileReport` serde serialization
- `--sort`/`--top`/`--exceeds-only` filter the output before serialization
  but preserve the FileReport structure (empty function arrays for files
  with all functions filtered out are omitted)

### CSV

```csv
file,language,function,line_start,line_end,cognitive,cyclomatic,sloc
src/main.rs,Rust,main,1,20,3,4,18
src/main.rs,Rust,process_input,22,38,8,6,15
```

- Always flat (one row per function)
- Header row always present
- Filters applied before output

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success — no threshold set, or no functions exceed threshold |
| 1 | Success — one or more functions exceed threshold |
| 2 | Error — file not found, unrecognized language, invalid args, etc. |

Precedence: error (2) > threshold exceeded (1) > success (0).
If analyzing multiple files and some fail while others exceed threshold,
exit code is 2.

## Error Output (stderr)

```text
error: file not found: src/nonexistent.rs
error: unrecognized language for file: data.xyz
error: --language is required when reading from stdin
warning: skipping unreadable file: src/private.rs (permission denied)
```

- Errors prefixed with `error:`, warnings with `warning:`
- Per-file failures during directory traversal are warnings (analysis continues)
- Fatal errors (no valid input, invalid args) are errors (immediate exit)
