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
    assert_that!(to_download).contains_exactly(remote);
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
    assert_that!(to_download).contains_exactly(vec![generate_resource(2)])
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
    assert_that!(to_download).contains_exactly(vec![])
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
    assert_that!(to_download).contains_exactly(vec![])
}

#[test]
fn to_download_remote_newer() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource_with_version(1, 1)];
    let remote: Vec<Resource> = vec![generate_resource_with_version(1, 2)];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![generate_resource_with_version(1, 2)])
}

#[test]
fn to_download_local_newer() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource_with_version(1, 2)];
    let remote: Vec<Resource> = vec![generate_resource_with_version(1, 1)];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![]);
}

#[test]
fn to_download_version_equal() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource_with_version(1, 1)];
    let remote: Vec<Resource> = vec![generate_resource_with_version(1, 1)];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![]);
}

#[test]
fn to_download_skip_outdated_on_skip() {
    // GIVEN
    env::set_var("OUTDATED", "skip");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "OUTDATED")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![]);
}

#[test]
fn to_download_skip_outdated_on_delete() {
    // GIVEN
    env::set_var("OUTDATED", "delete");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "OUTDATED")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![]);
}

#[test]
fn to_download_download_regular_on_outdated_skip() {
    // GIVEN
    env::set_var("OUTDATED", "skip");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![generate_resource_with_prefix(1, "")]);
}

#[test]
fn to_download_download_regular_on_outdated_delete() {
    // GIVEN
    env::set_var("OUTDATED", "delete");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![generate_resource_with_prefix(1, "")]);
}

#[test]
fn to_download_skip_unsupported_on_skip() {
    // GIVEN
    env::set_var("UNSUPPORTED", "skip");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "UNSUPPORTED")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![]);
}

#[test]
fn to_download_download_regular_on_unsupported_skip() {
    // GIVEN
    env::set_var("UNSUPPORTED", "skip");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![generate_resource_with_prefix(1, "")]);
}

#[test]
fn to_download_download_regular_on_unsupported_delete() {
    // GIVEN
    env::set_var("UNSUPPORTED", "delete");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![generate_resource_with_prefix(1, "")]);
}

#[test]
fn to_download_skip_unsupported_on_delete() {
    // GIVEN
    env::set_var("UNSUPPORTED", "delete");
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "UNSUPPORTED")];

    // WHEN
    let to_download = delta_builder::get_to_download(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_download).contains_exactly(vec![]);
}

#[test]
fn to_remove_empty_remote() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource(1)];
    let remote: Vec<Resource> = vec![];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![generate_resource(1)])
}

#[test]
fn to_remove_empty_local() {
    // GIVEN
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![generate_resource(1)];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![])
}

#[test]
fn to_remove_different_ids() {
    // GIVEN
    let local: Vec<Resource> = vec![generate_resource(1), generate_resource(2)];
    let remote: Vec<Resource> = vec![generate_resource(3), generate_resource(4)];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![generate_resource(1), generate_resource(2)])
}

#[test]
fn to_remove_empty_local_remote() {
    // GIVEN
    let local: Vec<Resource> = vec![];
    let remote: Vec<Resource> = vec![];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![])
}

#[test]
fn to_remove_local_outdated_skip() {
    // GIVEN
    env::set_var("OUTDATED", "skip");
    let local: Vec<Resource> = vec![generate_resource(1)];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "OUTDATED")];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![])
}

#[test]
fn to_remove_local_outdated_delete() {
    // GIVEN
    env::set_var("OUTDATED", "delete");
    let local: Vec<Resource> = vec![generate_resource(1)];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "OUTDATED")];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![generate_resource(1)])
}

#[test]
fn to_remove_local_unsupported_skip() {
    // GIVEN
    env::set_var("UNSUPPORTED", "skip");
    let local: Vec<Resource> = vec![generate_resource(1)];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "UNSUPPORTED")];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![])
}

#[test]
fn to_remove_local_unsupported_delete() {
    // GIVEN
    env::set_var("UNSUPPORTED", "delete");
    let local: Vec<Resource> = vec![generate_resource(1)];
    let remote: Vec<Resource> = vec![generate_resource_with_prefix(1, "UNSUPPORTED")];

    // WHEN
    let to_remove = delta_builder::get_to_remove(
        &to_map(&local),
        &to_map(&remote),
    );

    // THEN
    assert_that!(to_remove).contains_exactly(vec![generate_resource(1)])
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

fn generate_resource_with_prefix(id: u64, prefix: &str) -> Resource {
    Resource {
        id,
        tag_id: id.to_string(),
        name: id.to_string(),
        version: 0,
        prefix: prefix.to_string(),
        filename: format!("{}.zip", id),
        download_url: "".to_string(),
    }
}

fn generate_resource_with_version(id: u64, version: u64) -> Resource {
    Resource {
        id,
        tag_id: id.to_string(),
        name: id.to_string(),
        version,
        prefix: "".to_string(),
        filename: format!("{}.zip", id),
        download_url: "".to_string(),
    }
}