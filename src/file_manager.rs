use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use ureq::Response;

use crate::Resource;

/// Downloads a resource to the specified directory.
pub fn download(
    multiprogress_bar: &MultiProgress,
    pb_download: &ProgressBar,
    target_dir: &PathBuf,
    to_download: &Resource,
) {
    // TODO: Define proper user agent
    // Do a HEAD request to gain meta information about the file to download
    let head_response = ureq::head(&to_download.download_url)
        .call()
        .unwrap_or_else(|error| {
            panic!(
                "Failed to do HEAD request for: {} Error:\n{}",
                &to_download.download_url, error
            )
        });

    // Determines the absolute target file path to download the resource to.
    let target_file = build_target_filename(target_dir, &head_response);

    // Actually download the file
    let resolved_url = head_response.get_url();
    download_to_file(
        resolved_url,
        &target_file,
        multiprogress_bar,
        to_download.name.clone(),
    );

    pb_download.inc(1);
}

/// Determines the file name of the online resource http header response.
///
/// If the header does not contain the filename, parse it from the resolved url
fn build_target_filename(target_dir: &PathBuf, head_response: &Response) -> PathBuf {
    let contains = head_response
        .headers_names()
        .contains(&"content-disposition".to_string());

    let filename = if contains {
        get_filename_from_headers(head_response)
    } else {
        get_filename_from_url(head_response.get_url())
    };

    target_dir.join(&filename)
}

/// Parses the filename from the `content-disposition` header attribute value.
fn get_filename_from_headers(response: &Response) -> String {
    lazy_static! {
        static ref HEAD_PATTERN: Regex =
            Regex::new("attachment; filename=\"(?P<filename>.*)\"").unwrap();
    }
    let content_disposition = response
        .header("content-disposition")
        .expect("content-disposition not set");
    let caps = HEAD_PATTERN.captures(content_disposition).unwrap();
    caps.name("filename").unwrap().as_str().to_string()
}

/// Parses the filename out of the passed `url_string`.
fn get_filename_from_url(url_string: &str) -> String {
    lazy_static! {
        static ref URL_PATTERN: Regex = Regex::new(r"https://cdn\d*\.beamng\.com/mods/.*/\d*/(?P<filename>.*\.zip)\?md5=(?P<md5>.*)&expires=\d*").unwrap();
    }
    let caps = URL_PATTERN.captures(url_string).unwrap();
    caps.name("filename").unwrap().as_str().to_string()
}

/// Deletes the specified `to_delete` resource file located in the passed `target_dir`.
pub fn delete(target_dir: &PathBuf, to_delete: &Resource) {
    let resource_file_path = target_dir.join(&to_delete.filename);
    std::fs::remove_file(&resource_file_path).unwrap_or_else(|_| {
        panic!(
            "error deleting file {}",
            &resource_file_path.to_str().unwrap()
        )
    });
}

/// Downloads the specified url to the specified target_file and shows a progress bar
fn download_to_file(url: &str, target_file: &PathBuf, mp: &MultiProgress, visual_name: String) {
    let get_response = ureq::get(url).call().unwrap();
    let content_size: u64 = get_response
        .header("Content-Length")
        .unwrap()
        .parse()
        .unwrap();

    // Use a progress bar to ident download progress
    let dl_bar = mp.add(
        ProgressBar::new(content_size)
            .with_message(visual_name)
            .with_style(
            ProgressStyle::default_bar()
                .template(
                    "[{bar:.cyan/blue}] {bytes}/{total_bytes} @ {bytes_per_sec} {eta} {msg:.cyan}",
                )
                .unwrap()
                .progress_chars("##-"),
        ),
    );

    // Download the data chunk-wise to a byte vector
    let mut reader = get_response.into_reader();
    let buffer_size: usize = (content_size as usize) / 99;
    let mut total_buffer = Vec::new();
    loop {
        let mut buffer = vec![0; buffer_size];
        let buffer_size = reader.read(&mut buffer[..]).unwrap();
        buffer.truncate(buffer_size);
        if !buffer.is_empty() {
            total_buffer.extend(buffer.into_boxed_slice().into_vec().iter().cloned());
            dl_bar.inc(buffer_size as u64);
        } else {
            break;
        }
    }

    // Flush the collected data to a file
    File::create(target_file)
        .unwrap_or_else(|error| {
            panic!(
                "Failed to create file {}: {}",
                target_file.to_str().unwrap(),
                error
            )
        })
        .write_all(&total_buffer)
        .unwrap_or_else(|error| {
            panic!(
                "Failed to write file {}: {}",
                target_file.to_str().unwrap(),
                error
            )
        });

    // Change ownership of the file to 777
    let rwx_permission = std::fs::Permissions::from_mode(0o777);
    std::fs::set_permissions(target_file, rwx_permission).unwrap_or_else(|error| {
        panic!(
            "Failed to set permissions for file {}: {}",
            target_file.to_str().unwrap(),
            error
        )
    });

    dl_bar.finish_and_clear();
}
