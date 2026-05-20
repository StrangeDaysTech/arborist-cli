// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use std::io::Read;
use std::path::{Path, PathBuf};

use arborist::{AnalysisConfig, FileReport, Language};

use crate::cli::AnalyzeArgs;
use crate::error::ArboristError;
use crate::traversal;

pub fn build_config(args: &AnalyzeArgs) -> AnalysisConfig {
    AnalysisConfig {
        cognitive_threshold: args.threshold,
        include_methods: !args.no_methods,
    }
}

pub fn analyze_file(path: &Path, config: &AnalysisConfig) -> Result<FileReport, ArboristError> {
    arborist::analyze_file_with_config(path, config)
        .map_err(|e| ArboristError::Analysis(e.to_string()))
}

pub fn analyze_stdin(language: &str, config: &AnalysisConfig) -> Result<FileReport, ArboristError> {
    let mut source = String::new();
    std::io::stdin()
        .read_to_string(&mut source)
        .map_err(ArboristError::Io)?;

    let lang: Language = language
        .parse()
        .map_err(|e: String| ArboristError::Analysis(e))?;

    arborist::analyze_source_with_config(&source, lang, config)
        .map_err(|e| ArboristError::Analysis(e.to_string()))
}

pub fn analyze_paths(
    paths: &[PathBuf],
    config: &AnalysisConfig,
    args: &AnalyzeArgs,
) -> Result<(Vec<FileReport>, Vec<String>), ArboristError> {
    let files = traversal::collect_files(paths, args.languages.as_deref(), args.gitignore)?;

    let mut reports = Vec::new();
    let mut errors = Vec::new();

    for file in &files {
        match analyze_file(file, config) {
            Ok(report) => reports.push(report),
            Err(e) => errors.push(format!("{}: {e}", file.display())),
        }
    }

    Ok((reports, errors))
}

pub fn apply_filters(reports: &[FileReport], args: &AnalyzeArgs) -> (Vec<FileReport>, bool) {
    let mut threshold_exceeded = false;
    let mut filtered: Vec<FileReport> = Vec::new();

    for report in reports {
        let mut report = report.clone();

        if let Some(threshold) = args.threshold {
            let has_exceeding = report.functions.iter().any(|f| f.cognitive > threshold);

            if has_exceeding {
                threshold_exceeded = true;
            }

            if args.exceeds_only {
                report.functions.retain(|f| f.cognitive > threshold);
                if report.functions.is_empty() {
                    continue;
                }
            }
        }

        filtered.push(report);
    }

    (filtered, threshold_exceeded)
}

#[derive(Debug, Clone)]
pub struct FlatFunction {
    pub name: String,
    pub file_path: String,
    pub language: String,
    pub line_start: usize,
    pub line_end: usize,
    pub cognitive: u64,
    pub cyclomatic: u64,
    pub sloc: u64,
}

pub fn flatten_reports(reports: &[FileReport]) -> Vec<FlatFunction> {
    let mut flat = Vec::new();
    for report in reports {
        for func in &report.functions {
            flat.push(FlatFunction {
                name: func.name.clone(),
                file_path: report.path.clone(),
                language: report.language.to_string(),
                line_start: func.start_line,
                line_end: func.end_line,
                cognitive: func.cognitive,
                cyclomatic: func.cyclomatic,
                sloc: func.sloc,
            });
        }
    }
    flat
}

pub fn sort_and_top(
    flat: &mut Vec<FlatFunction>,
    sort: &crate::cli::SortMetric,
    top: Option<usize>,
) {
    use crate::cli::SortMetric;

    match sort {
        SortMetric::Cognitive => flat.sort_by_key(|b| std::cmp::Reverse(b.cognitive)),
        SortMetric::Cyclomatic => flat.sort_by_key(|b| std::cmp::Reverse(b.cyclomatic)),
        SortMetric::Sloc => flat.sort_by_key(|b| std::cmp::Reverse(b.sloc)),
        SortMetric::Name => flat.sort_by(|a, b| a.name.cmp(&b.name)),
    }

    if let Some(n) = top {
        flat.truncate(n);
    }
}
