use std::process::ExitCode;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const REPO_OWNER: &str = "StrangeDaysTech";
const REPO_NAME: &str = "arborist-cli";

pub fn run(check_only: bool) -> ExitCode {
    println!("arborist-cli v{CURRENT_VERSION}");

    if is_cargo_installed() {
        eprintln!(
            "hint: this binary appears to be installed via cargo. \
             To update, run: cargo install arborist-cli"
        );
        if check_only {
            return check_latest_version();
        }
        return ExitCode::SUCCESS;
    }

    if check_only {
        return check_latest_version();
    }

    perform_update()
}

fn check_latest_version() -> ExitCode {
    print!("Checking for updates... ");

    match self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("arborist")
        .current_version(CURRENT_VERSION)
        .build()
    {
        Ok(updater) => match updater.get_latest_release() {
            Ok(release) => {
                let latest = &release.version;
                if latest == CURRENT_VERSION {
                    println!("already up to date.");
                    ExitCode::SUCCESS
                } else {
                    println!("v{latest} available (current: v{CURRENT_VERSION})");
                    println!("Run `arborist update` to install it.");
                    ExitCode::SUCCESS
                }
            }
            Err(e) => {
                eprintln!("failed to check for updates: {e}");
                ExitCode::from(2)
            }
        },
        Err(e) => {
            eprintln!("failed to configure updater: {e}");
            ExitCode::from(2)
        }
    }
}

fn perform_update() -> ExitCode {
    println!("Checking for updates...");

    let result = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name("arborist")
        .current_version(CURRENT_VERSION)
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(true)
        .build();

    match result {
        Ok(updater) => match updater.update() {
            Ok(status) => {
                if status.updated() {
                    println!("Updated to v{}.", status.version());
                } else {
                    println!("Already up to date (v{CURRENT_VERSION}).");
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: update failed: {e}");
                ExitCode::from(2)
            }
        },
        Err(e) => {
            eprintln!("error: failed to configure updater: {e}");
            ExitCode::from(2)
        }
    }
}

/// Heuristic: if the binary is inside a cargo bin directory, it was likely
/// installed via `cargo install`.
fn is_cargo_installed() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let path = exe.to_string_lossy();
    path.contains(".cargo/bin") || path.contains(".cargo\\bin")
}
