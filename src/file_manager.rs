use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;

use crate::Resource;

/// Downloads a resource to the specified directory.
pub fn download(
    multiprogress_bar: &MultiProgress,
    pb_download: &ProgressBar,
    target_dir: &Path,
    resource_info: &Resource,
) -> Result<(), Box<dyn std::error::Error>> {
    let download_url = &resource_info.download_url;
    let get_response = ureq::get(download_url).call()?;
    let content_size: u64 = get_response
        .header("Content-Length")
        .ok_or("Missing Content-Length header")?
        .parse()?;

    // Determine the filename
    let filename = get_filename_from_url(get_response.get_url());
    let target_file = &target_dir.join(&filename);

    // Setup progress bar
    let visual_name = resource_info.name.clone();
    let dl_bar = multiprogress_bar.add(
        ProgressBar::new(content_size)
            .with_message(visual_name)
            .with_style(
            ProgressStyle::default_bar()
                .template(
                    "[{bar:.cyan/blue}] {bytes}/{total_bytes} @ {bytes_per_sec} {eta} {msg:.cyan}",
                )?
                .progress_chars("##-"),
        ),
    );

    // Download the data chunk-wise
    let mut reader = get_response.into_reader();
    let mut file = File::create(target_file)?;
    let mut buffer = vec![0; 8192]; // 8 KB buffer
    let mut total_downloaded = 0;

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        file.write_all(&buffer[..bytes_read])?;
        total_downloaded += bytes_read as u64;
        dl_bar.set_position(total_downloaded);
    }

    // Set secure file permissions
    let rw_permission = std::fs::Permissions::from_mode(0o644);
    std::fs::set_permissions(target_file, rw_permission)?;

    dl_bar.finish_and_clear();
    pb_download.inc(1);

    Ok(())
}

/// Parses the filename out of the passed `url_string`.
fn get_filename_from_url(url_string: &str) -> String {
    lazy_static! {
        static ref URL_PATTERN: Regex =
            Regex::new(r"(?:https://|http://).*?/mods/.*?/\d*/(?P<filename>.*?\.zip)(\?|$)")
                .unwrap();
    }
    let caps = URL_PATTERN.captures(url_string).unwrap();
    caps.name("filename").unwrap().as_str().to_string()
}

/// Deletes the specified `to_delete` resource file located in the passed `target_dir`.
pub fn delete(target_dir: &Path, to_delete: &Resource) {
    let resource_file_path = target_dir.join(&to_delete.filename);
    std::fs::remove_file(&resource_file_path).unwrap_or_else(|_| {
        panic!(
            "error deleting file {}",
            &resource_file_path.to_str().unwrap()
        )
    });
}
