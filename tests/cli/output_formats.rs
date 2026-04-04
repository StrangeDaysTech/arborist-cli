use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("arborist-cli").unwrap()
}

/// T020: verify `--format json` produces valid JSON with FileReport schema fields
#[test]
fn json_output_valid_schema() {
    let output = cmd()
        .args(["tests/fixtures/simple.rs", "--format", "json"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let arr = parsed.as_array().unwrap();
    assert_eq!(arr.len(), 1);

    let report = &arr[0];
    assert!(report.get("path").is_some());
    assert!(report.get("language").is_some());
    assert!(report.get("functions").is_some());
    assert!(report.get("file_cognitive").is_some());
    assert!(report.get("file_cyclomatic").is_some());
    assert!(report.get("file_sloc").is_some());
}

/// T023: verify JSON output fields match arborist-metrics FileReport serialization
#[test]
fn json_output_function_fields() {
    let output = cmd()
        .args(["tests/fixtures/simple.rs", "--format", "json"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let func = &parsed[0]["functions"][0];

    assert!(func.get("name").is_some());
    assert!(func.get("start_line").is_some());
    assert!(func.get("end_line").is_some());
    assert!(func.get("cognitive").is_some());
    assert!(func.get("cyclomatic").is_some());
    assert!(func.get("sloc").is_some());
}

/// T056: verify `--format csv` produces CSV with correct header row
#[test]
fn csv_output_header() {
    cmd()
        .args(["tests/fixtures/simple.rs", "--format", "csv"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with(
            "file,language,function,line_start,line_end,cognitive,cyclomatic,sloc\n",
        ));
}

/// T057: verify CSV with directory produces multiple rows across files
#[test]
fn csv_directory_multiple_rows() {
    let output = cmd()
        .args(["tests/fixtures/nested_project/src/", "--format", "csv"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    // header + at least 3 functions (compute, transform, main)
    assert!(lines.len() >= 4, "expected header + 3+ data rows, got {lines:?}");
}

/// T058: verify CSV with no-function file produces header only
#[test]
fn csv_no_functions_header_only() {
    let output = cmd()
        .args(["tests/fixtures/no_functions.rs", "--format", "csv"])
        .output()
        .unwrap();

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 1, "expected header only, got {lines:?}");
    assert!(lines[0].starts_with("file,language,function"));
}
