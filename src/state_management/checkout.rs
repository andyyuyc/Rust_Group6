use std::{io::{self, Write, Error}, fs::File, path::PathBuf};
use crate::file_management::{hash::Hash, commit::{self, commit}};

use crate::{interface::io::RepositoryInterface, file_management::{commit::Commit, hash::DVCSHash, directory::Directory}};

pub fn checkout(file_system: RepositoryInterface, commit: Commit) -> io::Result<()> {
    // Move the head to the branch (set it to the hash)

    // Remove the files in the current directory

    // Get the directory to reconstruct from and then reconstruct the files
    let directory: Directory = file_system.get_serialized_object(commit.get_dir_hash())?;
    directory.get_key_value_pairs()
        .try_for_each(|(dir_path, blob_ref)| {
            // Get data from the blob 
            let data = file_system.get_object(blob_ref.get_content_hash().clone())?;

            // Reconstruct the directory structure
            // Might throw an error if there is no parent
            // println!("Directory Path: {}", &dir_path.display());
            std::fs::create_dir_all(&dir_path.parent().unwrap())?;
                    
            // Recreate the file and copy the data to it
            let mut file = File::create(&dir_path)?;
            file.write_all(&data)?;
            file.flush()?;

            Ok::<(), Error>(())
        })?;

    Ok(())
}

#[test]
fn checkout_test() -> std::io::Result<()> {
    let repo = RepositoryInterface::new(&PathBuf::from(".")).unwrap();

    let path1 = PathBuf::from("test/test.txt");
    let path2 = PathBuf::from("test/idk/test.txt");

    let mut file_paths = vec![
        &path1,
        &path2
    ];

    let dir = repo.create_dir_from_files(&file_paths)?;
    let commit = commit("Justin", &vec![], dir, "Initial commit", &repo).unwrap();

    std::fs::remove_dir_all("test");
    checkout(repo, commit);

    Ok(())
}