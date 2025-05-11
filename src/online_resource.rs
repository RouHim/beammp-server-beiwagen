use colour::red_ln;
use scraper::{Html, Selector};

use crate::Resource;

/// Retrieves all meta information of an online available mod resource by the passed `mod_id`.
pub fn read(mod_id: &str) -> Option<Resource> {
    let mod_url = format!("https://www.beamng.com/resources/{}", mod_id);

    let mut response = match ureq::get(&mod_url).call() {
        Ok(response) => response,
        Err(error) => {
            red_ln!("Could not find Mod {}: {}", mod_id, error.to_string());
            return None;
        }
    };

    let response_html = response.body_mut().read_to_string().unwrap();
    let document = Html::parse_document(response_html.as_str());

    let id: u64 = mod_id.parse().unwrap();
    let tag_id = get_tag_id(&document);
    let name = get_name(&document);
    let version = get_version(&document);
    let download_url = get_download_url(&id, &version);
    let prefix = get_prefix(&document);
    let filename = "".to_string();

    Some(Resource {
        id,
        tag_id,
        name,
        version,
        prefix,
        filename,
        download_url,
    })
}

/// Returns the mod prefix of the html document
fn get_prefix(html: &Html) -> String {
    let selector = Selector::parse("h1 > span.prefix").unwrap();
    let selection = html.select(&selector).next();

    if selection.is_none() {
        return "".to_string();
    }

    selection.unwrap().inner_html()
}

/// Parses the mod `version` out of the html document.
fn get_version(html: &Html) -> u64 {
    let selector = Selector::parse("label.downloadButton > a").unwrap();

    let selection = html
        .select(&selector)
        .find(|entry| entry.inner_html().contains(".zip"));

    let download_url = selection.unwrap().value().attr("href").unwrap().to_string();

    download_url
        .split_once("version=")
        .unwrap()
        .1
        .parse()
        .unwrap()
}

/// Builds the download url based on `id` and `version` of the mod.
fn get_download_url(id: &u64, version: &u64) -> String {
    format!(
        "https://www.beamng.com/resources/{}/download?version={}",
        id, version
    )
}

/// Parses the mod `name` out of the passed html document.
fn get_name(html: &Html) -> String {
    let selector = Selector::parse("head > title").unwrap();
    let selection = html.select(&selector).next();
    let title = selection.unwrap().inner_html();

    let without_pipe = title.split_once('|').unwrap().0.trim().to_string();

    let split_by_minus = without_pipe.split_once('-');
    if split_by_minus.is_none() {
        return without_pipe;
    };

    split_by_minus.unwrap().1.to_string().trim().to_string()
}

/// Parses the mod `tag_id` out of the html document.
fn get_tag_id(html: &Html) -> String {
    let selector = Selector::parse("div#resourceInfo dl").unwrap();
    let selection = html.select(&selector);
    let tag_id_html_row = selection
        .into_iter()
        .find(|entry| entry.inner_html().contains("Unique ID"))
        .unwrap();

    let first = tag_id_html_row
        .select(&Selector::parse("dd").unwrap())
        .next()
        .unwrap()
        .inner_html();
    first
}
