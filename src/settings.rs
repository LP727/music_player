//! # Settings
//!
//! This modules exposes the Settings Struct and a method to parse it.

mod toml_source;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub use toml_source::read_settings;
pub use toml_source::write_settings;

// Root location
pub const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub assets_folder_path: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            assets_folder_path: PathBuf::from("./assets"),
        }
    }
}
