use crate::file_management::directory::Directory;

pub fn show_diff(dir1: &Directory, dir2: &Directory) {
    //comparing file paths hashes in dir1 against dir2
    for (path, blob_ref) in dir1.get_key_value_pairs() {
        match dir2.get_file_ref(&path) {
            Some(blob_ref2) => {
                //logic to compare the hashes/names of the paths
                if blob_ref.get_content_hash() != blob_ref2.get_content_hash() {
                    println!("Modified: {}", path.to_string_lossy());
                }
            },
            None => {
            //displays files that are in dir 1 but not in dir2
                println!("--- dir_1{} ", path.to_string_lossy());
            },
        }
    }

    //checks for files that are in dir2 but not in dir1
    for (path, _) in dir2.get_key_value_pairs() {
        if dir1.get_file_ref(&path).is_none() {
            println!("Files ");
            println!("+++ dir_2{}", path.to_string_lossy());
        }
    }
}