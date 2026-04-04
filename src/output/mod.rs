pub mod csv_output;
pub mod json;
pub mod table;

use arborist::FileReport;

use crate::analysis;
use crate::cli::{AnalyzeArgs, OutputFormat};
use crate::error::ArboristError;

pub fn write_output(
    reports: &[FileReport],
    args: &AnalyzeArgs,
    _threshold_exceeded: bool,
) -> Result<(), ArboristError> {
    let use_flat_mode = args.sort.is_some() || args.top.is_some();

    if use_flat_mode {
        let mut flat = analysis::flatten_reports(reports);

        if let Some(ref sort) = args.sort {
            analysis::sort_and_top(&mut flat, sort, args.top);
        } else if let Some(top) = args.top {
            // --top without --sort: default sort by cognitive descending
            analysis::sort_and_top(&mut flat, &crate::cli::SortMetric::Cognitive, Some(top));
        }

        // Apply threshold filtering to flat results
        if args.exceeds_only
            && let Some(threshold) = args.threshold
        {
            flat.retain(|f| f.cognitive > threshold);
        }

        match args.format {
            OutputFormat::Table => table::write_flat(&flat, args),
            OutputFormat::Json => json::write_flat(&flat),
            OutputFormat::Csv => csv_output::write_flat(&flat),
        }
    } else {
        match args.format {
            OutputFormat::Table => table::write_reports(reports, args),
            OutputFormat::Json => json::write_reports(reports),
            OutputFormat::Csv => csv_output::write_reports(reports),
        }
    }
}
