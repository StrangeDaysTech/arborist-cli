# Contributing to arborist-cli

Thank you for your interest in contributing! This document covers the process for submitting changes.

## Contributor License Agreement (CLA)

All contributors must sign the CLA before their pull request can be merged. The CLA bot will automatically comment on your first PR with instructions. You only need to sign once — it covers all future contributions to Strange Days Tech repositories.

If the CLA check fails, follow the link in the bot's comment and sign using your GitHub account.

## Reporting issues

Please report bugs and feature requests through [GitHub Issues](https://github.com/StrangeDaysTech/arborist-cli/issues). Include:

- Your OS and Rust toolchain version (`rustc --version`)
- The command you ran and the input file (if shareable)
- Expected vs. actual output
- The full error message or backtrace, if applicable

## Submitting changes

1. Fork the repository and create a branch from `main`:
   - `feature/` or `feat/` — new functionality
   - `fix/` — bug fixes
   - `docs/` — documentation changes
   - `refactor/` — code improvements without behavior change
   - `test/` — test additions or fixes

2. Write your code. Run the checks:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```

3. Use [conventional commits](https://www.conventionalcommits.org/):
   ```
   feat: add Python 3.12 match-case support
   fix: handle empty files without panic
   docs: clarify --threshold behavior in README
   ```

4. Open a pull request against `main`. The CLA bot and CI checks must pass before review.

## Development setup

```bash
git clone https://github.com/StrangeDaysTech/arborist-cli.git
cd arborist-cli
cargo build
cargo test
```

Requires Rust 1.85+ (Edition 2024).

## Code of conduct

Be respectful. We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct).

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 dual license, at the user's choice.
