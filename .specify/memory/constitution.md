<!--
Sync Impact Report
- Version change: 1.0.0 → 1.1.0
- Modified principles: none
- Added sections: Rust Idioms, Testing Strategy
- Removed sections: none
- Templates requiring updates:
  - .specify/templates/plan-template.md — ⚠️ pending (Constitution Check gates to be defined per principles)
  - .specify/templates/spec-template.md — ⚠️ pending (no changes needed yet)
  - .specify/templates/tasks-template.md — ⚠️ pending (no changes needed yet)
- Follow-up TODOs: none
-->

# arborist-cli Constitution

## Core Principles

### I. Zero Configuration

arborist-cli MUST work out of the box on any codebase containing files in
supported languages. No config files, no setup steps, no mandatory flags.

- Sensible defaults for every option (e.g., no threshold means report all)
- Config file (`.arborist.toml`) is deferred until real-world demand proves
  teams repeatedly pass the same flags — CLI args MUST always take precedence
- Language detection MUST be automatic via file extension and tree-sitter grammar

**Rationale**: The two target audiences — AI agents and developers doing quick
checks — need instant results. Any setup friction defeats the purpose.

### II. Composable CLI

arborist-cli MUST behave as a well-mannered Unix citizen that plays well with
pipes, `jq`, `grep`, `xargs`, and other standard tools.

- Text protocol: arguments + stdin → stdout; errors → stderr
- Support structured output formats: human-readable table (default), JSON, CSV
- Exit codes MUST be meaningful and machine-readable:
  `0` (ok), `1` (threshold exceeded), `2` (error)
- No interactive prompts, no color when output is piped (detect TTY)
- Single-purpose: report metrics, nothing else

**Rationale**: Composability multiplies value. A tool that fits into existing
workflows is adopted faster than one that demands its own ecosystem.

### III. AI-Agent Friendly

JSON output MUST be structured, predictable, and match the `arborist-metrics`
library's `FileReport` serialization format for zero-translation consumption.

- `--format json` output is the primary contract for programmatic consumers
- Output schema MUST remain stable within a major version
- Stdin analysis (`echo "code" | arborist --language rust`) MUST be supported
  so agents can analyze generated or in-memory code without temp files
- Exit codes serve as the gate mechanism for CI and agent decision loops

**Rationale**: AI agents are a first-class audience, not an afterthought.
Predictable structured output enables agents to evaluate complexity as part
of larger workflows (documentation triggers, review decisions, refactoring).

### IV. Reference Implementation

arborist-cli MUST expose every public API in `arborist-metrics`, serving as
living documentation of what the library can do.

- Every `arborist-metrics` public function MUST have a corresponding CLI path
  (direct flag or flag combination)
- The CLI imports `arborist-metrics` with `features = ["all"]`
- The CLI MUST NOT re-implement analysis logic — it is a thin wrapper that
  delegates all computation to the library
- When `arborist-metrics` adds new public API surface, arborist-cli MUST be
  updated to expose it

**Rationale**: The library repo (`arborist`) maintains a no-CLI constitution.
This project exists to prove and document the full library surface area through
a real, usable tool.

### V. Fast & Minimal Overhead

The CLI MUST add negligible overhead on top of tree-sitter parsing and
`arborist-metrics` computation.

- No unnecessary allocations or copies in the hot path (file read → parse →
  report)
- Directory traversal MUST respect `.gitignore` by default to avoid scanning
  irrelevant files
- No runtime dependencies beyond what `clap`, `serde_json`, directory traversal
  (`ignore` crate), and terminal formatting require
- No `--watch` mode — live feedback is solved by external composition
  (`watch`, `entr`, `watchexec`), keeping the binary focused and lean

**Rationale**: tree-sitter is already fast. The CLI must not become the
bottleneck. Developers expect sub-second results on typical projects; agents
expect minimal latency in batch pipelines.

## Boundaries

arborist-cli is deliberately scoped. These boundaries are constitutional:

- **Not a linter** — reports metrics, does not enforce rules
- **Not a CI gate** — use the library directly or check exit codes; the CLI
  does not manage pipelines
- **Not a replacement for DevTrail** — DevTrail uses complexity as *input* to
  documentation decisions; this CLI reports the raw metrics
- **Thin wrapper only** — all analysis logic lives in `arborist-metrics`;
  the CLI adds argument parsing, directory traversal, and output formatting
- **Rust edition 2024** — aligned with `arborist-metrics`; binary crate
  distributed via `cargo install`

## Development Workflow

- All changes go through feature branches and pull requests (never commit
  to `main` directly)
- Conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `chore:`,
  `test:`, `perf:`
- Changes that touch public CLI flags or output format MUST be documented
  in the PR description with before/after examples
- Constitution Check in plans MUST verify:
  1. Does this change require configuration? If yes, justify against
     Principle I
  2. Does output remain composable (parseable, no interactive prompts)?
     Verify against Principle II
  3. Does JSON output schema change? If yes, verify backward compatibility
     per Principle III
  4. Does this expose a new `arborist-metrics` API? If no, verify it's not
     re-implementing library logic per Principle IV
  5. Does this add a runtime dependency? If yes, justify against Principle V

## Rust Idioms

arborist-cli follows idiomatic Rust practices aligned with edition 2024.

### Error Handling

- Use `thiserror` for defining CLI-specific error types; propagate library
  errors via `From` implementations, not string formatting
- `main` returns `ExitCode` — map errors to exit codes at the top level,
  not inside business logic
- MUST NOT `unwrap()` or `expect()` in non-test code; use `?` propagation
  or explicit error handling
- User-facing error messages go to stderr and MUST include actionable context
  (e.g., "file not found: src/main.rs" not just "file not found")

### Code Quality

- `cargo clippy -- -D warnings` MUST pass with zero warnings before merge
- `cargo fmt --check` MUST pass — no style debates, `rustfmt` decides
- Prefer `&str` over `String` in function signatures where ownership is not
  needed
- Prefer iterators and combinators over manual loops when intent is clearer
- Avoid `clone()` unless required by ownership boundaries — prefer borrowing
- Use `#[must_use]` on functions whose return value should not be silently
  discarded

### Dependencies

- Minimal dependency footprint — every new crate MUST be justified against
  Principle V (Fast & Minimal Overhead)
- Approved core dependencies: `clap` (args), `serde`/`serde_json` (JSON),
  `ignore` (directory traversal), `thiserror` (errors)
- Terminal formatting crate to be decided during implementation (e.g.,
  `comfy-table`, `tabled`, or similar)
- MUST NOT vendor or fork `arborist-metrics` — always depend on the
  published crate version

### Patterns

- Struct-based configuration over loose parameters — pass `AnalysisConfig`
  from clap parsing down to library calls
- Separation of concerns: CLI layer (clap + output formatting) MUST NOT
  contain analysis logic; that belongs in `arborist-metrics`
- Use `impl Display` for human-readable output and `impl Serialize` for
  structured output — keep both paths explicit, not overloaded

## Testing Strategy

arborist-cli uses a pragmatic testing approach: integration tests are the
primary safety net since the CLI is a thin wrapper; unit tests are added
only where they provide clear value.

### Integration Tests (mandatory)

End-to-end CLI tests that exercise the real binary with real inputs:

- **CLI contract tests**: verify that flag combinations produce expected
  output format, exit codes, and stderr behavior
  - `arborist <file>` → table output, exit 0
  - `arborist <file> --format json` → valid JSON matching `FileReport` schema
  - `arborist <file> --threshold 1 --exceeds-only` → exit 1 when exceeded
  - `arborist nonexistent.rs` → stderr message, exit 2
- **Directory traversal tests**: verify recursive analysis, `--languages`
  filter, and `.gitignore` respect using fixture directories
- **Stdin tests**: verify `echo "fn main(){}" | arborist --language rust`
  produces correct output
- Use `assert_cmd` + `predicates` crates for CLI testing
- Test fixtures live in `tests/fixtures/` with small, representative source
  files in supported languages

### Unit Tests (where they add value)

Unit tests are justified for logic that lives in the CLI layer itself:

- Output formatting (table rendering, CSV generation)
- Argument validation and normalization (e.g., language alias mapping)
- Exit code mapping logic
- Any helper functions with non-trivial branching

Unit tests are NOT justified for:

- Re-testing `arborist-metrics` analysis logic (that's the library's job)
- Thin pass-through functions that just forward to the library
- Clap argument definitions (integration tests cover this)

### Test Organization

```text
tests/
├── cli/              # Integration tests (assert_cmd)
│   ├── single_file.rs
│   ├── directory.rs
│   ├── stdin.rs
│   ├── output_formats.rs
│   └── exit_codes.rs
├── fixtures/         # Small source files for testing
│   ├── simple.rs
│   ├── complex.rs
│   ├── simple.py
│   └── nested_project/
└── unit/             # Unit tests (only if justified)
    └── formatting.rs
```

### Test Discipline

- Integration tests MUST be written before or alongside implementation,
  not deferred to "later"
- Every new CLI flag MUST have at least one integration test covering its
  happy path and one covering its error path
- Test names MUST describe the behavior, not the implementation:
  `json_output_matches_file_report_schema` not `test_json_flag`
- `cargo test` MUST pass before every commit

## Governance

This constitution supersedes all other development practices for arborist-cli.
Amendments require:

1. A documented proposal explaining the change and its rationale
2. Review and approval via pull request
3. Version bump following semantic versioning:
   - **MAJOR**: Principle removal or backward-incompatible redefinition
   - **MINOR**: New principle or materially expanded guidance
   - **PATCH**: Clarifications, wording, non-semantic refinements
4. Migration plan if the amendment invalidates existing work

All PRs and code reviews MUST verify compliance with these principles.
Complexity or scope beyond what these principles allow MUST be justified
in the PR description.

**Version**: 1.1.0 | **Ratified**: 2026-04-02 | **Last Amended**: 2026-04-02
