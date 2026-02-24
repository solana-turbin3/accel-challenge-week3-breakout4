use crate::{
    commands::hash_object::{hash_object, hex_to_sha_bytes},
    index::{IndexEntry, read_index, write_index},
};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn add(paths: Vec<String>) -> std::io::Result<()> {
    let repo_path = std::env::current_dir().unwrap().join(".it");
    let mut entries = read_index(&repo_path).unwrap();

    for path_str in &paths {
        let path = PathBuf::from(path_str);
        if !path.exists() {
            println!("error: '{}' did not match any files", path_str);
            continue;
        }
        collect_files(&path, &mut entries).unwrap();
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    write_index(&repo_path, &entries)
}

fn collect_files(path: &Path, entries: &mut Vec<IndexEntry>) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            if entry.file_name() == ".it" || entry.file_name() == "target" {
                continue;
            }
            collect_files(&entry_path, entries)?;
        }
    } else {
        stage_file(path, entries)?;
    }
    Ok(())
}

fn stage_file(path: &Path, entries: &mut Vec<IndexEntry>) -> std::io::Result<()> {
    let data = fs::read(path)?;
    let sha_hex = hash_object(&data)?;
    let path_str = path.to_string_lossy().to_string().replace("\\", "/");
    entries.retain(|e| e.path != path_str);

    let flags = (path_str.len() as u16) & 0x0FFF; // better than doing min
    entries.push(IndexEntry {
        sha: hex_to_sha_bytes(&sha_hex),
        flags,
        path: path_str,
    });

    Ok(())
}
