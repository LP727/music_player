//! # Controller
//!
//! This module contains the main control loop exposed to the main app.
//! It holds the state and the AudioManager and exposes commands

use crossterm::event::{Event, KeyCode, poll, read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::audio::manager::AudioManager;
use crate::state::AppState;

struct Controller {
    manager: Arc<Mutex<AudioManager>>,
}

impl Controller {
    pub fn new() -> Self {
        let manager = Arc::new(Mutex::new(AudioManager::new()));

        Controller { manager }
    }

    pub fn main_loop(&self) -> anyhow::Result<()> {
        let running = Arc::new(AtomicBool::new(true));
        let man = Arc::clone(&self.manager);

        let event_handler = thread::Builder::new()
            .name(String::from("Main event handler"))
            .spawn(move || -> anyhow::Result<()> {
                let mut current = AppState::new(man);
                while running.load(Ordering::Acquire) {
                    if poll(Duration::from_millis(100))? {
                        match read()? {
                            Event::FocusGained => println!("FocusGained"),
                            Event::FocusLost => println!("FocusLost"),
                            Event::Key(key) => {
                                println!("{:?}", key);
                                match key.code {
                                    KeyCode::Up => {
                                        current = match current {
                                            AppState::Stop(s) => {
                                                AppState::Play(s.play(Vec::new(), None))
                                            }
                                            AppState::Pause(p) => {
                                                AppState::Play(p.play(Vec::new(), None))
                                            }
                                            AppState::Play(p) => AppState::Play(p),
                                        };
                                    }
                                    KeyCode::Down => {
                                        current = match current {
                                            AppState::Stop(s) => AppState::Stop(s), // pause on stop has no effect
                                            AppState::Pause(p) => AppState::Pause(p),
                                            AppState::Play(p) => AppState::Pause(p.pause()),
                                        };
                                    }
                                    KeyCode::Left => {
                                        println!("Implement Previous!");
                                    }
                                    KeyCode::Right => {
                                        println!("Implement Next!");
                                    }
                                    KeyCode::Backspace => {
                                        current = match current {
                                            AppState::Stop(s) => AppState::Stop(s),
                                            AppState::Pause(p) => AppState::Stop(p.stop()),
                                            AppState::Play(p) => AppState::Stop(p.stop()),
                                        };
                                    }
                                    _ => {}
                                }
                            }
                            Event::Mouse(event) => println!("{:?}", event),
                            _ => {} // do nothing for other events
                        }
                    }
                }
                Ok(())
            });
        Ok(())
    }
}
