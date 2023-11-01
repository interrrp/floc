use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn find_files_with_extension(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in walkdir::WalkDir::new(dir) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().is_none() {
            continue;
        }

        if path.is_file() && path.extension().unwrap() == ext {
            files.push(path.to_path_buf());
        }
    }

    files
}

pub fn count_lines(path: &Path) -> usize {
    let contents = fs::read_to_string(path).unwrap();
    contents.lines().count()
}
