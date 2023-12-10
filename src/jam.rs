mod file_management;
mod diff;
mod interface;
mod status_check;

use std::path::PathBuf;
use crate::file_management::directory::{Directory, BlobRef};
use crate::file_management::hash::Hash;
use crate::status_check::status::status_checker::check_status;

use std::fs;

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
                let blob_ref = BlobRef::new(&file_path.to_string_lossy(), file_hash);

                directory.insert_file_ref(&file_path, blob_ref);
            }
        }
    }

    directory
}

fn main() {
    println!("Welcome to Group 6 DVCS Project!");
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: dvcs diff <path_to_dir1> <path_to_dir2>");
        return;
    }

    match args[1].as_str() {
        "diff" => {
            if args.len() < 4 {
                println!("Usage: dvcs diff <path_to_dir1> <path_to_dir2>");
                return;
            }

            let dir1_path = PathBuf::from(&args[2]);
            let dir2_path = PathBuf::from(&args[3]);

            //read the paths of the directories.
            let dir1 = directory_path(&dir1_path);
            let dir2 = directory_path(&dir2_path);

            diff::show_diff(&dir1, &dir2);
        },
        "status" => {
            // Assuming repository_path is the path where your DVCS is initialized
            let repository_path = PathBuf::from("./");
            if let Err(e) = check_status(repository_path.to_str().unwrap()) {
                println!("Error checking status: {}", e);
            }
        },
        _ => println!("Unknown command"),
    }
}
