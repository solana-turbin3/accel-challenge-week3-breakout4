use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;
use crate::commands::hash_object::{build_object, compute_hash, compress_data, store_object};
use crate::commands::write_tree::write_tree;
use crate::index::read_index;

pub fn commit(message: &str) -> Result<(), io::Error> {
    let repo_path = std::env::current_dir()?.join(".it");

    // Build tree from index
    let entries = read_index(&repo_path)?;
    if entries.is_empty() {
        println!("nothing to commit, index is empty");
        return Ok(());
    }
    let tree_hash = write_tree(&entries)?;

    // Resolve parent from current branch ref
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
    let parent: Option<String> = if head_content.starts_with("ref:") {
        let ref_path_str = head_content.trim_start_matches("ref:").trim().to_string();
        let ref_path = repo_path.join(&ref_path_str);
        if ref_path.exists() {
            Some(fs::read_to_string(&ref_path)?.trim().to_string())
        } else {
            None // first commit on this branch
        }
    } else {
        None
    };

    commit_tree(&tree_hash, parent.as_deref(), message)?;
    Ok(())
}

fn commit_tree(tree_hash: &str, parent: Option<&str>, message: &str) -> Result<String, io::Error> {
    let author_name = std::env::var("GIT_AUTHOR_NAME").unwrap_or_else(|_| "Unknown".to_string());
    let author_email = std::env::var("GIT_AUTHOR_EMAIL").unwrap_or_else(|_| "unknown@unknown.com".to_string());
    let committer_name = author_name.clone();
    let committer_email = author_email.clone();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
        .as_secs();

    let timezone_offset: i64 = -18000;
    let timezone = "-0500";
    let local_timestamp = now as i64 + timezone_offset;

    let mut content = String::new();
    content.push_str(&format!("tree {}\n", tree_hash));
    if let Some(parent_hash) = parent {
        content.push_str(&format!("parent {}\n", parent_hash));
    }
    content.push_str(&format!(
        "author {} <{}> {} {}\n",
        author_name, author_email, local_timestamp, timezone
    ));
    content.push_str(&format!(
        "committer {} <{}> {} {}\n",
        committer_name, committer_email, local_timestamp, timezone
    ));
    content.push_str("\n");
    content.push_str(message);

    let full_data = build_object(content.as_bytes(), "commit");
    let commit_hash = compute_hash(&full_data);
    let compressed_data = compress_data(&full_data)?;
    store_object(&commit_hash, &compressed_data)?;

    let head_path = Path::new(".it/HEAD");
    let head_content = fs::read_to_string(head_path)?;
    if head_content.starts_with("ref:") {
        let ref_path_str = head_content.trim_start_matches("ref:").trim();
        let ref_path = Path::new(".it").join(ref_path_str);
        fs::create_dir_all(ref_path.parent().unwrap())?;
        fs::write(&ref_path, &commit_hash)?;
    } else {
        println!("HEAD is detached; commit created without updating refs");
    }

    println!("Commit created: {}", commit_hash);
    Ok(commit_hash)
}