use std::{io::{self, Read}, path::PathBuf, fs::File};

use serde::{Serialize, Deserialize};
use crate::{file_management::hash::Hash, interface::io::{get_serialized_object, add_serialized_object, add_object}};

use super::{directory::{Directory, BlobRef}, hash::DVCSHash};

pub enum Change<'a> {
    Add { path: &'a str },
    Remove { path: &'a str },
}

// Can possible use builder to build this?
#[derive(Serialize, Deserialize)]
pub struct Commit {
    parent_hashes: Vec<Hash>,
    dir_hash: Hash,
    author: String,
    message: String,
    time_stamp: String,
}

impl Commit {
    pub fn get_dir_hash(&self) -> Hash {
        self.dir_hash.clone()
    }
}

impl DVCSHash for Commit {
    fn get_hash(&self) -> Hash {
        let mut hash = String::new();
        
        let parent_hash = self.parent_hashes.iter()
            .fold(String::new(), |mut str, curr_hash| {
                str.push_str(&curr_hash.as_string());
                str
            });
        hash.push_str(&parent_hash);
        hash.push_str(&self.dir_hash.as_string());
        hash.push_str(&self.author);
        hash.push_str(&self.message);
        hash.push_str(&self.time_stamp);

        Hash::new(&hash)
    }
}

// What if there is no parent
pub fn commit(
    author: &str,
    parent_hash: Option<Hash>, 
    changes: &Vec<Change>,
    message: &str
) -> io::Result<Commit> {
    // Get parent directory content and process revisions to get 
    // a new directory object
    let dir = match &parent_hash {
        Some(hash) => {
            let parent_commit: Commit = get_serialized_object(hash.clone())?;
            let parent_dir: Directory = get_serialized_object(parent_commit.dir_hash)?;
            process_revisions(parent_dir, changes)
        },
        None => {
            process_revisions(Directory::new(), changes)
        }
    }?;

    // Get curr time
    let curr_time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    // Serialize the dir and store it with a hash
    add_serialized_object(&dir)?;

    let mut parent_hashes = Vec::new();
    if parent_hash.is_some() {parent_hashes.push(parent_hash.unwrap());}

    // Create the commit
    let commit = Commit {
        parent_hashes,
        dir_hash: dir.get_hash(),
        author: String::from(author),
        message: String::from(message),
        time_stamp: curr_time
    };

    // Serialize the commit
    add_serialized_object(&commit)?;

    Ok(commit)
}

// We cant forget the process of actually hashing the content and adding and removing the files
fn process_revisions(dir: Directory, changes: &Vec<Change>) -> io::Result<Directory> {
    changes.iter()
        .try_fold(dir, |mut acc, change| {
            match change {
                Change::Add { path } => {
                    // Create the blob file and return its hash
                    // Then create a ref to the blob and insert it in the directory
                    let hash = process_addition(path)?;
                    let blob_ref = BlobRef::new(path, hash);
                    acc.insert_file_ref(&PathBuf::from(path), blob_ref)
                        .ok_or(io::Error::new(io::ErrorKind::Other, "Failed to insert file ref"))?;
                    println!("Added change {}", path);
                },
                // I guess remove would be used in case there is a move of a file?
                Change::Remove { path } => {
                    // We technically shouldn't remove the actual blob file since blobs are hashed
                    // by content and multiple blobrefs can reference the same blob
                    let remove_path = PathBuf::from(path);
                    acc.remove_file_ref(&remove_path);
                },
            }
            Ok(acc)
        })
}

fn process_addition(path:&str) -> io::Result<Hash> {
    let add_path = PathBuf::from(path);

    // Open the added file and read the data to a vector
    let mut file = File::open(&add_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // Hash the data and add it as a blob
    let hash = Hash::from(&data);
    add_object(hash.clone(), &data)?;

    Ok(hash)
}

