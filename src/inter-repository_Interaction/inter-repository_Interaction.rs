use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write, stdin, stdout};
use std::path::Path;


fn main() {
    println!("pull/push");
    let mut action = String::new();
    stdin().read_line(&mut action).expect("Failed to read line");

    match action.trim() {
        "pull" => {
            let (remote_path, local_path) = get_paths();
            if let Err(e) = pull(&remote_path, &local_path) {
                println!("Error during pull: {}", e);
            }
        },
        "push" => {
            let (local_path, remote_path) = get_paths();
            if let Err(e) = push(&local_path, &remote_path) {
                println!("Error during push: {}", e);
            }
        },
        _ => println!("Invalid action. Please enter 'pull' or 'push'."),
    }
}


fn get_paths() -> (String, String) {
    println!("Enter the source path:");
    let mut source_path = String::new();
    stdin().read_line(&mut source_path).expect("Failed to read source path");

    println!("Enter the destination path:");
    let mut destination_path = String::new();
    stdin().read_line(&mut destination_path).expect("Failed to read destination path");

    (source_path.trim().to_string(), destination_path.trim().to_string())
}


fn detect_changes(file1: &str, file2: &str) -> io::Result<bool> {
    let f1 = File::open(file1)?;
    let f2 = File::open(file2)?;

    let file1_lines = BufReader::new(f1).lines();
    let file2_lines = BufReader::new(f2).lines();

    let mut conflict = false;
    for (line1, line2) in file1_lines.zip(file2_lines) {
        match (line1, line2) {
            (Ok(l1), Ok(l2)) => {
                if l1 != l2 {
                    println!("Conflict found:\nFile1: {}\nFile2: {}", l1, l2);
                    conflict = true;
                }
            },
            _ => break,
        }
    }

    Ok(conflict)
}


fn synchronize_changes(source: &str, destination: &str) -> io::Result<()> {
    let conflict = detect_changes(source, destination)?;

    if conflict {
        println!("Merge conflict detected. Manual resolution required.");
    } else {
        fs::copy(source, destination)?;
    }

    Ok(())
}


fn pull(remote_path: &str, local_path: &str) -> io::Result<()> {
    let remote_files = fs::read_dir(remote_path)?;
    for entry in remote_files {
        let entry = entry?;
        let file_name = entry.file_name();
        let source_file = Path::new(remote_path).join(&file_name);
        let dest_file = Path::new(local_path).join(&file_name);

        if Path::exists(&dest_file) {
            synchronize_changes(source_file.to_str().unwrap(), dest_file.to_str().unwrap())?;
        } else {
            fs::copy(source_file, dest_file)?;
        }
    }

    println!("Pull completed.");
    Ok(())
}


fn push(local_path: &str, remote_path: &str) -> io::Result<()> {
    let local_files = fs::read_dir(local_path)?;
    for entry in local_files {
        let entry = entry?;
        let file_name = entry.file_name();
        let source_file = Path::new(local_path).join(&file_name);
        let dest_file = Path::new(remote_path).join(&file_name);

        fs::copy(source_file, dest_file)?;
    }

    println!("Push completed.");
    Ok(())
}
