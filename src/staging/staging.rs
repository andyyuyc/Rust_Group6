use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::env;

// Adds a file to the staging area
pub fn stage_add(repository_path: &str, file_path: &str) -> io::Result<()> {
    let repo_path = Path::new(repository_path);
    validate_repository_path(&repo_path)?;

    let file_path = repo_path.join(file_path);
    if !file_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File does not exist"));
    }

    let staging_path = repo_path.join(".staging");
    fs::create_dir_all(&staging_path)?;

    let destination = staging_path.join(file_path.file_name().unwrap());
    fs::copy(file_path, destination)?;

    Ok(())
}

// Removes a file from the staging area
pub fn stage_remove(repository_path: &str, file_path: &str) -> io::Result<()> {
    let repo_path = Path::new(repository_path);
    validate_repository_path(&repo_path)?;

    let staging_path = repo_path.join(".staging");
    let file_to_remove = staging_path.join(Path::new(file_path).file_name().unwrap());

    if file_to_remove.exists() {
        fs::remove_file(file_to_remove)?;
    } else {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found in staging area"));
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

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("Usage: {} <repository_path> <add/remove> <file_path>", args[0]);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Insufficient arguments"));
    }

    let repository_path = &args[1];
    let command = &args[2];
    let file_path = &args[3];

    match command.as_str() {
        "add" => {
            stage_add(repository_path, file_path)?;
            println!("File successfully added to the staging area.");
        },
        "remove" => {
            stage_remove(repository_path, file_path)?;
            println!("File successfully removed from the staging area.");
        },
        _ => {
            eprintln!("Invalid command: {}", command);
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid command"));
        }
    }

    Ok(())
}
