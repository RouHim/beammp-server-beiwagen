extern crate core;

use std::collections::HashMap;
use std::fs::DirEntry;
use std::{env, fmt, fs};

use indicatif::{
    MultiProgress, ParallelProgressIterator, ProgressBar, ProgressIterator, ProgressStyle,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

mod delta_builder;
mod local_resource;
mod online_resource;

#[cfg(test)]
mod delta_builder_test;
mod file_manager;

fn main() {
    let local_mods_path: String = env::var("BW_CLIENT_MODS_DIR").unwrap_or("mods".to_string());

    let local_mods = analyse_local_mods(&local_mods_path);

    let online_mods_string = fetch_online_information();

    download_mods(&local_mods_path, &local_mods, &online_mods_string);

    delete_obsolete(&local_mods_path, &local_mods, &online_mods_string);
}

/// Deletes no longer needed mods
fn delete_obsolete(
    local_mods_path: &String,
    local_mods: &HashMap<u64, Resource>,
    online_mods_string: &HashMap<u64, Resource>,
) {
    let pg_delete = ProgressBar::new_spinner().with_message("Deleting obsolete mods");

    delta_builder::get_to_remove(local_mods, online_mods_string)
        .iter()
        .progress_with(pg_delete)
        // .inspect(|resource| println!(" - {}", resource))
        .for_each(|resource| file_manager::delete(local_mods_path, resource));
}

/// Evaluates which mods needs to be downloaded or updated and downloads them
fn download_mods(
    local_mods_path: &String,
    local_mods: &HashMap<u64, Resource>,
    online_mods_string: &HashMap<u64, Resource>,
) {
    let to_download = delta_builder::get_to_download(local_mods, online_mods_string);

    let multi_progress_bar = MultiProgress::new();
    let pb_download = multi_progress_bar.add(
        ProgressBar::new(to_download.len() as u64)
            .with_style(
                ProgressStyle::default_bar()
                    .template("{msg} {pos}/{len}")
                    .unwrap(),
            )
            .with_message("Downloading missing or updated"),
    );

    to_download.par_iter().for_each(|resource| {
        file_manager::download(&multi_progress_bar, &pb_download, local_mods_path, resource)
    });
    pb_download.finish_and_clear();
}

/// Reads desired mod list from env ($BW_MODS) and looks-it-up on beamng.com/resources
fn fetch_online_information() -> HashMap<u64, Resource> {
    let pg_remote = ProgressBar::new_spinner().with_message("Fetching remote information");

    let wanted_mods: Vec<String> = env::var("BW_MODS")
        .expect("BW_MODS env var not found")
        .split(',')
        .map(|entry| entry.to_string())
        .collect();

    wanted_mods
        .par_iter()
        .progress_with(pg_remote)
        .filter_map(|mod_id| online_resource::read(mod_id))
        // .inspect(|resource| println!(" - {}", resource))
        .map(|entry| (entry.id, entry))
        .collect()
}

/// Reads all available mods from the local mods directory
fn analyse_local_mods(local_mods_path: &String) -> HashMap<u64, Resource> {
    let pg_local = ProgressBar::new_spinner().with_message("Analysing local mods");

    fs::read_dir(local_mods_path)
        .unwrap_or_else(|_| panic!("Failed to read local mods directory: {}", local_mods_path))
        .progress_with(pg_local)
        .map(|dir_entry| dir_entry.unwrap())
        .filter(is_zip_file)
        .map(|zip_file| fs::canonicalize(zip_file.path()).unwrap())
        .filter_map(local_resource::read)
        // .inspect(|resource| println!(" - {}", resource))
        .map(|entry| (entry.id, entry))
        .collect()
}

/// Checks if the passed entry is a zip file.
fn is_zip_file(dir_entry: &DirEntry) -> bool {
    let is_file = dir_entry.file_type().unwrap().is_file();
    let is_zip = dir_entry.file_name().to_str().unwrap().ends_with(".zip");
    is_file && is_zip
}

/// Represents a BeamNG mod resource with its metadata.
#[derive(Debug, Clone)]
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
        self.id.eq(&other.id)
    }
}

/// Implement the `Display` trait for `[Resource]` struct.
impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[id={}, tag_id={}, name={}, version={}, prefix={}, filename={}, download_url={}]",
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
