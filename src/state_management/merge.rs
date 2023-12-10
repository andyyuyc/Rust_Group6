use crate::{file_management::{hash::{Hash, DVCSHash}, directory::{Directory, BlobRef}, commit::{Commit, self}}, interface::io::RepositoryInterface};
use std::{io::{self, Error}, collections::{HashSet, VecDeque}};

pub fn merge(file_system: &RepositoryInterface, commit1: Commit, commit2: Commit) -> io::Result<Directory> { 
    // Find a common ancestor
    let parent_commit = get_common_ancestor(&file_system, &commit1, &commit2)?;

    // Get directories
    let parent_dir: Directory = file_system.get_serialized_object(parent_commit.get_dir_hash())?;
    let commit1_dir: Directory = file_system.get_serialized_object(commit1.get_dir_hash())?;
    let commit2_dir: Directory = file_system.get_serialized_object(commit2.get_dir_hash())?;

    // Get all files in all directories
    let all_files: HashSet<_> = parent_dir.get_key_value_pairs()
        .into_iter()
        .chain(commit1_dir.get_key_value_pairs())
        .chain(commit2_dir.get_key_value_pairs())
        .collect();

    all_files.into_iter()
        .try_fold(Directory::new(), |mut merged_dir, (path, blob_ref)| {
            let ancestor_data = parent_dir.get_file_ref(&path);
            let commit1_data = commit1_dir.get_file_ref(&path);
            let commit2_data = commit2_dir.get_file_ref(&path);

            match (ancestor_data, commit1_data, commit2_data) {
                (Some(ancestor), Some(ref1), None) => {
                    // If branch 1 modifies the file but branch 2 deletes
                    // merge conflict - else don't add it (git deletes it)
                    if ref1.get_content_hash() != ancestor.get_content_hash() {
                        // Merge conflict
                        let message = format!(
                            "Merge Conflict. Branch 1 modifies {} but branch 2 deletes",
                            &path.display()
                        );
                        return Err(io::Error::new(io::ErrorKind::Other, message))
                    }
                    Ok(merged_dir)
                },
                (Some(ancestor), None, Some(ref2)) => {
                    // If branch 2 modifies the file but branch 1 deletes
                    // merge conflict - else don't add it (git deletes it)
                    if ref2.get_content_hash() != ancestor.get_content_hash() {
                        // Merge conflict
                        let message = format!(
                            "Merge Conflict. Branch 2 modifies {} but branch 1 deletes",
                            &path.display()
                        );
                        return Err(io::Error::new(io::ErrorKind::Other, message))
                    }
                    Ok(merged_dir)
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
                    } else if ancestor.get_content_hash() != ref1.get_content_hash() &&
                              ref1.get_content_hash() != ref2.get_content_hash() &&
                              ref2.get_content_hash() != ancestor.get_content_hash()
                    {
                        // Three way diffy merge if none of them have the same contents
                        // diffy::merge_bytes(ancestor, ours, theirs)
                        let ancestor_data = file_system.get_object(ancestor.get_content_hash().clone())?;
                        let ref1_data = file_system.get_object(ref1.get_content_hash().clone())?;
                        let ref2_data = file_system.get_object(ref2.get_content_hash().clone())?;

                        let merged_content = diffy::merge(
                            &String::from_utf8_lossy(&ancestor_data), 
                            &String::from_utf8_lossy(&ref1_data), 
                            &String::from_utf8_lossy(&ref2_data)
                        );

                        // If the merge works out, create a new blob and blobref
                        if let Ok(string) = merged_content {
                            let blob_hash = file_system.create_blob(string.as_bytes())?;
                            let blob_ref = BlobRef::new(blob_hash);
                            merged_dir.insert_file_ref(&path, blob_ref);
                        } else {
                            return Err(io::Error::new(io::ErrorKind::Other, merged_content.unwrap_err()))
                        }
                    }
                    Ok(merged_dir)
                },
                (None, Some(ref1), Some(ref2)) => {
                    if ref1.get_content_hash() == ref2.get_content_hash() {
                        // Just add one of them - doesn't matter
                        merged_dir.insert_file_ref(&path, ref1.clone());
                    } else if ref1.get_content_hash() != ref2.get_content_hash() {
                        // Three way diffy merge with the ancestor as ""
                        let ref1_data = file_system.get_object(ref1.get_content_hash().clone())?;
                        let ref2_data = file_system.get_object(ref2.get_content_hash().clone())?;

                        let merged_content = diffy::merge(
                            "", 
                            &String::from_utf8_lossy(&ref1_data), 
                            &String::from_utf8_lossy(&ref2_data)
                        );

                        // If the merge works out, create a new blob and blobref
                        if let Ok(string) = merged_content {
                            let blob_hash = file_system.create_blob(string.as_bytes())?;
                            let blob_ref = BlobRef::new(blob_hash);
                            merged_dir.insert_file_ref(&path, blob_ref);
                        } else {
                            return Err(
                                io::Error::new(io::ErrorKind::Other, 
                                    {
                                        let mut string = format!("Merge Conflict at {}", &path.display());
                                        string.push_str(&merged_content.unwrap_err());
                                        string
                                    }
                                )
                            );
                        }
                    }
                    Ok(merged_dir)
                },
                (None, Some(ref1), None) | (None, None, Some(ref1)) => {
                    // Just add it
                    merged_dir.insert_file_ref(&path, ref1.clone());
                    Ok(merged_dir)
                },
                (_, _, _) => Ok(merged_dir)
            }
        })
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

pub fn merge_cmd(other_branch: &str) -> std::io::Result<()> {
    // Initialize repository
    let curr_path = std::env::current_dir()?;
    let repo = RepositoryInterface::new(&curr_path)
        .ok_or(Error::new(io::ErrorKind::Other, "Directory is not a repo"))?;

    // Get the branch you are merging into
    // let merge_into_hash = repo.get_current_head()
    //     .ok_or(Error::new(io::ErrorKind::Other, "Failed to retrieve current head"))?;
    // let merge_into = repo.get_serialized_object(merge_into_hash.clone())
    //     .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve target branch"))?;
    let merge_into_branch = repo.get_current_head()
        .ok_or(Error::new(io::ErrorKind::Other, "Failed to retrieve current head"))?;
    let merge_into_hash = repo.get_hash_from_branch(&merge_into_branch)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve current branch"))?;
    let merge_into = repo.get_serialized_object(merge_into_hash.clone())
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve current branch"))?;

    // Get the other branch
    let merge_from_hash = repo.get_hash_from_branch(other_branch)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Other branch does not exist"))?;
    let merge_from = repo.get_serialized_object(merge_from_hash)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to retrieve target branch"))?;

    // Merge directories
    let merged_dir = merge(&repo, merge_into, merge_from)?;

    // Merge commit
    let commit = commit::commit (
        "DVCS MERGE",
        &vec![merge_into_hash.clone()],
        merged_dir,
        "Merge Commit",
        &repo
    )?;

    // Update branch head 
    let branch = repo.get_branch_from_hash(merge_into_hash.clone())
        .ok_or(Error::new(io::ErrorKind::Other, "Failed to retrieve branch hash"))?;
    repo.update_branch_head(&branch, merge_into_hash)
        .map_err(|_| Error::new(io::ErrorKind::Other, "Failed to update branch head to merge commit"))?;

    // // Update the repo head NOT NECESSARY SINCE THE HEAD DOES NOT CHANGE
    // repo.update_current_head(&branch);

    Ok(())
}