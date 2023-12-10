use std::fs::{self, File};
use std::io::{self, Write, Read, stdin, stdout};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use walkdir::{WalkDir, DirEntry};

// Adds a file to the staging area
pub fn stage_add(repository_path: &str, file_path: &str) -> io::Result<()> {
    let repo_path = Path::new(repository_path);
    validate_repository_path(&repo_path)?;

    let file_to_track = repo_path.join(Path::new(file_path));
    if !file_to_track.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File does not exist"));
    }

    // Invalid input if it is a directory or is a file_to_track
    if file_to_track.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cannot add directories to staging area"));
    } else if file_to_track.starts_with(".my-dvcs") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cannot add dvcs files to staging area"));
    }

    let mut tracked_files = read_files(&repo_path)?;
    if !tracked_files.contains(file_path) {
        tracked_files.insert(file_path.to_string());
        save_files(&repo_path, &tracked_files)?;
        println!("File '{}' has been added to staging area.", file_path);
    } else {
        println!("File '{}' is already in staging area.", file_path);
    }

    Ok(())
}

// Removes a file from the staging area
pub fn stage_remove(repository_path: &str, file_path: &str) -> io::Result<()> {
    let repo_path = Path::new(repository_path);
    validate_repository_path(&repo_path)?;

    let mut tracked_files = read_files(&repo_path)?;

    if tracked_files.remove(file_path) {
        save_files(&repo_path, &tracked_files)?;
        println!("File '{}' has been removed from staging area.", file_path);
    } else {
        println!("File '{}' was not in staging area.", file_path);
    }

    Ok(())
}

// Returns staged files
pub fn get_staged_files(repo_path: &Path) -> io::Result<Vec<PathBuf>> {
    let files = read_files(repo_path)?;

    Ok(
        files.into_iter()
            .map(|x| PathBuf::from(x))
            .collect()
    )
}

pub fn clear_staged_files(repo_path: &Path) -> io::Result<()> {
    let tracked_files = HashSet::new();
    save_files(repo_path, &tracked_files)
}

// Load the set of tracked files from the repository
fn read_files(repo_path: &Path) -> io::Result<HashSet<String>> {
    let file_path = repo_path.join(".my-dvcs/.tracked_files");
    if file_path.exists() {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents.lines().map(|s| s.to_string()).collect())
    } else {
        Ok(HashSet::new())
    }
}

// Save the set of tracked files to the repository
fn save_files(repo_path: &Path, tracked_files: &HashSet<String>) -> io::Result<()> {
    let file_path = repo_path.join(".my-dvcs/.tracked_files");
    let mut file = File::create(file_path)?;

    for file_path in tracked_files {
        writeln!(file, "{}", file_path)?;
    }

    Ok(())
}

// Validates the repository path
fn validate_repository_path(repo_path: &Path) -> io::Result<()> {
    if !repo_path.exists() || !repo_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid repository path"));
    }
    Ok(())
}

/// Recursively adds all files 
pub fn stage_all_files(repo_path: &Path) -> io::Result<()> {
    for entry in WalkDir::new(repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !is_excluded(e)) 
    {
        let path = entry.path();

        // Check if it's a file
        if path.is_file() {
            // Getting the relative path
            if let Ok(relative_path) = path.strip_prefix(repo_path) {
                if let Err(e) = stage_add(
                    &repo_path.display().to_string(), 
                    &relative_path.display().to_string()
                ) {
                    if e.kind() != io::ErrorKind::InvalidInput {
                        return Err(e);
                    }
                }
            } else {
                // Handle the case where strip_prefix fails
                return Err(std::io::Error::new(io::ErrorKind::Other, "Failed to strip prefix from path during staging"));
            }
        }
    }

    Ok(())
}

// Excludes .my-dvcs from being added to the staging area
fn is_excluded(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with(".my-dvcs"))
        .unwrap_or(false)
}