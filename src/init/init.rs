use git2::{Repository, Error};
use std::io::{self, Write};

/// 初始化位于指定路径的新 Git 仓库
fn init_repo(path: &str) -> Result<Repository, Error> {
    Repository::init(path)
}

/// 克隆远程 Git 仓库到指定路径
fn clone_repo(url: &str, path: &str) -> Result<Repository, Error> {
    Repository::clone(url, path)
}

/// 从用户获取输入
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // 确保立即显示提示
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

fn init() { 
    // 获取用户输入的仓库路径
    let repo_path = get_input("Enter the path to initialize a new repository: ");

    // 尝试初始化仓库
    match init_repo(&repo_path) {
        Ok(_) => println!("Repository initialized at {}", repo_path),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }

    // 获取克隆的 URL 和目标路径
    let clone_url = get_input("Enter the URL of the repository to clone: ");
    let clone_path = get_input("Enter the path to clone the repository to: ");

    // 尝试克隆仓库
    match clone_repo(&clone_url, &clone_path) {
        Ok(_) => println!("Repository cloned to {}", clone_path),
        Err(e) => println!("Failed to clone repository: {}", e),
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;

    #[test]
    fn test_init_repo_success() {
        let temp_dir = tempdir::TempDir::new("test_init_repo_success").unwrap();
        let repo_path = temp_dir.path().join("my_repo");
    
        println!("Creating temp directory: {:?}", temp_dir.path());
    
        match init_repo(repo_path.to_str().unwrap()) {
            Ok(repo) => {
                
            }
            Err(e) => {
                eprintln!("Failed to initialize repository: {}", e);
                panic!("Failed to initialize repository");
            }
        }
    }

    #[test]
    fn test_init_repo_failure() {
        // Provide an invalid path to trigger an error
        match init_repo("/invalid/path") {
            Ok(_) => {
                panic!("Expected initialization to fail, but it succeeded.");
            }
            Err(_) => {
                // Expected behavior
            }
        }
    }

    #[test]
    fn test_clone_repo_success() {
        let temp_dir = tempdir::TempDir::new("test_clone_repo_success").unwrap();
        let repo_path = temp_dir.path().join("my_repo");
        let url = "https://github.com/andyyuyc/MyFuelTracker.git";

        match clone_repo(url, repo_path.to_str().unwrap()) {
            Ok(_) => {
                // Perform any additional checks as needed
            }
            Err(_) => {
                panic!("Failed to clone repository");
            }
        }
    }

    #[test]
    fn test_clone_repo_failure_invalid_url() {
        let temp_dir = tempdir::TempDir::new("test_clone_repo_failure").unwrap();
        let repo_path = temp_dir.path().join("my_repo");
        let invalid_url = "invalid_url";

        match clone_repo(invalid_url, repo_path.to_str().unwrap()) {
            Ok(_) => {
                panic!("Expected cloning to fail, but it succeeded.");
            }
            Err(_) => {
                // Expected behavior
            }
        }
    }

    #[test]
    fn test_clone_repo_failure_invalid_path() {
        // Provide an invalid destination path to trigger an error
        match clone_repo("https://github.com/andyyuyc/MyFuelTracker.git", "/invalid/path") {
            Ok(_) => {
                panic!("Expected cloning to fail, but it succeeded.");
            }
            Err(_) => {
                // Expected behavior
            }
        }
    }

    #[test]
    fn test_existing_repository() {
        let temp_dir = tempdir::TempDir::new("test_existing_repository").unwrap();
        let repo_path = temp_dir.path().join("my_repo");

        // Initialize the repository once
        match init_repo(repo_path.to_str().unwrap()) {
            Ok(_) => {
                // Try to initialize it again
                match init_repo(repo_path.to_str().unwrap()) {
                    Ok(repo) => {
                        
                    }
                    Err(_) => {
                        panic!("Failed to reopen existing repository");
                    }
                }
            }
            Err(_) => {
                panic!("Failed to initialize repository");
            }
        }
    }
}
