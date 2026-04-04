# Data Model: Core CLI MVP

**Phase**: 1 — Design & Contracts  
**Date**: 2026-04-02

## Entities

### FileReport (from arborist-metrics)

Represents analysis results for a single source file. Owned by the library;
the CLI consumes it read-only.

| Field | Type | Description |
|-------|------|-------------|
| path | file path | Analyzed file path (or "stdin" for piped input) |
| language | string | Detected programming language |
| functions | list of FunctionMetrics | Per-function analysis results |
| file_cognitive | integer | Sum of cognitive complexity across all functions |
| file_cyclomatic | integer | Sum of cyclomatic complexity across all functions |
| file_sloc | integer | Total source lines of code in file |

### FunctionMetrics (from arborist-metrics)

Represents metrics for a single function within a file.

| Field | Type | Description |
|-------|------|-------------|
| name | string | Function/method name |
| line_start | integer | First line of function |
| line_end | integer | Last line of function |
| cognitive | integer | Cognitive complexity score |
| cyclomatic | integer | Cyclomatic complexity score |
| sloc | integer | Source lines of code in function |

### AnalysisConfig (from arborist-metrics)

User-provided configuration that controls analysis behavior.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| cognitive_threshold | optional integer | none | Flag functions exceeding this value |
| include_methods | boolean | true | Include method-level analysis |

### CliArgs (CLI-native)

Parsed command-line arguments. Maps to clap derive struct.

| Field | Type | Default | CLI Flag |
|-------|------|---------|----------|
| paths | list of paths | empty (stdin) | positional |
| format | enum (table/json/csv) | table | `--format` |
| language | optional string | none (auto-detect) | `--language` |
| threshold | optional integer | none | `--threshold` |
| exceeds_only | boolean | false | `--exceeds-only` |
| sort_by | optional enum (cognitive/cyclomatic/sloc/name) | none | `--sort` |
| top_n | optional integer | none | `--top` |
| languages_filter | optional list of strings | none | `--languages` |
| gitignore | boolean | false | `--gitignore` |
| no_methods | boolean | false | `--no-methods` |

### FlatFunction (CLI-native)

Denormalized view of a function with its source file context. Used when
`--sort` or `--top` flatten results across multiple files.

| Field | Type | Description |
|-------|------|-------------|
| name | string | Function name |
| file_path | file path | Source file this function belongs to |
| language | string | Detected language of source file |
| line_start | integer | First line of function |
| line_end | integer | Last line of function |
| cognitive | integer | Cognitive complexity score |
| cyclomatic | integer | Cyclomatic complexity score |
| sloc | integer | Source lines of code |

### ArboristError (CLI-native)

Error type for all CLI failure modes.

| Variant | Source | Exit Code |
|---------|--------|-----------|
| Io | std::io::Error | 2 |
| Analysis | arborist-metrics errors | 2 |
| NoLanguage | stdin without --language | 2 |
| InvalidArgument | bad flag combination | 2 |

## Relationships

```text
CliArgs ──parses-to──→ AnalysisConfig (threshold, include_methods)
CliArgs ──drives──→ Traversal (paths, languages_filter, gitignore)
CliArgs ──selects──→ OutputFormatter (format, sort_by, top_n, exceeds_only)

arborist-metrics(file, config) ──produces──→ FileReport
FileReport ──contains──→ [FunctionMetrics]

OutputFormatter(FileReport[], CliArgs) ──renders──→ stdout
ArboristError ──maps-to──→ ExitCode (always 2)
```

## State Transitions

N/A — stateless CLI. Each invocation is a single pass:
`parse args → resolve inputs → analyze → format → output → exit`.

## Validation Rules

- If `paths` is empty and stdin is not a pipe → error (nothing to analyze)
- If stdin is a pipe and `--language` is not set → `ArboristError::NoLanguage`
- `--exceeds-only` without `--threshold` → treat as no-op (show all, nothing flagged)
- `--sort` or `--top` with single file → still works (sorts functions within file, flat output)
- `--languages` filter only applies to directory traversal, ignored for explicit file paths
