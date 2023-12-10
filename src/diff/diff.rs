use crate::{file_management::directory::Directory, interface::io::RepositoryInterface};
use std::io::{self, Error};


pub fn diff_cmd() -> std::io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?; 
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Get the current commit (head)
    


    Ok(())
}

pub fn diff(dir1: Directory, dir2: Directory) {

}
