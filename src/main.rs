use std::process::ExitCode;

use clap::Parser;

use arborist_cli::cli::CliArgs;

fn main() -> ExitCode {
    let args = CliArgs::parse();

    match arborist_cli::run(&args) {
        Ok(report) => report.exit_code(),
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(2)
        }
    }
}
