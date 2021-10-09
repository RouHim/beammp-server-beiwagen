use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;

use isahc::{ReadResponseExt, Request, RequestExt, ResponseExt};
use isahc::config::{Configurable, RedirectPolicy};
use lazy_static::lazy_static;
use regex::Regex;

use crate::resource::Resource;

pub fn download(target_dir: &String, to_download: &Resource) {
    let mut head = Request::head(&to_download.download_url)
        .redirect_policy(RedirectPolicy::Follow)
        .body(()).unwrap()
        .send().unwrap();

    // TODO: sometimes the url does not resolve itself,
    // instead a header property is returned where we can find the filename: 'attachment; filename="autodrom_day.zip"'
    let resolved_url = head.effective_uri().unwrap().to_string();
    println!("url: {}", &resolved_url);

    let filename = parse_filename(&resolved_url);
    println!("filename: {}",  &filename);

    let mut resource_file_path = PathBuf::from(&target_dir);
    resource_file_path.push(&filename);

    // TODO: runs on timeout sometimes (50/50)
    Request::get(&to_download.download_url)
        .timeout(Duration::from_secs(60))
        .redirect_policy(RedirectPolicy::Limit(1))
        .body(()).unwrap()
        .send().unwrap()
        .copy_to_file(resource_file_path);
}

fn parse_filename(url_string: &String) -> String {
    lazy_static! {
        static ref URL_PATTERN: Regex = Regex::new(r"https://cdn\d*\.beamng\.com/mods/.*/\d*/(?P<filename>.*\.zip)\?md5=(?P<md5>.*)&expires=\d*").unwrap();
    }
    let caps = URL_PATTERN.captures(&url_string).unwrap();
    caps.name("filename").unwrap().as_str().to_string()
}

pub fn delete(target_dir: &String, to_delete: &Resource) {
    let mut resource_file_path = PathBuf::from(&target_dir);
    resource_file_path.push(&to_delete.filename);
    std::fs::remove_file(resource_file_path);
}