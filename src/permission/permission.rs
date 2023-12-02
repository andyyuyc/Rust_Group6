use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Permission {
    Read,
    Write,
    Execute,
}

fn verify_permissions(user: &str, file_path: &str, permission: Permission) -> io::Result<()> {
    // Simulate permission checking logic
    if user.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid user"));
    }
    if file_path.is_empty() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Invalid file path"));
    }
    // Insufficient permission
    if user == "valid_user" && file_path == "valid_path" && (permission == Permission::Read || permission == Permission::Execute) {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Insufficient permission"));
    }

    Ok(())
}

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
    let permission = match Permission::from_str(&permission_input) {
        Ok(p) => p,
        Err(_) => {
            println!("Invalid permission");
            return;
        },
    };

    let result = verify_permissions(&user, &file_path, permission);
    match result {
        Ok(()) => println!("Permission granted for {:?} on {:?} with {:?}", user, file_path, permission),
        Err(e) => println!("Permission denied for {:?} on {:?} with {:?}: {}", user, file_path, permission, e),
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
