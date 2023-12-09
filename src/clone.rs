use git2::{Repository, Error};
use std::io::{self, Write};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io::Result as IoResult;
use std::process::Command;
use std::fs::DirBuilder;



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

pub fn clone() { 
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