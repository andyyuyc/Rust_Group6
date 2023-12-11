use std::{io::{self, Write, Error}, fs::File, path::PathBuf};
use crate::file_management::{commit::{self, commit}};

use crate::{interface::io::RepositoryInterface, file_management::{commit::Commit, hash::DVCSHash, directory::Directory}};

/// Replaces the files in the repository with that of a specific commit.
/// Also updates the head to the specified commit
pub fn checkout(file_system: RepositoryInterface, commit: Commit) -> io::Result<()> {
    // Move the head to the commit (set it to the hash)
    let curr_hash = commit.get_hash();
    let curr_branch = file_system.get_branch_from_hash(curr_hash)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;
    file_system.update_current_head(&curr_branch);

    // Remove the files in the current directory
    file_system.clear_directory()
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to clear directory for staging"))?;

    // Get the directory to reconstruct from and then reconstruct the files
    let directory: Directory = file_system.get_serialized_object(commit.get_dir_hash())?;
    directory.get_key_value_pairs()
        .try_for_each(|(dir_path, blob_ref)| {
            // Get data from the blob 
            let data = file_system.get_object(blob_ref.get_content_hash().clone())?;

            // Reconstruct the directory structure
            std::fs::create_dir_all(&dir_path.parent().unwrap())?;

            // Recreate the file and copy the data to it
            let directory_path = file_system.get_repo_path().join(&dir_path);
            let mut file = File::create(&directory_path)?;
            file.write_all(&data)?;
            file.flush()?;

            Ok::<(), Error>(())
        })?;

    Ok(())
}

pub fn checkout_cmd(branch_name: &str) -> std::io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?;
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Get the head commit for the branch
    let branch_hash = repo.get_hash_from_branch(branch_name)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve branch. Branch does not exist"))?;
    let commit = repo.get_serialized_object(branch_hash)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve commit for checkout"))?;

    // Checkout to the head commit
    checkout(repo, commit)
        .map_err(|x| x)
}


#[test]
fn checkout_test() -> std::io::Result<()> {
    let repo = RepositoryInterface::new(&PathBuf::from("test")).unwrap();

    let path1 = PathBuf::from("test.txt");
    let path2 = PathBuf::from("idk/test.txt");

    let mut file_paths = vec![
        path1,
        path2
    ];

    let dir = repo.create_dir_from_files(&file_paths)?;
    let commit = commit("Justin", &vec![], dir, "Initial commit", &repo).unwrap();

    checkout(repo, commit);

    Ok(())
}
