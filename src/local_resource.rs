use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use colour::red_ln;
use regex::Regex;
use serde_json::Value;
use zip::ZipArchive;

use crate::Resource;

/// Retrieves all meta information of an local mod resource by the passed `mod_file`.
pub fn read(mod_file: PathBuf) -> Option<Resource> {
    let json_string = read_mod_info(&mod_file);

    if json_string.is_err() {
        red_ln!(
            " - {} | {} | no auto-updates available",
            mod_file.file_name().unwrap().to_str().unwrap(),
            json_string.unwrap_err()
        );
        return None;
    }

    let info_json: Value = serde_json::from_str(&json_string.unwrap()).unwrap();

    return Some(Resource {
        id: info_json["resource_id"].as_u64().unwrap(),
        tag_id: info_json["tagid"].as_str().unwrap().to_string(),
        name: info_json["title"].as_str().unwrap().to_string(),
        version: info_json["current_version_id"].as_u64().unwrap(),
        prefix: info_json["prefix_title"].as_str().unwrap().to_string(),
        filename: info_json["filename"].as_str().unwrap().to_string(),
        download_url: "".to_string(),
    });
}

/// Extracts all mod metadata out of the local `mod_file` zip.
fn read_mod_info(mod_file: &Path) -> Result<String, String> {
    let zip_file_path = mod_file.to_str().unwrap();
    let file = File::open(zip_file_path).unwrap_or_else(|_| {
        panic!("Could not open file: {}", zip_file_path);
    });
    let maybe_archive = ZipArchive::new(BufReader::new(&file));

    if maybe_archive.is_err() {
        std::fs::remove_file(zip_file_path).expect("could not delete");
        return Err("Invalid archive -> deleting it".to_string());
    };
    let mut archive = maybe_archive.unwrap_or_else(|_| {
        panic!("Could not open zip archive: {}", zip_file_path);
    });

    let info_json_full_path = find_file_path(&mut archive, r"mod_info/.*/info.json");

    match info_json_full_path {
        Ok(info_json) => read_content(&mut archive, info_json),
        Err(()) => Err("info.json not found".to_string()),
    }
}

/// Reads the content of the `filename_to_read` located in the zip `archive`.
fn read_content(
    archive: &mut ZipArchive<BufReader<&File>>,
    filename_to_read: String,
) -> Result<String, String> {
    let mut info_json_compressed = archive
        .by_name(filename_to_read.as_str())
        .expect("filepath not found in zip");

    let mut file_content = String::new();
    info_json_compressed
        .read_to_string(&mut file_content)
        .expect("Read zip content");

    Ok(file_content)
}

/// Finds the first file in the specified zip `archive` that matches `file_to_read_regex` pattern.
fn find_file_path(
    archive: &mut ZipArchive<BufReader<&File>>,
    file_to_read_regex: &str,
) -> Result<String, ()> {
    let info_json_pattern = Regex::new(file_to_read_regex).unwrap();

    for idx in 0..archive.len() {
        let entry = archive.by_index(idx).unwrap();
        let name = entry.enclosed_name();
        let full_name = name.unwrap().to_str().unwrap();

        if info_json_pattern.is_match(full_name) {
            return Ok(full_name.to_string());
        }
    }

    Err(())
}
