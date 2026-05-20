// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use std::process::ExitCode;

use clap::Parser;

use arborist_cli::cli::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::About) => {
            arborist_cli::about::print();
            ExitCode::SUCCESS
        }
        Some(Command::Update { check }) => arborist_cli::update::run(check),
        None => match arborist_cli::run(&cli.analyze) {
            Ok(report) => report.exit_code(),
            Err(err) => {
                eprintln!("error: {err}");
                ExitCode::from(2)
            }
        },
    }
}