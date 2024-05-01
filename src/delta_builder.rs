use std::collections::HashMap;

use crate::Resource;

/// Builds a delta list of mods to download, based on the local available and remote available mods.
pub struct DeltaBuilder {
    /// What to do with outdated mods
    pub(crate) unsupported: DeltaAction,
    /// What to do with unsupported mods
    pub(crate) outdated: DeltaAction,
}

#[derive(PartialEq)]
pub enum DeltaAction {
    Ignore,
    Delete,
    Skip,
}

impl DeltaBuilder {
    /// Builds a delta list of mods to download, based on the local available and remote available mods.
    ///
    /// `local_list` contains local available mods
    ///
    /// `remote_list` contains wanted remote online available mods
    ///
    /// `returns` a vector of mods that needs to be downloaded
    pub fn get_to_download(
        &self,
        local_list: &HashMap<u64, Resource>,
        remote_list: &HashMap<u64, Resource>,
    ) -> Vec<Resource> {
        let new_entries: Vec<Resource> = remote_list
            .iter()
            .filter(|(key, _val)| !local_list.contains_key(key))
            .map(|(_key, val)| val.clone())
            .filter(|entry| !self.should_skip_unsupported(entry))
            .filter(|entry| !self.should_skip_outdated(entry))
            .collect();

        let mut updated_entries: Vec<Resource> = remote_list
            .iter()
            .filter(|(key, _val)| local_list.contains_key(key))
            .filter(|(key, val)| local_list.get(key).unwrap().version < val.version)
            .map(|(_key, val)| val.clone())
            .filter(|entry| !self.should_skip_unsupported(entry))
            .filter(|entry| !self.should_skip_outdated(entry))
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
    pub fn get_to_remove(
        &self,
        local_list: &HashMap<u64, Resource>,
        remote_list: &HashMap<u64, Resource>,
    ) -> Vec<Resource> {
        let deleted_entries: Vec<Resource> = local_list
            .iter()
            .filter(|(key, _val)| !remote_list.contains_key(key))
            .map(|(_key, val)| val.clone())
            .collect();

        let mut outdated_entries: Vec<Resource> = remote_list
            .iter()
            .filter(|(key, _val)| local_list.contains_key(key))
            .filter(|(_key, val)| self.should_delete_outdated(val))
            .map(|(key, _val)| local_list.get(key).unwrap().clone())
            .collect();

        let mut unsupported_entries: Vec<Resource> = remote_list
            .iter()
            .filter(|(key, _val)| local_list.contains_key(key))
            .filter(|(_key, val)| self.should_delete_unsupported(val))
            .map(|(key, _val)| local_list.get(key).unwrap().clone())
            .collect();

        let mut to_delete = deleted_entries;
        to_delete.append(&mut outdated_entries);
        to_delete.append(&mut unsupported_entries);
        to_delete
    }

    /// Checks if the passed resource should be deleted.
    ///
    /// returns `true` if `OUTDATED` config is set to `delete`
    ///         AND the resource prefix is set to `Outdated`,
    ///         otherwise `false`.
    ///
    /// Check `delta_builder_test.rs` for example usages
    fn should_delete_outdated(&self, val: &Resource) -> bool {
        // read unsupported config
        self.outdated == DeltaAction::Delete && val.prefix.eq_ignore_ascii_case("Outdated")
    }

    /// Checks if the passed resource should be deleted.
    ///
    /// returns `true` if `UNSUPPORTED` config is set to `delete`
    ///         AND the resource prefix is set to `Unsupported`,
    ///         otherwise `false`.
    ///
    /// Check `delta_builder_test.rs` for example usages
    fn should_delete_unsupported(&self, val: &Resource) -> bool {
        self.unsupported == DeltaAction::Delete && val.prefix.eq_ignore_ascii_case("Unsupported")
    }

    /// Checks if the passed resource should be skipped when downloading.
    ///
    /// returns `true` if `OUTDATED` config is set to `delete` or `skip`
    ///         AND the resource prefix is set to `Outdated`,
    ///         otherwise `false`.
    ///
    /// Check `delta_builder_test.rs` for example usages
    fn should_skip_outdated(&self, val: &Resource) -> bool {
        let is_delete_or_skip =
            self.outdated == DeltaAction::Delete || self.outdated == DeltaAction::Skip;
        let has_outdated_prefix = val.prefix.eq_ignore_ascii_case("Outdated");
        has_outdated_prefix && is_delete_or_skip
    }

    /// Checks if the passed resource should be skipped when downloading.
    ///
    /// returns `true` if `UNSUPPORTED` config is set to `delete` or `skip`
    ///         AND the resource prefix is set to `Unsupported`,
    ///         otherwise `false`.
    ///
    /// Check `delta_builder_test.rs` for example usages
    fn should_skip_unsupported(&self, val: &Resource) -> bool {
        let is_delete_or_skip =
            self.unsupported == DeltaAction::Delete || self.unsupported == DeltaAction::Skip;
        let has_unsupported_prefix = val.prefix.eq_ignore_ascii_case("Unsupported");
        has_unsupported_prefix && is_delete_or_skip
    }
}
