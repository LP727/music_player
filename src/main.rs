//! Main
//!
//! Main module that takes in arguments, initializes various modules
//! Start the app and handles the teardown.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::mpsc;

use anyhow::Ok;
use directories::{ProjectDirs, UserDirs};
use env_logger::Env;
use log::{Level, debug, error};

use music_player::controller::Controller;
use music_player::library::{InMemoryLibrary, Library};
use music_player::settings::{Settings, read_settings};
use music_player::ui::{main_loop, spawn_input_thread};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> RawModeGuard {
        enable_raw_mode().expect("Raw mode enable Failed");
        debug!("Terminal Raw Mode enabled");
        Self
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        disable_raw_mode().expect("Raw mode disable Failed");
        debug!("Terminal Raw Mode disabled");
    }
}

fn main() -> anyhow::Result<()> {
    // This ensure we restore the terminal on exit
    let _raw_guard = RawModeGuard::new();

    // Default form doesn't have the \r on raw mode so we use a custom format
    let mut logger_builder =
        env_logger::Builder::from_env(Env::default().default_filter_or("error"));
    logger_builder
        .format(|buf, record| {
            let level_style = buf.default_level_style(record.level());
            let style = anstyle::Style::new()
                .fg_color(Some(anstyle::AnsiColor::Green.into()))
                .bg_color(Some(anstyle::AnsiColor::BrightBlack.into()));

            if record.level() == Level::Error {
                writeln!(
                    buf,
                    "[{} {level_style}{}{level_style:#} {}] - {}\r\n {style} {} Line: {}{style} \r",
                    buf.timestamp(),
                    record.level(),
                    record.metadata().target(),
                    record.args(),
                    record.file().unwrap_or("Unknown"),
                    record.line().unwrap_or(0xFF),
                )
            } else {
                writeln!(
                    buf,
                    "[{} {level_style}{}{level_style:#} {}] - {}\r",
                    buf.timestamp(),
                    record.level(),
                    record.metadata().target(),
                    record.args(),
                )
            }
        })
        .init();

    // Get the user's music directory location
    let music_path: PathBuf;
    if let Some(user_path) = UserDirs::new() {
        music_path = user_path.audio_dir().unwrap().to_path_buf();
    } else {
        error!("Unable to access user directories");
        process::exit(1)
    }

    // Get the app's directory location
    let settings_path: PathBuf;
    if let Some(project_path) = ProjectDirs::from("com", "lpbeliveau", "music_app") {
        settings_path = project_path.config_dir().to_path_buf();
    } else {
        error!("Unable to access project directories");
        process::exit(1)
    }

    // Temporary debug logs
    debug!("{:?}", &music_path);
    debug!("{:?}", &settings_path);

    // Load user settings
    let user_settings: Settings = read_settings(&settings_path, &music_path).unwrap_or_else(|| {
        error!("Failed to extract settings");
        Settings::default()
    });

    let assets_path = &Path::new(&user_settings.assets_folder_path);

    // Scan for music
    let song_lib = InMemoryLibrary::new(assets_path);

    let (ui_tx, rx) = mpsc::channel();

    let mut controller = Controller::new(ui_tx.clone());
    controller.load_songlist(song_lib.all_tracks());

    let handle1 = spawn_input_thread(ui_tx);
    let handle2 = main_loop(controller, rx);

    _ = handle1.join().unwrap();
    _ = handle2.join().unwrap();
    // TODO: Start UI

    Ok(())
}
