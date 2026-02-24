use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn hash_object(data: &[u8]) -> Result<String, std::io::Error> {
    let blob = build_object(data, "blob");
    let hash = compute_hash(&blob);
    let compressed = compress_data(&blob)?;
    store_object(&hash, &compressed)?;
    Ok(hash)
}

pub(crate) fn build_object(data: &[u8], obj_type: &str) -> Vec<u8> {
    let header = format!("{} {}\0", obj_type, data.len());
    let mut store_data = header.into_bytes();
    store_data.extend_from_slice(data);
    store_data
}

pub(crate) fn compute_hash(data: &[u8]) -> String {
    format!("{:x}", Sha1::digest(data))
}

pub(crate) fn compress_data(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(data)?;
    e.finish()
}

pub(crate) fn store_object(hash: &str, compressed_data: &[u8]) -> Result<(), std::io::Error> {
    let objects_dir = Path::new(".it/objects").join(&hash[0..2]);
    fs::create_dir_all(&objects_dir)?;
    let object_path = objects_dir.join(&hash[2..]);
    if !object_path.exists() {
        fs::write(&object_path, compressed_data)?;
    }
    Ok(())
}

pub fn hex_to_sha_bytes(hex: &str) -> [u8; 20] {
    let mut bytes = [0u8; 20];
    for i in 0..20 {
        bytes[i] = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap();
    }
    bytes
}
