use std::borrow::BorrowMut;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use regex::Regex;
use serde_json::Value;
use zip::ZipArchive;

pub struct Resource {
    id: String,
    name: String,
    url: String,
    version: i32,
    latest_version: i32,
    prefix: String,
}

pub fn read(mod_file: PathBuf) -> Resource {
    let json_data = read_mod_info(mod_file)
        .expect("could not read info.json from zip file");

    let v: Value = serde_json::from_str(&json_data).unwrap();

    println!("{}", v["resource_id"]);

    return Resource {
        id: "".to_string(),
        name: "".to_string(),
        url: "".to_string(),
        version: 0,
        latest_version: 0,
        prefix: "".to_string(),
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