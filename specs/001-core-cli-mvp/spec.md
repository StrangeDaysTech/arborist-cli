# Feature Specification: Core CLI MVP

**Feature Branch**: `001-core-cli-mvp`  
**Created**: 2026-04-02  
**Status**: Draft  
**Input**: User description: "CLI for arborist-metrics exposing cognitive/cyclomatic complexity and SLOC metrics for developers and AI agents"

## Clarifications

### Session 2026-04-02

- Q: If a file fails during directory traversal (permissions, corrupt parsing), what should happen? → A: Continue analyzing remaining files, report per-file errors to stderr, exit code 2 at the end.
- Q: Can the user pass multiple files/directories as positional arguments? → A: Yes, accept multiple files and directories in the same invocation.
- Q: How do --sort/--top interact with default file-grouped output? → A: Without --sort/--top, output is grouped by file. With --sort or --top, output becomes a flat function list across all files.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Single File Complexity Check (Priority: P1)

A developer wants to quickly check the complexity of a specific source file
during code review or exploratory analysis. They run a single command and get
an immediate, human-readable summary of all functions in the file with their
cognitive complexity, cyclomatic complexity, and SLOC.

**Why this priority**: This is the most fundamental use case — the atomic unit
of analysis that all other features build on. Without single-file analysis,
nothing else works.

**Independent Test**: Can be fully tested by running `arborist <file>` on any
recognized source file and verifying the table output contains function-level
metrics.

**Acceptance Scenarios**:

1. **Given** a valid Rust source file with 3 functions, **When** the user runs
   `arborist src/main.rs`, **Then** a table is displayed showing each function's
   name, line range, cognitive complexity, cyclomatic complexity, and SLOC.
2. **Given** a valid Python source file, **When** the user runs
   `arborist script.py`, **Then** the language is auto-detected and the same
   metrics table is displayed.
3. **Given** a file with an unrecognized extension, **When** the user runs
   `arborist unknown.xyz`, **Then** an error message is printed to stderr and
   the tool exits with code 2.

---

### User Story 2 - AI Agent JSON Consumption (Priority: P1)

An AI agent needs to programmatically evaluate the complexity of source code
as part of a larger workflow (e.g., deciding whether a change needs extra
documentation or review). The agent passes a file or piped source code and
receives structured JSON output matching the library's serialization format.

**Why this priority**: AI agents are a first-class audience. JSON output is
the primary contract for programmatic consumers and must be available from
the start.

**Independent Test**: Can be fully tested by running
`arborist <file> --format json` and validating the output parses as JSON with
expected fields matching `FileReport` structure.

**Acceptance Scenarios**:

1. **Given** a source file, **When** the user runs
   `arborist src/main.rs --format json`, **Then** valid JSON is written to
   stdout containing file path, language, function-level metrics, and
   file-level aggregates.
2. **Given** source code piped via stdin, **When** the user runs
   `echo "fn main() {}" | arborist --language rust --format json`, **Then**
   valid JSON is produced with the same schema as file-based analysis.
3. **Given** JSON output from arborist, **When** processed with `jq`, **Then**
   individual fields (function names, complexity scores) are extractable
   without additional parsing.

---

### User Story 3 - Directory-Wide Analysis (Priority: P2)

A developer or AI agent wants to find the most complex functions across an
entire project directory. They run arborist on a directory and get aggregated
results, optionally sorted and filtered by complexity metrics.

**Why this priority**: Directory traversal multiplies the tool's value from
single-file checks to project-wide insights. Depends on single-file analysis
working correctly.

**Independent Test**: Can be fully tested by running `arborist src/` on a
directory with multiple recognized files and verifying all files are analyzed
recursively.

**Acceptance Scenarios**:

1. **Given** a directory with Rust and Python files, **When** the user runs
   `arborist src/`, **Then** all recognized files are analyzed recursively
   and results are displayed.
2. **Given** a directory with a `.gitignore`, **When** the user runs
   `arborist . --gitignore`, **Then** files matching gitignore patterns are
   excluded from analysis.
3. **Given** a directory with multiple languages, **When** the user runs
   `arborist src/ --languages rust,python`, **Then** only Rust and Python
   files are analyzed.

---

### User Story 4 - Threshold-Based Filtering (Priority: P2)

A developer or CI pipeline wants to identify functions that exceed a
complexity threshold. They set a threshold and optionally filter to show
only exceeding functions, using exit codes to signal pass/fail.

**Why this priority**: Threshold filtering transforms the tool from a
reporter into a decision-making aid for both humans and automated pipelines.

**Independent Test**: Can be fully tested by running
`arborist <file> --threshold 10 --exceeds-only` and verifying only functions
above the threshold appear, with exit code 1 when any exceed.

**Acceptance Scenarios**:

1. **Given** a file with functions of varying complexity, **When** the user
   runs `arborist src/main.rs --threshold 10`, **Then** all functions are
   shown and those exceeding the threshold are visually flagged.
2. **Given** a file with functions exceeding threshold, **When** the user
   runs `arborist src/main.rs --threshold 5 --exceeds-only`, **Then** only
   exceeding functions are displayed and exit code is 1.
3. **Given** a file with no functions exceeding threshold, **When** the user
   runs `arborist src/main.rs --threshold 50`, **Then** all functions are
   shown unflagged and exit code is 0.

---

### User Story 5 - Sorting and Top-N Results (Priority: P3)

A developer wants to find the most complex functions in a project quickly.
They sort results by a metric and optionally limit to the top N entries.

**Why this priority**: Sorting and top-N are convenience features that build
on directory traversal and filtering. Useful but not essential for core value.

**Independent Test**: Can be fully tested by running
`arborist src/ --sort cognitive --top 5` and verifying output is ordered by
cognitive complexity descending and limited to 5 entries.

**Acceptance Scenarios**:

1. **Given** a directory with many functions, **When** the user runs
   `arborist src/ --sort cognitive`, **Then** results are ordered by cognitive
   complexity descending.
2. **Given** a directory with many functions, **When** the user runs
   `arborist src/ --top 5 --sort cyclomatic`, **Then** only the 5 functions
   with highest cyclomatic complexity are shown.

---

### User Story 6 - CSV Output for Data Analysis (Priority: P3)

A developer or data analyst wants to export complexity metrics to a
spreadsheet or data analysis tool. They request CSV output and pipe or
redirect it to a file.

**Why this priority**: CSV is a convenience output format. JSON covers
programmatic needs; CSV serves a niche audience for spreadsheet workflows.

**Independent Test**: Can be fully tested by running
`arborist src/ --format csv` and verifying output is valid CSV with headers.

**Acceptance Scenarios**:

1. **Given** a source file, **When** the user runs
   `arborist src/main.rs --format csv`, **Then** CSV output with headers
   is written to stdout.
2. **Given** CSV output, **When** opened in a spreadsheet application,
   **Then** data is correctly parsed into columns.

---

### Edge Cases

- What happens when stdin is provided but `--language` is not specified?
  The tool MUST print an error to stderr and exit with code 2.
- What happens when a directory contains no recognized files?
  The tool MUST print an informational message to stderr and exit with code 0.
- What happens when a file contains no functions (e.g., only constants)?
  The tool MUST output an empty result set (empty table, empty JSON array,
  empty CSV with headers only).
- What happens when `--top` is greater than the number of available results?
  The tool MUST display all available results without error.
- What happens when the file is valid but contains syntax errors?
  The tool MUST handle tree-sitter partial parsing gracefully, reporting
  metrics for successfully parsed functions and noting unparseable regions.
- What happens when output is piped (not a TTY)?
  The tool MUST suppress colors and decorative formatting automatically.
- What happens when one file in a directory traversal fails (permissions,
  corrupt content)?
  The tool MUST continue analyzing remaining files, report each failure
  to stderr with the file path and reason, and exit with code 2 at the end.
- What happens when multiple positional arguments are provided?
  The tool MUST accept and process all arguments (files and directories
  mixed) in the order provided.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST analyze a single source file and report per-function
  cognitive complexity, cyclomatic complexity, and SLOC metrics
- **FR-002**: System MUST accept source code via stdin when `--language` flag
  is provided
- **FR-003**: System MUST accept multiple files and directories as positional
  arguments and recursively traverse directories to analyze all recognized
  source files
- **FR-004**: System MUST auto-detect programming language from file extension
- **FR-005**: System MUST support three output formats: human-readable table
  (default), JSON (`--format json`), and CSV (`--format csv`)
- **FR-006**: System MUST produce JSON output matching the `arborist-metrics`
  `FileReport` serialization schema
- **FR-007**: System MUST support a `--threshold <N>` flag to set cognitive
  complexity threshold and visually flag exceeding functions
- **FR-008**: System MUST support `--exceeds-only` to filter output to
  functions exceeding the threshold
- **FR-009**: System MUST support `--sort <metric>` to order results by
  `cognitive`, `cyclomatic`, `sloc`, or `name`
- **FR-010**: System MUST support `--top <N>` to limit output to the N most
  complex functions
- **FR-011**: System MUST support `--languages <list>` to filter directory
  traversal by language
- **FR-012**: System MUST support `--gitignore` to respect `.gitignore`
  patterns during directory traversal
- **FR-013**: System MUST support `--no-methods` to exclude method-level
  analysis
- **FR-014**: System MUST return exit code 0 when no threshold is set or no
  functions exceed it, exit code 1 when functions exceed threshold, and exit
  code 2 on errors. When both threshold violations and file errors occur,
  exit code 2 (error) takes precedence
- **FR-015**: System MUST print error messages to stderr, never to stdout
- **FR-016**: System MUST suppress colors and decorative output when stdout
  is not a TTY
- **FR-017**: When analyzing directories, if individual files fail (permissions,
  parsing errors), the system MUST continue processing remaining files, report
  each failure to stderr, and set exit code to 2
- **FR-018**: When `--sort` or `--top` are used with multi-file input, output
  MUST switch from file-grouped format to a flat function list ranked across
  all analyzed files. Without these flags, output MUST group results by file

### Key Entities

- **FileReport**: Represents analysis results for a single file — contains
  file path, detected language, list of function metrics, and file-level
  aggregates (total cognitive, cyclomatic, SLOC)
- **FunctionMetrics**: Represents metrics for a single function — contains
  function name, line range (start-end), cognitive complexity score,
  cyclomatic complexity score, and SLOC count
- **AnalysisConfig**: User-provided configuration that controls analysis
  behavior — contains optional cognitive threshold and include-methods flag

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can check complexity of any single recognized file in
  one command with zero configuration
- **SC-002**: An AI agent can consume JSON output without any post-processing
  beyond standard JSON parsing
- **SC-003**: Directory analysis of a 1,000-file project completes in under
  10 seconds on standard hardware
- **SC-004**: All supported output formats (table, JSON, CSV) are producible
  for any input type (file, stdin, directory)
- **SC-005**: Exit codes reliably signal threshold pass/fail for use in
  automated pipelines
- **SC-006**: Every public function in `arborist-metrics` 0.1.x is
  exercisable through CLI flags or flag combinations

## Assumptions

- `arborist-metrics` 0.1.x crate is published on crates.io and its public
  API is stable for the duration of MVP development
- Target users have Rust toolchain installed (binary distributed via
  `cargo install`)
- The set of languages supported by the CLI is determined by what
  `arborist-metrics` supports — the CLI does not add language support
- No configuration file (`.arborist.toml`) in MVP — all options are
  CLI flags only
- No `--watch` mode — users compose with external tools (`watch`, `entr`)
- No `--diff` mode in MVP — deferred to iteration 2
- Tree-sitter grammars are bundled by `arborist-metrics`, not by the CLI
