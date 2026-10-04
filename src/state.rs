//! # States
//!
//! This module provides API to track the current AppState.
//! Things like the current index, the list to track, volume, etc.

use std::{cell::RefCell, rc::Rc};

use crate::{audio::manager::AudioManager, library::Track};

struct AppState {}

struct StopState {
    // This allows us to share the audio manager and call play/pause/sop on it while others can
    // handle volume, next, ext.
    pub man: Rc<RefCell<AudioManager>>,
}

struct PauseState {
    man: Rc<RefCell<AudioManager>>,
    index: usize,
    tracks: Vec<Track>,
}

struct PlayState {
    man: Rc<RefCell<AudioManager>>,
    index: usize,
    tracks: Vec<Track>,
}

impl AppState {
    pub fn new(manager: Rc<RefCell<AudioManager>>) -> StopState {
        StopState { man: manager }
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
    pub fn play(self, tracks: Vec<Track>, index: Option<usize>) -> PlayState {
        if tracks.len() > 0 {
            self.man
                .borrow_mut()
                .play(Some(&tracks[index.unwrap_or(0)]));
        }
        PlayState {
            man: self.man,
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
    pub fn play(mut self, tracks: Vec<Track>, index: Option<usize>) -> PlayState {
        if tracks.len() > 0 {
            self.man
                .borrow_mut()
                .play(Some(&tracks[index.unwrap_or(0)]));
            self.tracks = tracks;
        } else {
            // if no new list is provided, we just unpause the song loaded in the manager
            self.man.borrow_mut().play(None);
        }
        PlayState {
            man: self.man,
            index: index.unwrap_or(0),
            tracks: self.tracks,
        }
    }

    pub fn stop(self) -> StopState {
        self.man.borrow_mut().stop();
        StopState { man: self.man }
    }
}

impl PlayState {
    pub fn pause(self) -> PauseState {
        self.man.borrow_mut().pause();
        PauseState {
            man: self.man,
            index: self.index,
            tracks: self.tracks,
        }
    }

    pub fn stop(self) -> StopState {
        self.man.borrow_mut().stop();
        StopState { man: self.man }
    }
}

#[cfg(test)]
mod test {

    use std::rc::Rc;
    use std::thread;
    use std::time::Duration;
    use std::{cell::RefCell, path::Path};

    use crate::{audio::manager::AudioManager, library::scan_assets, state::AppState};

    #[test]
    fn state_cycle_test() -> anyhow::Result<(), String> {
        // Audio manager init
        let manager = Rc::new(RefCell::new(AudioManager::new()));

        // path list
        let assets_dir = &format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
        let asset_path: &Path = Path::new(assets_dir);
        let songs = scan_assets(asset_path);
        assert!(songs.len() >= 2, "Need at least 2 songs for there tests");
        assert!(songs[0].duration >= Duration::from_secs(1));

        let state = AppState::new(manager);
        // manager should
        assert!(state.man.borrow().is_stopped());

        let state = state.play(songs.clone(), None);
        assert_eq!(state.index, 0);
        assert_eq!(state.tracks.len(), songs.len());
        assert!(state.man.borrow().is_playing());
        thread::sleep(Duration::from_millis(100));
        assert!(state.man.borrow().position() > Duration::from_millis(0));
        assert!(state.man.borrow().position() < Duration::from_millis(200));

        let state = state.pause();
        assert_eq!(state.index, 0);
        assert_eq!(state.tracks.len(), songs.len());
        assert!(state.man.borrow().is_paused());
        assert!(state.man.borrow().position() > Duration::from_millis(0));
        assert!(state.man.borrow().position() < Duration::from_millis(200));

        let state = state.play(Vec::new(), None);
        assert!(state.man.borrow().is_playing());
        assert!(state.man.borrow().position() > Duration::from_millis(0));
        assert!(state.man.borrow().position() < Duration::from_millis(200));

        let state = state.stop();
        // giving time for the player to flush the mixer
        thread::sleep(Duration::from_millis(100));
        assert!(state.man.borrow().is_stopped());

        let state = state.play(songs.clone(), Some(1));
        assert_eq!(state.index, 1);
        thread::sleep(Duration::from_millis(300));
        assert!(state.man.borrow().position() > Duration::from_millis(200));
        assert!(state.man.borrow().position() < Duration::from_millis(400));

        let state = state.pause();
        assert!(state.man.borrow().position() > Duration::from_millis(200));
        assert!(state.man.borrow().position() < Duration::from_millis(400));
        assert_eq!(state.index, 1);

        let state = state.stop();
        // giving time for the player to flush the mixer
        thread::sleep(Duration::from_millis(100));
        assert!(state.man.borrow().is_stopped());
        Ok(())
    }
}
