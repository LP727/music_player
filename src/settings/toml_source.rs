//! TOML
//!
//! This module exposes methods to extract settings from a toml file

use std::{eprintln, fs};

pub fn read_settings() -> Option<super::Settings> {
    let settings_file = &format!("{}/settings.json", super::MANIFEST_DIR);
    let text_settings = match fs::read_to_string(settings_file) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Could not read file : `{}` : {}", settings_file, err);
            return None;
        }
    };

    let settings: Option<super::Settings> = match toml::from_str(&text_settings) {
        Ok(d) => d,
        _ => None,
    };

    settings
}

pub fn write_settings(_new_settings: &super::Settings) -> anyhow::Result<()> {
    // TODO implement a way to write new settings from the app
    Ok(())
}
