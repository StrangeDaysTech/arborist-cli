# arborist-cli

Code complexity metrics for your codebase, powered by [arborist-metrics](https://crates.io/crates/arborist-metrics).

`arborist-cli` analyzes source files and reports **cognitive complexity**, **cyclomatic complexity**, and **SLOC** per function. It works with single files, entire directories, or piped stdin. Output is available as a human-readable table, JSON, or CSV.

## Installation

### Pre-built binaries (recommended)

Download the latest release for your platform — no Rust toolchain required.

**Linux / macOS:**

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/StrangeDaysTech/arborist-cli/releases/latest/download/arborist-cli-installer.sh | sh
```

**Windows (PowerShell):**

```powershell
powershell -ExecutionPolicy ByPass -c "irm https://github.com/StrangeDaysTech/arborist-cli/releases/latest/download/arborist-cli-installer.ps1 | iex"
```

### cargo binstall (pre-compiled, no build)

If you have [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) installed:

```bash
cargo binstall arborist-cli
```

### cargo install (from source)

Requires Rust 1.85+ (Edition 2024):

```bash
cargo install arborist-cli
```

### From source

```bash
git clone https://github.com/StrangeDaysTech/arborist-cli.git
cd arborist-cli
cargo build --release
# Binary at ./target/release/arborist-cli
```

## Updating

The update method depends on how you installed:

| Installed via | Update command |
|---------------|----------------|
| Shell/PowerShell installer | `arborist update` |
| Direct binary download | `arborist update` |
| cargo binstall | `cargo binstall arborist-cli` |
| cargo install | `cargo install arborist-cli` |

Check for updates without installing:

```bash
arborist update --check
```

## Quick start

```bash
# Analyze a single file
arborist src/main.rs

# Analyze an entire directory
arborist src/

# Find the 5 most complex functions
arborist src/ --sort cognitive --top 5

# Fail CI if any function exceeds cognitive complexity 15
arborist src/ --threshold 15
```

## Usage

```
arborist [OPTIONS] [PATHS]... [COMMAND]
```

### Commands

| Command | Description |
|---------|-------------|
| `update` | Check for updates and install the latest version |
| `update --check` | Check for available updates without installing |

### Arguments

| Argument | Description |
|----------|-------------|
| `PATHS` | Files or directories to analyze. Omit to read from stdin. |

### Options

| Option | Default | Description |
|--------|---------|-------------|
| `--format <table\|json\|csv>` | `table` | Output format |
| `--threshold <N>` | — | Cognitive complexity threshold. Functions exceeding it are flagged with `!`. Exit code becomes `1`. |
| `--exceeds-only` | — | Show only functions that exceed `--threshold` |
| `--sort <cognitive\|cyclomatic\|sloc\|name>` | — | Sort results (flat list). Numeric sorts are descending; `name` is ascending. |
| `--top <N>` | — | Show only the top N results (implies flat list) |
| `--languages <lang,...>` | — | Filter directory traversal by language (e.g. `rust,python`) |
| `--gitignore` | — | Respect `.gitignore` patterns during directory traversal |
| `--no-methods` | — | Exclude method-level analysis |
| `--language <lang>` | — | Language hint for stdin input (required when piping) |

### Exit codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Threshold exceeded (at least one function above `--threshold`) |
| `2` | Error (file not found, unrecognized language, I/O failure) |

Error (code 2) takes precedence over threshold (code 1).

## Output formats

### Table (default)

```
tests/fixtures/complex.rs (Rust) — 3 functions, 42 SLOC
 Function           Lines  Cognitive  Cyclomatic  SLOC
 simple_function    1-3    0          1           3
 complex_function   5-32   27         11          28
 moderate_function  34-44  4          3           11
```

With `--threshold 5`, functions exceeding the threshold are marked:

```
 complex_function   5-32   27 !       11          28
```

### JSON

```bash
arborist src/main.rs --format json
```

```json
[
  {
    "path": "src/main.rs",
    "language": "Rust",
    "functions": [
      {
        "name": "main",
        "start_line": 1,
        "end_line": 17,
        "cognitive": 3,
        "cyclomatic": 3,
        "sloc": 15
      }
    ],
    "file_cognitive": 3,
    "file_cyclomatic": 3,
    "file_sloc": 15
  }
]
```

### CSV

```bash
arborist src/lib.rs --format csv
```

```
file,language,function,line_start,line_end,cognitive,cyclomatic,sloc
src/lib.rs,Rust,hello,1,3,0,1,3
src/lib.rs,Rust,add,5,7,0,1,3
```

## Supported languages

Rust, Python, JavaScript, TypeScript, TSX, JSX, Java, Go, C, C++, C#, Ruby, Swift, Kotlin, PHP.

## Use with AI agents

`arborist-cli` is designed to integrate into automated workflows. AI agents can use it to assess code quality before or after generating changes.

### Recommended patterns

**Gate a PR on complexity** (CI/CD):

```bash
arborist src/ --threshold 15 --format json
# Exit code 1 if any function exceeds threshold
# JSON output is machine-parseable for further processing
```

**Identify refactoring targets**:

```bash
arborist src/ --sort cognitive --top 10 --format json
```

**Analyze generated code from stdin**:

```bash
echo "$GENERATED_CODE" | arborist --language rust --format json
```

**Filter by language in a polyglot repo**:

```bash
arborist . --languages rust,python --gitignore --format csv
```

**Check only threshold violations** (minimal output):

```bash
arborist src/ --threshold 10 --exceeds-only --format csv
```

### Machine-readable output

- `--format json` returns an array of `FileReport` objects. Each contains `path`, `language`, `functions[]` (with per-function metrics), and file-level aggregates (`file_cognitive`, `file_cyclomatic`, `file_sloc`).
- `--format csv` returns RFC 4180 CSV with header: `file,language,function,line_start,line_end,cognitive,cyclomatic,sloc`.
- Exit code `1` reliably signals threshold violations without parsing output.

---

## For developers

### Project structure

```
src/
  main.rs           Entry point — parses args, dispatches to analyze or update
  lib.rs            Orchestration — stdin detection, path analysis, output dispatch
  cli.rs            CLI argument definitions (clap derive) with optional subcommands
  analysis.rs       arborist-metrics wrapper, threshold filtering, sorting
  traversal.rs      Directory walking (ignore crate), language detection by extension
  error.rs          Error types (thiserror), ExitReport with exit code logic
  update.rs         Self-update from GitHub Releases (self_update crate)
  output/
    mod.rs          Format dispatcher (grouped vs. flat mode)
    table.rs        Human-readable table output (comfy-table)
    json.rs         JSON serialization (serde_json)
    csv_output.rs   CSV output (csv crate)
tests/
  cli/              Integration tests organized by feature area
  fixtures/         Test input files (Rust, Python, multi-file projects)
```

### Building

```bash
cargo build            # Debug
cargo build --release  # Optimized
```

Requires **Rust 1.85+** (Edition 2024).

### Running tests

```bash
cargo test
```

Tests are integration tests that invoke the compiled binary via `assert_cmd`. They cover all input modes, output formats, filtering options, and exit code semantics.

### Linting

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

The project enforces `clippy::all = "deny"` in `Cargo.toml`.

### Releasing

See [RELEASING.md](RELEASING.md) for the full release procedure. In short:

1. Bump version in `Cargo.toml`
2. `git tag v0.2.0 && git push origin main --tags`
3. CI builds binaries for all platforms and creates the GitHub Release

### Reporting issues

File bugs and feature requests at [GitHub Issues](https://github.com/StrangeDaysTech/arborist-cli/issues). See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Contributing

We welcome patches! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a PR. All contributors must sign the **Contributor License Agreement (CLA)** — the CLA bot will guide you on your first pull request.

## Built with AI · Powered by StrayMark

`arborist-cli` is an experiment in **disciplined AI-assisted development**.
The implementation — the `clap`-derived CLI surface, the three output
formats (table / JSON / CSV), the directory traversal layer, self-update
from GitHub Releases, and the 29-test integration suite — was built
largely by AI agents under human direction.

To make that velocity sustainable, we use **[StrayMark](https://straymark.dev)**:
a CLI for *cognitive discipline in AI-assisted engineering*. It turned every
architectural choice into an AIDEC record and every implementation block into
an AILOG — all under `.straymark/`, append-only and audit-ready. The
governance artifacts emerged **alongside** the code, not as homework after.

> StrayMark is built by Strange Days Tech — the same team behind `arborist-cli`.
> It is the tool we made to solve our own problem.

## License

Licensed under either of

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## Copyright

**`arborist-cli`** is © 2026 **[Strange Days Tech S.A.S. de C.V.](https://strangedays.tech/)** — original author and intellectual-property holder of the source code.

The CLI is released under the dual MIT / Apache-2.0 license above; this notice records authorship and does **not** modify those license terms. Each source file carries an SPDX header reflecting the same.

> Built by **[Strange Days Tech](https://strangedays.tech/)** — México.
