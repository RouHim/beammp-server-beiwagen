use std::path::PathBuf;
use std::time::Duration;

use isahc::{ReadResponseExt, Request, RequestExt, ResponseExt};
use isahc::config::{Configurable, RedirectPolicy};
use isahc::http::HeaderMap;
use lazy_static::lazy_static;
use regex::Regex;

use crate::Resource;

/// Downloads a resource to the specified directory.
pub fn download(target_dir: &String, to_download: &Resource) {
    let head = Request::head(&to_download.download_url)
        .redirect_policy(RedirectPolicy::Follow)
        .body(()).unwrap()
        .send().unwrap();

    // If the header does not contain the filename, parse it from the resolved url
    let filename = if head.headers().contains_key("content-disposition") {
        get_filename_from_headers(head.headers())
    } else {
        get_filename_frim_url(&head.effective_uri().unwrap().to_string())
    };

    let mut resource_file_path = PathBuf::from(&target_dir);
    resource_file_path.push(&filename);

    Request::get(&to_download.download_url)
        .timeout(Duration::from_secs(60))
        .redirect_policy(RedirectPolicy::Limit(1))
        .body(()).unwrap()
        .send().unwrap()
        .copy_to_file(resource_file_path)
        .expect(format!("error downloading file {}", &to_download.download_url).as_str());
}

/// Parses the filename from the `content-disposition` header attribute value.
fn get_filename_from_headers(headers: &HeaderMap) -> String {
    lazy_static! {
        static ref HEAD_PATTERN: Regex = Regex::new("attachment; filename=\"(?P<filename>.*)\"").unwrap();

    }
    let content_disposition = headers.get("content-disposition")
        .expect("content-disposition not set")
        .to_str().unwrap().to_string();
    let caps = HEAD_PATTERN.captures(&content_disposition).unwrap();
    caps.name("filename").unwrap()
        .as_str().to_string()
}

/// Parses the filename out of the passed `url_string`.
fn get_filename_frim_url(url_string: &String) -> String {
    lazy_static! {
        static ref URL_PATTERN: Regex = Regex::new(r"https://cdn\d*\.beamng\.com/mods/.*/\d*/(?P<filename>.*\.zip)\?md5=(?P<md5>.*)&expires=\d*").unwrap();
    }
    let caps = URL_PATTERN.captures(&url_string).unwrap();
    caps.name("filename").unwrap().as_str().to_string()
}

/// Deletes the specified `to_delete` resource file located in the the passed `target_dir`.
pub fn delete(target_dir: &String, to_delete: &Resource) {
    let mut resource_file_path = PathBuf::from(&target_dir);
    resource_file_path.push(&to_delete.filename);
    std::fs::remove_file(&resource_file_path)
        .expect(format!("error deleting file {}", &resource_file_path.to_str().unwrap()).as_str());
}