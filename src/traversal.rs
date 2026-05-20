// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::error::ArboristError;

/// Known file extensions mapped to language names for filtering.
fn extension_to_language(ext: &str) -> Option<&'static str> {
    match ext {
        "rs" => Some("rust"),
        "py" => Some("python"),
        "js" => Some("javascript"),
        "ts" => Some("typescript"),
        "tsx" => Some("tsx"),
        "jsx" => Some("jsx"),
        "java" => Some("java"),
        "go" => Some("go"),
        "c" | "h" => Some("c"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("cpp"),
        "cs" => Some("c_sharp"),
        "rb" => Some("ruby"),
        "swift" => Some("swift"),
        "kt" | "kts" => Some("kotlin"),
        "php" => Some("php"),
        _ => None,
    }
}

pub fn collect_files(
    paths: &[PathBuf],
    languages_filter: Option<&[String]>,
    gitignore: bool,
) -> Result<Vec<PathBuf>, ArboristError> {
    let mut files = Vec::new();

    for path in paths {
        if path.is_file() {
            files.push(path.clone());
        } else if path.is_dir() {
            collect_directory(path, languages_filter, gitignore, &mut files)?;
        } else {
            return Err(ArboristError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("file not found: {}", path.display()),
            )));
        }
    }

    Ok(files)
}

fn collect_directory(
    dir: &Path,
    languages_filter: Option<&[String]>,
    gitignore: bool,
    files: &mut Vec<PathBuf>,
) -> Result<(), ArboristError> {
    let walker = WalkBuilder::new(dir)
        .git_ignore(gitignore)
        .git_global(false)
        .git_exclude(false)
        .hidden(false)
        .build();

    let languages_lower: Option<Vec<String>> =
        languages_filter.map(|langs| langs.iter().map(|l| l.to_lowercase()).collect());

    for entry in walker {
        let entry = entry.map_err(|e| ArboristError::Io(std::io::Error::other(e.to_string())))?;

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext,
            None => continue,
        };

        let lang = match extension_to_language(ext) {
            Some(lang) => lang,
            None => continue,
        };

        if let Some(ref filter) = languages_lower
            && !filter.iter().any(|f| f == lang)
        {
            continue;
        }

        files.push(path.to_path_buf());
    }

    files.sort();
    Ok(())
}
