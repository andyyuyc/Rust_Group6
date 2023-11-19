use std::io;
use std::path::Path;
use std::process::Command;


fn is_valid_dir_path(dir_path: &str) -> bool {
    let path = Path::new(dir_path);
    path.exists() && path.is_dir()
}

fn is_valid_branch(branch_name: &str) -> bool {
    !branch_name.is_empty() 
}

fn pull_changes(remote_repo_path: &str, local_branch: &str) -> io::Result<()> {
    if is_valid_dir_path(remote_repo_path) && is_valid_branch(local_branch) {
        let status = Command::new("git")
            .arg("pull")
            .arg(remote_repo_path)
            .arg(local_branch)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to pull changes"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path or branch"))
    }
}

fn push_changes(local_repo_path: &str, remote_branch: &str) -> io::Result<()> {
    if is_valid_dir_path(local_repo_path) && is_valid_branch(remote_branch) {
        let status = Command::new("git")
            .current_dir(local_repo_path)
            .arg("push")
            .arg("origin")
            .arg(remote_branch)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to push changes"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path or branch"))
    }
}

fn detect_changes(local_repo_path: &str, remote_repo_path: &str) -> io::Result<()> {
    if is_valid_dir_path(local_repo_path) && is_valid_dir_path(remote_repo_path) {
        let status = Command::new("git")
            .current_dir(local_repo_path)
            .arg("diff")
            .arg(remote_repo_path)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Failed to detect changes"))
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path"))
    }
}

fn synchronize_changes(local_repo_path: &str, remote_repo_path: &str) -> io::Result<()> {
    if is_valid_dir_path(local_repo_path) && is_valid_dir_path(remote_repo_path) {
        pull_changes(remote_repo_path, "master")?;

        push_changes(local_repo_path, "master")?;

        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Invalid path"))
    }
}
