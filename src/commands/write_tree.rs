use crate::commands::hash_object::{
    build_object, compress_data, compute_hash, hex_to_sha_bytes, store_object,
};
use crate::error::ItError;
use crate::index::{IndexEntry, read_index};
use std::collections::BTreeMap;
use std::path::Path;

pub fn write_tree() -> Result<String, ItError> {
    let repo_path = Path::new(".it");
    let entries = read_index(repo_path)?;

    if entries.is_empty() {
        return Err(ItError::NothingToCommit);
    }

    let hash = build_tree(&entries, "")?;
    println!("{}", hash);

    Ok(hash)
}

//      prefix="" and path="src/main.rs" -> component="src", rest="main.rs"
//      prefix="" and path="README.md"   -> component="README.md", rest=""
fn build_tree(entries: &[IndexEntry], prefix: &str) -> Result<String, ItError> {
    let mut tree_content: Vec<u8> = Vec::new();
    let mut entries_by_name: BTreeMap<String, Vec<&IndexEntry>> = BTreeMap::new();

    for entry in entries {
        let rel_path = if prefix.is_empty() {
            &entry.path
        } else {
            match entry.path.strip_prefix(&format!("{}/", prefix)) {
                Some(p) => p,
                None => continue,
            }
        };

        let name = rel_path.split('/').next().unwrap().to_string();
        entries_by_name.entry(name).or_default().push(entry);
    }

    for (name, group) in entries_by_name {
        let has_subdirs = group.iter().any(|e| {
            let rel = if prefix.is_empty() {
                &e.path
            } else {
                e.path.strip_prefix(&format!("{}/", prefix)).unwrap()
            };
            rel.contains('/')
        });

        if !has_subdirs {
            let entry = group[0];
            let header = format!("100644 {}\0", name);
            tree_content.extend_from_slice(header.as_bytes());
            tree_content.extend_from_slice(&entry.sha);
        } else {
            let sub_prefix = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", prefix, name)
            };
            let sub_hash = build_tree(entries, &sub_prefix)?;
            let header = format!("040000 {}\0", name);
            tree_content.extend_from_slice(header.as_bytes());
            tree_content.extend_from_slice(&hex_to_sha_bytes(&sub_hash));
        }
    }

    let tree_object = build_object(&tree_content, "tree");
    let hash = compute_hash(&tree_object);
    let compressed = compress_data(&tree_object)?;
    store_object(&hash, &compressed)?;

    Ok(hash)
}
