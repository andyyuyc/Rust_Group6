use std::{path::PathBuf, fs::{File, read_to_string}};


pub fn cat(path: &PathBuf) -> std::io::Result<String> {
    let file = File::open(path)?;
    read_to_string(path)
}