//! TOML
//!
//! This module exposes methods to extract settings from a toml file

use std::io::ErrorKind;
use std::{eprintln, fs, path::Path, println};

/// Fetch the user settings file, create one if non was found, calling write settings
/// against the provided location for settings and storing the provided location for assets (music)
///
/// # Arguments
///
/// - `settings_path` (`&Path`) - Path to look for settings, providing an alternate path to
///   exsisting settings will create a new file there
/// - `music_path` (`&Path`) - Path to look for assets, providing an alternate path will create a
///   new file there
///
/// # Returns
///
/// - `Option<super::Settings>` - Settings struct, currently only holds the assets (music) location
///
/// # Errors
///
/// TODO: Fill with AI
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let asset_path: &Path = Path::new("./assets");
/// let settings_file = &format!("{}/settings.toml", env!("CARGO_MANIFEST_DIR"));
/// let settings_path: &Path = Path::new(settings_file);
/// let settings =  read_settings(settings_path, asset_path).unwrap();
/// ```
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

/// Write default settings (currently just the location of the Music)
///
/// # Arguments
///
/// - `settings_path` (`&Path`) - Path to look for settings, providing an alternate path will create
///   a new file there
/// - `music_path` (`&Path`) - Path to look for assets, providing an alternate path will create a
///   new file there
///
/// # Returns
///
/// - `anyhow::Result<()>` - Ok(()) if successful, io::Result if fs::write fails
///
/// # Errors
///
/// TODO: Fill with AI
///
/// # Examples
/// TODO: Fill with AI
/// ```
/// use crate::...;
///
/// let _ = write_settings();
/// ```
pub fn write_settings(settings_path: &Path, new_settings: &super::Settings) -> anyhow::Result<()> {
    let setting = toml::to_string(new_settings)?;
    fs::write(settings_path, setting)?;
    Ok(())
}

/// Edit existing settings. Reads existing settings, if different, writes the new ones, relies on
/// read_settings and write_settings.
///
/// TODO: When (if) we add more settings, need to create a way to
/// just edit one, and maybe have a settings manager to have a RAM copy to avoid file operations #
/// Arguments
///
/// - `settings_path` (`&Path`) - Path for potentially existing settings
/// - `new_settings` (`&super`) - new settings value
///
/// # Returns
///
/// - `anyhow::Result<()>` - read_settings or write settings error pattern
///
/// # Errors
/// TODO: Fill with AI
///
/// Describe possible errors.
///
/// # Examples
/// TODO: Fill with AI
/// ```
/// use crate::...;
///
/// let _ = edit_settings();
/// ```
pub fn edit_settings(settings_path: &Path, new_settings: &super::Settings) -> anyhow::Result<()> {
    // if there are no settings, this will update them to the new value
    let old_settings = read_settings(settings_path, &new_settings.assets_folder_path).unwrap();

    // If new settings differ from the existing one we
    if old_settings != *new_settings {
        return write_settings(settings_path, new_settings);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::settings::{Settings, read_settings, write_settings};
    use std::{mem::type_info::Str, path::Path};

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

    #[test]
    fn test_eddit_settings() -> anyhow::Result<(), String> {
        // TODO: Fill with AI
        Ok(())
    }
}
