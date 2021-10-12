use std::{env, fs};
use std::collections::HashMap;
use std::fs::DirEntry;

use crate::resource::Resource;

mod resource;
mod online_resource;
mod delta_builder;

#[cfg(test)]
mod delta_builder_test;
mod file_manager;

fn main() {
    let client_mods_path: String = env::var("BEAMMP_CLIENT_MODS_DIR")
        .unwrap_or("mods".to_string());

    // read all local available mods
    println!("Local mods:");
    let local_mods: HashMap<u64, Resource> = fs::read_dir(&client_mods_path).unwrap()
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| is_zip_file(&dir_entry))
        .map(|zip_file| fs::canonicalize(zip_file.path()).unwrap())
        .filter_map(|absolute_path| resource::read(absolute_path))
        .inspect(|resource| println!(" - {}", resource))
        .map(|entry| (entry.id, entry))
        .collect();

    // read desired mod list and look it up on beamng.com/resources
    println!("Mods wanted:");
    let online_mods_string: HashMap<u64, Resource> = env::var("BEAMMP_MODS")
        .expect("no BEAMMP_MODS env found")
        .split(",")
        .filter_map(|mod_id| online_resource::read(mod_id))
        .inspect(|resource| println!(" - {}", resource))
        .map(|entry| (entry.id, entry))
        .collect();

    // find updated or new mods
    println!("Downloading missing or updated mods:");
    delta_builder::get_to_download(&local_mods, &online_mods_string)
        .iter()
        .inspect(|resource| println!(" - {}", &resource))
        .for_each(|resource| file_manager::download(&client_mods_path, resource));

    // delete no longer needed mods
    println!("Deleting no longer needed mods:");
    delta_builder::get_to_remove(&local_mods, &online_mods_string)
        .iter()
        .inspect(|resource| println!(" - {}", resource))
        .for_each(|resource| file_manager::delete(&client_mods_path, resource));
}

fn is_zip_file(dir_entry: &DirEntry) -> bool {
    let is_file = dir_entry.file_type().unwrap().is_file();
    let is_zip = dir_entry.file_name().to_str().unwrap().ends_with(".zip");
    is_file && is_zip
}
