use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};

use crate::file_management::commit::Commit; 

// Write the file
// This function takes a `Commit` object and appends its information to the log file.
pub fn dvcs_log(commit: &Commit) -> std::io::Result<()> {
    // Make sure the `.my-dvcs/logs` directory exists.
    let dvcs_dir = Path::new(".my-dvcs");
    let logs_dir = dvcs_dir.join("logs");
    create_dir_all(&logs_dir)?;

    // Open the log file and create it if it doesn't exist
    let log_file_path = logs_dir.join("log.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(log_file_path)?;

    // Write the commit details to a file
    writeln!(file, "{}", commit)?;

    Ok(())
}


// Read the file
pub fn log() -> io::Result<()> {
    // Define the path to the log file
    let dvcs_dir = Path::new(".my-dvcs");
    let logs_dir = dvcs_dir.join("logs");
    let log_file_path = logs_dir.join("log.txt");

    // Open the log file
    let mut file = File::open(log_file_path)?;

    // Read the contents of the file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the contents to the console
    println!("{}", contents);

    Ok(())
}



#[cfg(test)]
mod log_tests {
    use super::*;

    #[test]
    fn test_log_with_no_file() {
        // Make sure the log.txt file does not exist
        assert!(log().is_err());
    }

    
}