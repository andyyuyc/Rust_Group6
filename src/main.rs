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

                // Split the input string and collect the parts into a vector.
                let parts: Vec<&str> = input.split_whitespace().collect();

                // Check if the input format is correct.
                if parts.len() == 2 && parts[0] == "checkout" {
                    // Extract the commit hash.
                    let commit_hash = parts[1];

                    // Perform the checkout operation.
                    let _ = std::fs::remove_dir_all("test");
                    let _ = checkout(Hash::from_hashed(commit_hash));
                } else {
                    println!("Invalid input format. Expected 'checkout <commit_hash>'.");
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