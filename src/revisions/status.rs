use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::fs::DirEntry;


fn read_tracked_files(file_path: &Path) -> io::Result<HashSet<String>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|s| s.to_string()).collect())
}

fn list_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            files.extend(list_files(&path)?);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}

pub fn track_status(repo_path: &Path) -> io::Result<(HashSet<String>, HashSet<String>)> {
    let mut tracked_files = HashSet::new();
    let mut untracked_files = HashSet::new();

    // 读取已跟踪的文件
    let tracked_file_path = repo_path.join(".my-dvcs/.tracked_files");
    if tracked_file_path.exists() {
        tracked_files = read_tracked_files(&tracked_file_path)?;
    }

    // 遍历仓库目录
    for file_path in list_files(repo_path)? {
        let relative_path = file_path.strip_prefix(repo_path).unwrap().to_str().unwrap().to_string();
        if tracked_files.contains(&relative_path) {
            // 文件被跟踪
            tracked_files.insert(relative_path);
        } else {
            // 文件未被跟踪
            untracked_files.insert(relative_path);
        }
    }

    Ok((tracked_files, untracked_files))
}


// Excludes .my-dvcs from being added to the staging area
fn is_excluded(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with(".my-dvcs"))
        .unwrap_or(false)
}