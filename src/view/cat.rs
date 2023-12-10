use std::{path::PathBuf, io::{self, Error, Write}};
use crate::{interface::io::RepositoryInterface, file_management::{commit::Commit, directory::Directory}};
use crate::file_management::hash::Hash;


/// Inspect a file of a given revision
pub fn cat_cmd(hash: Hash, file_name: &PathBuf) -> io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?;
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Retrieve file
    let commit: Commit = repo.get_serialized_object(hash)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve commit"))?;
    let dir: Directory = repo.get_serialized_object(commit.get_dir_hash())
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve directory"))?;
    let blobref = dir.get_file_ref(&file_name)
        .ok_or(Error::new(io::ErrorKind::Other, "Failed to retrieve blobref"))?;
    let file = repo.get_object(blobref.get_content_hash().clone())
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve file contents"))?;

    // Write output to stdout
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&file)?;

    Ok(())
}