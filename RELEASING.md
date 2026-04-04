# Releasing arborist-cli

This project uses [cargo-dist](https://opensource.axo.dev/cargo-dist/) to automate cross-platform builds and GitHub Releases.

## Release procedure

1. Ensure `main` is clean and CI passes.

2. Bump the version in `Cargo.toml`:
   ```bash
   # Edit Cargo.toml: version = "0.2.0"
   ```

3. Commit and tag:
   ```bash
   git commit -am "chore: release v0.2.0"
   git tag v0.2.0
   git push origin main --tags
   ```

4. The `Release` workflow triggers automatically and:
   - Builds binaries for all targets (Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64)
   - Creates shell and PowerShell installer scripts
   - Creates a GitHub Release with all artifacts

## Target platforms

| Target | OS |
|--------|----|
| `x86_64-unknown-linux-gnu` | Linux (x86_64) |
| `aarch64-unknown-linux-gnu` | Linux (ARM64) |
| `x86_64-apple-darwin` | macOS (Intel) |
| `aarch64-apple-darwin` | macOS (Apple Silicon) |
| `x86_64-pc-windows-msvc` | Windows (x86_64) |

## Pre-releases

Use a pre-release suffix in the tag to mark it as a pre-release on GitHub:

```bash
git tag v0.2.0-rc.1
git push origin --tags
```

## Publishing to crates.io

To also publish to crates.io, add a `publish-jobs` entry to `dist-workspace.toml`:

```toml
publish-jobs = ["./publish-crates-io"]
```

Or publish manually:

```bash
cargo publish
```

## Regenerating CI

If you update `dist-workspace.toml`, regenerate the workflow:

```bash
dist generate
```
