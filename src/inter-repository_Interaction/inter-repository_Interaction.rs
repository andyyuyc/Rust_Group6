use std::fs::{self, File};
use std::io::{self, Write, stdin, Read, BufReader};
use std::path::Path;
use std::collections::HashSet;


fn main() {
    println!("pull/push");
    let mut action = String::new();
    stdin().read_line(&mut action).expect("Failed to read line");

    match action.trim() {
        "pull" => {
            let (remote_path, local_path) = get_paths("pull");
            if let Err(e) = pull(&remote_path, &local_path) {
                println!("Error during pull: {}", e);
            }
        },
        "push" => {
            let (local_path, remote_path) = get_paths("push");
            if let Err(e) = push(&local_path, &remote_path) {
                println!("Error during push: {}", e);
            }
        },
        _ => println!("Invalid action. Please enter 'pull' or 'push'."),
    }
}


fn get_paths(action: &str) -> (String, String) {
    let mut first_path = String::new();
    let mut second_path = String::new();

    if action == "pull" {
        println!("Enter the remote path:");
        stdin().read_line(&mut first_path).expect("Failed to read remote path");
        println!("Enter the local path:");
        stdin().read_line(&mut second_path).expect("Failed to read destination path");
    } else {
        println!("Enter the local path:");
        stdin().read_line(&mut first_path).expect("Failed to read local path");
        println!("Enter the remote path:");
        stdin().read_line(&mut second_path).expect("Failed to read destination path");
    }

    (first_path.trim().to_string(), second_path.trim().to_string())
}


fn detect_changes(source_lines: &[Vec<u8>], dest_lines: &[Vec<u8>], is_pull_operation: bool) -> bool {
    if is_pull_operation {
        !dest_lines.iter().all(|line| source_lines.contains(line))
    } else {
        !source_lines.iter().all(|line| dest_lines.contains(line))
    }
}


fn synchronize_changes(source_path: &str, dest_path: &str, is_pull_operation: bool) -> io::Result<bool> {
    let mut source_file = File::open(source_path)?;
    let mut dest_file = File::open(dest_path)?;

    let mut source_contents = Vec::new();
    let mut dest_contents = Vec::new();
    source_file.read_to_end(&mut source_contents)?;
    dest_file.read_to_end(&mut dest_contents)?;

    let source_lines: Vec<Vec<u8>> = source_contents.split(|&b| b == b'\n').map(Vec::from).collect();
    let dest_lines: Vec<Vec<u8>> = dest_contents.split(|&b| b == b'\n').map(Vec::from).collect();

    let conflict = detect_changes(&source_lines, &dest_lines, is_pull_operation);

    if conflict {
        println!("Merge conflict detected. Manual resolution required.");
        Ok(true)
    } else {
        let data_to_write = if is_pull_operation { &source_contents } else { &dest_contents };
        File::create(dest_path)?.write_all(data_to_write)?;
        Ok(false)
    }
}


fn pull(remote_path: &str, local_path: &str) -> io::Result<()> {
    let remote_files = fs::read_dir(remote_path)?;
    let mut conflict_occurred = false;

    for entry in remote_files {
        let entry = entry?;
        let file_name = entry.file_name();
        let source_file = Path::new(remote_path).join(&file_name);
        let dest_file = Path::new(local_path).join(&file_name);

        if Path::exists(&dest_file) {
            if synchronize_changes(source_file.to_str().unwrap(), dest_file.to_str().unwrap(), true)? {
                conflict_occurred = true;
                break;
            }
        } else {
            fs::copy(source_file, dest_file)?;
        }
    }

    if !conflict_occurred {
        println!("Pull completed.");
    }

    Ok(())
}


fn push(local_path: &str, remote_path: &str) -> io::Result<()> {
    let local_files = fs::read_dir(local_path)?;
    let mut conflict_occurred = false;

    for entry in local_files {
        let entry = entry?;
        let file_name = entry.file_name();
        let source_file = Path::new(local_path).join(&file_name);
        let dest_file = Path::new(remote_path).join(&file_name);

        if Path::exists(&dest_file) {
            if synchronize_changes(source_file.to_str().unwrap(), dest_file.to_str().unwrap(), true)? {
                conflict_occurred = true;
                break;
            }
        } else {
            fs::copy(source_file, dest_file)?;
        }
    }

    if !conflict_occurred {
        println!("Push completed.");
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use tempfile::TempDir;

    fn setup_test_environment() -> io::Result<(TempDir, TempDir)> {
        let remote_dir = TempDir::new()?;
        let local_dir = TempDir::new()?;
        Ok((remote_dir, local_dir))
    }

    fn create_test_file(dir: &Path, file_name: &str, content: &[u8]) -> io::Result<()> {
        let file_path = dir.join(file_name);
        let mut file = File::create(file_path)?;
        file.write_all(content)?;
        Ok(())
    }

    #[test]
    fn test_pull_valid_paths() -> io::Result<()> {
        let (remote_dir, local_dir) = setup_test_environment()?;
        create_test_file(remote_dir.path(), "test.txt", b"Hello, world!")?;

        let result = pull(remote_dir.path().to_str().unwrap(), local_dir.path().to_str().unwrap());
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_pull_invalid_paths() {
        let result = pull("non_existent_directory", "another_non_existent_directory");
        assert!(result.is_err());
    }

    #[test]
    fn test_push_valid_paths() -> io::Result<()> {
        let (local_dir, remote_dir) = setup_test_environment()?;
        create_test_file(local_dir.path(), "test.txt", b"Hello, world!")?;

        let result = push(local_dir.path().to_str().unwrap(), remote_dir.path().to_str().unwrap());
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_push_invalid_paths() {
        let result = push("non_existent_directory", "another_non_existent_directory");
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_no_changes() {
        let content = b"Hello, world!\nThis is a test.";
        let source_lines = content.split(|&b| b == b'\n').map(Vec::from).collect::<Vec<_>>();
        let dest_lines = source_lines.clone();

        let changes_detected = detect_changes(&source_lines, &dest_lines, true);
        assert!(!changes_detected);
    }

    #[test]
    fn test_detect_changes() {
        let source_content = b"Hello, world!\nThis is a test.";
        let dest_content = b"Hello, world!\nThis is a modified test.";
        let source_lines = source_content.split(|&b| b == b'\n').map(Vec::from).collect::<Vec<_>>();
        let dest_lines = dest_content.split(|&b| b == b'\n').map(Vec::from).collect::<Vec<_>>();

        let changes_detected = detect_changes(&source_lines, &dest_lines, true);
        assert!(changes_detected);
    }

    #[test]
    fn test_synchronize_no_conflict() -> io::Result<()> {
        let (source_dir, dest_dir) = setup_test_environment()?;
        let file_name = "test.txt";
        let source_file_path = source_dir.path().join(file_name);
        let dest_file_path = dest_dir.path().join(file_name);

        File::create(&source_file_path)?.write_all(b"Test content")?;
        File::create(&dest_file_path)?.write_all(b"Test content")?;

        let conflict = synchronize_changes(source_file_path.to_str().unwrap(), dest_file_path.to_str().unwrap(), true)?;
        assert!(!conflict);
        
        Ok(())
    }

    #[test]
    fn test_synchronize_conflict() -> io::Result<()> {
        let (source_dir, dest_dir) = setup_test_environment()?;
        let file_name = "test.txt";
        let source_file_path = source_dir.path().join(file_name);
        let dest_file_path = dest_dir.path().join(file_name);

        File::create(&source_file_path)?.write_all(b"Original content")?;
        File::create(&dest_file_path)?.write_all(b"Modified content")?;

        let conflict = synchronize_changes(source_file_path.to_str().unwrap(), dest_file_path.to_str().unwrap(), true)?;
        assert!(conflict);

        Ok(())
    }
}