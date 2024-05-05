use crate::config;
use std::{env, fs};

#[test]
fn test_from_config_file() {
    // GIVEN a config file with the following content
    let config_file_content = r#"
    client_mods_dir = "/path/to/BeamNG.drive/client-mods"
    outdated = "skip"
    unsupported = "delete"
    mods = [
        "https://www.beamng.com/resources/sic_igct-powertrain-kit.30373/this-is-a-test",
        "30414",
        "https://www.beamng.com/resources/ibishu-pessima-awd-turbo.30372/"
    ]
    "#;

    let config_file_name = random_file_name();
    fs::write(&config_file_name, config_file_content).unwrap();

    // WHEN the config file is read
    let config = config::from_config_file(&config_file_name);

    // Check if the values are as expected
    assert_eq!(
        config.client_mods_dir,
        Some("/path/to/BeamNG.drive/client-mods".to_string())
    );
    assert_eq!(config.mods, vec!["30373", "30414", "30372"]);
    assert_eq!(config.outdated, Some("skip".to_string()));
    assert_eq!(config.unsupported, Some("delete".to_string()));

    // Clean up
    fs::remove_file(config_file_name).unwrap();
}

#[test]
fn test_from_env_vars() {
    // Set up environment variables
    env::set_var("BW_CLIENT_MODS_DIR", "/path/to/client_mods");
    env::set_var(
        "BW_MODS",
        "123,456,https://www.beamng.com/resources/ibishu-pessima-awd-turbo.30372/",
    );
    env::set_var("BW_OUTDATED", "skip");
    env::set_var("BW_UNSUPPORTED", "delete");

    let config = config::from_env_vars();

    assert_eq!(
        config.client_mods_dir,
        Some("/path/to/client_mods".to_string())
    );
    assert_eq!(config.mods, vec!["123", "456", "30372"]);
    assert_eq!(config.outdated, Some("skip".to_string()));
    assert_eq!(config.unsupported, Some("delete".to_string()));
}

#[test]
#[should_panic]
fn test_invalid_config_values() {
    // GIVEN a config file with invalid values
    let config_file_content = r#"
    client_mods_dir = 123
    outdated = 456
    unsupported = 789
    mods = 101112
    "#;
    let config_file_name = random_file_name();
    fs::write(&config_file_name, config_file_content).unwrap();

    // WHEN the config file is read
    config::from_config_file(&config_file_name);

    // THEN the parsing should panic
}

#[test]
fn test_missing_config_file() {
    // WHEN the config file is read
    let config_file_name = random_file_name();
    let config = config::from_config_file(&config_file_name);

    // Check if the values are None
    assert_eq!(config.client_mods_dir, None);
    assert_eq!(config.mods, Vec::<String>::new());
    assert_eq!(config.outdated, None);
    assert_eq!(config.unsupported, None);
}

#[test]
#[should_panic]
fn test_empty_config_file() {
    // GIVEN an empty config file
    let config_file_name = random_file_name();
    fs::write(&config_file_name, "").unwrap();

    // WHEN the config file is read
    config::from_config_file(&config_file_name);

    // THEN the parsing should panic
}

#[test]
#[should_panic]
fn test_invalid_mods_urls() {
    // GIVEN a config file with invalid URLs in the mods array
    let config_file_content = r#"
    mods = [
        "invalid_url",
        "another_invalid_url"
    ]
    "#;
    let config_file_name = random_file_name();
    fs::write(&config_file_name, config_file_content).unwrap();

    // WHEN the config file is read
    config::from_config_file(&config_file_name);

    // THEN the parsing should panic
}

fn random_file_name() -> String {
    let file_name = format!("{}.toml", rand::random::<u64>());
    env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(file_name)
        .to_str()
        .unwrap()
        .to_string()
}
