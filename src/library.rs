//! # Library
//!
//! This module provides the APIs to scan the environment
//! and locate the music files.

use crate::audio::is_supported_format;
use std::{path::PathBuf, println};

use walkdir::WalkDir;

use crate::settings;

pub fn scan_assets() -> Vec<PathBuf> {
    let mut songs = Vec::new();

    let settings = settings::read_settings().unwrap_or_else(|| {
        println!("Failed to extract settings");
        settings::Settings::default()
    });

    for entry in WalkDir::new(settings.assets_folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
    // ignore errors like denied permissions
    {
        let entry_path: PathBuf = entry.into_path();
        if is_supported_format(&entry_path) {
            songs.push(entry_path);
        } else {
            println!("Ignored file: {}", entry_path.to_str().unwrap());
        }
    }
    songs
}

#[cfg(test)]
mod test {
    use crate::library::scan_assets;
    use lofty::file::AudioFile;
    use lofty::read_from_path;

    #[test]
    fn scan_test() -> anyhow::Result<(), String> {
        let songs_path = scan_assets();

        if songs_path.len() == 2 {
            for song in songs_path {
                if song.to_str().unwrap().contains("Linkin") == false {
                    return Err(String::from("Wrong file name"));
                }
                match read_from_path(&song) {
                    Ok(file) => {
                        let chan_num = file.properties().channels().unwrap_or_else(|| 0);
                        if chan_num != 2 {
                            return Err(format!("Wrong number of channel: {}", chan_num));
                        }
                    }
                    Err(e) => {
                        return Err(format!(
                            "Failed to parse song {}, error: {}",
                            &song.to_str().unwrap(),
                            e
                        ));
                    }
                }
            }
            return Ok(());
        } else {
            return Err(String::from("Failed to get song paths."));
        }
    }
}
