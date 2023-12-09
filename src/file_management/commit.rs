use std::{io::{self, Read}, path::{PathBuf, Path}, fs::{File, create_dir_all}, fmt::Display};

use serde::{Serialize, Deserialize};
use crate::{file_management::{hash::Hash, directory}, interface::io::RepositoryInterface};

use super::{directory::{Directory, BlobRef}, hash::DVCSHash};

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    parent_hashes: Vec<Hash>,
    dir_hash: Hash,
    author: String,
    message: String,
    time_stamp: String,
}

impl Commit {
    pub fn get_parent_hashes(&self) -> Vec<Hash> {
        self.parent_hashes.clone()
    }

    pub fn get_dir_hash(&self) -> Hash {
        self.dir_hash.clone()
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_time_stamp(&self) -> &str {
        &self.time_stamp
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

impl Display for Commit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Commit:\nhash: {}\nauthor: {}\nmessage: {}\ntime: {}", 
            self.get_hash().as_string(), 
            self.author, 
            self.message, 
            self.time_stamp
        )
    }
}

// Creates a new commit
pub fn commit(
    author: &str,
    parent_hashes: &Vec<Hash>, 
    new_dir: Directory,
    message: &str,
    file_system: &RepositoryInterface
) -> io::Result<Commit> {
    // Get curr time
    let curr_time = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string();

    // Create the commit
    let commit = Commit {
        parent_hashes: parent_hashes.clone(),
        dir_hash: new_dir.get_hash(),
        author: String::from(author),
        message: String::from(message),
        time_stamp: curr_time
    };

    // Serialize the commit and directory
    println!("Need to add serialized obj");
    file_system.add_serialized_object(&new_dir)?;
    println!("Serialized directory");
    file_system.add_serialized_object(&commit)?;
    println!("Serialized commit");

    Ok(commit)
}

#[test]
fn commit_test() {
    use crate::interface::io::RepositoryInterface;

    let repo = RepositoryInterface::new(&PathBuf::from("test")).unwrap();

    let path1 = PathBuf::from("test.txt");
    let path2 = PathBuf::from("idk/test.txt");

    let mut file_paths = vec![
        &path1,
        &path2
    ];

    let dir = repo.create_dir_from_files(&file_paths).unwrap();

    println!("Created directory");
    let commit = commit("Justin", &vec![], dir, "Initial commit", &repo).unwrap();
}

