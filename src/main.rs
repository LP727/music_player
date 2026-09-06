//! Main
//!
//! Main module that takes in arguments, initializes various modules
//! Start the app and handles the teardown.

use std::path::{Path, PathBuf};
use std::println;
use std::process;

use anyhow::Ok;
use directories::{ProjectDirs, UserDirs};
use music_player::library::InMemoryLibrary;
use music_player::settings::{Settings, read_settings};

fn main() -> anyhow::Result<()> {
    let music_path: PathBuf;
    if let Some(user_path) = UserDirs::new() {
        music_path = user_path.audio_dir().unwrap().to_path_buf();
    } else {
        eprintln!("Unable to access user directories");
        process::exit(1)
    }

    let settings_path: PathBuf;
    if let Some(project_path) = ProjectDirs::from("com", "lpbeliveau", "music_app") {
        settings_path = project_path.config_dir().to_path_buf();
    } else {
        eprintln!("Unable to access project directories");
        process::exit(1)
    }

    println!("{:?}", &music_path);
    println!("{:?}", &settings_path);

    // Load user settings
    let user_settings: Settings = read_settings(&settings_path, &music_path).unwrap_or_else(|| {
        println!("Failed to extract settings");
        Settings::default()
    });

    let assets_path = &Path::new(&user_settings.assets_folder_path);

    // Scan for music
    let song_lib = InMemoryLibrary::new(assets_path);
    println!("{}", song_lib);

    // TODO: Set state

    // TODO: Start UI

    Ok(())
}
