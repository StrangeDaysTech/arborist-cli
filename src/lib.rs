// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

pub mod about;
pub mod analysis;
pub mod cli;
pub mod error;
pub mod output;
pub mod traversal;
pub mod update;

use cli::AnalyzeArgs;
use error::{ArboristError, ExitReport};

pub fn run(args: &AnalyzeArgs) -> Result<ExitReport, ArboristError> {
    let is_stdin = args.paths.is_empty() && atty::is(atty::Stream::Stdin).not_tty();

    if is_stdin {
        let language = args.language.as_deref().ok_or(ArboristError::NoLanguage)?;
        let config = analysis::build_config(args);
        let report = analysis::analyze_stdin(language, &config)?;
        let reports = vec![report];
        let (reports, threshold_exceeded) = analysis::apply_filters(&reports, args);
        output::write_output(&reports, args, threshold_exceeded)?;
        return Ok(ExitReport {
            threshold_exceeded,
            had_errors: false,
        });
    }

    if args.paths.is_empty() {
        return Err(ArboristError::InvalidArgument(
            "no files or directories specified and stdin is not a pipe".into(),
        ));
    }

    let config = analysis::build_config(args);
    let (reports, errors) = analysis::analyze_paths(&args.paths, &config, args)?;

    for err in &errors {
        eprintln!("warning: {err}");
    }

    let (filtered, threshold_exceeded) = analysis::apply_filters(&reports, args);
    output::write_output(&filtered, args, threshold_exceeded)?;

    Ok(ExitReport {
        threshold_exceeded,
        had_errors: !errors.is_empty(),
    })
}

mod atty {
    pub enum Stream {
        Stdin,
    }

    pub struct AttyResult(bool);

    impl AttyResult {
        pub fn not_tty(self) -> bool {
            !self.0
        }
    }

    pub fn is(stream: Stream) -> AttyResult {
        match stream {
            Stream::Stdin => {
                use std::io::IsTerminal;
                AttyResult(std::io::stdin().is_terminal())
            }
        }
    }
}
