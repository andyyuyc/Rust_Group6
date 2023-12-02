use crate::{file_management::{hash::{Hash, DVCSHash}, directory::Directory, commit::Commit}, interface::io::get_serialized_object};
use std::{io, collections::{HashMap, HashSet, VecDeque}};

pub fn merge(commit1: Commit, commit2: Commit) -> io::Result<()> { 
    // Find a common ancestor
    let parent_commit = get_common_ancestor(&commit1, &commit2)?;

    // Get directories
    let parent_dir: Directory = get_serialized_object(parent_commit.get_dir_hash())?;
    let commit1_dir: Directory = get_serialized_object(commit1.get_dir_hash())?;
    let commit2_dir: Directory = get_serialized_object(commit2.get_dir_hash())?;

    // init merged directory
    let mut merged_dir = parent_dir.clone();

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
                (Some(ancestor), Some(ref1), Some(ref2)) => {
                    if ref1.get_content_hash() == ancestor.get_content_hash() {
                        // Accept ref2 changes
                    } else if ref2.get_content_hash() == ancestor.get_content_hash() {
                        // Accept ref1 changes
                    } else if ancestor.get_content_hash() == ref1.get_content_hash() &&
                              ref1.get_content_hash() == ref2.get_content_hash() 
                    {
                        // Three way diffy merge
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
                },
                (_, _, _) => {
                    unreachable!();
                }
            }
        });


    todo!()
}

fn get_common_ancestor(commit1: &Commit, commit2: &Commit) -> io::Result<Commit> { 
    let mut ancestors: HashSet<Hash> = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(commit1.get_hash());
    queue.push_back(commit2.get_hash());
    
    while let Some(hash) = queue.pop_front() {
        // Common ancestor is found
        if ancestors.contains(&hash) {
            return get_serialized_object(hash);
        } else {
            ancestors.insert(hash.clone());

            // Add the parent hashes of this commit to the queue
            let commit: Commit = get_serialized_object(hash)?;
            queue.append(&mut commit.get_parent_hashes().into_iter().collect());
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "No common ancestor found"))
}