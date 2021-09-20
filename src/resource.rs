use std::path::PathBuf;
use std::io::{BufReader, Read};
use std::fs::File;
use std::borrow::BorrowMut;
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
    let file_path = mod_file.to_str().unwrap();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut archive = zip::ZipArchive::new(reader).unwrap();

    let mod_info = archive.by_name("mod_info").unwrap();
    mod_info.

    return Resource {
        id: "".to_string(),
        name: "".to_string(),
        url: "".to_string(),
        version: 0,
        latest_version: 0,
        prefix: "".to_string(),
    };
}