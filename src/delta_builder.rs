use std::collections::HashMap;
use std::env;

use crate::Resource;

/// Builds a delta list of mods to download, based on the local available and remote available mods.
///
/// `local_list` contains local available mods
///
/// `remote_list` contains wanted remote online available mods
///
/// `returns` a vector of mods that needs to be downloaded
pub fn get_to_download(local_list: &HashMap<u64, Resource>, remote_list: &HashMap<u64, Resource>) -> Vec<Resource> {
    let new_entries: Vec<Resource> = remote_list
        .iter()
        .filter(|(key, _val)| !local_list.contains_key(key))
        .map(|(_key, val)| val.clone())
        .filter(|entry| !should_skip_unsupported(entry))
        .filter(|entry| !should_skip_outdated(entry))
        .collect();

    let mut updated_entries: Vec<Resource> = remote_list
        .iter()
        .filter(|(key, _val)| local_list.contains_key(key))
        .filter(|(key, val)| local_list.get(key).unwrap().version < val.version)
        .map(|(_key, val)| val.clone())
        .filter(|entry| !should_skip_unsupported(entry))
        .filter(|entry| !should_skip_outdated(entry))
        .collect();

    let mut to_download = new_entries;
    to_download.append(&mut updated_entries);
    to_download
}

/// Builds a list of mods that should be deleted, based on the local available and remote available mods.
///
/// `local_list` contains local available mods
///
/// `remote_list` contains wanted remote online available mods
///
/// `returns` a vector of mods that needs to be deleted
pub fn get_to_remove(local_list: &HashMap<u64, Resource>, remote_list: &HashMap<u64, Resource>) -> Vec<Resource> {
    let deleted_entries: Vec<Resource> = local_list
        .iter()
        .filter(|(key, _val)| !remote_list.contains_key(key))
        .map(|(_key, val)| val.clone())
        .collect();

    let mut outdated_entries: Vec<Resource> = remote_list
        .iter()
        .filter(|(key, _val)| local_list.contains_key(key))
        .filter(|(_key, val)| should_delete_outdated(val))
        .map(|(key, _val)| local_list.get(key).unwrap().clone())
        .collect();

    let mut unsupported_entries: Vec<Resource> = remote_list
        .iter()
        .filter(|(key, _val)| local_list.contains_key(key))
        .filter(|(_key, val)| should_delete_unsupported(val))
        .map(|(key, _val)| local_list.get(key).unwrap().clone())
        .collect();

    let mut to_delete = deleted_entries;
    to_delete.append(&mut outdated_entries);
    to_delete.append(&mut unsupported_entries);
    to_delete
}

/// Checks if the passed resource should be deleted.
///
/// returns `true` if `BW_OUTDATED` is set to `delete`
///         AND the resource prefix is set to `Outdated`,
///         otherwise `false`.
///
/// Check `delta_builder_test.rs` for example usages
fn should_delete_outdated(val: &Resource) -> bool {
    is_env_var("BW_OUTDATED", "delete")
        && val.prefix.eq_ignore_ascii_case("Outdated")
}

/// Checks if the passed resource should be deleted.
///
/// returns `true` if `BW_UNSUPPORTED` is set to `delete`
///         AND the resource prefix is set to `Unsupported`,
///         otherwise `false`.
///
/// Check `delta_builder_test.rs` for example usages
fn should_delete_unsupported(val: &Resource) -> bool {
    is_env_var("BW_UNSUPPORTED", "delete")
        && val.prefix.eq_ignore_ascii_case("Unsupported")
}

/// Checks if the passed resource should be skipped when downloading.
///
/// returns `true` if `BW_OUTDATED` is set to `delete` or `skip`
///         AND the resource prefix is set to `Outdated`,
///         otherwise `false`.
///
/// Check `delta_builder_test.rs` for example usages
fn should_skip_outdated(val: &Resource) -> bool {
    (is_env_var("BW_OUTDATED", "delete")
        || is_env_var("BW_OUTDATED", "skip"))
        && val.prefix.eq_ignore_ascii_case("Outdated")
}

/// Checks if the passed resource should be skipped when downloading.
///
/// returns `true` if `BW_UNSUPPORTED` is set to `delete` or `skip`
///         AND the resource prefix is set to `Unsupported`,
///         otherwise `false`.
///
/// Check `delta_builder_test.rs` for example usages
fn should_skip_unsupported(val: &Resource) -> bool {
    (is_env_var("BW_UNSUPPORTED", "delete")
        || is_env_var("BW_UNSUPPORTED", "skip"))
        && val.prefix.eq_ignore_ascii_case("Unsupported")
}

/// returns `true` if the env var specified by `env_name` is set to the value specified in `env_value`.
fn is_env_var(env_name: &str, env_value: &str) -> bool {
    let env_var = env::var(env_name);
    env_var.is_ok()
        && env_var.unwrap().eq_ignore_ascii_case(env_value)
}