use crate::resource::Resource;
use std::collections::HashMap;

pub fn get_to_download(local_list: &HashMap<u64, Resource>, remote_list: &HashMap<u64, Resource>) -> Vec<Resource> {
    return remote_list
        .iter()
        // if local already exists and version local is older or equal -> no download
        .inspect(|(key, val)| println!(" x {} {}", val.version, ))
        .filter(|(key, val)| local_list.contains_key(&key) && val.version <= local_list[&key].version)
        .map(|(key, val)| val.clone())
        .collect();

}

pub(crate) fn get_to_remove() -> Vec<Resource> {
    vec![]
}