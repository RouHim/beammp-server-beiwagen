use std::borrow::BorrowMut;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use regex::Regex;
use serde_json::Value;
use zip::ZipArchive;
use core::fmt;

pub struct Resource {
    id: String,
    tag_id: String,
    name: String,
    version: i32,
    prefix: String,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[id={}, tag_id={}, name={}, version={}, prefix={}]",
               self.id,
               self.tag_id,
               self.name,
               self.version,
               self.prefix,
        )
    }
}

pub fn read(mod_file: PathBuf) -> Resource {
    let json_string = read_mod_info(mod_file)
        .expect("could not read info.json from zip file");

    let info_json: Value = serde_json::from_str(&json_string).unwrap();

    return Resource {
        id: info_json["resource_id"].to_string(),
        tag_id: info_json["tagid"].to_string(),
        name: info_json["title"].to_string(),
        version: info_json["current_version_id"].to_string().parse().unwrap(),
        prefix: info_json["prefix_title"].to_string(),
    };
}

fn read_mod_info(mod_file: PathBuf) -> Result<String, String> {
    let zip_file_path = mod_file.to_str().unwrap();
    let file = File::open(zip_file_path).unwrap();
    let mut archive = zip::ZipArchive::new(BufReader::new(&file)).unwrap();

    let info_json_full_path = find_file_path(&mut archive, r"mod_info/.*/info.json")
        .expect("no info.json found in zip file");

    read_content(&mut archive, info_json_full_path)
}

fn read_content(archive: &mut ZipArchive<BufReader<&File>>, info_json_full_path: String) -> Result<String, String> {
    let mut info_json_compressed = archive
        .by_name(info_json_full_path.as_str())
        .expect("filepath not found in zip");

    let mut file_content = String::new();
    info_json_compressed.read_to_string(&mut file_content);

    return Ok(file_content);
}

fn find_file_path(archive: &mut ZipArchive<BufReader<&File>>, file_to_read: &str) -> Result<String, ()> {
    let info_json_pattern = Regex::new(file_to_read).unwrap();

    for idx in 0..archive.len() {
        let entry = archive.by_index(idx).unwrap();
        let name = entry.enclosed_name();
        let full_name = name.unwrap().to_str().unwrap();

        if info_json_pattern.is_match(full_name) {
            return Ok(full_name.to_string());
        }
    };

    Err(())
}