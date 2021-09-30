use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use regex::Regex;
use crate::hasher;
use crate::resource::Resource;

pub fn download(target_dir: &String, to_download: &Resource) {
    let client = reqwest::blocking::Client::new();
    let head = client.head(&to_download.download_url).send().unwrap();

    let url_pattern = Regex::new(r"https://cdn\d*\.beamng\.com/mods/.*/\d*/(?P<filename>.*\.zip)\?md5=(?P<md5>.*)&expires=\d*").unwrap();
    let url_string = head.url().to_string();

    let caps = url_pattern.captures(&url_string).unwrap();
    let filename = caps.name("filename").unwrap().as_str();
    let md5 = caps.name("md5").unwrap().as_str();

    println!("{} {}", filename, md5);

    let bytes = reqwest::blocking::get(&to_download.download_url).unwrap()
        .bytes().unwrap()
        .to_vec();

    let memory_sha256 = hasher::sha256(&bytes);

    if memory_sha256.eq(md5) {
        println!("md5 hash did not match {}", &to_download);
    }

    let mut mods_dir = PathBuf::from(&target_dir);
    mods_dir.push(&filename);
    let mut resource_file = File::create(&mods_dir).unwrap();

    // write bytes to local file
    resource_file.write_all(&bytes).unwrap();
}

pub fn delete(target_dir: &String, to_delete: &Resource) {

}