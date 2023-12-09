use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

fn read_file(path: &str) -> io::Result<String> {
    let path = Path::new(path);

    // 确保文件存在且可读
    if !path.exists() || !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    fs::read_to_string(path)
}

fn write_file(path: &str, contents: &str) -> io::Result<()> {
    fs::write(path, contents)
}

fn append_to_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(contents.as_bytes())
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

pub fn ErrorHandling() {
    let max_attempts = 3;

    // 尝试读取文件
    for attempt in 1..=max_attempts {
        let read_path = get_input("Enter the path of the file to read: ");
        match read_file(&read_path) {
            Ok(contents) => {
                println!("File contents: \n{}", contents);
                break;
            },
            Err(e) => {
                println!("Failed to read file: {}", e);
                if attempt < max_attempts {
                    println!("The path was not correct. Maybe the file doesn't exist. {} attempts remaining...", max_attempts - attempt);
                }
            }
        }
    }

    // 尝试写入文件
    for attempt in 1..=max_attempts {
        let write_path = get_input("Enter the path of the file to write: ");
        let write_contents = get_input("Enter the contents to write: ");
        match write_file(&write_path, &write_contents) {
            Ok(_) => {
                println!("File written successfully.");
                break;
            },
            Err(e) => {
                println!("Failed to write file: {}", e);
                if attempt < max_attempts {
                    println!("The path was not correct. {} attempts remaining...", max_attempts - attempt);
                }
            }
        }
    }

    // 尝试追加到文件
    for attempt in 1..=max_attempts {
        let append_path = get_input("Enter the path of the file to append to: ");
        let append_contents = get_input("Enter the contents to append: ");
        match append_to_file(&append_path, &append_contents) {
            Ok(_) => {
                println!("File appended successfully.");
                break;
            },
            Err(e) => {
                println!("Failed to append to file: {}", e);
                if attempt < max_attempts {
                    println!("The path was not correct. {} attempts remaining...", max_attempts - attempt);
                }
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_temp_dir_with_file(filename: &str, contents: &str) -> io::Result<PathBuf> {
        let dir = tempdir()?;
        let file_path = dir.path().join(filename);
        let mut file = File::create(&file_path)?;
        writeln!(file, "{}", contents)?;
        Ok(dir.into_path())
    }

    #[test]
    fn test_read_file_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt").to_str().unwrap().to_owned();

        // Create the file first
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content").unwrap();

        // Now append to the file
        append_to_file(&file_path, "text").unwrap();
        assert_eq!(fs::read_to_string(file_path).unwrap(), "Initial content\ntext");
    }

    #[test]
    fn test_read_file_failure() {
        let non_existent_path = "/non/existent/path/test.txt";
        assert!(read_file(non_existent_path).is_err());
    }

    #[test]
    fn test_write_file_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("write_test.txt").to_str().unwrap().to_owned();
        write_file(&file_path, "Sample text").unwrap();
        assert_eq!(fs::read_to_string(file_path).unwrap(), "Sample text");
    }

    #[test]
    fn test_write_file_failure() {
        let invalid_path = "\0";
        assert!(write_file(invalid_path, "Sample text").is_err());
    }

    #[test]
    fn test_append_to_file_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("append_test.txt").to_str().unwrap().to_owned();

        // Create the file first
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Initial content").unwrap();

        // Now append to the file
        append_to_file(&file_path, "Appended text").unwrap();
        assert_eq!(fs::read_to_string(file_path).unwrap(), "Initial content\nAppended text");
    }

    #[test]
    fn test_append_to_file_failure() {
        let non_existent_path = "/non/existent/path/append_test.txt";
        assert!(append_to_file(non_existent_path, "Test content").is_err());
    }
}
