//! # Library
//!
//! This module provides the APIs to scan the environment
//! and locate the music files.

use std::{fs, path::PathBuf, println};

use lofty::file::FileType;
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
        if is_supported_audio(&entry_path) {
            songs.push(entry_path);
        } else {
            println!("Ignored file: {}", entry_path.to_str().unwrap());
        }
    }
    songs
}

pub fn is_supported_audio(path: &PathBuf) -> bool {
    // TODO: Implement Symphonia Probing in audio.rs and call that here
    true
}
