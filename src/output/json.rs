use std::io::{self, Write};

use arborist::FileReport;

use crate::analysis::FlatFunction;
use crate::error::ArboristError;

pub fn write_reports(reports: &[FileReport]) -> Result<(), ArboristError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let json = serde_json::to_string_pretty(reports)
        .map_err(|e| ArboristError::Analysis(e.to_string()))?;
    writeln!(out, "{json}")?;
    Ok(())
}

pub fn write_flat(flat: &[FlatFunction]) -> Result<(), ArboristError> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct FlatJson<'a> {
        name: &'a str,
        file: &'a str,
        language: &'a str,
        line_start: usize,
        line_end: usize,
        cognitive: u64,
        cyclomatic: u64,
        sloc: u64,
    }

    let items: Vec<FlatJson<'_>> = flat
        .iter()
        .map(|f| FlatJson {
            name: &f.name,
            file: &f.file_path,
            language: &f.language,
            line_start: f.line_start,
            line_end: f.line_end,
            cognitive: f.cognitive,
            cyclomatic: f.cyclomatic,
            sloc: f.sloc,
        })
        .collect();

    let stdout = io::stdout();
    let mut out = stdout.lock();
    let json =
        serde_json::to_string_pretty(&items).map_err(|e| ArboristError::Analysis(e.to_string()))?;
    writeln!(out, "{json}")?;
    Ok(())
}
