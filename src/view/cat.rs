use std::{path::PathBuf, fs::{File, read_to_string}};


pub fn cat(path: &PathBuf) -> std::io::Result<String> {
    read_to_string(path)
}