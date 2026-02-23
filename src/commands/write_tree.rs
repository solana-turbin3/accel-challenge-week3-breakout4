use std::io;
use crate::index::IndexEntry;
use crate::commands::hash_object::{build_object, compute_hash, compress_data, store_object};

pub fn write_tree(entries: &[IndexEntry]) -> Result<String, io::Error> {
    let mut tree_data: Vec<u8> = Vec::new();

    for entry in entries {
        // format: "<mode> <path>\0<20-byte-sha>"
        tree_data.extend_from_slice(b"100644 ");
        tree_data.extend_from_slice(entry.path.as_bytes());
        tree_data.push(0);
        tree_data.extend_from_slice(&entry.sha);
    }

    let full_data = build_object(&tree_data, "tree");
    let tree_hash = compute_hash(&full_data);
    let compressed = compress_data(&full_data)?;
    store_object(&tree_hash, &compressed)?;

    Ok(tree_hash)
}