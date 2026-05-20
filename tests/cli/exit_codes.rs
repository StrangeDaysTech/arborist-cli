// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin("arborist").unwrap()
}

/// T037: verify `--threshold 5` exits with code 1 when functions exceed threshold
#[test]
fn exit_code_1_threshold_exceeded() {
    cmd()
        .args(["tests/fixtures/complex.rs", "--threshold", "5"])
        .assert()
        .code(1);
}

/// T038: verify `--threshold 100` exits with code 0 when nothing exceeds
#[test]
fn exit_code_0_nothing_exceeds() {
    cmd()
        .args(["tests/fixtures/simple.rs", "--threshold", "100"])
        .assert()
        .code(0);
}

/// T041/T067: verify error takes precedence over threshold — exit code 2
/// When mixing a valid file (with threshold exceeded) and a nonexistent file,
/// exit code should be 2 (error), not 1 (threshold).
#[test]
fn exit_code_error_precedence_over_threshold() {
    cmd()
        .args([
            "tests/fixtures/complex.rs",
            "nonexistent_file.rs",
            "--threshold",
            "5",
        ])
        .assert()
        .code(2);
}

/// Verify nonexistent file exits with code 2
#[test]
fn exit_code_2_on_error() {
    cmd().arg("nonexistent.rs").assert().code(2);
}