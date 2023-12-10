use std::io::{self, Error};

use crate::interface::io::RepositoryInterface;

/// Creates a branch with the specified branch_name
pub fn create_branch_cmd(branch_name: &str) -> std::io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?; 
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Create a new branch with the hash of the current commit
    let curr_hash = repo.get_current_head()
        .ok_or(Error::new(io::ErrorKind::Other, "Failed to retrieve head hash"))?;
    repo.create_branch(branch_name, curr_hash)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to create branch"))
}

pub fn get_branches_cmd() -> std::io::Result<String> {
    // Initialize repository
    let curr_path = std::env::current_dir()?; 
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Retrieve branches file names and concatenate them
    let branches = repo.get_branches()
        .ok_or(Error::new(io::ErrorKind::Other, "Failed to retrieve branches. There may be none"))
        .and_then(|vec| {
            Ok(vec.join("\n"))
        });

    branches
}