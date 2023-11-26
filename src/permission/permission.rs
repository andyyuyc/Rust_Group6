use std::io;

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
    if user == "valid_user" && file_path == "valid_path" && (permission == Permission::Read || permission == Permission::Execute){
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Insufficient permission"));
    }

    Ok(())
}

fn main() {
    let tests = vec![
        ("valid_user", "valid_path", Permission::Read),
        ("valid_user", "valid_path", Permission::Execute),
        ("", "valid_path", Permission::Write),
        ("valid_user", "", Permission::Execute),
        ("valid_user", "valid_path", Permission::Write),
    ];

    for (user, file_path, permission) in tests {
        let result = verify_permissions(user, file_path, permission);
        match result {
            Ok(()) => println!("Permission granted for {:?} on {:?} with {:?}", user, file_path, permission),
            Err(e) => println!("Permission denied for {:?} on {:?} with {:?}: {}", user, file_path, permission, e),
        }
    }
}