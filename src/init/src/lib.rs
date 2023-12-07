use git2::{Repository, Error};
use std::io::{self, Write};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io::Result as IoResult;
use std::process::Command;
use std::fs::DirBuilder;

/// 使用命令行工具初始化位于指定路径的新 Git 仓库
fn init_repo(path: &str) -> IoResult<()> {
    let status = Command::new("git")
        .arg("init")
        .arg(path)
        .status()?;

    let repo_path = Path::new(path);

    // 创建主目录 .my-dvcs
    let dvcs_path = repo_path.join(".my-dvcs");
    DirBuilder::new().recursive(true).create(&dvcs_path)?;

    // 创建 obj, branches 和 heads 子目录
    for subdir in ["obj", "branches", "heads"].iter() {
        DirBuilder::new().recursive(true).create(dvcs_path.join(subdir))?;
    }
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to initialize git repository"))
    }
}



/// 从本地路径克隆 Git 仓库到指定路径
fn clone_local(src_path: &str, dest_path: &str) -> io::Result<()> {
    // 确保源路径存在且是一个目录
    let src = Path::new(src_path);
    if !src.is_dir() {
        return Err(io::Error::new(io::ErrorKind::Other, "Source is not a directory"));
    }

    // 创建目标路径
    let dest = Path::new(dest_path);
    fs::create_dir_all(dest)?;

    // 递归地复制文件和目录
    copy_dir_recursive(src, dest)
}

/// 递归地复制目录
fn copy_dir_recursive(src: &Path, dest: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if entry_path.is_dir() {
            fs::create_dir_all(&dest_path)?;
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(entry_path, dest_path)?;
        }
    }
    Ok(())
}

/// 克隆远程 Git 仓库到指定路径
fn clone_repo(url: &str, path: &str) -> Result<Repository, Error> {
    Repository::clone(url, path)
}


/// 克隆远程 Git 仓库到指定路径并删除.git目录
fn clone_repo_and_remove_git(url: &str, path: &str) -> Result<(), Error> {
    let repo = Repository::clone(url, path)?;
    let git_dir = Path::new(path).join(".git");

    // 检查.git目录是否存在，然后删除
    if git_dir.exists() && git_dir.is_dir() {
        fs::remove_dir_all(git_dir).map_err(|e| {
            Error::from_str(&format!("Failed to remove .git directory: {}", e))
        })?;
    }

    Ok(())
}



/// 从用户获取输入
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // 确保立即显示提示
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

fn main() { 
    // 获取用户输入的仓库路径
    let repo_path = get_input("Enter the path to initialize a new repository: ");

    // 尝试初始化仓库
    match init_repo(&repo_path) {
        Ok(_) => println!("Repository initialized at {}", repo_path),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }


    // 获取用户输入的克隆方法
    let clone_method = get_input("Choose clone method (1: git2 clone, 2: local clone): ");

    // 获取克隆的 URL/路径 和目标路径
    let clone_source = get_input("Enter the URL/path of the repository to clone: ");
    let clone_path = get_input("Enter the path to clone the repository to: ");

    // 根据用户选择的方法克隆仓库
    match clone_method.as_str() {
        "1" => {
            match clone_repo_and_remove_git(&clone_source, &clone_path) {
                Ok(_) => println!("Repository cloned and .git removed at {}", clone_path),
                Err(e) => println!("Failed to clone repository: {}", e),
            }
        },
        "2" => {
            match clone_local(&clone_source, &clone_path) {
                Ok(_) => println!("Local repository cloned to {}", clone_path),
                Err(e) => println!("Failed to clone local repository: {}", e),
            }
        },
        _ => println!("Invalid clone method selected."),
    }

}












#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;


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





    // Helper function to create a file with content
    fn create_file(dir: &Path, file_name: &str, content: &[u8]) {
        let file_path = dir.join(file_name);
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(content).expect("Failed to write to file");
    }

    #[test]
    fn test_clone_local_with_file() {
        let src_dir = tempdir().expect("Failed to create temporary directory");
        let dest_dir = tempdir().expect("Failed to create temporary directory");

        // Create a file in the source directory
        create_file(src_dir.path(), "test.txt", b"Hello, world!");

        // Clone the local repository
        clone_local(src_dir.path().to_str().unwrap(), dest_dir.path().to_str().unwrap()).expect("Failed to clone local repository");

        // Check if the file exists in the destination directory
        assert!(dest_dir.path().join("test.txt").exists());
    }

    #[test]
    fn test_clone_local_with_empty_directory() {
        let src_dir = tempdir().expect("Failed to create temporary directory");
        let dest_dir = tempdir().expect("Failed to create temporary directory");

        // Clone the local repository
        clone_local(src_dir.path().to_str().unwrap(), dest_dir.path().to_str().unwrap()).expect("Failed to clone local repository");

        // Check if the destination directory is created and empty
        assert!(dest_dir.path().is_dir());
        assert_eq!(fs::read_dir(dest_dir.path()).expect("Failed to read directory").count(), 0);
    }
}
