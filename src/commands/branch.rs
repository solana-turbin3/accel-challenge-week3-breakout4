use std::fs;

pub fn branch(name: Option<String>) -> std::io::Result<()> {
    let repo_path = std::env::current_dir()?.join(".it");
    let heads_path = repo_path.join("refs/heads");

    match name {
        Some(branch_name) => {
            let new_branch_path = heads_path.join(branch_name.clone());
            if new_branch_path.exists() {
                println!("branch {branch_name} already exists");
                return Ok(());
            }

            let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
            let current_ref = head_content.trim_start_matches("ref: ").trim();
            let current_ref_path = repo_path.join(current_ref);

            if !current_ref_path.exists() {
                println!("not a valid ref {current_ref}");
                return Ok(());
            }

            let current_hash = fs::read_to_string(current_ref_path)?;
            fs::write(new_branch_path, current_hash)?;
            println!("branch '{branch_name}' created");
        }

        None => {
            let entries = fs::read_dir(heads_path)?;
            let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
            let current_branch = head_content.trim_start_matches("ref: refs/heads/").trim();

            for entry in entries {
                let entry = entry?;
                if current_branch.eq(entry.file_name().to_str().unwrap()) {
                    println!("* {}", entry.file_name().to_string_lossy());
                } else {
                    println!("  {}", entry.file_name().to_string_lossy());
                }
            }
        }
    }

    Ok(())
}
