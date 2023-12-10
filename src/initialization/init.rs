
// use git2::{Repository, Error};
use std::io::{self, Write};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::io::Result as IoResult;
use std::process::Command;
use std::fs::DirBuilder;
use crate::interface::io::RepositoryInterface;


pub fn init_repo(path: &str) -> IoResult<()> {
    let repo_path = Path::new(path);

    // Proceed with the repository initialization
    let status = Command::new("git")
        .arg("init")
        .arg(path)
        .status()?;
    
    // Create the home directory .my-dvcs
    let dvcs_path = repo_path.join(".my-dvcs");
    DirBuilder::new().recursive(true).create(&dvcs_path)?;

    // Create the obj, branches and heads subdirectories.
    for subdir in ["obj", "branches"].iter() {
        DirBuilder::new().recursive(true).create(dvcs_path.join(subdir))?;
    }

    // Create a head file in the branches directory with the name of the default branch
    let default_branch = ""; // Set the name of the default branch here
    let head_path = dvcs_path.join("head");
    fs::write(head_path, default_branch)?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to initialize repository"))
    }
}

/// Getting input from the user
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompts are displayed immediately
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

pub fn init() { 
    // Get the path to the repository as entered by the user
    let repo_path = get_input("Enter the path to initialize a new repository: ");

    // Try to initialize the repository
    match init_repo(&repo_path) {
        Ok(_) => println!("Repository initialized at {}", repo_path),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }
}









