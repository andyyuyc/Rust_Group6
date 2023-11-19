use std::io;

#[derive(Debug, PartialEq)]
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
