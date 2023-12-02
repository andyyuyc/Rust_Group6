use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};

use super::hash::{Hash, DVCSHash};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Directory {
    files: HashMap<String, BlobRef>,
}

impl Directory {
    pub fn new() -> Directory {
        Directory {
            files: HashMap::new(),
        }
    }

    pub fn contains_file_ref(&self, path: &PathBuf) -> bool {
        self.files.contains_key(&path.to_string_lossy().into_owned())
    }

    // Returns the hashed reference to a file given its parent path
    pub fn get_file_ref(&self, path: &PathBuf) -> Option<&BlobRef> {
        self.files.get(&path.to_string_lossy().into_owned())
    }

    // Inserts a hash reference to a file 
    pub fn insert_file_ref(&mut self, path: &PathBuf, file_ref: BlobRef) {
        let path_as_str = path.to_string_lossy().into_owned();
        if !self.files.contains_key(&path_as_str) { 
            self.files.insert(path_as_str, file_ref);
        }
    }

    pub fn remove_file_ref(&mut self, path: &PathBuf) -> Option<(String, BlobRef)> {
        self.files.remove_entry(&path.to_string_lossy().into_owned())
    }

    pub fn modify_file_ref(&mut self, path: &PathBuf, hash: Hash) -> Option<()> {
        let path_as_str = path.to_string_lossy().into_owned();
        match self.files.get_mut(&path_as_str) {
            Some(blob_ref) => {
                blob_ref.update_content_hash(hash);
                Some(())
            },
            None => None,
        }
    }

    pub fn get_key_value_pairs(&self) -> impl Iterator<Item = (PathBuf, BlobRef)> + '_ {
        self.files.iter()
            .map(|(path, blobref)| 
                (PathBuf::from(path), blobref.clone())
            )
    }
}

impl DVCSHash for Directory {
    fn get_hash(&self) -> Hash {
        let mut hash = String::new();
        for (path, blobref) in self.get_key_value_pairs() {
            hash.push_str(path.to_string_lossy().into_owned().as_str());
            hash.push_str(blobref.get_content_hash().as_string().as_str());
        }
        Hash::new(&hash)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BlobRef {
    name: String,
    content_hash: Hash                          // Hash of the blob it references
}

impl BlobRef {
    pub fn new(name: &str, content_hash: Hash) -> BlobRef {
        BlobRef { 
            name: String::from(name), 
            content_hash
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_content_hash(&self) -> &Hash {
        &self.content_hash
    }

    pub fn update_content_hash(&mut self, hash: Hash) {
        self.content_hash = hash
    }
}

#[test]
fn serialize_blob_test() {
    let blob = BlobRef::new("test-blob", Hash::new("test-hash"));

    let serialized_blob = serde_json::to_string(&blob).unwrap();
    println!("Serialized: {}", serde_json::to_string(&blob).unwrap());

    let deserialized_blob: BlobRef = serde_json::from_str(&serialized_blob).unwrap();
    println!("Deserialized: {:?}", deserialized_blob);
}

#[test]
fn directory_serialize_deserialize_test() {
    use sha256::digest;

    let mut directory = Directory::new();

    let path1 = PathBuf::from("file1");
    let file_ref1 = BlobRef::new("file1", Hash::new("od-file1-hash"));
    directory.insert_file_ref(&path1, file_ref1);

    let path2 = PathBuf::from("something/file2");
    let file_ref2 = BlobRef::new("inner-file1", Hash::new("od-filein-hash"));
    directory.insert_file_ref(&path2, file_ref2);
    
    let serialized_dir = serde_json::to_string(&directory).unwrap();
    println!("Serialized: {}", serialized_dir);

    let deserialized_dir: Directory = serde_json::from_str(&serialized_dir).unwrap();
    println!("Deserialized: {:?}", deserialized_dir);

    let blob_ref = deserialized_dir.get_file_ref(&PathBuf::from("file1")).unwrap();
    assert_eq!(blob_ref.get_content_hash().as_string(), digest("od-file1-hash"));

    let blob_ref2 = deserialized_dir.get_file_ref(&PathBuf::from("something/file2")).unwrap();
    assert_eq!(blob_ref2.get_content_hash().as_string(), digest("od-filein-hash"));
}