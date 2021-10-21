use std::{env, fmt, fs};
use std::collections::HashMap;
use std::fs::DirEntry;

mod local_resource;
mod online_resource;
mod delta_builder;

#[cfg(test)]
mod delta_builder_test;
mod file_manager;

fn main() {
    // read all local available mods from the BEAMMP_CLIENT_MODS_DIR directory
    println!("Local mods:");
    let client_mods_path: String = env::var("BEAMMP_CLIENT_MODS_DIR")
        .unwrap_or("mods".to_string());
    let local_mods: HashMap<u64, Resource> = fs::read_dir(&client_mods_path).unwrap()
        .map(|dir_entry| dir_entry.unwrap())
        .filter(|dir_entry| is_zip_file(&dir_entry))
        .map(|zip_file| fs::canonicalize(zip_file.path()).unwrap())
        .filter_map(|absolute_path| local_resource::read(absolute_path))
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
    // TODO: parallelize downloads to improve update speed
    // TODO: pretty print with a progress bar: https://docs.rs/indicatif
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

/// Checks if the passed entry is a zip file.
fn is_zip_file(dir_entry: &DirEntry) -> bool {
    let is_file = dir_entry.file_type().unwrap().is_file();
    let is_zip = dir_entry.file_name().to_str().unwrap().ends_with(".zip");
    is_file && is_zip
}

/// Represents a BeamNG mod resource with its metadata.
#[derive(Debug, Hash, Clone)]
pub struct Resource {
    pub id: u64,
    pub tag_id: String,
    pub name: String,
    pub version: u64,
    pub prefix: String,
    pub filename: String,
    pub download_url: String,
}

/// Implement the `PartialEq` trait for `[Resource]` struct.
impl PartialEq<Self> for Resource {
    fn eq(&self, other: &Self) -> bool {
        return self.id.eq(&other.id);
    }
}

/// Implement the `Display` trait for `[Resource]` struct.
impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[id={}, tag_id={}, name={}, version={}, prefix={}, filename={}, download_url={}]",
               self.id,
               self.tag_id,
               self.name,
               self.version,
               self.prefix,
               self.filename,
               self.download_url,
        )
    }
}
