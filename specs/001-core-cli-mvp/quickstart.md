# Quickstart: arborist-cli

## Install

```bash
cargo install arborist-cli
```

## Basic Usage

```bash
# Check complexity of a single file
arborist src/main.rs

# Analyze an entire directory
arborist src/

# Pipe source code directly
echo 'fn main() { if true { if true { } } }' | arborist --language rust
```

## Output Formats

```bash
# JSON for AI agents and scripting
arborist src/main.rs --format json

# CSV for spreadsheets
arborist src/ --format csv > metrics.csv
```

## Filtering

```bash
# Find functions exceeding complexity threshold
arborist src/ --threshold 15 --exceeds-only

# Top 10 most complex functions
arborist src/ --sort cognitive --top 10

# Filter by language in a mixed project
arborist . --languages rust,python --gitignore
```

## CI Integration

```bash
# Exit code 1 if any function exceeds threshold
arborist src/ --threshold 20 || echo "Complex functions detected"
```

## Verify Installation

After installing, run the following to confirm everything works:

```bash
# Should print version
arborist --version

# Should analyze this file (or any .rs file you have)
echo 'fn hello() { println!("world"); }' | arborist --language rust
```

Expected output:

```text
stdin (Rust) — 1 function, 1 SLOC

  Function    Lines    Cognitive  Cyclomatic  SLOC
  hello       1-1      0          1           1
```
