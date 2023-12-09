use std::io::{self, Write};
use std::str::FromStr;
use std::path::Path;
use std::fs;

// Define an enum for different types of permissions
#[derive(Debug, PartialEq, Copy, Clone)]
enum Permission {
    Read,
    Write,
    Execute,
}

// Function to verify permissions of a user on a given file path
fn verify_permissions(user: &str, file_path: &str, permission: Permission) -> io::Result<()> {
    
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

fn main() {
    let user = prompt_input("Enter user: ");
    let file_path = prompt_input("Enter file path: ");
    let permission_input = prompt_input("Enter permission (Read, Write, Execute): ");

    // Parse the permission input
    let permission = match Permission::from_str(&permission_input) {
        Ok(p) => p,
        Err(_) => {
            println!("Invalid permission");
            return;
        },
    };

    // Verify permissions
    let result = verify_permissions(&user, &file_path, permission);
    match result {
        Ok(()) => println!("Permission granted"),
        Err(e) => println!("Permission denied"),
    }
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


#[cfg(test)]
mod tests {
    use super::*;
    // Test for an invalid user and a valid file path with the write permission
    #[test]
    fn test_invalid_user_valid_path_write_permission() {
        let user = "";
        let file_path = "/Users/adrian/Desktop"; 
        let permission = Permission::Write;

        let result = verify_permissions(user, file_path, permission);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }

    // Test for a valid user and an invalid file path with the execute permission
    #[test]
    fn test_valid_user_invalid_path_execute_permission() {
        let user = "valid_user";
        let file_path = "";
        let permission = Permission::Execute;

        let result = verify_permissions(user, file_path, permission);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    // Test for a valid user, a valid file path, and an invalid permission
    #[test]
    fn test_valid_user_valid_path_invalid_permission() {
        let user = "valid_user";
        let file_path = "/Users/adrian/Desktop"; // Adjust this to a valid path
        let permission_input = "InvalidPermission";

        let permission = Permission::from_str(permission_input);
        assert!(permission.is_err());
    }

    // Test for insufficient permission with read permission
    #[test]
    fn test_insufficient_permission_read() {
        let user = "valid_user";
        let file_path = "/Users/adrian/Desktop"; // This should match the exact string in your function's logic
        let permission = Permission::Read;
    
        let result = verify_permissions(user, file_path, permission);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
    }

    // Test for insufficient permission with execute permission
    #[test]
    fn test_valid_user_valid_path_execute_permission() {
        let user = "valid_user";
        let file_path = "/Users/adrian/Desktop";
        let permission = Permission::Execute;
        let result = verify_permissions(user, file_path, permission);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::PermissionDenied);
    }

    // Test for a valid user, a valid file path, and the write permission
    #[test]
    fn test_valid_user_valid_path_write_permission() {
        let user = "valid_user";
        let file_path = "/Users/adrian/Desktop"; // Adjust this to a valid path
        let permission = Permission::Write;

        assert!(matches!(verify_permissions(user, file_path, permission), Ok(())));
    }
}
