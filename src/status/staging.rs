pub mod stager {
    use std::fs::{File};
    use std::io::{self, Write, Read, stdin, stdout};
    use std::path::Path;
    use std::collections::HashSet;
    
    // Adds a file to the staging area
    pub fn stage_add(repository_path: &str, file_path: &str) -> io::Result<()> {
        let repo_path = Path::new(repository_path);
        validate_repository_path(&repo_path)?;
    
        let file_to_track = Path::new(file_path);
        if !file_to_track.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File does not exist"));
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
    
    // Load the set of tracked files from the repository
    fn read_files(repo_path: &Path) -> io::Result<HashSet<String>> {
        let file_path = repo_path.join(".tracked_files");
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
        let file_path = repo_path.join(".tracked_files");
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
    
}