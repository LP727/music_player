//! # States
//!
//! This module provides API to track the current AppState.
//! Things like the current index, the list to track, volume, etc.

use std::{cell::RefCell, rc::Rc, time::Duration};

use crate::{audio::manager::AudioManager, library::Track};

struct AppState {
    // This allows us to share the audio manager and call play/pause/sop on it while others can
    // handle volume, next, ext.
    man: Rc<RefCell<AudioManager>>,
}

struct StopState {
    man: Rc<RefCell<AudioManager>>,
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
    pub fn play(mut self, tracks: Vec<Track>, index: Option<usize>) -> PlayState {
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
    pub fn pause(mut self) -> PauseState {
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
