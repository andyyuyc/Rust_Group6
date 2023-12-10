use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn show_diff(current_dir: &str, previous_dir: &str) -> io::Result<()> {
    let current_files = read_dir_files(current_dir, current_dir)?;
    let previous_files = read_dir_files(previous_dir, previous_dir)?;

    let current_set: HashSet<_> = current_files.into_iter().collect();
    let previous_set: HashSet<_> = previous_files.into_iter().collect();

    for file in current_set.difference(&previous_set) {
        println!("Added: {}", file.display());
    }

    for file in previous_set.difference(&current_set) {
        println!("Removed: {}", file.display());
    }

    // Compare file contents for files existing in both directories
    for file in current_set.intersection(&previous_set) {
        compare_file_contents(&format!("{}/{}", current_dir, file.display()), &format!("{}/{}", previous_dir, file.display()))?;
    }

    Ok(())
}

fn read_dir_files<'a>(dir_path: &str, root: &str) -> io::Result<HashSet<PathBuf>> {
    let mut files = HashSet::new();
    for entry in fs::read_dir(Path::new(dir_path))? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let subdir_files = read_dir_files(path.to_str().unwrap(), root)?;
            for file in subdir_files {
                files.insert(file);
            }
        } else {
            let relative_path = path.strip_prefix(Path::new(root)).unwrap().to_path_buf();
            files.insert(relative_path);
        }
    }
    Ok(files)
}

fn compare_file_contents(current_file_path: &str, previous_file_path: &str) -> io::Result<()> {
    let current_file = fs::File::open(current_file_path)?;
    let previous_file = fs::File::open(previous_file_path)?;

    let current_lines: Vec<_> = io::BufReader::new(current_file).lines().collect::<Result<_, _>>()?;
    let previous_lines: Vec<_> = io::BufReader::new(previous_file).lines().collect::<Result<_, _>>()?;

    // Simple line-by-line comparison
    for (i, (current_line, previous_line)) in current_lines.iter().zip(previous_lines.iter()).enumerate() {
        if current_line != previous_line {
            println!("{}: - {}", i + 1, previous_line);
            println!("{}: + {}", i + 1, current_line);
        }
    }

    Ok(())
}

