use std::{io::{self, Write, Read}, fs::{File, read_to_string}, path::PathBuf};
use serde::{Serialize, de::DeserializeOwned};
use crate::file_management::hash::Hash;
use crate::file_management::hash::DVCSHash;

pub fn add_serialized_object<T>(obj: &T) -> io::Result<()> 
    where T: DVCSHash + Serialize
{
    // Serialize the object and write it to the file with the hash
    let hash = obj.get_hash();
    let serialized_data = serde_json::to_string(obj)?;
    add_object(hash, serialized_data.as_bytes())?;

    Ok(())
}

pub fn get_serialized_object<T: DeserializeOwned>(hash: Hash) -> io::Result<T> {
    // Get serialized string
    let data = get_object(hash)?;
    let serialized_string = String::from_utf8_lossy(&data).into_owned();

    // Return the deserialized object
    serde_json::from_str(&serialized_string)
        .map_err(|e| 
            io::Error::new(io::ErrorKind::Other, e
        ))
}

pub fn add_object(hash: Hash, data: &[u8]) -> io::Result<()> {
    // Get file path
    let path = get_path(hash);

    if !path.exists() {
        // Create directory for prefix if nonexistant
        let parent_path = path.parent()
            .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid Path"))?;
        std::fs::create_dir_all(parent_path)?;

        // Create the new object file
        let mut file = File::create(path)?;

        // Write data to the file
        file.write_all(&data)?;
        file.flush()?;
    }

    Ok(())
}

pub fn get_object(hash: Hash) -> io::Result<Vec<u8>> {
    // Get file path
    let path = get_path(hash);

    // Read data from file into a vector
    let mut file = File::open(path)?;
    let mut data = Vec::new(); 
    file.read_to_end(&mut data)?;

    Ok(data)
}

fn get_path(hash: Hash) -> PathBuf {
    let hash = hash.as_string();
    let prefix = &hash[0..2];
    let postfix = &hash[2..];
    PathBuf::from("obj")
        .join(prefix)
        .join(format!("{}.obj", postfix))
}

#[test]
fn create_dir_test() {
    let dir_path = PathBuf::from("potato/pot.txt");
    std::fs::create_dir_all(&dir_path.parent().unwrap());
}