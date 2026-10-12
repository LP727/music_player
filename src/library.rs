//! # Library
//!
//! This module provides the APIs to scan the environment
//! and locate the music files.

use crate::audio::is_supported_format;

use core::result::Result::Ok;
use std::collections::BTreeSet;
use std::fmt::Display;

use std::path::Path;
use std::time::Duration;
use std::writeln;
use std::{path::PathBuf, println};

use lofty::prelude::ItemKey;
use lofty::{file::AudioFile, file::TaggedFileExt, read_from_path};

use walkdir::WalkDir;

#[derive(Debug, PartialEq, Clone)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub file_path: PathBuf,
    pub duration: Duration,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            title: String::from("Unknown"),
            artist: String::from("Unknown"),
            album: String::from("Unknown"),
            file_path: PathBuf::from("Unknown"),
            duration: Duration::from_secs(0),
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
    pub fn from_path(track_path: &Path) -> anyhow::Result<Track> {
        let mut track: Track = Track::default();

        let taggedfile = read_from_path(track_path)?;

        let file_tag = taggedfile
            .primary_tag()
            .or_else(|| taggedfile.first_tag())
            .ok_or_else(|| anyhow::anyhow!("No tag data found for {:?}", track_path))?;

        track.title = file_tag
            .get_string(ItemKey::TrackTitle)
            .unwrap_or("Unknown")
            .to_string();
        track.album = file_tag
            .get_string(ItemKey::AlbumTitle)
            .unwrap_or("Unknown")
            .to_string();
        track.artist = file_tag
            .get_string(ItemKey::TrackArtist)
            .unwrap_or("Unknown")
            .to_string();
        track.file_path = track_path.to_path_buf();

        track.duration = taggedfile.properties().duration();

        Ok(track)
    }
}

pub trait Library {
    fn all_tracks(&self) -> Vec<Track>;
    fn all_artists(&self) -> Vec<String>;
    fn all_albums(&self) -> Vec<String>;
    fn albums_by_artist(&self, artist_name: &str) -> Vec<String>;
    fn tracks_by_artist(&self, artist_name: &str) -> Vec<&Track>;
    fn tracks_by_album(&self, album_name: &str) -> Vec<&Track>;
    fn tracks_by_list(&self, list_name: &str) -> Vec<&Track>;
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
    fn all_tracks(&self) -> Vec<Track> {
        self.tracks.iter().cloned().collect()
    }
    fn all_artists(&self) -> Vec<String> {
        // let mut artists_set = BTreeSet::<String>::new();
        // for track in &self.tracks {
        //     // This clones prior to filtering so it costs an allocate + free on each collision
        //     artists_set.insert(track.artist.clone());
        // }

        // equivalent to
        // let mut artists = Vec::with_capacity(artists_set.len());
        // for artist in artists_set {
        //     artists.push(artist);
        // }
        // artists
        //artists_set.into_iter().collect()

        // So a better version is the following:
        // let mut artists_set = BTreeSet::<&String>::new();
        // for tracks in &self.tracks {
        //     // only attempt to push by ref, no insert
        //     artists_set.insert(&tracks.artist);
        // }

        // let mut artists = Vec::<String>::with_capacity(artists_set.len());
        // for artist in artists_set {
        //     // allocate only in the vector
        //     artists.push(artist.clone());
        // }
        // artists

        // Shortest form
        //  self.tracks.iter().map(|t|&t.artist): take all artists in tracks (by ref using &) into a
        // vec collect::<BTreeSet<&String>>(): populate a BTreeSet of strings ref
        // .into_iter().cloned().collect() clones the strings into a new vector through collect
        self.tracks
            .iter()
            .map(|t| &t.artist)
            .collect::<BTreeSet<&String>>()
            .into_iter()
            .cloned()
            .collect()
    }

    fn all_albums(&self) -> Vec<String> {
        self.tracks
            .iter()
            .map(|t| &t.album)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .cloned()
            .collect()
    }

    fn albums_by_artist(&self, artist_name: &str) -> Vec<String> {
        self.tracks
            .iter()
            .filter(|x| x.artist == artist_name)
            .map(|t| &t.album)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .cloned()
            .collect()
    }

    fn tracks_by_artist(&self, artist_name: &str) -> Vec<&Track> {
        self.tracks
            .iter()
            .filter(|x| x.artist == artist_name)
            .collect()
    }

    // TODO: Assess if we do deep copy all the time or have lifetime parameters in state
    fn tracks_by_album(&self, album_name: &str) -> Vec<&Track> {
        self.tracks
            .iter()
            .filter(|x| x.album == album_name)
            .collect()
    }

    fn tracks_by_list(&self, _list_name: &str) -> Vec<&Track> {
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
        // Rely on Symphonia to verify support as Lofty support doesn't guarantee audio support
        if is_supported_format(&entry_path) {
            let track = match Track::from_path(&entry_path) {
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
        let assets_dir = &format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
        let asset_path: &Path = Path::new(assets_dir);
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
