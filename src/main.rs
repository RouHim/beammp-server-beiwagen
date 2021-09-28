use std::{env, fs, io};
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::resource::Resource;

mod resource;
mod online_resource;
mod delta_builder;

#[cfg(test)]
mod delta_builder_test;

fn main() {
    // read local available mods
    println!("The following mods are local available:");
    let local_mods: HashMap<u64, Resource> = fs::read_dir("mods").unwrap()
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| is_zip_file(&dir_entry))
        .map(|zip_file| fs::canonicalize(zip_file.path()).unwrap())
        .map(|absolute_path| resource::read(absolute_path))
        .filter(|resource| resource.is_some())
        .map(|resource| resource.unwrap())
        .inspect(|resource| println!(" - {}", resource))
        .map(|entry| (entry.id, entry))
        .collect();

    // read desired mod list and look it up online
    println!("The following mods are online available:");
    let online_mods_string: HashMap<u64, Resource> = env::var("BEAMMP_MODS")
        .expect("no BEAMMP_MODS env found")
        .split(",")
        .map(|absolute_path| online_resource::read(absolute_path))
        .filter(|resource| resource.is_some())
        .map(|resource| resource.unwrap())
        .inspect(|resource| println!(" - {}", resource))
        .map(|entry| (entry.id, entry))
        .collect();


    // find updated or new mods
    println!("To download:");
    let to_download: Vec<&Resource> = delta_builder::get_to_download(&local_mods, &online_mods_string)
        .iter()
        .inspect(|resource| println!(" - {}", resource))
        .collect()
        ;

    // find updated or new mods
    println!("To delete:");
    // download missing
    let to_delete: Vec<&Resource> = delta_builder::get_to_remove(&local_mods, &online_mods_string)
        .iter()
        .inspect(|resource| println!(" - {}", resource))
        .collect()
        ;

    // delete obsolete (not longer wanted or 'Outdated' or 'Unsupported')
}

fn is_zip_file(dir_entry: &DirEntry) -> bool {
    let is_file = dir_entry.file_type().unwrap().is_file();
    let is_zip = dir_entry.file_name().to_str().unwrap().ends_with(".zip");
    is_file && is_zip
}
