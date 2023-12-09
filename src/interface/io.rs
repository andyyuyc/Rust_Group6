use std::{io::{self, Write, Read}, fs::{File, OpenOptions}, path::PathBuf};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use crate::file_management::{hash::Hash, directory::{Directory, BlobRef}};
use crate::file_management::hash::DVCSHash;

pub struct RepositoryInterface {
    dir_path: PathBuf
}

impl RepositoryInterface {
    /// Instantiate a `RepositoryInterface` from a given dir path.
    /// Returns [`None`] if the specified dir path is not a directory
    /// or does not contain a dvcs repository
    pub fn new(dir_path: &PathBuf) -> Option<RepositoryInterface> {
        if Self::is_repo(dir_path) {
            return Some(RepositoryInterface { 
                dir_path: dir_path.clone() 
            })
        }
        None
    }

    /// Returns true if the specified directory contains a repo
    pub fn is_repo(dir_path: &PathBuf) -> bool {
        if dir_path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(dir_path) {
                return entries.into_iter()
                    .filter_map(|entry| entry.ok())
                    .any(|entry| entry.path().is_dir() && entry.file_name() == ".my-dvcs")
            }
            return false
        } 
        false
    }

    /// Get the repo path
    pub fn get_repo_path(&self) -> PathBuf {
        self.dir_path.clone()
    }

    /// Gets the relative path to the object associated with the hash 
    pub fn get_relative_obj_path(hash: Hash) -> PathBuf {
        let hash = hash.as_string();
        let prefix = &hash[0..2];
        let postfix = &hash[2..];
        PathBuf::from(".my-dvcs")
            .join("obj")
            .join(prefix)
            .join(format!("{}.obj", postfix))
    }

    // Adds an object to the hashed path. If the object already exists,
    // no copying is performed
    pub fn add_object(&self, hash: Hash, data: &[u8]) -> io::Result<()> {
        // Get file path
        let relative_path = Self::get_relative_obj_path(hash);
        let path = self.dir_path.join(&relative_path);

        if !path.exists() {
            // Create directory for prefix if nonexistant
            let parent_path = relative_path.parent()
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

    /// Retrieves the data stored in the blob at the hashed path
    pub fn get_object(&self, hash: Hash) -> io::Result<Vec<u8>> {
        // Get file path
        let path = self.dir_path.join(Self::get_relative_obj_path(hash));

        // Read data from file into a vector
        let mut file = File::open(path)?;
        let mut data = Vec::new(); 
        file.read_to_end(&mut data)?;

        Ok(data)
    }

    // Adds a serialized object to the hashed path. If the object already exists
    // no copying is performed
    pub fn add_serialized_object<T>(&self, obj: &T) -> io::Result<()> 
        where T: DVCSHash + Serialize
    {
        // Serialize the object and write it to the file with the hash
        let hash = obj.get_hash();
        let serialized_data = serde_json::to_string(obj)?;
        self.add_object(hash, serialized_data.as_bytes())?;

        Ok(())
    }


    /// Retrieves the serialized string from the hashed path
    pub fn get_serialized_object<T: DeserializeOwned>(&self, hash: Hash) -> io::Result<T> {
        // Get serialized string
        let data = self.get_object(hash)?;
        let serialized_string = String::from_utf8_lossy(&data).into_owned();

        // Return the deserialized object
        serde_json::from_str(&serialized_string)
            .map_err(|e| 
                io::Error::new(io::ErrorKind::Other, e
            ))
    }

    /// Converts a file to a blob and adds it to the repository.
    /// Returns a Hash containing the reference to the blob
    // pub fn create_blob(&self, add_path: &PathBuf) -> std::io::Result<Hash> {
    //     // Open the added file and read the data to a vector
    //     let mut file = File::open(&add_path)?;
    //     let mut data = Vec::new();
    //     file.read_to_end(&mut data)?;

    //     // Hash the data and add it as a blob
    //     let hash = Hash::from(&data);
    //     self.add_object(hash.clone(), &data)?;

    //     Ok(hash)
    // }

    /// Converts a file to a blob and adds it to the repository.
    /// Returns a Hash containing the reference to the blob
    pub fn create_blob(&self, data: &[u8]) -> std::io::Result<Hash> {
        // Open the added file and read the data to a vector
        // Hash the data and add it as a blob
        let hash = Hash::from(data);
        self.add_object(hash.clone(), &data)?;

        Ok(hash)
    }

    /// Instantiates a directory from a [`&Vec<&PathBuf>`]
    pub fn create_dir_from_files(&self, file_paths: &Vec<&PathBuf>) -> std::io::Result<Directory> {
        file_paths.iter()
            .try_fold(Directory::new(), |mut acc, &path| {
                // Read the data from the files
                let mut file = File::open(&path)?;
                let mut data = Vec::new();
                file.read_to_end(&mut data)?;

                // Save data as a blob and then insert a blobref to it in the dir
                let hash = self.create_blob(&data)?;
                let blob_ref = BlobRef::new(hash);
                acc.insert_file_ref(path, blob_ref);

                Ok(acc)
        })
    }

    /// Returns the path for a branch in the repo
    pub fn get_branch_path(&self, branch_name: &str) -> PathBuf {
        self.get_repo_path()
            .join(".my-dvcs")
            .join("branches")
            .join(branch_name)
    }

    /// Creates a branch. Returns std::io::Err<()> if the branch already exists
    pub fn create_branch(&self, branch_name: &str, current_hash: Hash) -> std::io::Result<()> {
        let branch_path = self.get_branch_path(branch_name);

        if !branch_path.exists() {
            // Create the file and put the hash inside 
            let mut file = File::create(branch_path)?;
            file.write_all(current_hash.as_string().as_bytes())?;
        } 

        Err(io::Error::new(io::ErrorKind::AlreadyExists, "Branch already exists"))
    }

    /// Updates the hash for the specified branch. Returns std::io::Error<()> if the branch 
    /// does not exist.
    pub fn update_branch_head(&self, branch_name: &str, new_hash: Hash) -> std::io::Result<()> {
        let branch_path = self.get_branch_path(branch_name);

        if branch_path.exists() {
            let mut file = OpenOptions::new()
                .write(true)
                .open(branch_path)?;

            file.write_all(new_hash.as_string().as_bytes());
        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Branch does not exist"))
    }

    /// Retrieves branch name from hash
    pub fn get_branch_from_hash(&self, hash: Hash) -> Option<String> {
        let branches_dir = self.get_repo_path()
            .join(".my-dvcs")
            .join("branches");

        let entries = std::fs::read_dir(branches_dir).ok()?;
        let hash_string = hash.as_string();

        for entry in entries {
            let entry = entry.ok()?;
            let path = entry.path();
            let mut file = File::open(&path).ok()?;

            // Read the string from the file, compare it to the hash
            // and return the branch name if it is the same
            let mut file_hash = String::new();
            file.read_to_string(&mut file_hash).ok()?;
            
            if file_hash.trim() == hash_string {
                return path.file_name()
                    .and_then(|os_str| os_str.to_str())
                    .map(|s| s.to_owned());
            }
        }
    
        None
    } 

    /// Returns the current overall head (not the branch head)
    pub fn get_current_head(&self) -> Option<Hash> {
        let head_path = self.get_repo_path()
            .join(".my-dvcs")
            .join("head");

        let mut file = File::open(head_path).ok()?;
        let mut head = String::new();

        // Copy the hash from the head file  and convert it to a hash
        file.read_to_string(&mut head).ok()?;
        Some(Hash::from_hashed(&head))
    } 

    /// Updates the hash for the specified branch
    pub fn update_current_head(&self, new_hash: Hash) -> Option<()> {
        let head_path = self.get_repo_path()
            .join(".my-dvcs")
            .join("head");
    
        let mut file = File::create(head_path).ok()?;
        file.write_all(new_hash.as_string().as_bytes()).ok()?;

        Some(())
    }

    /// Clears the directory
    pub fn clear_directory(&self) -> io::Result<()> {
        let entries = std::fs::read_dir(&self.dir_path)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
    
            if path.is_file() {
                std::fs::remove_file(path)?;
            }
        }
    
        Ok(())
    }
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
    let repo = RepositoryInterface::new(&PathBuf::from(".")).unwrap();
    repo.add_serialized_object(&test_struct);

    let path = RepositoryInterface::get_relative_obj_path(test_struct.get_hash());
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);

    assert_eq!(r#"{"data":"hello"}"#, s);

    let deserialized_object: TestStruct = repo.get_serialized_object(test_struct.get_hash()).unwrap();
    assert_eq!(deserialized_object.data, "hello")
}