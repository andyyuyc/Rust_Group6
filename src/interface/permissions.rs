use std::io::{self, Write};
use std::str::FromStr;
use std::path::Path;
use std::fs;

// Define an enum for different types of permissions
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Permission {
    Read,
    Write,
    Execute,
}

// Function to verify permissions of a user on a given file path
pub fn verify_permissions(user: &str, file_path: &str, permission: Permission) -> io::Result<()> {
    if user.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid user"));
    }

    if file_path.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path: empty"));
    }
    if !Path::new(file_path).is_absolute() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path: path format error"));
    }
    if !fs::metadata(file_path).is_ok() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid file path: path does not exist"));
    }

    if permission == Permission::Read || permission == Permission::Execute {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Insufficient permission"));
    }

    Ok(())
}

// Function to prompt the user for input
fn prompt_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


impl FromStr for Permission {
    type Err = ();

    fn from_str(input: &str) -> Result<Permission, Self::Err> {
        match input {
            "Read" => Ok(Permission::Read),
            "Write" => Ok(Permission::Write),
            "Execute" => Ok(Permission::Execute),
            _ => Err(()),
        }
    }
}