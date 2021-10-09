use core::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use colour::red_ln;
use regex::Regex;
use serde_json::Value;
use zip::ZipArchive;

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

impl PartialEq<Self> for Resource {
    fn eq(&self, other: &Self) -> bool {
        return self.id.eq(&other.id);
    }
}

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

pub fn read(mod_file: PathBuf) -> Option<Resource> {
    let json_string = read_mod_info(&mod_file);

    if json_string.is_err() {
        red_ln!(
            " - {} | {} | no auto-updates available",
            mod_file.file_name().unwrap().to_str().unwrap(),
            json_string.unwrap_err(),
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

fn read_mod_info(mod_file: &PathBuf) -> Result<String, String> {
    let zip_file_path = mod_file.to_str().unwrap();
    let file = File::open(zip_file_path).unwrap();
    let mut archive = zip::ZipArchive::new(BufReader::new(&file)).unwrap();

    let info_json_full_path = find_file_path(&mut archive, r"mod_info/.*/info.json");

    match info_json_full_path {
        Ok(info_json) => read_content(&mut archive, info_json),
        Err(()) => Err("info.json not found".to_string())
    }
}

fn read_content(archive: &mut ZipArchive<BufReader<&File>>, info_json_full_path: String) -> Result<String, String> {
    let mut info_json_compressed = archive
        .by_name(info_json_full_path.as_str())
        .expect("filepath not found in zip");

    let mut file_content = String::new();
    info_json_compressed.read_to_string(&mut file_content).expect("Read zip content");

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