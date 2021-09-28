use std::collections::HashMap;
use std::env;

use assertor::*;

use crate::delta_builder;
use crate::resource::Resource;

#[test]
fn to_download_all() {
    // GIVEN
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).is_equal_to(remote);
}

#[test]
fn to_download_one() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource(1)];
    let remote: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).is_equal_to(vec![generate_resource(2)])
}

#[test]
fn to_download_empty() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];
    let remote: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).is_equal_to(vec![])
}

#[test]
fn to_download_no_remote() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];
    let remote: Vec<Resource> = vec![];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).is_equal_to(vec![])
}

#[test]
fn to_remove_empty() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];
    let remote: Vec<Resource> = vec![generate_resource(1)];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).is_equal_to(vec![generate_resource(2)])
}

fn to_map(input_vec: &Vec<Resource>) -> HashMap<u64, Resource> {
    let mut map = HashMap::new();
    for element in input_vec {
        map.insert(element.id, element.clone());
    }
    return map;
}

fn generate_resource(id: u64) -> Resource {
    Resource {
        id,
        tag_id: id.to_string(),
        name: id.to_string(),
        version: 0,
        prefix: "".to_string(),
        filename: format!("{}.zip", id),
        download_url: "".to_string(),
    }
}

fn generate_resource_with_tag(id: u64, tag: &str) -> Resource {
    Resource {
        id,
        tag_id: id.to_string(),
        name: id.to_string(),
        version: 0,
        prefix: tag.to_string(),
        filename: format!("{}.zip", id),
        download_url: "".to_string(),
    }
}