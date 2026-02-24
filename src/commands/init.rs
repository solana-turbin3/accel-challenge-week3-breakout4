use std::{env, fs};

pub fn init() -> Result<(), std::io::Error> {
    let repo_path = env::current_dir()?.join(".it");
    if repo_path.exists() && repo_path.is_dir() {
        println!("Already a IT Repository");
        return Ok(());
    }

    fs::create_dir_all(repo_path.join("objects"))?;
    fs::create_dir_all(repo_path.join("refs/heads"))?;

    fs::write(repo_path.join("HEAD"), format!("ref: refs/heads/main\n"))?;

    // LOG FILES
    let logs_path = repo_path.join("logs");
    fs::create_dir_all(&logs_path)?;
    fs::create_dir_all(logs_path.join("refs/heads"))?;
    fs::write(logs_path.join("HEAD.md"), "")?;

    // for it add command (staging area)
    fs::File::create(repo_path.join("index"))?;
    Ok(())
}
