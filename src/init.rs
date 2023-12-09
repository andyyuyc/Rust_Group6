use git2::{Repository, Error};
use std::io::{self, Write};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io::Result as IoResult;
use std::process::Command;
use std::fs::DirBuilder;

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
    for subdir in ["obj", "branches"].iter() {
        DirBuilder::new().recursive(true).create(dvcs_path.join(subdir))?;
    }

    // 在 branches 目录下创建一个 head 文件，并写入默认分支的名称
    let branches_path = dvcs_path.join("branches");
    let default_branch = ""; // 这里设置默认分支的名称
    let head_path = branches_path.join("HEAD");
    fs::write(head_path, default_branch)?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to initialize git repository"))
    }
}




/// 从用户获取输入
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // 确保立即显示提示
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

pub fn init() { 
    // 获取用户输入的仓库路径
    let repo_path = get_input("Enter the path to initialize a new repository: ");

    // 尝试初始化仓库
    match init_repo(&repo_path) {
        Ok(_) => println!("Repository initialized at {}", repo_path),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }


}










