// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("arborist").unwrap()
}

/// T021: verify stdin with `--language rust --format json` produces valid JSON
#[test]
fn stdin_json_output() {
    let output = cmd()
        .args(["--language", "rust", "--format", "json"])
        .write_stdin("fn main() {}\n")
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["functions"][0]["name"], "main");
}

/// T022: verify stdin without `--language` prints error and exits with code 2
#[test]
fn stdin_no_language_error() {
    cmd()
        .write_stdin("fn main() {}\n")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("--language"));
}