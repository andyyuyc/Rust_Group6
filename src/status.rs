
use reqwest;
use serde_json::Value;
use std::process::Command;
use std::io;
use std::fs::File;
use std::io::Write;
use regex::Regex;

fn git_status(path: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("status")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Git command failed"))
    }
}

fn git_log(path: &str) -> Result<String, std::io::Error> {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .arg("log")
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Git log command failed"))
    }
}

async fn get_github_commits(repo_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"github\.com/([^/]+)/([^/.]+)").unwrap();
    let caps = re.captures(repo_url).ok_or("Invalid GitHub URL")?;
    
    let user = caps.get(1).map_or("", |m| m.as_str());
    let repo = caps.get(2).map_or("", |m| m.as_str());

    let api_url = format!("https://api.github.com/repos/{}/{}/commits", user, repo);

    let client = reqwest::Client::new();
    let res = client.get(&api_url)
        .header("User-Agent", "request")
        .send()
        .await?;

    let commits: Value = res.json().await?;

    let mut commits_info = String::new();

    if let Value::Array(commits_array) = commits {
        for commit in commits_array {
            let committer = &commit["commit"]["committer"];
            let time = committer["date"].as_str().unwrap_or("");
            let name = committer["name"].as_str().unwrap_or("");
            let events_url = commit["committer"]["events_url"].as_str().unwrap_or("");
            let sha = commit["sha"].as_str().unwrap_or("");
            let url = commit["url"].as_str().unwrap_or("");

            commits_info.push_str(&format!("Time: {},\nName: {},\nEvents URL: {},\nSHA: {},\nURL: {},\n\n", time, name, events_url, sha, url));
        }
    }

    Ok(commits_info)
}

#[tokio::main]
pub async fn status() -> Result<(), Box<dyn std::error::Error>> {
    println!("Select the operation:");
    println!("1. Get GitHub commit history");
    println!("2. Perform a local git status check");
    println!("3. View the change log");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => {
            println!("Enter the GitHub repository URL:");
            let mut repo_url = String::new();
            io::stdin().read_line(&mut repo_url)?;
            let repo_url = repo_url.trim();

            match get_github_commits(repo_url).await {
                Ok(commits_info) => {
                    println!("GitHub commits fetched successfully.");

                    let file_path = "github_commits.txt";
                    let mut file = File::create(file_path)?;
                    file.write_all(commits_info.as_bytes())?;

                    println!("Commits have been saved to {}", file_path);
                },
                Err(e) => println!("Error occurred while fetching GitHub commits: {}", e),
            }
        }
        "2" => {
            println!("Enter the repository path:");
            let mut path = String::new();
            io::stdin().read_line(&mut path)?;
            let path = path.trim();
        
            match git_status(path) {
                Ok(status) => println!("Git Status:\n{}", status),
                Err(e) => println!("Error occurs: {}", e),
            }
        }
        "3" => {
            println!("Enter the repository path:");
            let mut path = String::new();
            io::stdin().read_line(&mut path)?;
            let path = path.trim();

            match git_log(path) {
                Ok(log) => {
                    println!("Git Log:\n{}", log);

                    let log_file_path = format!("{}/git_log.txt", path);
                    let mut file = File::create(log_file_path)?;
                    file.write_all(log.as_bytes())?;

                    println!("Log has been saved to git_log.txt in the specified directory.");
                }
                Err(e) => println!("Error occurs: {}", e),
            }
        }
        _ => println!("Invalid choice"),
    }

    Ok(())
}

pub mod status_checker {
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
