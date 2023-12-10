use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

pub fn track_status(repo_path: &Path) -> io::Result<(HashSet<String>, HashSet<String>)> {
    let mut tracked_files = HashSet::new();
    let mut untracked_files = HashSet::new();

    //Tracked file
    let file_path = repo_path.join(".my-dvcs/.tracked_files");
    if file_path.exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        tracked_files = contents.lines().map(|s| s.to_string()).collect();
    }

    //Untracked file
    if repo_path.exists() && repo_path.is_dir() {
        for entry in fs::read_dir(repo_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                
                let relative_path = path.strip_prefix(repo_path).unwrap().to_str().unwrap().to_string();
                if !tracked_files.contains(&relative_path) {
                    untracked_files.insert(relative_path);
                }
            }
        }
    }

    Ok((tracked_files, untracked_files))
}