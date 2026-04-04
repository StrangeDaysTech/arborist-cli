use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("arborist-cli").unwrap()
}

/// T031: verify multiple positional arguments analyzes both files
#[test]
fn multi_file_input() {
    cmd()
        .args(["tests/fixtures/simple.rs", "tests/fixtures/simple.py"])
        .assert()
        .success()
        .stdout(predicate::str::contains("simple.rs"))
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("simple.py"))
        .stdout(predicate::str::contains("Python"));
}
