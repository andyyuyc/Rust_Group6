use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use walkdir::WalkDir;

pub fn track_status(repo_path: &Path) -> io::Result<(HashSet<String>, HashSet<String>)> {
    let mut tracked_files = HashSet::new();
    let mut untracked_files = HashSet::new();

    // 读取已跟踪的文件
    let file_path = repo_path.join(".my-dvcs/.tracked_files");
    if file_path.exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        tracked_files = contents.lines().map(|s| s.to_string()).collect();
    }

    // 使用 walkdir 遍历整个仓库目录
    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file()) 
    {
        let path = entry.path();
        let relative_path = path.strip_prefix(repo_path).unwrap().to_str().unwrap().to_string();
        if !tracked_files.contains(&relative_path) {
            untracked_files.insert(relative_path);
        }
    }

    Ok((tracked_files, untracked_files))
}