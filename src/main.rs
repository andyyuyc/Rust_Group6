use std::{path::PathBuf};
use std::{io, fs};
use std::io::Error;

use diff::diff::show_diff;
use file_management::directory::Directory;
use file_management::hash::Hash;
use file_management::{commit::{Commit, self, commit_cmd}, directory::BlobRef};
use initialization::{init::init, clone::clone_local};
use interface::io::RepositoryInterface;
use revisions::{staging::{stage_add, stage_all_files}, status};
use state_management::{merge::{merge, merge_cmd}, branch::{get_branches_cmd, create_branch_cmd}};
use view::cat::cat;

use crate::{file_management::{commit::commit, hash::DVCSHash}, state_management::checkout::checkout};
use crate::revisions::staging;

pub mod file_management;
pub mod interface;
pub mod state_management;
pub mod diff;
pub mod revisions;
pub mod view;
pub mod initialization;


use std::io::{Write, stdin, stdout, ErrorKind};
use std::env;

// main.rs
use tokio::runtime;
// mod clone;
// mod status;
// mod errorhandling;

// pub fn main() {}

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

    match args[1].as_str() {
        "init" => {
            match initialization::init::init_repo(&path_as_str) {
                Ok(_) => println!("Successfully initialized repo"),
                Err(_) => println!("Failed to initialize repo"),
            };
        },
        "clone" => {
            if args.len() == 3 {
                match clone_local(&path_as_str, &args[2]) {
                    Ok(_) => println!("Successfully clone to local"),
                    Err(_) => println!("Failed to clone"),
                }
            } else {
                println!("Correct Usage: dvcs clone <other-dir>")
            }
        },
        // "errorhandling" => errorhandling::errorhandling(),
        "status" => {
            if let Err(e) = status::status().await {
                println!("Error: {}", e);
            } 
        },
        "commit" => {
            if args.len() == 3 {
                let msg = &args[2];
                match commit_cmd(&msg, "You") {
                    Ok(commit) => todo!("Add a new file log"),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct Usage: dvcs commit <message>");
            }
        },
        "branch" => {
            if args.len() == 2 {
                match get_branches_cmd() {
                    Ok(branch_info) => println!("{}", branch_info),
                    Err(e) => println!("Error: {}", e),
                }
            } else if args.len() == 3 {
                let branch_name = &args[2];
                match create_branch_cmd(&branch_name) {
                    Ok(_) => println!("Created branch {}", &branch_name),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Correct usage: dvcs branch | dvcs branch <branch_name>")
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
                            Err(e) => println!("Erro: {}", e),
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
        "cat" => {
            if args.len() == 2 {
                match cat(&path) {
                    Ok(content) => println!("{}", content),
                    Err(_) => println!("Error: Failed to read file"),
                };
            } else {
                println!("Correct usage: dvcs cat <file_path>")
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
                println!("Correct usage: dvcs diff")
            }
        }
        _ => println!("Unknown command"),
    }
}