use std::path::PathBuf;

const GIT_FOLDER: &str = ".git";

pub fn is_repo(target_path: &PathBuf) -> bool {
    let target_string = target_path.to_str().unwrap();
    let target_git_path = PathBuf::from(format!("{}/{}", target_string, GIT_FOLDER));

    println!("{:?}", target_git_path);
    target_git_path.is_dir()
}