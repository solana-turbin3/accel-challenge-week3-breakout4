use crate::commands::log::log_commit;
use crate::error::ItError;
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn reset() -> Result<(), Box<dyn std::error::Error>> {
    let repo_path = std::path::Path::new(".it");
    let head_content = fs::read_to_string(repo_path.join("HEAD"))?;
    let branch_ref = head_content.trim_start_matches("ref:").trim();
    let branch_path = repo_path.join(branch_ref);
    let current_hash = fs::read_to_string(&branch_path)?.trim().to_string();

    let commit_data = read_and_decompress_object(&current_hash)?;
    let commit_text = String::from_utf8_lossy(&commit_data);

    let parent_hash = commit_text
        .lines()
        .find(|line| line.starts_with("parent "))
        .map(|line| line.trim_start_matches("parent ").trim().to_string());

    if let Some(parent) = parent_hash {
        fs::write(&branch_path, &parent)?;
        restore_from_hash(&parent)?;
        // maybe log?
        println!("moved to parent branch {}", parent);
    } else {
        println!("no parent commit found.");
    }

    Ok(())
}

pub fn restore_from_hash(commit_hash: &str) -> Result<(), Box<dyn std::error::Error>> {
    let commit_data = read_and_decompress_object(commit_hash)?;
    let commit_text = String::from_utf8_lossy(&commit_data);

    // format is tree <hash>
    let tree_hash = commit_text
        .lines()
        .find(|line| line.starts_with("tree "))
        .map(|line| line.trim_start_matches("tree ").trim())
        .ok_or("no tree found")?;

    println!("restoring to tree: {}", tree_hash);
    restore_tree(tree_hash, Path::new("."))?;
    Ok(())
}

fn restore_tree(tree_hash: &str, current_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let tree_data = read_and_decompress_object(tree_hash)?;
    let mut pos = 0;

    // find space to get mode, \0 to get filename adn get hash and write to fs and loop until every change is done
    while pos < tree_data.len() {
        let space_pos = tree_data[pos..]
            .iter()
            .position(|&x| x == b' ')
            .ok_or("wrong tree: missing space")?;
        let mode = std::str::from_utf8(&tree_data[pos..pos + space_pos])?;
        pos += space_pos + 1;

        let null_pos = tree_data[pos..]
            .iter()
            .position(|&x| x == 0)
            .ok_or("Malformed tree: missing null")?;
        let name = std::str::from_utf8(&tree_data[pos..pos + null_pos])?;
        pos += null_pos + 1;

        if pos + 20 > tree_data.len() {
            break;
        }
        let sha_bytes = &tree_data[pos..pos + 20];
        let sha_hex = hex::encode(sha_bytes);
        pos += 20;

        let full_path = current_path.join(name);

        if mode == "040000" {
            fs::create_dir_all(&full_path)?;
            restore_tree(&sha_hex, &full_path)?;
        } else {
            let blob_data = read_and_decompress_object(&sha_hex)?;
            fs::write(&full_path, blob_data)?;
        }
    }
    Ok(())
}

fn read_and_decompress_object(hash: &str) -> std::io::Result<Vec<u8>> {
    let path = Path::new(".it/objects").join(&hash[0..2]).join(&hash[2..]);
    let file = fs::File::open(path)?;
    let mut decoder = ZlibDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;

    // if null byte is present, return the data according ie \0<data>
    if let Some(null_pos) = buffer.iter().position(|&x| x == 0) {
        return Ok(buffer[null_pos + 1..].to_vec());
    }

    Ok(buffer)
}
