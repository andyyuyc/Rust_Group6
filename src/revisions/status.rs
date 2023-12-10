use std::process::Command;
use std::{path::PathBuf};

fn get_git_status() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();
    let path = std::env::current_dir().unwrap_or(PathBuf::from("."));
    let path_as_str = &path.as_os_str().to_string_lossy().to_string();

    let output = Command::new("git")
        .arg("-C")
        .arg(path_as_str)
        .arg("status")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(String::from_utf8_lossy(&output.stderr).to_string())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn status() -> Result<(), String> {
    match get_git_status() {
        Ok(status) => {
            println!("Git Status:\n{}", status);
            Ok(())
        }
        Err(e) => Err(e),
    }
}