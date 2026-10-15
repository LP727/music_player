//! # States
//!
//! This module provides API to track the current AppState.
//! Things like the current index, the list to track, volume, etc.

use crate::{audio::manager::AudioManager, library::Track};
use log::debug;

pub enum AppState {
    Stop(StopState),
    Pause(PauseState),
    Play(PlayState),
}

pub struct StopState {}

pub struct PauseState {
    pub index: usize,
    pub tracks: Vec<Track>,
}

pub struct PlayState {
    pub index: usize,
    pub tracks: Vec<Track>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState::Stop(StopState {})
    }
}

impl StopState {
    /// Play from stop state, consumes the state returns a PlayState
    ///
    /// # Arguments
    ///
    /// - `tracks` (`Vec<Track>`) - List of songs to play, moved in the state
    /// - `index` (`u32`) - Index where to start playing, if not provided, starts at 0
    ///
    /// # Returns
    ///
    /// - `PlayState` - State that keeps track of the index playing and list being played
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let song_lib = InMemoryLibrary::new(assets_path);
    /// let state = AppState::new() // stopped
    /// let state = state.play(song_lib.all_tracks()); // playing from index 0
    /// ```
    pub fn play(
        self,
        man: &mut AudioManager,
        tracks: Vec<Track>,
        index: Option<usize>,
    ) -> PlayState {
        if tracks.len() > 0 {
            man.play(Some(&tracks[index.unwrap_or(0)]));
        }

        debug!("Going from Stop to Play");
        PlayState {
            index: index.unwrap_or(0),
            tracks,
        }
    }
}

impl PauseState {
    /// Play form paused state, play takes ownership of everything
    /// If passing in a new list of songs, starts playing form that list
    /// resumes from that list at previous index otherwise. The passed
    /// manager holds the position information.
    ///
    /// This assumes a song is loaded in the manager since we can only reach a PauseState from a
    /// PlayState
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    /// let song_lib = InMemoryLibrary::new(assets_path);
    /// let state = AppState::new() // stopped
    /// let state = state.play(song_lib.all_tracks()); // playing from index 0
    /// let state = state.plause() // pause
    /// let state = state.play()
    /// ```
    pub fn play(
        mut self,
        man: &mut AudioManager,
        tracks: Vec<Track>,
        index: Option<usize>,
    ) -> PlayState {
        if tracks.len() > 0 {
            man.play(Some(&tracks[index.unwrap_or(0)]));
            self.tracks = tracks;
        } else if let Some(i) = index {
            man.play(Some(&self.tracks[i]));
        } else {
            // if no new list nore index is provided, we just unpause the song loaded in the manager
            // if it has one loaded
            man.play(None);
        }
        debug!("Going from Pause to Play");
        PlayState {
            index: index.unwrap_or(0),
            tracks: self.tracks,
        }
    }

    pub fn stop(self, man: &AudioManager) -> StopState {
        man.stop();
        debug!("Going from Pause to Stop");
        StopState {}
    }
}

impl PlayState {
    pub fn pause(self, man: &AudioManager) -> PauseState {
        man.pause();
        debug!("Going from Play to Pause");
        PauseState {
            index: self.index,
            tracks: self.tracks,
        }
    }

    pub fn stop(self, man: &AudioManager) -> StopState {
        man.stop();
        debug!("Going from Play to Stop");
        StopState {}
    }
}

#[cfg(test)]
mod test {

    use std::path::Path;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use crate::{audio::manager::AudioManager, library::scan_assets, state::AppState};

    #[test]
    fn state_cycle_test() -> anyhow::Result<(), String> {
        // Audio manager init
        let (tx, _rx) = mpsc::channel();
        let mut man = AudioManager::new(tx);

        // path list
        let assets_dir = &format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
        let asset_path: &Path = Path::new(assets_dir);
        let songs = scan_assets(asset_path);
        assert!(songs.len() >= 2, "Need at least 2 songs for there tests");
        assert!(songs[0].duration >= Duration::from_secs(1));

        let state = AppState::new();
        let state = match state {
            AppState::Stop(s) => s,
            _ => return Err("expected Stop state".into()),
        };
        assert!(man.is_stopped());

        let state = state.play(&mut man, songs.clone(), None);
        assert_eq!(state.index, 0);
        assert_eq!(state.tracks.len(), songs.len());
        assert!(man.is_playing());
        thread::sleep(Duration::from_millis(100));
        assert!(man.position() > Duration::from_millis(0));
        assert!(man.position() < Duration::from_millis(300));

        let state = state.pause(&man);
        assert_eq!(state.tracks.len(), songs.len());
        assert!(man.is_paused());
        assert!(man.position() > Duration::from_millis(0));
        assert!(man.position() < Duration::from_millis(300));

        let state = state.play(&mut man, Vec::new(), None);
        assert!(man.is_playing());
        thread::sleep(Duration::from_millis(100));
        assert!(man.position() > Duration::from_millis(0));
        assert!(man.position() < Duration::from_millis(300));

        let state = state.stop(&man);
        // giving time for the player to flush the mixer
        thread::sleep(Duration::from_millis(100));
        assert!(man.is_stopped());

        let state = state.play(&mut man, songs.clone(), Some(1));
        assert_eq!(state.index, 1);
        thread::sleep(Duration::from_millis(300));
        assert!(man.position() > Duration::from_millis(200));
        assert!(man.position() < Duration::from_millis(500));

        let state = state.pause(&man);
        assert!(man.position() > Duration::from_millis(200));
        assert!(man.position() < Duration::from_millis(500));

        let state = state.stop(&man);
        // giving time for the player to flush the mixer
        thread::sleep(Duration::from_millis(100));
        assert!(man.is_stopped());

        // consecutive calls test, just expecting no crash
        let state = state.play(&mut man, songs.clone(), Some(1));
        let state = state.pause(&man);
        let _state = state.stop(&man);

        // giving time for the player to flush the mixer
        thread::sleep(Duration::from_millis(100));
        assert!(man.is_stopped());
        Ok(())
    }
}
