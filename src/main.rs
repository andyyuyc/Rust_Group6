use std::path::PathBuf;
use std::io;
use std::io::Error;

use file_management::commit::{Commit, self, commit_cmd};
use interface::io::RepositoryInterface;
use revisions::staging::stage_add;
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


use std::io::{Write, stdin, stdout, ErrorKind};
use std::env;

// main.rs
// use tokio::runtime;
// mod init;
// mod clone;
// mod status;
// mod errorhandling;

// pub fn main() {}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = std::env::current_dir().unwrap_or(PathBuf::from("."));

    if args.len() < 2 {
        println!("Usage: dvcs <command>");
        return;
    }

    match args[1].as_str() {
        // "init" => init::init(),
        // "clone" => clone::clone(),
        // "errorhandling" => errorhandling::errorhandling(),
        // "status" => {
        //     if let Err(e) = status::status().await {
        //         println!("Error: {}", e);
        //     }
        // },
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
                        stage_add(&path, )
                    },
                    _ => {
                        stage_add()
                    }
                }
            } else {
                println!("Correct usage: dvcs add <file_path> | dvcs add *")
            }
        }
        _ => println!("Unknown command"),
        "cat" => {
            if args.len() == 2 {
                match cat(&path) {
                    Ok(content) => println!("{}", content),
                    Err(_) => println!("Error: Failed to read file"),
                };
            } 
        }
    }
}