use crate::delta_builder::DeltaAction;
use argh::FromArgs;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::{env, fs};

/// Automatically downloads BeamNG mods from beamng.com/resources
#[derive(FromArgs, Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    /// the path to the BeamNG client mods directory, e.g. /path/to/BeamNG.drive/client-mods
    #[argh(option, short = 'p')]
    pub client_mods_dir: Option<String>,

    /// list of mod ids to download, e.g. 123,456,789
    #[argh(option, short = 'm')]
    pub mods: Vec<String>,

    /// specify how to handle outdated mods. Either skip or delete.
    #[argh(option)]
    pub outdated: Option<String>,

    /// specify how to handle unsupported mods. Either skip or delete.
    #[argh(option)]
    pub unsupported: Option<String>,
}

/// Parses the command line arguments and returns the AppConfig struct.
pub fn parse_args() -> AppConfig {
    // First we build the AppConfig struct from env vars.
    let env_var_config: AppConfig = from_env_vars();

    // Then we parse the command line arguments.
    let mut cli_args_config: AppConfig = argh::from_env();
    cli_args_config.mods = cli_args_config.mods.iter().map(|s| get_mod_id(s)).collect();

    // After that we are checking for a config file
    let config_file_config: AppConfig = from_config_file("beiwagen.toml");

    // Merge mods vector from env vars, cli args and config file
    let mut mods = env_var_config.mods.clone();
    mods.extend(cli_args_config.mods.clone());
    mods.extend(config_file_config.mods.clone());

    // We merge the three configurations, env > cli > file
    let mut merged_config = AppConfig {
        client_mods_dir: env_var_config
            .client_mods_dir
            .or(cli_args_config.client_mods_dir)
            .or(config_file_config.client_mods_dir),
        mods,
        outdated: env_var_config
            .outdated
            .or(cli_args_config.outdated)
            .or(config_file_config.outdated),
        unsupported: env_var_config
            .unsupported
            .or(cli_args_config.unsupported)
            .or(config_file_config.unsupported),
    };

    // Verify that the client_mods_dir and at least one of mod is present.
    if merged_config.client_mods_dir.is_none() {
        eprintln!("Error: client_mods_dir is required.");
        std::process::exit(1);
    }

    // Verify that at least one mod is present.
    if merged_config.mods.is_empty() {
        eprintln!("Error: mods is required.");
        std::process::exit(1);
    }

    // Parse tilde in the client_mods_dir
    merged_config.client_mods_dir = merged_config.client_mods_dir.map(|client_mods_dir| {
        if client_mods_dir.contains("~") {
            let home_dir = env::var("HOME").unwrap_or_else(|_| String::from("/"));
            client_mods_dir.replace("~", &home_dir)
        } else {
            client_mods_dir
        }
    });

    merged_config
}

/// Builds the AppConfig struct from a config file.
/// The config file should be in the same directory as the executable and should be named beiwagen.toml.
/// The file should look like this:
/// ```toml
/// client_mods_dir = "/path/to/BeamNG.drive/client-mods"
/// outdated = "skip"
/// unsupported = "delete"
/// mods =  [
///   "https://www.beamng.com/resources/sic_igct-powertrain-kit.30373/" ,
///   "30414",
///   "https://www.beamng.com/resources/ibishu-pessima-awd-turbo.30372/
/// ]
/// ```
pub fn from_config_file(path: &str) -> AppConfig {
    // Read the config file, from the same directory as the executable.
    let path = env::current_exe().unwrap().parent().unwrap().join(path);
    let config_file = fs::read_to_string(path).ok();
    if let Some(config_file) = config_file {
        let mut toml_config: AppConfig = toml::from_str(&config_file)
            .unwrap_or_else(|error| panic!("Failed to parse config file. Error:\n{}", error));

        // Extract the mod ids from the URLs.
        toml_config.mods = toml_config.mods.iter().map(|s| get_mod_id(s)).collect();

        return toml_config;
    }
    AppConfig {
        client_mods_dir: None,
        mods: vec![],
        outdated: None,
        unsupported: None,
    }
}

/// Builds the AppConfig struct from environment variables
pub fn from_env_vars() -> AppConfig {
    let client_mods_dir = env::var("BW_CLIENT_MODS_DIR").ok();
    let mods: Vec<String> = env::var("BW_MODS")
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| get_mod_id(s.trim()))
        .collect();
    let outdated = env::var("BW_OUTDATED").ok();
    let unsupported = env::var("BW_UNSUPPORTED").ok();

    AppConfig {
        client_mods_dir,
        mods,
        outdated,
        unsupported,
    }
}

/// If the mod value is numeric, it is returned as is.
/// If the mod value is a URL, the mod id is extracted from the URL.
/// Example url is https://www.beamng.com/resources/sic_igct-powertrain-kit.30373/
/// Regular expression is used to extract the mod id from the URL.
/// If the mod value is not numeric or a URL, fails with an error.
fn get_mod_id(mod_value: &str) -> String {
    // First check if the mod value is numeric.
    if mod_value.parse::<u64>().is_ok() {
        return mod_value.to_string();
    }

    // Try to extract the mod id from the URL.
    lazy_static! {
        static ref BEAMNG_RESOURCE_PATTERN: Regex = Regex::new(
            // https://www.beamng.com/resources/sic_igct-powertrain-kit.30373/
            // https://www.beamng.com/resources/sic_igct-powertrain-kit.<mod-id>
            // Where mod-id should be extracted.
            // The last slash is optional.
            r"https://www.beamng.com/resources/.*\.(\d+)/?.*"
        ).unwrap();
    }

    if let Some(captures) = BEAMNG_RESOURCE_PATTERN.captures(mod_value) {
        return captures.get(1).unwrap().as_str().to_string();
    }

    // If the mod value is not numeric or a URL, fail with an error.
    panic!("Invalid mod value: {}", mod_value);
}

/// Parses the delta action string and returns the corresponding DeltaAction enum.
/// The string should be either skip or delete.
/// If the string is not skip or delete, DeltaAction::Ignore is returned.
/// If the string is None, DeltaAction::Ignore is returned.
pub fn parse_delta_action(delta_action_string: &Option<String>) -> DeltaAction {
    match delta_action_string {
        Some(action) => match action.as_str().to_lowercase().trim() {
            "skip" => DeltaAction::Skip,
            "delete" => DeltaAction::Delete,
            _ => DeltaAction::Ignore,
        },
        None => DeltaAction::Ignore,
    }
}
