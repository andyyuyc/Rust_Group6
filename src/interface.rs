
pub mod interface {
    use std::fs;
    use std::path::Path;    

    //logic for creating the init directory
    pub fn init(repo_root_path: &str) {
    let repo_path = Path::new(repo_root_path).join(".mydvcs");
    if !repo_path.exists() {
        fs::create_dir_all(&repo_path).expect("Failed to create repository directory");
        
        println!("Initialized empty repository in {:?}", repo_path);
    } else {
        println!("Repository already exists at {:?}", repo_path);
    }
}
    //logic for teh clone method
    pub fn clone(src_repo_path: &str, dst_repo_path: &str) {
        // this assumes local system paths
        let src_path = Path::new(src_repo_path).join(".mydvcs");
        let dst_path = Path::new(dst_repo_path).join(".mydvcs");
    
        fs::create_dir_all(&dst_path).expect("Failed to create destination directory");
        fs::copy(&src_path, &dst_path).expect("Failed to clone repository");
        println!("Cloned repository from {:?} to {:?}", src_path, dst_path);

    }
    
    //logic for commit
    pub fn commit(repo_root_path: &str, commit_message: &str) {
        let repo_path = Path::new(repo_root_path).join(".mydvcs");
        if repo_path.exists() {
            //detecting and saving commit
            println!("Committed changes: {}", commit_message);
        } else {
            println!("No repository found at {:?}", repo_path);
        }
    }

}

