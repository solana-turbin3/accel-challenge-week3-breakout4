use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    commands::hash_object::{compress_data, compute_hash, store_object},
    error::ItError,
};

pub fn commit_tree(
    tree_hash: &str,
    parent: Option<String>,
    message: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let author_name = std::env::var("GIT_AUTHOR_NAME").unwrap_or_else(|_| "Unknown".to_string());
    let author_email = std::env::var("GIT_AUTHOR_EMAIL").unwrap_or_else(|_| "Unknown".to_string());

    let committer_name = author_name.clone();
    let committer_email = author_email.clone();

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    // TODO: use utc
    let timezone_offset = 19800;
    let timezone = "+0530";
    let local_timestamp = now + timezone_offset;

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

    let content_bytes = content.as_bytes();
    let header = format!("commit {}\0", content_bytes.len());
    let mut full_data = Vec::new();
    full_data.extend_from_slice(header.as_bytes());
    full_data.extend_from_slice(content_bytes);

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

    println!("committed: {}", commit_hash[..8].to_string());
    Ok(commit_hash)
}

pub fn get_parent() -> Result<Option<String>, ItError> {
    let repo_path = std::env::current_dir()?.join(".it");
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;

    // if head is detached
    if !head_content.starts_with("ref:") {
        return Ok(Some(head_content.trim().to_string()));
    }

    let ref_path_str = head_content.trim_start_matches("ref:").trim();
    let ref_path = repo_path.join(ref_path_str);

    if ref_path.exists() {
        let parent_hash = fs::read_to_string(ref_path)?;
        Ok(Some(parent_hash.trim().to_string()))
    } else {
        Ok(None)
    }
}
