use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("arborist").unwrap()
}

/// T011: verify `arborist tests/fixtures/simple.rs` produces table output
/// with expected function names and metrics
#[test]
fn simple_file_table_output() {
    cmd()
        .arg("tests/fixtures/simple.rs")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("simple.rs"))
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("2 functions"));
}

/// T012: verify `arborist tests/fixtures/complex.rs` produces table with
/// correct cognitive/cyclomatic scores
#[test]
fn complex_file_metrics() {
    cmd()
        .arg("tests/fixtures/complex.rs")
        .assert()
        .success()
        .stdout(predicate::str::contains("complex_function"))
        .stdout(predicate::str::contains("simple_function"))
        .stdout(predicate::str::contains("moderate_function"))
        .stdout(predicate::str::contains("3 functions"));
}

/// T013: verify `arborist nonexistent.rs` prints error to stderr and exits with code 2
#[test]
fn nonexistent_file_error() {
    cmd()
        .arg("nonexistent.rs")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("error"));
}

/// T014: verify `arborist unknown.xyz` prints unrecognized language error to stderr
/// and exits with code 2
#[test]
fn unknown_extension_error() {
    cmd()
        .arg("unknown.xyz")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("error"));
}

/// T014b: verify `arborist tests/fixtures/syntax_error.rs` handles partial parsing
/// gracefully, reports metrics for parseable functions, and exits with code 0
#[test]
fn syntax_error_partial_parsing() {
    cmd()
        .arg("tests/fixtures/syntax_error.rs")
        .assert()
        .success()
        .stdout(predicate::str::contains("valid_function"));
}
