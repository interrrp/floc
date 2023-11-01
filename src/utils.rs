use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn find_files_with_extensions(dir: &Path, exts: &[String]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().is_none() {
            continue;
        }

        if path.is_file() && exts.contains(&path.extension().unwrap().to_str().unwrap().to_owned())
        {
            files.push(path.to_path_buf());
        }
    }

    files
}

pub fn count_lines(path: &Path) -> usize {
    let contents = fs::read_to_string(path).unwrap();
    contents.lines().count()
}

pub fn format_extensions_list(exts: &[String]) -> String {
    let mut formatted = String::new();
    for (idx, ext) in exts.iter().enumerate() {
        let is_last = idx == exts.len() - 1;

        if is_last && exts.len() > 1 {
            formatted.push_str(&format!("and {}", ext));
            continue;
        }

        formatted.push_str(&format!("{}", ext));

        if !is_last && exts.len() > 2 {
            formatted.push(',');
        }

        if !is_last {
            formatted.push(' ');
        }
    }
    formatted
}
