//! # Controller
//!
//! This module contains the main control loop exposed to the main app.
//! It holds the ui state, app state and the AudioManager and exposes commands

use std::mem;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::audio::manager::AudioManager;
use crate::event::{AppEvent, Command, PlaybackEvent};
use crate::library::{Library, Track};
use crate::state::AppState;
use crate::ui::ui_state::{Choice, Footer, UIState, UiDisplay};
use log::{debug, error};

// TODO: Move into settings
const RESTART_THRESHOLD: Duration = Duration::from_secs(3);

pub struct Controller<'a, T: Library> {
    manager: AudioManager,
    app_state: AppState,
    ui_state: UIState,
    library: &'a T,
}

impl<'a, T: Library> Controller<'a, T> {
    // TODO: replace with Arc to have dynamically reloading library
    pub fn new(event_channel: Sender<AppEvent>, lib: &'a T) -> Self {
        let manager = AudioManager::new(event_channel);
        let app_state = AppState::new();
        let ui_state = UIState::new();
        // First display
        ui_state.ui_display();
        Controller {
            manager,
            app_state,
            ui_state,
            library: lib,
        }
    }

    pub fn handle_command(&mut self, cmd: &Command) {
        let current = mem::replace(&mut self.app_state, AppState::new());
        self.app_state = match (current, cmd) {
            (AppState::Stop(s), Command::PlayPause) => {
                // Hitting play on a stoped state ignores the transition
                AppState::Stop(s)
            }
            (AppState::Pause(p), Command::PlayPause) => {
                AppState::Play(p.play(&mut self.manager, Vec::new(), None))
            }
            (AppState::Play(p), Command::PlayPause) => AppState::Pause(p.pause(&self.manager)),
            (AppState::Play(p), Command::Stop) => AppState::Stop(p.stop(&self.manager)),
            (AppState::Pause(p), Command::Stop) => AppState::Stop(p.stop(&self.manager)),
            (AppState::Pause(mut p), Command::Next) => {
                if p.tracks.len() > 0 && p.index < p.tracks.len() - 1 {
                    p.index += 1;
                    let nex_index = p.index;
                    // State calls play on unpause
                    AppState::Play(p.play(&mut self.manager, Vec::new(), Some(nex_index)))
                } else {
                    AppState::Stop(p.stop(&self.manager))
                }
            }
            (AppState::Play(mut p), Command::Next) => {
                if p.tracks.len() > 0 && p.index < p.tracks.len() - 1 {
                    p.index += 1;
                    self.manager.play(Some(&p.tracks[p.index]));
                    AppState::Play(p)
                } else {
                    AppState::Stop(p.stop(&self.manager))
                }
            }
            (AppState::Pause(mut p), Command::Previous) => {
                if self.manager.position() > RESTART_THRESHOLD {
                    match self.manager.seek(Duration::from_secs(0)) {
                        Ok(_) => debug!("Seek successful"),
                        Err(e) => error!("Encounterer error : {}", e),
                    }
                    AppState::Play(p.play(&mut self.manager, Vec::new(), None))
                } else {
                    if p.index > 0 {
                        p.index -= 1;
                        let nex_index = p.index;
                        // State calls play on unpause
                        AppState::Play(p.play(&mut self.manager, Vec::new(), Some(nex_index)))
                    } else {
                        AppState::Stop(p.stop(&self.manager))
                    }
                }
            }
            (AppState::Play(mut p), Command::Previous) => {
                if self.manager.position() > RESTART_THRESHOLD {
                    match self.manager.seek(Duration::from_secs(0)) {
                        Ok(_) => debug!("Seek successful"),
                        Err(e) => error!("Encounterer error : {}", e),
                    }
                    AppState::Play(p)
                } else {
                    if p.index > 0 {
                        p.index -= 1;
                        // State calls play on unpause
                        self.manager.play(Some(&p.tracks[p.index]));
                        AppState::Play(p)
                    } else {
                        AppState::Stop(p.stop(&self.manager))
                    }
                }
            }
            (other, Command::Quit) => {
                // stop playback here, terminates the thread, which will drop the channel tx clone
                self.manager.stop();
                other
            }
            (other, _) => other,
        }
    }

    pub fn handle_playback_event(&mut self, evt: PlaybackEvent) {
        match evt {
            PlaybackEvent::TrackFinished => {
                self.handle_command(&Command::Next);
            }
        }
    }

    pub fn handle_browse_event(&mut self, idx: usize) {
        let choice = self.ui_state.resolve(idx);
        let current = mem::replace(&mut self.ui_state, UIState::new());
        self.ui_state = match (current, choice) {
            (_, Some(Choice::Footer(Footer::MENU))) => UIState::new(),
            (state, Some(Choice::Footer(Footer::BACK))) => {
                match state {
                    UIState::AlbumSongs(s) => {
                        UIState::AllAlbum(s.back_to_all_album(self.library))
                        // TODO: Add way to figure out back to all album vs back to menu
                    }
                    UIState::ArtistSongs(s) => UIState::AllArtists(s.back(self.library)),
                    UIState::ArtistAlbums(s) => UIState::AllArtists(s.back(self.library)),
                    _ => state,
                }
            }
            (state, Some(Choice::All)) => match state {
                UIState::AllSongs(us) => {
                    self.play_helper(us.get_all(), None);
                    UIState::AllSongs(us)
                }
                UIState::ArtistSongs(us) => {
                    self.play_helper(us.get_all(), None);
                    UIState::ArtistSongs(us)
                }
                UIState::AlbumSongs(us) => {
                    self.play_helper(us.get_all(), None);
                    UIState::AlbumSongs(us)
                }
                UIState::ArtistAlbums(us) => UIState::ArtistSongs(us.get_all(self.library)),
                _ => state,
            },
            (state, Some(Choice::Item(idx))) => match state {
                UIState::AllSongs(us) => {
                    self.play_helper(us.get_all(), Some(idx));
                    UIState::AllSongs(us)
                }
                UIState::AlbumSongs(us) => {
                    self.play_helper(us.get_all(), Some(idx));
                    UIState::AlbumSongs(us)
                }
                UIState::ArtistSongs(us) => {
                    self.play_helper(us.get_all(), Some(idx));
                    UIState::ArtistSongs(us)
                }
                UIState::ArtistAlbums(us) => UIState::AlbumSongs(us.select(idx, self.library)),
                UIState::AllAlbum(us) => UIState::AlbumSongs(us.select(idx, self.library)),
                UIState::AllArtists(us) => UIState::ArtistAlbums(us.select(idx, self.library)),
                UIState::BrowseMenu(us) => {
                    // Browse Menu's choices are always: "All songs", "Artists", "Albums"
                    match idx {
                        0 => UIState::AllSongs(us.all_songs(self.library)),
                        1 => UIState::AllArtists(us.artists(self.library)),
                        2 => UIState::AllAlbum(us.albums(self.library)),
                        _ => UIState::new(),
                    }
                }
            },
            /*
               Ignored Choices:
               Any state, None: No Choice
               "State without all(Browse Menu, etc)", all
            */
            (other, _) => other,
        };

        // UPDATE DISPLAY AFTER CHOICE
        self.ui_state.ui_display();
    }

    // HELPERS
    fn play_helper(&mut self, track_list: Vec<Track>, idx: Option<usize>) {
        let appstate = mem::replace(&mut self.app_state, AppState::new());
        self.app_state = appstate.play(&mut self.manager, track_list, idx);
    }
}
