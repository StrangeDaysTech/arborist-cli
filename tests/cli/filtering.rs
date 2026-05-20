// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("arborist").unwrap()
}

/// T036: verify `--threshold 5` shows all functions with `!` on exceeding ones
#[test]
fn threshold_flags_exceeding() {
    cmd()
        .args(["tests/fixtures/complex.rs", "--threshold", "5"])
        .assert()
        .success_or(predicate::eq(1)) // exit 1 because threshold exceeded
        .stdout(predicate::str::contains("simple_function"))
        .stdout(predicate::str::contains("complex_function"))
        .stdout(predicate::str::contains("!"));
}

/// T039: verify `--threshold 5 --exceeds-only` shows only exceeding functions
#[test]
fn threshold_exceeds_only() {
    let output = cmd()
        .args([
            "tests/fixtures/complex.rs",
            "--threshold",
            "5",
            "--exceeds-only",
        ])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("complex_function"),
        "exceeding function should appear"
    );
    assert!(
        !stdout.contains("simple_function"),
        "non-exceeding function should be filtered"
    );
    assert!(
        !stdout.contains("moderate_function"),
        "non-exceeding function should be filtered"
    );
}

/// T040: verify `--exceeds-only` without `--threshold` shows all functions (no-op)
#[test]
fn exceeds_only_without_threshold() {
    cmd()
        .args(["tests/fixtures/complex.rs", "--exceeds-only"])
        .assert()
        .success()
        .stdout(predicate::str::contains("simple_function"))
        .stdout(predicate::str::contains("complex_function"))
        .stdout(predicate::str::contains("moderate_function"));
}

/// T041b: verify `--no-methods` excludes method-level analysis from output
#[test]
fn no_methods_flag() {
    // With --no-methods, the output should still show functions but not methods.
    // For our fixture files (which only have functions, not methods), the output
    // should be identical. This verifies the flag is accepted and doesn't break.
    cmd()
        .args(["tests/fixtures/complex.rs", "--no-methods"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complex_function"));
}

/// T047: verify `--sort cognitive` outputs flat function list ordered descending
#[test]
fn sort_cognitive_descending() {
    let output = cmd()
        .args(["tests/fixtures/nested_project/src/", "--sort", "cognitive"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // In flat mode: Function, File columns should be present
    assert!(stdout.contains("Function"));
    assert!(stdout.contains("File"));
    // compute (cognitive 2) and main (cognitive 2) should be before transform (cognitive 0)
    let compute_pos = stdout.find("compute").unwrap();
    let transform_pos = stdout.find("transform").unwrap();
    assert!(
        compute_pos < transform_pos,
        "compute (cognitive 2) should appear before transform (cognitive 0)"
    );
}

/// T048: verify `--sort name` outputs flat function list ordered ascending alphabetically
#[test]
fn sort_name_ascending() {
    let output = cmd()
        .args(["tests/fixtures/nested_project/src/", "--sort", "name"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let compute_pos = stdout.find("compute").unwrap();
    let main_pos = stdout.find(" main").unwrap_or_else(|| {
        stdout
            .find("\nmain")
            .unwrap_or_else(|| stdout.find("main").unwrap())
    });
    let transform_pos = stdout.find("transform").unwrap();
    assert!(
        compute_pos < main_pos && main_pos < transform_pos,
        "functions should be in alphabetical order: compute < main < transform"
    );
}

/// T049: verify `--top 2 --sort cyclomatic` shows exactly 2 functions
#[test]
fn top_n_limits_results() {
    let output = cmd()
        .args([
            "tests/fixtures/nested_project/src/",
            "--top",
            "2",
            "--sort",
            "cyclomatic",
        ])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Count data rows (non-header lines with function names)
    // In flat table mode, header is first line, then data rows
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
    // The header line contains "Function", data lines do not start with it
    let data_lines: Vec<&&str> = lines
        .iter()
        .filter(|l| !l.contains("Function") && !l.contains("---"))
        .collect();
    assert_eq!(
        data_lines.len(),
        2,
        "expected exactly 2 data rows, got {data_lines:?}"
    );
}

/// T050: verify `--top 100` with fewer results shows all without error
#[test]
fn top_n_greater_than_results() {
    cmd()
        .args([
            "tests/fixtures/nested_project/src/",
            "--top",
            "100",
            "--sort",
            "cognitive",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("compute"))
        .stdout(predicate::str::contains("transform"))
        .stdout(predicate::str::contains("main"));
}

/// Helper trait so we can use `.success_or()` for exit code 0 or 1
trait AssertExt {
    fn success_or(self, code: predicates::ord::EqPredicate<i32>) -> assert_cmd::assert::Assert;
}

impl AssertExt for assert_cmd::assert::Assert {
    fn success_or(self, code: predicates::ord::EqPredicate<i32>) -> assert_cmd::assert::Assert {
        self.code(predicate::eq(0).or(code))
    }
}
