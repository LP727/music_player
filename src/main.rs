//! Main
//!
//! Main module that takes in arguments, initializes various modules
//! Start the app and handles the teardown.

use std::path::Path;
use std::println;

use anyhow::Ok;
use music_player::library::scan_assets;
use music_player::settings::{Settings, read_settings};

fn main() -> anyhow::Result<()> {
    // Load user settings
    let user_settings: Settings = read_settings().unwrap_or_else(|| {
        println!("Failed to extract settings");
        Settings::default()
    });

    let settings_path = &Path::new(&user_settings.assets_folder_path);

    // Scan for music
    let songs_path = scan_assets(settings_path);
    for song in songs_path {
        println!("{}", song);
    }

    // TODO: Set state

    // TODO: Start UI

    Ok(())
}
