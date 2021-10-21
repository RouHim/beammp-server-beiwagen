use std::path::PathBuf;
use std::time::Duration;

use backoff::ExponentialBackoff;
use isahc::{Body, ReadResponseExt, Request, RequestExt, Response, ResponseExt};
use isahc::config::{Configurable, RedirectPolicy};
use isahc::http::HeaderMap;
use lazy_static::lazy_static;
use regex::Regex;

use crate::Resource;

/// Downloads a resource to the specified directory.
pub fn download(target_dir: &String, to_download: &Resource) {
    // Do a HEAD request to gain meta information about the file to download
    let head = Request::head(&to_download.download_url)
        .redirect_policy(RedirectPolicy::Follow)
        .body(()).unwrap()
        .send().unwrap();

    // Determines the absolute file path to download the resource to.
    let resource_file_path = get_absolute_filename(target_dir, &head);

    // Actually download the file
    // TODO: this fails sometimes, add a retry mechanism: https://docs.rs/retry or https://docs.rs/backoff
    let response = fetch_url(&to_download.download_url);
    response.unwrap()
        .copy_to_file(resource_file_path)
        .expect(format!("error downloading file {}", &to_download.download_url).as_str());
}

/// Determines the file name of the online resource http header response.
///
/// If the header does not contain the filename, parse it from the resolved url
fn get_absolute_filename(target_dir: &String, head: &Response<Body>) -> PathBuf {
    let filename = if head.headers().contains_key("content-disposition") {
        get_filename_from_headers(head.headers())
    } else {
        get_filename_from_url(&head.effective_uri().unwrap().to_string())
    };

    let mut resource_file_path = PathBuf::from(&target_dir);
    resource_file_path.push(&filename);
    resource_file_path
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
fn get_filename_from_url(url_string: &String) -> String {
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

fn fetch_url(url: &str) -> Result<isahc::Response<Body>, backoff::Error<isahc::Error>> {
    let fetch_operation = || {
        println!("Fetching {}", url);
        Request::get(url)
            .timeout(Duration::from_secs(30))
            .redirect_policy(RedirectPolicy::Follow)
            .body(()).unwrap()
            .send()
            .map_err(|err| backoff::Error::from(err))
    };

    backoff::retry(ExponentialBackoff::default(), fetch_operation)
}