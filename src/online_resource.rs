use core::fmt;
use std::borrow::BorrowMut;
use std::fmt::Error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use colour::red_ln;
use regex::Regex;
use scraper::{Html, Selector};
use serde_json::Value;
use zip::ZipArchive;

use crate::resource::Resource;

pub fn read(mod_id: &str) -> Option<Resource> {
    let mod_url = format!("https://www.beamng.com/resources/{}", mod_id);
    let response = reqwest::blocking::get(mod_url).unwrap();
    let response_html = response.text().unwrap();
    let document = Html::parse_document(response_html.as_str());

    let id: u64 = mod_id.parse().unwrap();
    let tag_id = select_tag_id(&document);
    let name = select_name(&document);
    let version = parse_version(&document);
    let download_url = build_download_url(&id, &version);
    let prefix = select_prefix(&document);
    let filename = "".to_string();

    return Some(Resource {
        id,
        tag_id,
        name,
        version,
        prefix,
        filename,
        download_url,
    });
}

fn select_prefix(html: &Html) -> String {
    let selector = Selector::parse("h1 > span.prefix").unwrap();
    let selection = html.select(&selector)
        .nth(0);

    if selection.is_none() {
        return "".to_string();
    }

    return selection.unwrap()
        .inner_html();
}

fn parse_version(html: &Html) -> u64 {
    let selector = Selector::parse("label.downloadButton > a").unwrap();

    let selection = html.select(&selector)
        .into_iter()
        .filter(|entry| entry.inner_html().contains(".zip"))
        .nth(0);

    let download_url = selection.unwrap().value()
        .attr("href").unwrap()
        .to_string();

    download_url.split_once("version=")
        .unwrap().1
        .parse().unwrap()
}

fn build_download_url(id: &u64, version: &u64) -> String {
    format!(
        "https://www.beamng.com/resources/{}/download?version={}",
        id,
        version
    )
}

fn select_name(html: &Html) -> String {
    let selector = Selector::parse("head > title").unwrap();
    let selection = html.select(&selector).nth(0);
    let title = selection.unwrap().inner_html();

    let without_pipe = title.split_once("|").unwrap()
        .0.trim().to_string();

    let split_by_minus = without_pipe.split_once("-");
    if split_by_minus.is_none() {
        return without_pipe;
    };

    split_by_minus.unwrap()
        .1.to_string()
        .trim().to_string()
}

fn select_tag_id(html: &Html) -> String {
    let selector = Selector::parse("div#resourceInfo dl").unwrap();
    let selection = html.select(&selector);
    let tag_id_html_row = selection.into_iter()
        .filter(|entry| entry.inner_html().contains("Unique ID"))
        .nth(0).unwrap();

    let first = tag_id_html_row
        .select(&Selector::parse("dd").unwrap())
        .nth(0).unwrap()
        .inner_html();
    first
}