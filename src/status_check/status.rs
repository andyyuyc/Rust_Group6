pub mod status_checker{
    use std::fs::{self, File};
    use std::io::{self, Read};
    use std::path::Path;
    use std::collections::HashSet;
    use std::time::SystemTime;

    //struct for traversing files
    struct FileTraverser {
        dvcs_hidden: String,
    }

    impl FileTraverser {
        // constructor for FileTraverser
        pub fn new(dvcs_hidden: String) -> Self {
            FileTraverser { dvcs_hidden }
        }

        // recursively traverse files to collect file paths
        pub fn recursive_file_traversal(&self, starting_directory: &str, all_files: &mut HashSet<String>) -> io::Result<()> {
            let entries = match fs::read_dir(starting_directory) {
                Ok(entries) => entries,
                Err(e) => return Err(e),
            };

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.to_str().map_or(false, |s| s.contains(&self.dvcs_hidden)) {
                    continue;
                }

                if path.is_dir() {
                    self.recursive_file_traversal(path.to_str().unwrap(), all_files)?;
                } else if path.is_file() {
                    if let Some(path_str) = path.to_str() {
                        all_files.insert(path_str.to_string());
                    }
                }
            }
            Ok(())
        }
    }

    // function that checks the status of files in the repository
    pub fn check_status(repository_path: &str) -> io::Result<()> {
        let repo_path = Path::new(repository_path);
        validate_repository_path(&repo_path)?;

        let tracked_files = read_files(&repo_path)?;
        let all_files = read_all_files(&repo_path)?;

        let mut staged_files = HashSet::new();
        let mut modified_files = HashSet::new();
        let mut untracked_files = HashSet::new();

        for file_path in all_files {
            if tracked_files.contains(&file_path) {
                if is_file_modified(&repo_path, &file_path)? {
                    modified_files.insert(file_path);
                } else {
                    staged_files.insert(file_path);
                }
            } else {
                untracked_files.insert(file_path);
            }
        }

        display_status(&staged_files, "Staged files:");
        display_status(&modified_files, "Modified files:");
        display_status(&untracked_files, "Untracked files:");

        Ok(())
    }

//function to check for heads.
    pub fn heads(repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let heads_dir = Path::new(repo_path).join(".mydvcs/heads");
    
        if heads_dir.exists() && heads_dir.is_dir() {
            for entry in fs::read_dir(heads_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(branch_name) = path.file_name().and_then(|n| n.to_str()) {
                        println!("{}", branch_name);
                    }
                }
            }
        } else {
            println!("No heads found in the repository.");
        }
    
        Ok(())
    }

    //displaying the status of files
    fn display_status(files: &HashSet<String>, title: &str) {
        println!("{}", title);
        for file in files {
            println!("{}", file);
        }
        println!();
    }

    // function to validates the repository path
    fn validate_repository_path(repo_path: &Path) -> io::Result<()> {
        if !repo_path.exists() || !repo_path.is_dir() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid repository path"));
        }
        Ok(())
    }

    // method that reads the set of tracked files from the repository
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

    // reading all the fules in the directory
    fn read_all_files(repo_path: &Path) -> io::Result<HashSet<String>> {
        let mut file_traverser = FileTraverser::new(".git".to_string()); 
        let mut all_files = HashSet::new();
        file_traverser.recursive_file_traversal(repo_path.to_str().unwrap(), &mut all_files)?;
        Ok(all_files)
    }

    // function to check if a file is modified or not.
    // does not use the hash of committed files for simplicity
    fn is_file_modified(repo_path: &Path, file_path: &str) -> io::Result<bool> {
        let full_path = repo_path.join(file_path);
        let metadata = fs::metadata
        (&full_path)?;
        let modified_time = metadata.modified()?;

        // this is a placeholder for  comparison logic:
        let staged_time = SystemTime::now(); 
        Ok(modified_time > staged_time)
    }
}
