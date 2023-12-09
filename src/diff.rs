
pub mod file_diff {
    use std::fs;
    use std::path::Path;
    use std::collections::HashSet;
    use std::io::{self, BufRead};


//functions  for the diff logic
pub fn diff(current_dir: &str, previous_dir: &str) -> io::Result<()> {
    let current_files = read_dir_files(current_dir)?;
    let previous_files = read_dir_files(previous_dir)?;

    let current_set: HashSet<_> = current_files.iter().collect();
    let previous_set: HashSet<_> = previous_files.iter().collect();

    for file in current_set.difference(&previous_set) {
        println!("Added: {}", file);
    }

    for file in previous_set.difference(&current_set) {
        println!("Removed: {}", file);
    }

    for file in current_set.intersection(&previous_set) {
        compare_file_contents(&format!("{}/{}", current_dir, file), &format!("{}/{}", previous_dir, file))?;
    }

    Ok(())
}

fn read_dir_files(dir_path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(Path::new(dir_path))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                files.push(file_name.to_owned());
            }
        }
    }
    Ok(files)
}

fn compare_file_contents(current_file_path: &str, previous_file_path: &str) -> io::Result<()> {
    let current_file = fs::File::open(current_file_path)?;
    let previous_file = fs::File::open(previous_file_path)?;

    let current_lines: Vec<_> = io::BufReader::new(current_file).lines().collect::<Result<_, _>>()?;
    let previous_lines: Vec<_> = io::BufReader::new(previous_file).lines().collect::<Result<_, _>>()?;

    // this line by line comparison is used for checking changes in file contents
    for (i, (current_line, previous_line)) in current_lines.iter().zip(previous_lines.iter()).enumerate() {
        if current_line != previous_line {
            println!("{}: - {}", i + 1, previous_line);
            println!("{}: + {}", i + 1, current_line);
        }
    }

    Ok(())
}

}