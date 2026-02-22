use std::fs;

pub fn switch(branch_name: String) -> std::io::Result<()> {
    let repo_path = std::env::current_dir()?.join(".it");
    let heads_path = repo_path.join("refs/heads");

    let target_branch_path = heads_path.join(&branch_name);
    if !target_branch_path.exists() {
        println!("error: branch '{branch_name}' does not exist");
        return Ok(());
    }

    // read current HEAD to see if we're already on that branch
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
    let current_branch = head_content.trim_start_matches("ref: refs/heads/").trim();

    if current_branch == branch_name {
        println!("already on '{branch_name}'");
        return Ok(());
    }

    // point HEAD to the new branch
    fs::write(repo_path.join("HEAD"), format!("ref: refs/heads/{branch_name}\n"))?;
    println!("switched to branch '{branch_name}'");

    Ok(())
}
