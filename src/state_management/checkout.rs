use std::{io::{self, Write, Error}, fs::File};
use crate::file_management::{hash::Hash, commit::{self, commit}};

use crate::{interface::io::{get_serialized_object, get_object}, file_management::{commit::Commit, hash::DVCSHash, directory::Directory}};

pub fn checkout(commit: Commit) -> io::Result<()> {
    // Move the head to the branch (set it to the hash)


    // Remove the files in the current directory


    // Get the directory to reconstruct from and then reconstruct the files
    let directory: Directory = get_serialized_object(commit.get_dir_hash())?;
    directory.get_key_value_pairs()
        .try_for_each(|(dir_path, blob_ref)| {
            // Get data from the blob 
            let data = get_object(blob_ref.get_content_hash().clone())?;

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
fn checkout_test() {
    let mut changes = vec![];
    
    changes.push(commit::Change::Add {path: "test/test.txt".to_owned()});
    changes.push(commit::Change::Add {path: "test/test2.txt".to_owned()});
    changes.push(commit::Change::Add {path: "test/idk/test.txt".to_owned()});

    let commit = commit("Justin", None, &changes, "Added test2").unwrap();

    std::fs::remove_dir_all("test");
    checkout(commit);
}
