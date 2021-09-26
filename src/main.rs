use std::{env, fs, io};
use std::fs::DirEntry;
use std::path::PathBuf;

use crate::resource::Resource;
use std::collections::HashMap;

mod resource;
mod online_resource;
mod delta_builder;

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
    let to_download = delta_builder::get_to_download(&local_mods, &online_mods_string);
    // find outdated
    let to_remove = delta_builder::get_to_remove();

    // download missing
    println!("To download:");
    let bla = to_download.iter()
        .inspect(|resource| println!(" - {}", resource))
        .count()
        ;

    // delete obsolete (not longer wanted or 'Outdated' or 'Unsupported')
}

fn is_zip_file(dir_entry: &DirEntry) -> bool {
    let is_file = dir_entry.file_type().unwrap().is_file();
    let is_zip = dir_entry.file_name().to_str().unwrap().ends_with(".zip");
    is_file && is_zip
}
