use std::{io::{self, Write, Read}, fs::File, path::PathBuf};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use crate::file_management::hash::Hash;
use crate::file_management::hash::DVCSHash;

// Adds a serialized object to the hashed path. If the object already exists
// no copying is performed
pub fn add_serialized_object<T>(obj: &T) -> io::Result<()> 
    where T: DVCSHash + Serialize
{
    // Serialize the object and write it to the file with the hash
    let hash = obj.get_hash();
    let serialized_data = serde_json::to_string(obj)?;
    add_object(hash, serialized_data.as_bytes())?;

    Ok(())
}

// Retrieves the serialized string from the hashed path
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

// Adds an object to the hashed path. If the object already exists,
// no copying is performed
pub fn add_object(hash: Hash, data: &[u8]) -> io::Result<()> {
    // Get file path
    let path = get_relative_path(hash);

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

// Retrieves the data stored in the object at the hashed path
pub fn get_object(hash: Hash) -> io::Result<Vec<u8>> {
    // Get file path
    let path = get_relative_path(hash);

    // Read data from file into a vector
    let mut file = File::open(path)?;
    let mut data = Vec::new(); 
    file.read_to_end(&mut data)?;

    Ok(data)
}

// Gets the relative path associated with the hash 
pub fn get_relative_path(hash: Hash) -> PathBuf {
    let hash = hash.as_string();
    let prefix = &hash[0..2];
    let postfix = &hash[2..];
    PathBuf::from("dvcs")
        .join("obj")
        .join(prefix)
        .join(format!("{}.obj", postfix))
}

pub fn get_repository_path() -> io::Result<PathBuf> {
    todo!()
}

#[test]
fn test1() {
    #[derive(Serialize, Deserialize)]
    struct TestStruct {
        data: String
    }

    impl DVCSHash for TestStruct {
        fn get_hash(&self) -> Hash {
            Hash::new(&self.data)
        }
    }

    let test_struct = TestStruct {data: String::from("hello")};
    add_serialized_object(&test_struct);

    let path = get_relative_path(test_struct.get_hash());
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);

    assert_eq!(r#"{"data":"hello"}"#, s);

    let deserialized_object: TestStruct = get_serialized_object(test_struct.get_hash()).unwrap();
    assert_eq!(deserialized_object.data, "hello")
}