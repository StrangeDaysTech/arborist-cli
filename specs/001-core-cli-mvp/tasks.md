# Tasks: Core CLI MVP

**Input**: Design documents from `/specs/001-core-cli-mvp/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/cli-interface.md, quickstart.md

**Tests**: Integration tests are MANDATORY per constitution (Testing Strategy section). Unit tests only where justified.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root

---

## Phase 1: Setup

**Purpose**: Project initialization and Rust binary crate scaffolding

- [ ] T001 Initialize Rust binary crate with `cargo init --name arborist-cli` and set edition 2024 in Cargo.toml
- [ ] T002 Add core dependencies to Cargo.toml: `arborist-metrics` (features = ["all"]), `clap` (features = ["derive"]), `serde`, `serde_json`, `ignore`, `thiserror`, `comfy-table`, `csv`
- [ ] T003 Add dev-dependencies to Cargo.toml: `assert_cmd`, `predicates`
- [ ] T004 [P] Create test fixtures directory structure at `tests/fixtures/` with sample files: `simple.rs`, `complex.rs`, `simple.py`, `no_functions.rs`, `syntax_error.rs`, and `nested_project/` with `.gitignore`, `src/main.rs`, `src/lib.rs`, `ignored/generated.rs`
- [ ] T005 [P] Configure clippy lints in Cargo.toml or `.clippy.toml` with `-D warnings`
- [ ] T006 [P] Create `.rustfmt.toml` with project formatting rules (edition 2024)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and error handling that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T007 Define `ArboristError` enum with `thiserror` in `src/error.rs`: variants `Io(#[from] std::io::Error)`, `Analysis(String)`, `NoLanguage`, `InvalidArgument(String)`; implement `ExitCode` mapping (all variants → 2)
- [ ] T008 Define `CliArgs` struct with clap derive in `src/cli.rs`: positional `paths: Vec<PathBuf>`, `--format` enum (table/json/csv), `--language`, `--threshold`, `--exceeds-only`, `--sort` enum (cognitive/cyclomatic/sloc/name), `--top`, `--languages`, `--gitignore`, `--no-methods`; include argument validation
- [ ] T009 Create `src/main.rs` entry point: parse `CliArgs`, call `run()` function, map `Result` to `ExitCode` (0 success, 1 threshold exceeded, 2 error)
- [ ] T010 Create `src/lib.rs` exposing public `run(args: CliArgs) -> Result<ExitReport, ArboristError>` orchestration function with `ExitReport` struct containing `threshold_exceeded: bool`

**Checkpoint**: Foundation ready — `cargo build` succeeds, `arborist --help` prints usage, `arborist --version` prints version

---

## Phase 3: User Story 1 — Single File Complexity Check (Priority: P1) 🎯 MVP

**Goal**: Analyze a single source file and display per-function metrics in a human-readable table

**Independent Test**: `arborist tests/fixtures/simple.rs` displays table with function names, line ranges, cognitive/cyclomatic/SLOC

### Tests for User Story 1 ⚠️

- [ ] T011 [P] [US1] Integration test in `tests/cli/single_file.rs`: verify `arborist tests/fixtures/simple.rs` produces table output with expected function names and metrics
- [ ] T012 [P] [US1] Integration test in `tests/cli/single_file.rs`: verify `arborist tests/fixtures/complex.rs` produces table with correct cognitive/cyclomatic scores
- [ ] T013 [P] [US1] Integration test in `tests/cli/single_file.rs`: verify `arborist nonexistent.rs` prints error to stderr and exits with code 2
- [ ] T014 [P] [US1] Integration test in `tests/cli/single_file.rs`: verify `arborist unknown.xyz` prints unrecognized language error to stderr and exits with code 2
- [ ] T014b [P] [US1] Integration test in `tests/cli/single_file.rs`: verify `arborist tests/fixtures/syntax_error.rs` handles partial parsing gracefully, reports metrics for parseable functions, and exits with code 0

### Implementation for User Story 1

- [ ] T015 [US1] Implement single-file analysis in `src/analysis.rs`: function `analyze_path(path, config) -> Result<FileReport>` that calls `arborist_metrics::analyze_file` / `analyze_file_with_config` based on CliArgs
- [ ] T016 [US1] Implement table formatter in `src/output/table.rs`: render `FileReport` as human-readable table using `comfy-table` with file header (path, language, function count, SLOC) and function rows (name, lines, cognitive, cyclomatic, SLOC)
- [ ] T017 [US1] Create output dispatcher in `src/output/mod.rs`: match on `OutputFormat` enum and delegate to table/json/csv formatters (json and csv can be stubs returning `todo!()` for now)
- [ ] T018 [US1] Wire single-file analysis into `run()` in `src/lib.rs`: detect single file path → `analyze_path` → output dispatcher → stdout
- [ ] T019 [US1] Implement TTY detection in `src/output/table.rs`: suppress `comfy-table` styling and `⚠` markers when stdout is not a TTY

**Checkpoint**: `arborist src/main.rs` works end-to-end with table output. `cargo test` passes.

---

## Phase 4: User Story 2 — AI Agent JSON Consumption (Priority: P1)

**Goal**: Produce structured JSON output matching `FileReport` serde schema, and accept stdin input

**Independent Test**: `arborist tests/fixtures/simple.rs --format json | jq .` produces valid, parseable JSON

### Tests for User Story 2 ⚠️

- [ ] T020 [P] [US2] Integration test in `tests/cli/output_formats.rs`: verify `arborist tests/fixtures/simple.rs --format json` produces valid JSON array with FileReport schema fields (path, language, functions, file_cognitive, file_cyclomatic, file_sloc)
- [ ] T021 [P] [US2] Integration test in `tests/cli/stdin.rs`: verify `echo "fn main() {}" | arborist --language rust --format json` produces valid JSON
- [ ] T022 [P] [US2] Integration test in `tests/cli/stdin.rs`: verify `echo "code" | arborist` (no --language) prints error to stderr and exits with code 2
- [ ] T023 [P] [US2] Integration test in `tests/cli/output_formats.rs`: verify JSON output fields match `arborist-metrics` `FileReport` serialization exactly

### Implementation for User Story 2

- [ ] T024 [US2] Implement JSON formatter in `src/output/json.rs`: serialize `Vec<FileReport>` to JSON array via `serde_json::to_string_pretty` and write to stdout
- [ ] T025 [US2] Implement stdin analysis in `src/analysis.rs`: function `analyze_stdin(language, config) -> Result<FileReport>` reading from stdin, calling `arborist_metrics::analyze_source` / `analyze_source_with_config`
- [ ] T026 [US2] Wire stdin detection into `run()` in `src/lib.rs`: if `paths` is empty and stdin is pipe → require `--language` → `analyze_stdin` → output dispatcher

**Checkpoint**: JSON output works for both files and stdin. `echo code | arborist --language rust --format json` works. `cargo test` passes.

---

## Phase 5: User Story 3 — Directory-Wide Analysis (Priority: P2)

**Goal**: Recursively analyze all recognized files in a directory with language filtering and gitignore support

**Independent Test**: `arborist tests/fixtures/nested_project/` analyzes all files recursively, respects .gitignore with `--gitignore`

### Tests for User Story 3 ⚠️

- [ ] T027 [P] [US3] Integration test in `tests/cli/directory.rs`: verify `arborist tests/fixtures/nested_project/src/` analyzes all .rs files recursively
- [ ] T028 [P] [US3] Integration test in `tests/cli/directory.rs`: verify `arborist tests/fixtures/nested_project/ --gitignore` excludes `ignored/generated.rs`
- [ ] T029 [P] [US3] Integration test in `tests/cli/directory.rs`: verify `arborist tests/fixtures/ --languages rust` analyzes only .rs files, skips .py
- [ ] T030 [P] [US3] Integration test in `tests/cli/directory.rs`: verify `arborist tests/fixtures/nested_project/ignored/` (empty recognized files) prints info message and exits 0
- [ ] T031 [P] [US3] Integration test in `tests/cli/multi_input.rs`: verify `arborist tests/fixtures/simple.rs tests/fixtures/simple.py` analyzes both files

### Implementation for User Story 3

- [ ] T032 [US3] Implement directory traversal in `src/traversal.rs`: function `collect_files(paths, languages_filter, gitignore) -> Result<Vec<PathBuf>>` using `ignore` crate WalkBuilder, filtering by language extension, respecting gitignore when flagged
- [ ] T033 [US3] Implement multi-path resolution in `src/analysis.rs`: function `analyze_paths(paths, config) -> Result<(Vec<FileReport>, Vec<FileError>)>` that classifies each path as file/directory, uses `collect_files` for directories, analyzes each file, collects per-file errors without aborting
- [ ] T034 [US3] Wire directory analysis into `run()` in `src/lib.rs`: resolve all positional paths → `analyze_paths` → report per-file errors to stderr → output all FileReports → set exit code (2 if any errors, even if some succeed)
- [ ] T035 [US3] Update table formatter in `src/output/table.rs`: render multiple FileReports with per-file headers (grouped-by-file mode)

**Checkpoint**: `arborist src/` works with recursive analysis. Multiple positional args work. Gitignore filtering works. `cargo test` passes.

---

## Phase 6: User Story 4 — Threshold-Based Filtering (Priority: P2)

**Goal**: Flag functions exceeding a complexity threshold, filter to exceeding-only, and return meaningful exit codes

**Independent Test**: `arborist tests/fixtures/complex.rs --threshold 5 --exceeds-only` shows only complex functions and exits with code 1

### Tests for User Story 4 ⚠️

- [ ] T036 [P] [US4] Integration test in `tests/cli/filtering.rs`: verify `arborist tests/fixtures/complex.rs --threshold 5` shows all functions with `⚠` on exceeding ones
- [ ] T037 [P] [US4] Integration test in `tests/cli/exit_codes.rs`: verify `arborist tests/fixtures/complex.rs --threshold 5` exits with code 1
- [ ] T038 [P] [US4] Integration test in `tests/cli/exit_codes.rs`: verify `arborist tests/fixtures/simple.rs --threshold 100` exits with code 0 (nothing exceeds)
- [ ] T039 [P] [US4] Integration test in `tests/cli/filtering.rs`: verify `arborist tests/fixtures/complex.rs --threshold 5 --exceeds-only` shows only exceeding functions
- [ ] T040 [P] [US4] Integration test in `tests/cli/filtering.rs`: verify `--exceeds-only` without `--threshold` shows all functions (no-op)
- [ ] T041 [P] [US4] Integration test in `tests/cli/exit_codes.rs`: verify partial file failure + threshold exceeded → exit code 2 (error takes precedence)
- [ ] T041b [P] [US4] Integration test in `tests/cli/filtering.rs`: verify `arborist tests/fixtures/complex.rs --no-methods` excludes method-level analysis from output

### Implementation for User Story 4

- [ ] T042 [US4] Implement threshold filtering in `src/analysis.rs`: function `apply_filters(reports, threshold, exceeds_only) -> Vec<FileReport>` that marks/filters functions based on cognitive complexity threshold
- [ ] T043 [US4] Update table formatter in `src/output/table.rs`: add `⚠` marker to functions exceeding threshold in the cognitive column
- [ ] T044 [US4] Update JSON formatter in `src/output/json.rs`: filter functions from FileReports when `--exceeds-only` is active (omit files with empty function arrays after filtering)
- [ ] T045 [US4] Update `run()` in `src/lib.rs`: apply filters after analysis, determine `threshold_exceeded` flag for `ExitReport`, ensure exit code priority: error(2) > threshold(1) > success(0)
- [ ] T046 [US4] Implement `--no-methods` pass-through in `src/analysis.rs`: map CliArgs to `AnalysisConfig { include_methods: !no_methods }` and pass to arborist-metrics

**Checkpoint**: Threshold flagging works in table/JSON. Exit codes are correct. `--exceeds-only` filters correctly. `cargo test` passes.

---

## Phase 7: User Story 5 — Sorting and Top-N Results (Priority: P3)

**Goal**: Sort results by metric and limit to top N functions across all files

**Independent Test**: `arborist tests/fixtures/nested_project/src/ --sort cognitive --top 3` shows top 3 functions ranked by cognitive complexity

### Tests for User Story 5 ⚠️

- [ ] T047 [P] [US5] Integration test in `tests/cli/filtering.rs`: verify `arborist tests/fixtures/nested_project/src/ --sort cognitive` outputs flat function list ordered descending
- [ ] T048 [P] [US5] Integration test in `tests/cli/filtering.rs`: verify `arborist tests/fixtures/nested_project/src/ --sort name` outputs flat function list ordered ascending alphabetically
- [ ] T049 [P] [US5] Integration test in `tests/cli/filtering.rs`: verify `arborist tests/fixtures/nested_project/src/ --top 2 --sort cyclomatic` shows exactly 2 functions
- [ ] T050 [P] [US5] Integration test in `tests/cli/filtering.rs`: verify `--top 100` with fewer results shows all without error

### Implementation for User Story 5

- [ ] T051 [US5] Implement sort and top-N in `src/analysis.rs`: function `apply_sort_and_top(reports, sort_by, top_n) -> Vec<FlatFunction>` that flattens all functions across files, sorts by metric (descending numeric, ascending name), and truncates to top N
- [ ] T052 [US5] Create `FlatFunction` struct in `src/analysis.rs`: contains function metrics plus source file path for cross-file context
- [ ] T053 [US5] Update table formatter in `src/output/table.rs`: add flat-mode rendering for `Vec<FlatFunction>` with columns: Function, File, sorted metric
- [ ] T054 [US5] Update JSON formatter in `src/output/json.rs`: when sort/top active, still output FileReport array but with filtered/ordered functions
- [ ] T055 [US5] Wire sort/top into `run()` in `src/lib.rs`: detect `--sort` or `--top` → switch to flat mode → apply sort and top → output

**Checkpoint**: `arborist src/ --sort cognitive --top 5` works across files. `cargo test` passes.

---

## Phase 8: User Story 6 — CSV Output (Priority: P3)

**Goal**: Export metrics as CSV for spreadsheet/data analysis workflows

**Independent Test**: `arborist tests/fixtures/simple.rs --format csv` produces valid CSV with headers

### Tests for User Story 6 ⚠️

- [ ] T056 [P] [US6] Integration test in `tests/cli/output_formats.rs`: verify `arborist tests/fixtures/simple.rs --format csv` produces CSV with header row `file,language,function,line_start,line_end,cognitive,cyclomatic,sloc`
- [ ] T057 [P] [US6] Integration test in `tests/cli/output_formats.rs`: verify `arborist tests/fixtures/nested_project/src/ --format csv` produces CSV with multiple rows across files
- [ ] T058 [P] [US6] Integration test in `tests/cli/output_formats.rs`: verify `arborist tests/fixtures/no_functions.rs --format csv` produces CSV with header only

### Implementation for User Story 6

- [ ] T059 [US6] Implement CSV formatter in `src/output/csv.rs`: use `csv` crate Writer to serialize function metrics as flat rows with header `file,language,function,line_start,line_end,cognitive,cyclomatic,sloc`
- [ ] T060 [US6] Wire CSV into output dispatcher in `src/output/mod.rs`: remove `todo!()` stub, delegate to csv formatter

**Checkpoint**: `arborist src/ --format csv > metrics.csv` works. All three output formats functional. `cargo test` passes.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Final quality, documentation, and validation

- [ ] T061 [P] Run `cargo clippy -- -D warnings` and fix any warnings across all source files
- [ ] T062 [P] Run `cargo fmt --check` and fix any formatting issues
- [ ] T063 Validate quickstart.md scenarios: run each example from `specs/001-core-cli-mvp/quickstart.md` and verify expected output matches
- [ ] T064 [P] Add `--help` text descriptions for all flags in `src/cli.rs` (clap `#[arg(help = "...")]` annotations)
- [ ] T065 [P] Verify `arborist --version` prints correct version from Cargo.toml
- [ ] T066 Run full `cargo test` suite and ensure all integration + unit tests pass
- [ ] T067 Verify exit code precedence: create integration test in `tests/cli/exit_codes.rs` for mixed error + threshold scenario

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **US1 (Phase 3)**: Depends on Foundational — first complete story = MVP
- **US2 (Phase 4)**: Depends on Foundational — can run in parallel with US1 (different files: json.rs, stdin logic)
- **US3 (Phase 5)**: Depends on Foundational — can run in parallel with US1/US2 (different files: traversal.rs, directory logic)
- **US4 (Phase 6)**: Depends on US1 (table formatter exists) and US2 (json formatter exists) — filtering applies to existing outputs
- **US5 (Phase 7)**: Depends on US3 (directory analysis) and US4 (filtering) — sort/top operates on multi-file filtered results
- **US6 (Phase 8)**: Depends on Foundational only — csv.rs is independent (but benefits from US3 for multi-file testing)
- **Polish (Phase 9)**: Depends on all user stories complete

### User Story Dependencies

```text
          ┌──── US1 (P1: single file + table) ────┐
          │                                         │
Phase 2 ──┼──── US2 (P1: json + stdin) ────────────┼──── US4 (P2: threshold) ──── US5 (P3: sort/top) ──── Phase 9
          │                                         │
          ├──── US3 (P2: directory traversal) ──────┘
          │
          └──── US6 (P3: csv output) ──────────────────────────────────────────────────────────────────── Phase 9
```

### Within Each User Story

- Tests MUST be written and FAIL before implementation
- Core logic before formatters
- Formatters before wiring into run()
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks T004, T005, T006 can run in parallel
- US1, US2, US3, US6 can all start after Foundational (different files)
- All test tasks within a story marked [P] can run in parallel
- US4 must wait for US1+US2 formatters; US5 must wait for US3+US4

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (single file + table)
4. **STOP and VALIDATE**: `arborist src/main.rs` works end-to-end
5. Deployable MVP via `cargo install --path .`

### Incremental Delivery

1. Setup + Foundational → binary compiles, `--help` works
2. US1 → single file table output (MVP!)
3. US2 → JSON + stdin (AI agents can now use it)
4. US3 → directory traversal (project-wide analysis)
5. US4 → threshold filtering (CI integration ready)
6. US5 → sort/top (convenience features)
7. US6 → CSV output (data export)
8. Polish → production quality

### Parallel Developer Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 (table formatter)
   - Developer B: US2 (JSON + stdin)
   - Developer C: US3 (directory traversal)
   - Developer D: US6 (CSV output)
3. After US1+US2+US3 complete:
   - Developer A: US4 (threshold filtering)
   - Then: US5 (sort/top)
4. Polish phase: all developers

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story is independently completable and testable
- Tests MUST fail before implementing (red-green-refactor)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Exit code priority: error(2) > threshold(1) > success(0)
