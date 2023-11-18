use std::{io::{self, Write, Error}, fs::File};
use crate::file_management::{hash::Hash, commit::{self, commit}};

use crate::{interface::io::{get_branch, get_serialized_object, get_object}, file_management::{commit::Commit, hash::DVCSHash, directory::Directory}};

fn checkout(hash: Hash) -> io::Result<()> {
    // Grab the hash of the branch from 
    let commit: Commit = get_serialized_object(hash.clone())?;

    // Move the head to the branch (set it to the hash)


    // Get the directory to reconstruct from and then reconstruct the files
    let directory: Directory = get_serialized_object(commit.get_dir_hash())?;
    directory.get_key_value_pairs()
        .try_for_each(|(dir_path, blob_ref)| {
            // Get data from the blob 
            let data = get_object(blob_ref.get_content_hash().clone())?;

            // Reconstruct the directory structure
            // Might throw an error if there is no parent
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
    use crate::interface::io::*;
    use serde::{Serialize, Deserialize};
    use std::io::Read;

    let mut changes = vec![];
    
    changes.push(commit::Change::Add {path: "test/test.txt"});
    changes.push(commit::Change::Add {path: "test/test2.txt"});
    changes.push(commit::Change::Add {path: "test/chigen/chigen.txt" });

    let commit = commit("Justin", None, &changes, "Initial commit").unwrap();

    checkout(commit.get_hash());
}