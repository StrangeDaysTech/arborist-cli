// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use std::process::ExitCode;

#[derive(Debug, thiserror::Error)]
pub enum ArboristError {
    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("analysis error: {0}")]
    Analysis(String),

    #[error("--language is required when reading from stdin")]
    NoLanguage,

    #[error("invalid argument: {0}")]
    InvalidArgument(String),
}

pub struct ExitReport {
    pub threshold_exceeded: bool,
    pub had_errors: bool,
}

impl ExitReport {
    pub fn exit_code(&self) -> ExitCode {
        if self.had_errors {
            ExitCode::from(2)
        } else if self.threshold_exceeded {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }
}
