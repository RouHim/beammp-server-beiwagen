use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

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

    lazy_static! {
        static ref URL_PATTERN: Regex = Regex::new(r"https://cdn\d*\.beamng\.com/mods/.*/\d*/(?P<filename>.*\.zip)\?md5=(?P<md5>.*)&expires=\d*").unwrap();
    }

    let url_string = head.effective_uri().unwrap().to_string();

    let caps = URL_PATTERN.captures(&url_string).unwrap();
    let filename = caps.name("filename").unwrap().as_str();

    let mut resource_file_path = PathBuf::from(&target_dir);
    resource_file_path.push(&filename);

    Request::get(&to_download.download_url)
        .redirect_policy(RedirectPolicy::Follow)
        .body(()).unwrap()
        .send().unwrap()
        .copy_to_file(resource_file_path);
}

pub fn delete(target_dir: &String, to_delete: &Resource) {
    // TODO
}