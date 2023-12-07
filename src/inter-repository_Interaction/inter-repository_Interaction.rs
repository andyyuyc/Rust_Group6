use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write, stdin, stdout};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    let mut branches = HashMap::new();
    branches.insert("main".to_string(), "path/to/main/branch".to_string());

    println!("Available branches: {:?}", branches.keys());
    println!("enter/create/delete branch");
    print!("Command: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    let args: Vec<&str> = input.trim().split_whitespace().collect();

    match args.as_slice() {
        ["create", new_branch] => {
            if branches.contains_key(*new_branch) {
                println!("Branch '{}' already exists.", new_branch);
            } else {
                let new_path = format!("path/to/{}/branch", new_branch);
                branches.insert((*new_branch).to_string(), new_path);
                println!("Branch '{}' created.", new_branch);
            }
        },
        ["delete", branch_to_delete] => {
            if branches.remove(*branch_to_delete).is_some() {
                println!("Branch '{}' deleted.", branch_to_delete);
            } else {
                println!("Branch '{}' not found.", branch_to_delete);
            }
        },
        [branch_name] => {
            if let Some(branch_path) = branches.get(*branch_name) {
                execute_branch_action(branch_path);
            } else {
                println!("Branch '{}' not found.", branch_name);
            }
        },
        _ => {
            println!("Invalid command.");
            return;
        },
    }
}


fn execute_branch_action(branch_path: &str) {
    println!("pull/push");
    let mut action = String::new();
    stdin().read_line(&mut action).expect("Failed to read action");

    match action.trim() {
        "pull" => {
            let (remote_path, _) = get_paths();
            if let Err(e) = pull(&remote_path, branch_path) {
                println!("Error during pull: {}", e);
            }
        },
        "push" => {
            let (_, remote_path) = get_paths();
            if let Err(e) = push(branch_path, &remote_path) {
                println!("Error during push: {}", e);
            }
        },
        _ => println!("Invalid action"),
    }
}


fn get_branch_name(branches: &HashMap<String, String>) -> String {
    println!("Enter the branch name:");
    let mut branch_name = String::new();
    stdin().read_line(&mut branch_name).expect("Failed to read branch name");
    let branch_name = branch_name.trim();

    branches.get(branch_name).map_or_else(
        || {
            println!("Branch not found.");
            "main".to_string()
        },
        |_| branch_name.to_string(),
    )
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

