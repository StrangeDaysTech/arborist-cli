use std::io::{self, IsTerminal, Write};

use arborist::FileReport;
use comfy_table::{ContentArrangement, Table};

use crate::analysis::FlatFunction;
use crate::cli::AnalyzeArgs;
use crate::error::ArboristError;

pub fn write_reports(reports: &[FileReport], args: &AnalyzeArgs) -> Result<(), ArboristError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let is_tty = stdout.is_terminal();

    for (i, report) in reports.iter().enumerate() {
        if i > 0 {
            writeln!(out)?;
        }

        writeln!(
            out,
            "{} ({}) \u{2014} {} function{}, {} SLOC",
            report.path,
            report.language,
            report.functions.len(),
            if report.functions.len() == 1 { "" } else { "s" },
            report.file_sloc,
        )?;

        if report.functions.is_empty() {
            writeln!(out)?;
            continue;
        }

        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        if !is_tty {
            table.load_preset(comfy_table::presets::NOTHING);
        }

        table.set_header(vec!["Function", "Lines", "Cognitive", "Cyclomatic", "SLOC"]);

        for func in &report.functions {
            let cognitive_str = if let Some(threshold) = args.threshold {
                if func.cognitive > threshold {
                    if is_tty {
                        format!("{} \u{26a0}", func.cognitive)
                    } else {
                        format!("{} !", func.cognitive)
                    }
                } else {
                    func.cognitive.to_string()
                }
            } else {
                func.cognitive.to_string()
            };

            table.add_row(vec![
                func.name.clone(),
                format!("{}-{}", func.start_line, func.end_line),
                cognitive_str,
                func.cyclomatic.to_string(),
                func.sloc.to_string(),
            ]);
        }

        writeln!(out, "{table}")?;
    }

    Ok(())
}

pub fn write_flat(flat: &[FlatFunction], args: &AnalyzeArgs) -> Result<(), ArboristError> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let is_tty = stdout.is_terminal();

    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);

    if !is_tty {
        table.load_preset(comfy_table::presets::NOTHING);
    }

    table.set_header(vec!["Function", "File", "Cognitive", "Cyclomatic", "SLOC"]);

    for func in flat {
        let cognitive_str = if let Some(threshold) = args.threshold {
            if func.cognitive > threshold {
                if is_tty {
                    format!("{} \u{26a0}", func.cognitive)
                } else {
                    format!("{} !", func.cognitive)
                }
            } else {
                func.cognitive.to_string()
            }
        } else {
            func.cognitive.to_string()
        };

        table.add_row(vec![
            func.name.clone(),
            func.file_path.clone(),
            cognitive_str,
            func.cyclomatic.to_string(),
            func.sloc.to_string(),
        ]);
    }

    writeln!(out, "{table}")?;

    Ok(())
}
