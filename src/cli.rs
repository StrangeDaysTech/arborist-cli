use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortMetric {
    Cognitive,
    Cyclomatic,
    Sloc,
    Name,
}

#[derive(Debug, Parser)]
#[command(
    name = "arborist",
    version,
    about = "Code complexity metrics powered by arborist-metrics"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[command(flatten)]
    pub analyze: AnalyzeArgs,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Display project information
    About,
    /// Check for updates and install the latest version
    Update {
        /// Only check for available updates without installing
        #[arg(long)]
        check: bool,
    },
}

#[derive(Debug, clap::Args)]
pub struct AnalyzeArgs {
    /// Files or directories to analyze
    #[arg()]
    pub paths: Vec<PathBuf>,

    /// Output format
    #[arg(long, default_value = "table")]
    pub format: OutputFormat,

    /// Language for stdin input (required when piping)
    #[arg(long)]
    pub language: Option<String>,

    /// Cognitive complexity threshold
    #[arg(long)]
    pub threshold: Option<u64>,

    /// Show only functions exceeding the threshold
    #[arg(long)]
    pub exceeds_only: bool,

    /// Sort results by metric
    #[arg(long)]
    pub sort: Option<SortMetric>,

    /// Show only the top N results
    #[arg(long)]
    pub top: Option<usize>,

    /// Filter directory traversal by language (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub languages: Option<Vec<String>>,

    /// Respect .gitignore patterns during directory traversal
    #[arg(long)]
    pub gitignore: bool,

    /// Exclude method-level analysis
    #[arg(long)]
    pub no_methods: bool,
}
