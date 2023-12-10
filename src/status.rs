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





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_status_invalid() {
        let path = "invalid_path"; // Test for an invalid path
        let result = git_status(path);
        assert!(result.is_err());
    }


    #[test]
    fn test_git_log_invalid() {
        let path = "invalid_path"; // Test for an invalid path
        let result = git_log(path);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_github_commits_valid() {
        let repo_url = "https://github.com/andyyuyc/Jersey";  // Replace the GitHub repository URL with a valid one.
        let result = get_github_commits(repo_url).await;
        assert!(result.is_ok());
    }

}
