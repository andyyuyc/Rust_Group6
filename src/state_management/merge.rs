use crate::{file_management::{hash::{Hash, DVCSHash}, directory::Directory, commit::Commit}, interface::io::RepositoryInterface};
use std::{io, collections::{HashMap, HashSet, VecDeque}};

pub fn merge(file_system: RepositoryInterface, commit1: Commit, commit2: Commit) -> io::Result<()> { 
    // Find a common ancestor
    let parent_commit = get_common_ancestor(&file_system, &commit1, &commit2)?;

    // Get directories
    let parent_dir: Directory = file_system.get_serialized_object(parent_commit.get_dir_hash())?;
    let commit1_dir: Directory = file_system.get_serialized_object(commit1.get_dir_hash())?;
    let commit2_dir: Directory = file_system.get_serialized_object(commit2.get_dir_hash())?;

    // init merged directory
    let mut merged_dir = Directory::new();

    // Get all files in all directories
    let all_files: HashSet<_> = parent_dir.get_key_value_pairs()
        .into_iter()
        .chain(commit1_dir.get_key_value_pairs())
        .chain(commit2_dir.get_key_value_pairs())
        .collect();

    all_files.into_iter()
        .for_each(|(path, blob_ref)| {
            let ancestor_data = parent_dir.get_file_ref(&path);
            let commit1_data = commit1_dir.get_file_ref(&path);
            let commit2_data = commit2_dir.get_file_ref(&path);

            match (ancestor_data, commit1_data, commit2_data) {
                (Some(ancestor), Some(ref1), None) => {
                    // If branch 1 modifies the file but branch 2 deletes
                    // merge conflict - else don't add it (git deletes it)
                    if ref1.get_content_hash() != ancestor.get_content_hash() {
                        // Merge conflict 
                    }
                },
                (Some(ancestor), None, Some(ref2)) => {
                    // If branch 1 modifies the file but branch 2 deletes
                    // merge conflict - else don't add it (git deletes it)
                    if ref2.get_content_hash() != ancestor.get_content_hash() {
                        // Merge conflict
                    }
                }
                // What is ref1 == ref2 != ancestor
                // Might have to think more about this?
                (Some(ancestor), Some(ref1), Some(ref2)) => {
                    if ref1.get_content_hash() == ancestor.get_content_hash() {
                        // Accept ref2 changes
                        merged_dir.insert_file_ref(&path, ref2.clone());
                    } else if ref2.get_content_hash() == ancestor.get_content_hash() {
                        // Accept ref1 changes
                        merged_dir.insert_file_ref(&path, ref1.clone());
                    } else if ancestor.get_content_hash() == ref1.get_content_hash() &&
                              ref1.get_content_hash() == ref2.get_content_hash() 
                    {
                        // Three way diffy merge
                        // diffy::merge_bytes(ancestor, ours, theirs)
                    }
                },
                (None, Some(ref1), Some(ref2)) => {
                    if ref1.get_content_hash() == ref2.get_content_hash() {
                        // Just add one of them - doesn't matter
                    } else if ref1.get_content_hash() != ref2.get_content_hash() {
                        // Three way diffy merge with the ancestor as ""
                    }
                },
                (None, Some(ref1), None) | (None, None, Some(ref1)) => {
                    // Just add it
                    merged_dir.insert_file_ref(&path, ref1.clone());
                },
                (_, _, _) => {}
            }
        });


    todo!()
}

fn get_common_ancestor(file_system: &RepositoryInterface, commit1: &Commit, commit2: &Commit) -> io::Result<Commit> { 
    let mut ancestors: HashSet<Hash> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(commit1.get_hash());
    queue.push_back(commit2.get_hash());
    
    while let Some(hash) = queue.pop_front() {
        // Common ancestor is found
        if ancestors.contains(&hash) {
            return file_system.get_serialized_object(hash);
        } else {
            ancestors.insert(hash.clone());

            // Add the parent hashes of this commit to the queue
            let commit: Commit = file_system.get_serialized_object(hash)?;
            queue.append(&mut commit.get_parent_hashes().into_iter().collect());
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "No common ancestor found"))
}