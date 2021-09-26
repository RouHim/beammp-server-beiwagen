use crate::resource::Resource;
use std::collections::HashMap;

pub fn get_to_download(local_list: &HashMap<u64, Resource>, remote_list: &HashMap<u64, Resource>) -> Vec<Resource> {
    let mut new_entries: Vec<Resource> = remote_list
        .iter()
        .filter(|(key, val)| !local_list.contains_key(key))
        .map(|(key, val)| val.clone())
        .collect();

    let mut updated_entries: Vec<Resource> = remote_list
        .iter()
        .filter(|(key, val)| local_list.contains_key(key))
        .filter(|(key, val)| local_list.get(key).unwrap().version < val.version)
        .map(|(key, val)| val.clone())
        .collect();

    new_entries.append(&mut updated_entries);
    new_entries
}

pub(crate) fn get_to_remove() -> Vec<Resource> {
    vec![]
}