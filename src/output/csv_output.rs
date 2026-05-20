// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use std::io;

use arborist::FileReport;

use crate::analysis::FlatFunction;
use crate::error::ArboristError;

pub fn write_reports(reports: &[FileReport]) -> Result<(), ArboristError> {
    let stdout = io::stdout();
    let mut wtr = csv::Writer::from_writer(stdout.lock());

    wtr.write_record([
        "file",
        "language",
        "function",
        "line_start",
        "line_end",
        "cognitive",
        "cyclomatic",
        "sloc",
    ])
    .map_err(|e| ArboristError::Analysis(e.to_string()))?;

    for report in reports {
        let lang = report.language.to_string();
        for func in &report.functions {
            wtr.write_record([
                &report.path,
                &lang,
                &func.name,
                &func.start_line.to_string(),
                &func.end_line.to_string(),
                &func.cognitive.to_string(),
                &func.cyclomatic.to_string(),
                &func.sloc.to_string(),
            ])
            .map_err(|e| ArboristError::Analysis(e.to_string()))?;
        }
    }

    wtr.flush()?;
    Ok(())
}

pub fn write_flat(flat: &[FlatFunction]) -> Result<(), ArboristError> {
    let stdout = io::stdout();
    let mut wtr = csv::Writer::from_writer(stdout.lock());

    wtr.write_record([
        "file",
        "language",
        "function",
        "line_start",
        "line_end",
        "cognitive",
        "cyclomatic",
        "sloc",
    ])
    .map_err(|e| ArboristError::Analysis(e.to_string()))?;

    for func in flat {
        wtr.write_record([
            &func.file_path,
            &func.language,
            &func.name,
            &func.line_start.to_string(),
            &func.line_end.to_string(),
            &func.cognitive.to_string(),
            &func.cyclomatic.to_string(),
            &func.sloc.to_string(),
        ])
        .map_err(|e| ArboristError::Analysis(e.to_string()))?;
    }

    wtr.flush()?;
    Ok(())
}
