//! # Library
//!
//! This module provides the APIs to scan the environment
//! and locate the music files.

use crate::audio::is_supported_format;

use core::result::Result::Ok;
use std::fmt::Display;

use std::path::Path;
use std::writeln;
use std::{path::PathBuf, println};

use lofty::prelude::ItemKey;
use lofty::{file::TaggedFileExt, read_from_path};

use walkdir::WalkDir;

pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub file_path: PathBuf,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            title: String::from("Unknown"),
            artist: String::from("Unknown"),
            album: String::from("Unknown"),
            file_path: PathBuf::from("Unknown"),
        }
    }
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Title: {} \nArtist: {} \nAlbum: {} \nPath: {}",
            &self.title,
            &self.artist,
            &self.album,
            self.file_path.to_str().unwrap_or_default()
        )
    }
}

impl Track {
    pub fn from_path_buf(track_path: &PathBuf) -> anyhow::Result<Track> {
        let mut track: Track = Track::default();

        let taggedfile = read_from_path(track_path)?;

        let file_tag = taggedfile
            .primary_tag()
            .or_else(|| taggedfile.first_tag())
            .ok_or_else(|| anyhow::anyhow!("No tag data found for {:?}", track_path))?;

        track.title = file_tag
            .get_string(ItemKey::TrackTitle)
            .unwrap_or_else(|| "Unknown")
            .to_string();
        track.album = file_tag
            .get_string(ItemKey::AlbumTitle)
            .unwrap_or_else(|| "Unknown")
            .to_string();
        track.artist = file_tag
            .get_string(ItemKey::TrackArtist)
            .unwrap_or_else(|| "Unknown")
            .to_string();
        track.file_path = track_path.to_path_buf();

        Ok(track)
    }
}

pub trait Library {
    fn all_tracks(&self) -> Vec<&Track>;
    fn by_artist(&self, artist_name: &str) -> Vec<&Track>;
    fn by_album(&self, album_name: &str) -> Vec<&Track>;
    fn by_list(&self, list_name: &str) -> Vec<&Track>;
}

pub struct InMemoryLibrary {
    tracks: Vec<Track>,
}

impl InMemoryLibrary {
    pub fn new(asset_path: &Path) -> Self {
        Self {
            tracks: scan_assets(asset_path),
        }
    }
}

impl Display for InMemoryLibrary {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for track in self.tracks.iter() {
            println!("{}", track);
        }

        core::result::Result::Ok(())
    }
}

impl Library for InMemoryLibrary {
    fn all_tracks(&self) -> Vec<&Track> {
        self.tracks.iter().collect()
    }
    fn by_artist(&self, artist_name: &str) -> Vec<&Track> {
        self.tracks
            .iter()
            .filter(|x| x.artist == artist_name)
            .collect()
    }
    fn by_album(&self, album_name: &str) -> Vec<&Track> {
        self.tracks
            .iter()
            .filter(|x| x.album == album_name)
            .collect()
    }

    fn by_list(&self, _list_name: &str) -> Vec<&Track> {
        // TODO: Implement list storage in user data using directories
        self.tracks.iter().collect()
    }
}

pub fn scan_assets(asset_path: &Path) -> Vec<Track> {
    let mut songs = Vec::new();

    for entry in WalkDir::new(asset_path).into_iter().filter_map(|e| e.ok())
    // ignore errors like denied permissions
    {
        let entry_path: PathBuf = entry.into_path();
        if is_supported_format(&entry_path) {
            let track = match Track::from_path_buf(&entry_path) {
                Ok(value) => value,
                Err(e) => {
                    eprintln!("Could not read file : `{:?}` : {}", entry_path, e);
                    continue;
                }
            };

            songs.push(track);
        } else {
            println!("Ignored file: {}", entry_path.to_str().unwrap());
        }
    }
    songs
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use crate::library::scan_assets;
    use lofty::file::AudioFile;
    use lofty::read_from_path;

    #[test]
    fn scan_test() -> anyhow::Result<(), String> {
        let settings_file = &format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
        let asset_path: &Path = Path::new(settings_file);
        let songs_path = scan_assets(asset_path);

        if songs_path.len() == 3 {
            for song in songs_path {
                if song.file_path.to_str().unwrap().contains("Linkin") == false
                    && song.file_path.to_str().unwrap().contains("Beatles") == false
                {
                    return Err(String::from("Wrong file name"));
                }
                match read_from_path(&song.file_path) {
                    Ok(file) => {
                        let chan_num = file.properties().channels().unwrap_or_else(|| 0);
                        if chan_num != 2 {
                            return Err(format!("Wrong number of channel: {}", chan_num));
                        }
                    }
                    Err(e) => {
                        return Err(format!(
                            "Failed to parse song {}, error: {}",
                            &song.file_path.to_str().unwrap(),
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
