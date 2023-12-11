use std::{path::PathBuf};
use std::{io, fs};
use std::io::Error;

use initialization::clone;
use inter_repo_interactions::pull_push::{pull, push};
use interface::error_handling::errorhandling;
use revisions::staging::{stage_remove, clear_staged_files};
use state_management::checkout::checkout_cmd;
use status_check::log::{dvcs_log, log};
use tracking::diff::show_diff;
use file_management::directory::Directory;
use file_management::hash::Hash;
use file_management::{commit::{Commit, self, commit_cmd}, directory::BlobRef};
use initialization::{init::init, clone::clone_local};
use interface::io::RepositoryInterface;
use revisions::{staging::{stage_add, stage_all_files}, status};
use state_management::{merge::{merge, merge_cmd}, branch::{get_branches_cmd, create_branch_cmd}};
use status_check::cat::cat_cmd;

use crate::{file_management::{commit::commit, hash::DVCSHash}, state_management::checkout::checkout};
use crate::revisions::staging;

pub mod file_management;
pub mod interface;
pub mod state_management;
pub mod tracking;
pub mod revisions;
pub mod initialization;
pub mod status_check;
pub mod inter_repo_interactions;

use std::io::{Write, stdin, stdout, ErrorKind};
use std::env;

//this 
fn directory_path(path: &PathBuf) -> Directory {
    let mut directory = Directory::new();

    // Iterate over files in the directory
    if let Ok(dir_entries) = fs::read_dir(path) {
        for entry in dir_entries.filter_map(|e| e.ok()) {
            let file_path = entry.path();

            //checks for whether path is file or directory
            if file_path.is_file() {
                let file_hash = Hash::new(&file_path.to_string_lossy());
                let blob_ref = BlobRef::new(file_hash);

                directory.insert_file_ref(&file_path, blob_ref);
            }
        }
    }

    directory
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = std::env::current_dir().unwrap_or(PathBuf::from("."));
    let path_as_str = &path.as_os_str().to_string_lossy().to_string();

    if args.len() < 2 {
        println!("Usage: dvcs <command>");
        return;
    }

    if args[1].as_str() == "init" {
        match initialization::init::init_repo(&path_as_str) {
            Ok(_) => println!("Successfully initialized repo"),
            Err(_) => println!("Failed to initialize repo"),
        };
    }

    // Fixes the case where the methods are called without a repo
    if RepositoryInterface::new(&path).is_none() {
        println!("Not a repo. Do dvcs init to initialize");
        return;
    }

    match args[1].as_str() {
        "init" => {},
        "clone" => clone::clone(),
        "errorhandling" => {
            errorhandling()
        },
        "status" => {
            if args.len() == 2 {
                match status::track_status(&path) {
                    Ok((tracked, untracked)) => {
                        println!("Tracked files:");
                        for file in tracked {
                            println!("  - {}", file);
                        }
            
                        println!("Untracked files:");
                        for file in untracked {
                            println!("  - {}", file);
                        }
                    },
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct Usage: dvcs status")
            }
        },
        "checkout" => {
            if args.len() == 3 {
                match checkout_cmd(&args[2]) {
                    Ok(_) => println!("Successfully checked out {}", &args[2]),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct Usage: dvcs checkout <branch-name>")
            }
        }
        "commit" => {
            if args.len() == 3 {
                let msg = &args[2];
                match commit_cmd(&msg, "You") {
                    Ok(commit) => match dvcs_log(&commit) {
                        Ok(_) => println!("Successfully commited"),
                        Err(_) => println!("Failed to update log"),
                    },
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct Usage: dvcs commit <message>");
            }
        },
        "branch" => {
            if args.len() == 3 {
                let branch_name = &args[2];
                match create_branch_cmd(&branch_name) {
                    Ok(_) => println!("Created branch {}", &branch_name),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct usage: dvcs branch <branch_name>")
            }
        },
        "heads" => {
            if args.len() == 2 {
                match get_branches_cmd() {
                    Ok(branch_info) => println!("{}", branch_info),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct usage: dvcs heads")
            }
        }
        "merge" => {
            if args.len() == 3 {
                match merge_cmd(&args[2]) {
                    Ok(_) => println!("Successfully merged"),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct usage: dvcs merge <other-branch>")
            }   
        },
        "add" => {
            if args.len() == 3 {
                match args[2].as_str() {
                    "*" => {
                        match stage_all_files(&path) {
                            Ok(_) => println!("Successfully staged all files"),
                            Err(e) => println!("Error: {}", e),
                        }
                    },
                    path => {
                        match stage_add(&path_as_str, path) {
                            Ok(_) => {},
                            Err(e) => println!("Error: {}", e)
                        }
                    }
                }
            } else {
                println!("Correct usage: dvcs add <file_path> | dvcs add *")
            }
        },
        "remove" => {
            match args[2].as_str() {
                "*" => {
                    match clear_staged_files(&path) {
                        Ok(_) => println!("Successfully removed all files from staging area"),
                        Err(_) => println!("Error: Failed to clear staging area"),
                    }
                },
                path => {
                    match stage_remove(&path_as_str, path) {
                        Ok(_) => {},
                        Err(e) => println!("Error: {}", e)
                    }
                }
            }
        },
        "cat" => {
            if args.len() == 4 {
                let hash = Hash::from_hashed(&args[2]);
                let file_name = PathBuf::from(&args[3]);
                if let Err(e) = cat_cmd(hash, &file_name) {
                    println!("Error: {}", e)
                }
            } else {
                println!("Correct usage: dvcs cat <commit-hash> <file-name>")
            }
        },
        "diff" => {
            if args.len() >= 4 {
                let dir1_path = PathBuf::from(&args[2]);
                let dir2_path = PathBuf::from(&args[3]);

                let dir1 = directory_path(&dir1_path);
                let dir2 = directory_path(&dir2_path);

                show_diff(&dir1, &dir2);
            } else {    
                println!("Correct usage: dvcs diff <dir1> <dir2>")
            }
        },
        "log" => {
            if args.len() == 2 {
                match log() {
                    Ok(_) => {},
                    Err(_) => println!("Failed to print log. Commit to see logs"),
                }
            } else {
                println!("Correct usage: dvcs log")
            }
        },
        "pull" => {
            if args.len() == 3 {
                let remote_path = &args[2];
                match pull(remote_path, &path_as_str) {
                    Ok(_) => println!("Successfully pulled from remote"),
                    Err(e) => println!("Error during pull: {}", e),
                }
            } else {
                println!("Correct usage: dvcs pull <remote_path>");
            }
        },
        "push" => {
            if args.len() == 3 {
                let remote_path = &args[2];
                match push(&path_as_str, remote_path) {
                    Ok(_) => println!("Successfully pushed to remote"),
                    Err(e) => println!("Error during push: {}", e),
                }
            } else {
                println!("Correct usage: dvcs push <remote_path>");
            }
        },
        _ => println!("Unknown command"),
    }
}