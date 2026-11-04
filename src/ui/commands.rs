//! Commands
//! User facing commands, used prior to implementing the TUI

use std::{sync::mpsc, thread::JoinHandle};

use crate::controller::Controller;
use crate::event::{AppEvent, Command};
use crate::library::Library;

use crossterm::event::{Event, KeyCode, poll, read};
use log::debug;

use std::thread;
use std::time::Duration;

// TODO: Modify and re-insert into input_thread
/// Key to command translation until TUI is implemented
///
/// # Arguments
///
/// - `code` (`KeyCode`) - KeyCode
///
/// # Returns
///
/// - `Option<Command>` - Event to send mainloop
///
/// # Examples
///
/// ```
/// use crate::...;
///
///  Event::Key(key) => {
///      if let Some(cmd) = translate_key(key.code) {
///          tx.send(AppEvent::Command(cmd))?; // channel closed => return/break
///      }
///  }
/// ```
pub fn translate_key(code: KeyCode) -> Option<Command> {
    match code {
        KeyCode::Char('q') => Some(Command::Quit),
        KeyCode::Up => Some(Command::Play),
        KeyCode::Down => Some(Command::Pause),
        KeyCode::Left => Some(Command::Previous),
        KeyCode::Right => Some(Command::Next),
        KeyCode::Backspace => Some(Command::Stop),
        _ => None,
    }
}

// TODO: Implement Browsing
pub fn spawn_input_thread(tx: mpsc::Sender<AppEvent>) -> JoinHandle<anyhow::Result<()>> {
    let mut num_buf = String::new();
    thread::Builder::new()
        .name(String::from("Input Thread"))
        .spawn(move || -> anyhow::Result<()> {
            loop {
                if poll(Duration::from_millis(100))? {
                    match read()? {
                        // Focus & Mouse need crossterm::execute
                        Event::FocusGained => debug!("FocusGained"), // test
                        Event::FocusLost => debug!("FocusLost"),     // test
                        Event::Mouse(event) => debug!("{:?}", event), // test
                        Event::Key(key) => {
                            let cmd = match key.code {
                                // TODO: Make play/pause single button and use space
                                KeyCode::Up => Some(Command::Play),
                                KeyCode::Down => Some(Command::Pause),
                                KeyCode::Left => Some(Command::Previous),
                                KeyCode::Right => Some(Command::Next),
                                KeyCode::Char(' ') => Some(Command::Stop),
                                KeyCode::Char('q') => Some(Command::Quit),
                                KeyCode::Char(c) if c.is_ascii_digit() => {
                                    num_buf.push(c);
                                    None
                                }
                                KeyCode::Backspace => {
                                    num_buf.pop();
                                    None
                                }
                                KeyCode::Enter => {
                                    // Cheap way to select above 9, behaves like each selection is a
                                    // password, which is not ideal
                                    if let Ok(idx) = num_buf.parse::<usize>() {
                                        num_buf.clear();
                                        debug!("SelectedIndex:{}", &idx);
                                        Some(Command::SelectIndex(idx))
                                    } else {
                                        num_buf.clear();
                                        None
                                    }
                                }
                                _ => None,
                            };

                            if let Some(c) = cmd {
                                debug!("Sending command{:?}", c);
                                tx.send(AppEvent::Command(c.clone()))?;
                                if c == Command::Quit {
                                    return Ok(());
                                }
                            }
                        }
                        _ => {} // do nothing for other events
                    }
                }
            }
        })
        .expect("Failed to spawn command thread")
}

pub fn main_loop<T: Library + Send + 'static>(
    lib: T,
    tx: mpsc::Sender<AppEvent>,
    rx: mpsc::Receiver<AppEvent>,
) -> JoinHandle<()> {
    thread::Builder::new()
        .name(String::from("Event processing thread"))
        .spawn(move || {
            let song_lib = lib;
            let mut controller = Controller::new(tx.clone(), &song_lib);
            controller.load_songlist(song_lib.all_tracks());

            for evt in rx {
                match evt {
                    AppEvent::Command(Command::SelectIndex(idx)) => {
                        debug!("Processing select index: {:?}", idx);
                        controller.handle_browse_event(idx);
                    }
                    AppEvent::Command(cmd) => {
                        debug!("Processing user command: {:?}", cmd);
                        controller.handle_command(&cmd);
                        if cmd == Command::Quit {
                            return ();
                        }
                    }
                    AppEvent::Playback(evt) => {
                        debug!("Processing playback event: {:?}", evt);
                        controller.handle_playback_event(evt);
                    }
                }
            }
        })
        .expect("Failed to start event processing thread")
}
