#![allow(warnings)]

use std::path::PathBuf;
use std::io;
use std::io::Error;

use file_management::commit::{Commit, self};
use interface::io::RepositoryInterface;
use state_management::merge::merge;

use crate::{file_management::{commit::commit, hash::DVCSHash}, state_management::checkout::checkout};

pub mod file_management;
pub mod interface;
pub mod state_management;

pub fn main() {}

pub fn commit_cmd(message: &str, author: &str) -> std::io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?;
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Retrieves changes from the staging area
    let files = Vec::new();
    // let staged_files = read_files(curr_path)?.
    todo!("Silin implement retrieve changes");

    // Get parent hash
    let parent_hash = Vec::new();
    if let Some(head_commit_hash) = repo.get_current_head() {
        parent_hash.push(head_commit_hash);
    }

    // Commit 
    let dir = repo.create_dir_from_files(&files)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to create a commit"))?;

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
        Some(curr_head) => {
            // Existence of current head means there is at least 1 branch
            // Get the current branch and update the hash
            // WHAT IF YOU CHECKOUT TO A NON HEAD - YOU GET NO BRANCH FROM IT
            let branch_name = repo.get_branch_from_hash(curr_head)
                .ok_or(Error::new(io::ErrorKind::Other, "On a non head commit."))?;
            repo.update_branch_head(&branch_name, new_commit_hash)?;
        },
        None => {
            // No branch - create a branch called master
            repo.create_branch("master", new_commit_hash)
                .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to create master branch."));
        },
    }

    // Update the repo head
    repo.update_current_head(new_commit_hash);

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
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to check out files"))
}

pub fn merge_cmd() -> std::io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?;
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Get the branch you are merging into
    let merge_into_hash = repo.get_current_head()?;
    let merge_into = repo.get_serialized_object(merge_into_hash)?;

    // Get the other branch


    // Merge directories and handle any merge conflicts
    let merged_dir = merge(repo, merge_into, merge_from);
    match merged_dir {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }

    // Merge commit
    let commit = commit (
        "DVCS MERGE",
        todo!(),
        merged_dir,
        "Merge Commit",
        &repo
    )?;

    // commit(
    //     author,
    //     &parent_hash,
    //     dir,
    //     message,
    //     &repo
    // )?;
}

pub fn branch_cmd() {

}