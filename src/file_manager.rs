use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Response;
use reqwest::header::HeaderMap;

use crate::Resource;

/// Downloads a resource to the specified directory.
pub fn download(mp: &MultiProgress, pb_download: &ProgressBar, target_dir: &String, to_download: &Resource) {
    // Do a HEAD request to gain meta information about the file to download
    let client = reqwest::blocking::Client::new();
    let head_response = client.head(&to_download.download_url)
        .send().unwrap();

    // Determines the absolute file path to download the resource to.
    let resource_file_path = get_absolute_filename(target_dir, &head_response);

    // Actually download the file
    download_to_file(
        head_response.url().to_string(),
        &resource_file_path,
        mp,
        to_download.name.clone(),
    );

    pb_download.inc(1);
}

/// Determines the file name of the online resource http header response.
///
/// If the header does not contain the filename, parse it from the resolved url
fn get_absolute_filename(target_dir: &String, head_response: &Response) -> PathBuf {
    let filename = if head_response.headers().contains_key("content-disposition") {
        get_filename_from_headers(head_response.headers())
    } else {
        get_filename_from_url(&head_response.url().to_string())
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

/// Downloads the specified url to the specified target_file and shows a progress bar
fn download_to_file(url: String, target_file: &PathBuf, mp: &MultiProgress, visual_name: String) {
    let mut get_response = reqwest::blocking::get(&url).unwrap();
    let content_size = get_response.content_length().unwrap();

    // Use a progress bar to ident download progress
    let dl_bar = mp.add(
        ProgressBar::new(content_size)
            .with_message(visual_name)
            .with_style(ProgressStyle::default_bar()
                .template("[{bar:.cyan/blue}] {bytes}/{total_bytes} @ {bytes_per_sec} {eta} {msg:.cyan}")
                .progress_chars("##-"))
    );

    // Download the data chunk-wise to a byte vector
    let buffer_size: usize = (content_size as usize) / 99;
    let mut total_buffer = Vec::new();
    loop {
        let mut buffer = vec![0; buffer_size];
        let buffer_size = get_response.read(&mut buffer[..]).unwrap();
        buffer.truncate(buffer_size);
        if !buffer.is_empty() {
            total_buffer.extend(buffer.into_boxed_slice()
                .into_vec().iter()
                .cloned());
            dl_bar.inc(buffer_size as u64);
        } else {
            break;
        }
    }

    // Flush the collected data to a file
    File::create(&target_file).unwrap()
        .write_all(&total_buffer).unwrap();

    dl_bar.finish_and_clear();
}