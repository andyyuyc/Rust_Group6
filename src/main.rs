mod status;
mod staging;
mod diff;

use std::io::{self, Write, stdin, stdout, ErrorKind};
use staging::{stage_add, stage_remove}; 
use status::status_checker::{heads,check_status};
use diff::file_diff::diff;
use std::env;

fn main() -> io::Result<()> {
    let mut repository_path = String::new();
    let mut command = String::new();
    let mut file_path = String::new();

    print!("Enter repository path: ");
    stdout().flush()?; 
    stdin().read_line(&mut repository_path)?;
    repository_path = repository_path.trim().to_string();

    print!("Enter command (add/remove/status/heads/diff): "); 
    stdout().flush()?;
    stdin().read_line(&mut command)?;
    command = command.trim().to_string();

    match command.as_str() {
        "init" => {
            
        }
        "add" => {
            print!("Enter file path: ");
            stdout().flush()?; 
            stdin().read_line(&mut file_path)?;
            file_path = file_path.trim().to_string();
            stage_add(&repository_path, &file_path)?
        },
        "remove" => {
            print!("Enter file path: ");
            stdout().flush()?; 
            stdin().read_line(&mut file_path)?;
            file_path = file_path.trim().to_string();
            stage_remove(&repository_path, &file_path)?
        },
        "status" => check_status(&repository_path)?, 
        "diff" => {
            let current_dir = env::current_dir()?; // to be changed with the path of the current directory
            let previous_dir = "path/to/previous/dir"; //to be chaned with the path of the previous directory
            match diff(current_dir.to_str().unwrap(), previous_dir) {
                Ok(_) => (),
                Err(e) => println!("Error comparing directories: {}", e),
            }
        },
        "heads" => {
            match heads(&repository_path) {
                Ok(_) => println!("Heads listed successfully."),
                Err(e) => println!("Error listing heads: {}", e),
            }
        },
        _ => {
            eprintln!("Invalid command: {}", command);
            return Err(io::Error::new(ErrorKind::InvalidInput, "Invalid command"));
        }
    }

    Ok(())
}
