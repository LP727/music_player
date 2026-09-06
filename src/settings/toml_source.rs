//! TOML
//!
//! This module exposes methods to extract settings from a toml file

use std::io::ErrorKind;
use std::{eprintln, fs, path::Path, println};

pub fn read_settings(settings_path: &Path, music_path: &Path) -> Option<super::Settings> {
    let text_settings = fs::read_to_string(settings_path);

    if text_settings.is_err() {
        match text_settings.as_ref().unwrap_err().kind() {
            ErrorKind::NotFound => {
                let new_settings = super::Settings {
                    assets_folder_path: music_path.to_path_buf(),
                };
                if write_settings(settings_path, &new_settings).is_err() {
                    eprintln!(
                        "Could not read file : `{}` : {}",
                        settings_path.to_str().unwrap_or(""),
                        text_settings.unwrap_err()
                    );
                    return None;
                }

                return Some(new_settings);
            }
            _ => {
                eprintln!(
                    "Could not read file : `{}` : {}",
                    settings_path.to_str().unwrap_or(""),
                    text_settings.unwrap_err()
                );
                return None;
            }
        }
    } else {
        println!("{:?}", text_settings);

        let settings: Option<super::Settings> = match toml::from_str::<super::Settings>(
            text_settings
                .expect("Failed getting settings string")
                .as_str(),
        ) {
            Ok(d) => Some(d),
            _ => None,
        };
        settings
    }
}

// Write default settings (currently just the location of the Music)
pub fn write_settings(settings_path: &Path, new_settings: &super::Settings) -> anyhow::Result<()> {
    let setting = toml::to_string(new_settings)?;
    fs::write(settings_path, setting)?;
    Ok(())
}

pub fn _edit_settings(_new_settings: &super::Settings) -> anyhow::Result<()> {
    // TODO implement a way to write new settings from the app
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::settings::{Settings, read_settings, write_settings};
    use std::path::Path;

    #[test]
    fn test_default() {
        let settings = Settings::default();
        assert_eq!(settings.assets_folder_path, String::from("./assets"));
    }

    #[test]
    fn test_read_settings() -> anyhow::Result<(), String> {
        let asset_path: &Path = Path::new("./assets");
        let settings_file = &format!("{}/settings.toml", env!("CARGO_MANIFEST_DIR"));
        let settings_path: &Path = Path::new(settings_file);
        match read_settings(settings_path, asset_path) {
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

    #[test]
    fn test_write_settings() -> anyhow::Result<(), String> {
        let settings_file = &format!("{}/settings.toml", env!("CARGO_MANIFEST_DIR"));
        let settings_path: &Path = Path::new(settings_file);
        let settings = Settings::default();

        match write_settings(settings_path, &settings) {
            Ok(()) => {}
            Err(err) => {
                return Err(err.to_string());
            }
        }

        match read_settings(settings_path, &settings.assets_folder_path) {
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
