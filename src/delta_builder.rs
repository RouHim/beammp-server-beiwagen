use std::collections::HashMap;
use std::env;

use crate::resource::Resource;

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

fn should_delete_outdated(val: &Resource) -> bool {
    is_env_var("OUTDATED", "delete")
        && val.prefix.eq_ignore_ascii_case("Outdated")
}

fn should_delete_unsupported(val: &Resource) -> bool {
    is_env_var("UNSUPPORTED", "delete")
        && val.prefix.eq_ignore_ascii_case("Unsupported")
}

fn should_skip_outdated(val: &Resource) -> bool {
    (is_env_var("OUTDATED", "delete")
        || is_env_var("OUTDATED", "skip"))
        && val.prefix.eq_ignore_ascii_case("Outdated")
}

fn should_skip_unsupported(val: &Resource) -> bool {
    (is_env_var("UNSUPPORTED", "delete")
        || is_env_var("UNSUPPORTED", "skip"))
        && val.prefix.eq_ignore_ascii_case("Unsupported")
}

fn is_env_var(env_name: &str, env_value: &str) -> bool {
    let env_var = env::var(env_name);
    env_var.is_ok()
        && env_var.unwrap().eq_ignore_ascii_case(env_value)
}