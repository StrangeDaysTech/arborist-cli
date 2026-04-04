use std::process::ExitCode;

use clap::Parser;

use arborist_cli::cli::{Cli, Command};

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
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
