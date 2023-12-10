use std::{io::{self, Read, Error}, path::{PathBuf, Path}, fs::{File, create_dir_all}, fmt::Display};

use serde::{Serialize, Deserialize};
use crate::{file_management::{hash::Hash, directory}, interface::io::RepositoryInterface, revisions::staging::{get_staged_files, self, stage_add, clear_staged_files}};

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
    file_system.add_serialized_object(&new_dir)?;
    file_system.add_serialized_object(&commit)?;

    Ok(commit)
}

pub fn commit_cmd(message: &str, author: &str) -> std::io::Result<Commit> {
    // Initialize repository
    let curr_path = std::env::current_dir()?; 
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Retrieves changes from the staging area
    let staged_files = get_staged_files(&curr_path)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve files from staging area"))?;

    // Return Err if there is nothing in the staging area
    if staged_files.len() == 0 {
        println!("Nothing in staged files");
        return Err(Error::new(io::ErrorKind::Other, "Nothing in staged files"));
    }

    // Get parent hash
    let mut parent_hash = Vec::new();
    if let Some(curr_branch) = repo.get_current_head() {
        let head_commit_hash = repo.get_hash_from_branch(&curr_branch)
            .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve parent commit"))?;
        parent_hash.push(head_commit_hash);
    }

    // Commit 
    let dir = repo.create_dir_from_files(&staged_files)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to create a directory object"))?;

    let new_commit = commit(
        author,
        &parent_hash,
        dir,
        message,
        &repo
    ).map_err(|_| Error::new(io::ErrorKind::Other, "Failed to save commit to repo"))?;

    // In git, there are no branches until you commit. If there were no branches previously, commit makes
    // it so you move to a branch called master

    // Update the branch head 
    let new_commit_hash = new_commit.get_hash();
    match repo.get_current_head() {
        Some(curr_branch) => {
            // Existence of current head means there is at least 1 branch
            // Get the current branch and update the hash
            // WHAT IF YOU CHECKOUT TO A NON HEAD - YOU GET NO BRANCH FROM IT
            repo.update_branch_head(&curr_branch, new_commit_hash.clone())
                .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to update current branch head"))?;
        },
        None => {
            // No branch - create a branch called master
            println!("No branch - creating master branch");
            repo.create_branch("master", new_commit_hash.clone())
                .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to create master branch."))?;
        },
    }

    // // // Update the repo head NO NEED TO UPDATE THE CURR HEAD BRANCH
    // repo.update_current_head(new_commit_hash); 

    clear_staged_files(&curr_path)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to clear staging area"))?;

    Ok(new_commit)
}

#[test]
fn commit_test() {
    use crate::interface::io::RepositoryInterface;

    let repo = RepositoryInterface::new(&PathBuf::from("test")).unwrap();

    let path1 = PathBuf::from("test.txt");
    let path2 = PathBuf::from("idk/test.txt");

    let mut file_paths = vec![
        path1,
        path2
    ];

    let dir = repo.create_dir_from_files(&file_paths).unwrap();

    println!("Created directory");
    let commit = commit("Justin", &vec![], dir, "Initial commit", &repo).unwrap();
}

#[test]
fn commit_cmd_test() {
    use crate::staging;

    stage_add(".", "test.txt").unwrap();
    stage_add(".", "idk/test.txt").unwrap();
    println!("Added to staging area");

    commit_cmd("Commit 2", "Juicetin").unwrap();

    clear_staged_files(&PathBuf::from(".")).unwrap();
}

#[test]
fn commit_empty_cmd_test() {
    use crate::staging;

    commit_cmd("Empty Commit", "Juicetin").unwrap();

    clear_staged_files(&PathBuf::from(".")).unwrap();
}
