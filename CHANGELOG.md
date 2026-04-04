# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-04-04

### Added

- `arborist about` subcommand displaying project metadata (version, description, author, license, repository, website).

### Removed

- `homepage` field from Cargo.toml — repository URL is sufficient for crates.io.

## [0.1.1] - 2026-04-03

### Fixed

- Binary name set to `arborist` instead of `arborist-cli`.

## [0.1.0] - 2026-04-03

### Added

- Core CLI with cognitive complexity, cyclomatic complexity, and SLOC analysis powered by `arborist-metrics`.
- Multiple output formats: table, JSON, and CSV.
- `--threshold` and `--exceeds-only` flags for filtering by cognitive complexity.
- `--sort` flag to sort results by cognitive, cyclomatic, SLOC, or name.
- `--top N` flag to limit output to the top N results.
- `--languages` filter for directory traversal by language.
- `--gitignore` flag to respect `.gitignore` patterns during traversal.
- `--no-methods` flag to exclude method-level analysis.
- `--language` flag for specifying language when reading from stdin.
- `arborist update` subcommand for self-updating via GitHub releases.
- cargo-dist release infrastructure for cross-platform binary distribution.
- CI workflow with clippy lints and test suite.
- Published to crates.io as `arborist-cli`.

[Unreleased]: https://github.com/StrangeDaysTech/arborist-cli/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/StrangeDaysTech/arborist-cli/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/StrangeDaysTech/arborist-cli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/StrangeDaysTech/arborist-cli/releases/tag/v0.1.0
