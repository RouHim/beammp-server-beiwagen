use std::{fs, io};
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::resource::Resource;

mod resource;

fn main() {
    // read local available mods
    let local_mods: Vec<Resource> = fs::read_dir("mods").unwrap()
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| is_zip_file(&dir_entry))
        .map(|zip_file| fs::canonicalize(zip_file.path()).unwrap())
        .map(|canonicalized_path| resource::read(canonicalized_path))
        .collect();

    // read desired mod list

    // build delta

    // download missing

    // delete obsolete (not longer wanted or 'Outdated' or 'Unsupported')
}

fn is_zip_file(dir_entry: &DirEntry) -> bool {
    let is_file = dir_entry.file_type().unwrap().is_file();
    let is_zip = dir_entry.file_name().to_str().unwrap().ends_with(".zip");
    is_file && is_zip
}
