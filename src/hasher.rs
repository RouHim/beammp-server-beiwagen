use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

pub fn sha256(local_file_data: &Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(local_file_data);
    return format!("{:X}", hasher.finalize()).to_lowercase();
}

pub fn sha256_file(local_file_path: &str) -> String {
    fs::metadata(local_file_path)
        .expect(format!("{} not found", local_file_path).as_str());

    let file_data = fs::read(Path::new(local_file_path)).unwrap();
    return sha256(&file_data).to_lowercase();
}

pub fn sha256_path(local_file: &PathBuf) -> String {
    fs::metadata(local_file)
        .expect(format!("{} not found", local_file.display().to_string()).as_str());

    let file_data = fs::read(local_file).unwrap();
    return sha256(&file_data).to_lowercase();
}
