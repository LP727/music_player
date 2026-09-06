//! TOML
//!
//! This module exposes methods to extract settings from a toml file

use std::{eprintln, fs, println};

pub fn read_settings() -> Option<super::Settings> {
    let settings_file = &format!("{}/settings.toml", super::MANIFEST_DIR);
    let text_settings = match fs::read_to_string(settings_file) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Could not read file : `{}` : {}", settings_file, err);
            return None;
        }
    };

    println!("{}", text_settings);

    let settings: Option<super::Settings> = match toml::from_str::<super::Settings>(&text_settings)
    {
        Ok(d) => Some(d),
        _ => None,
    };
    settings
}

pub fn _write_settings(_new_settings: &super::Settings) -> anyhow::Result<()> {
    // TODO implement a way to write new settings from the app
    Ok(())
}

pub fn _edit_settings(_new_settings: &super::Settings) -> anyhow::Result<()> {
    // TODO implement a way to write new settings from the app
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::settings::{Settings, read_settings};

    #[test]
    fn test_default() {
        let settings = Settings::default();
        assert_eq!(settings.assets_folder_path, String::from("./assets"));
    }

    #[test]
    fn test_read_settings() -> anyhow::Result<(), String> {
        match read_settings() {
            Some(settings) => {
                if settings.assets_folder_path == String::from("./assets") {
                    return Ok(());
                } else {
                    return Err(String::from("Wrong settings retrieved."));
                }
            }
            _ => return Err(String::from("Failed to read default settings.")),
        };
    }
}
