//! # Controller
//!
//! This module contains the main control loop exposed to the main app.
//! It holds the state and the AudioManager and exposes commands

use std::mem;
use std::sync::mpsc::Sender;
use std::time::Duration;

use crate::audio::manager::AudioManager;
use crate::event::{AppEvent, Command, PlaybackEvent};
use crate::library::Track;
// TODO: Implement how to pass a list to the controller, (play)
// use crate::library::Track;
use crate::state::{AppState, StopState};
use log::{debug, error};

// TODO: Move into settings
const RESTART_THRESHOLD: Duration = Duration::from_secs(3);

pub struct Controller {
    manager: AudioManager,
    state: AppState,
}

impl Controller {
    pub fn new(event_channel: Sender<AppEvent>) -> Self {
        let manager = AudioManager::new(event_channel);
        let state = AppState::new();
        Controller { manager, state }
    }

    pub fn handle_command(&mut self, cmd: &Command) {
        let current = mem::replace(&mut self.state, AppState::new());
        self.state = match (current, cmd) {
            (AppState::Stop(s), Command::Play) => {
                AppState::Play(s.play(&mut self.manager, Vec::new(), None))
            }
            (AppState::Pause(p), Command::Play) => {
                AppState::Play(p.play(&mut self.manager, Vec::new(), None))
            }
            (AppState::Play(p), Command::Pause) => AppState::Pause(p.pause(&self.manager)),

            (AppState::Play(p), Command::Stop) => AppState::Stop(p.stop(&self.manager)),

            (AppState::Pause(p), Command::Stop) => AppState::Stop(p.stop(&self.manager)),

            (AppState::Pause(mut p), Command::Next) => {
                if p.index < p.tracks.len() {
                    p.index += 1;
                    let nex_index = p.index;
                    // State calls play on unpause
                    AppState::Play(p.play(&mut self.manager, Vec::new(), Some(nex_index)))
                } else {
                    AppState::Stop(p.stop(&self.manager))
                }
            }
            (AppState::Play(mut p), Command::Next) => {
                if p.index < p.tracks.len() {
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

    pub fn load_songlist(&mut self, list: Vec<Track>) {
        // TODO: implement way to do this through command in handle command
        //
        let current = mem::replace(&mut self.state, AppState::new());
        self.state = match current {
            // TODO: ONLY WORKS FROM STOP FOR NOW MERGE IN THE COMMANDS HANDLING ABOVE
            AppState::Stop(s) => {
                let temp = s.play(&mut self.manager, list, None);
                AppState::Pause(temp.pause(&self.manager))
            }

            other => other,
        }
    }

    pub fn handle_playback_event(&mut self, evt: PlaybackEvent) {
        match evt {
            PlaybackEvent::TrackFinished => {
                self.handle_command(&Command::Next);
            }
        }
    }
}
