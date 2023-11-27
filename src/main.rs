#![allow(warnings)]

use file_management::commit::Commit;

use crate::{file_management::{commit::{Change, commit}, hash::DVCSHash}, state_management::checkout::checkout};

pub mod file_management;
pub mod interface;
pub mod state_management;

fn main() {

    let commit_1 = first_commit();
    println!("{commit_1}");
    println!("Added test/test.txt");
    println!("Added test/idk/something.txt");
    println!("");

    let commit_2 = second_commit(commit_1.get_hash());
    println!("{commit_2}");
    println!("Added test/test2.txt");
    println!("");

    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Trim the input to remove any trailing newline characters
                let input = input.trim();

                match input {
                    "checkout 1" => {
                        let _ = std::fs::remove_dir_all("test");
                        let _ = checkout(commit_1.get_hash());
                    },
                    "checkout 2" => {
                        let _ = std::fs::remove_dir_all("test");
                        let _ = checkout(commit_2.get_hash());
                    },
                    _ => {}
                }
            },
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}

fn first_commit() -> Commit {
    let mut changes = vec![];
    
    changes.push(Change::Add {path: "test/test.txt".to_owned()});
    changes.push(Change::Add {path: "test/idk/something.txt".to_owned()});

    commit("Justin", None, &changes, "Initial commit").unwrap()
}

use crate::file_management::hash::Hash;
fn second_commit(parent_hash: Hash) -> Commit {
    let mut changes = vec![];
    
    changes.push(Change::Add {path: "test/test.txt".to_owned()});
    changes.push(Change::Add {path: "test/test2.txt".to_owned()});
    changes.push(Change::Add {path: "test/idk/something.txt".to_owned()});

    commit("Justin", Some(parent_hash), &changes, "Added test2").unwrap()
}