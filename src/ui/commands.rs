//! Commands
//! User facing commands, used prior to implementing the TUI

use std::{sync::mpsc, thread::JoinHandle};

use crate::controller::Controller;
use crate::event::{AppEvent, Command};
use crossterm::event::{Event, KeyCode, poll, read};
use std::thread;
use std::time::Duration;

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
        KeyCode::Esc => Some(Command::Quit),
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
    thread::Builder::new()
        .name(String::from("Command processing thread"))
        .spawn(move || -> anyhow::Result<()> {
            loop {
                if poll(Duration::from_millis(100))? {
                    match read()? {
                        // Focus & Mouse need crossterm::execute
                        Event::FocusGained => println!("FocusGained"), // test
                        Event::FocusLost => println!("FocusLost"),     // test
                        Event::Mouse(event) => println!("{:?}", event), // test
                        Event::Key(key) => {
                            if let Some(cmd) = translate_key(key.code) {
                                println!("Sending command{:?}", cmd);
                                tx.send(AppEvent::Command(cmd))?; // channel closed => return/break
                            }
                        }
                        // TODO: Implement quiting here
                        _ => {} // do nothing for other events
                    }
                }
            }
        })
        .expect("Failed to spawn command thread")
}

pub fn main_loop(mut ctrl: Controller, rx: mpsc::Receiver<AppEvent>) -> JoinHandle<()> {
    thread::Builder::new()
        .name(String::from("Event processing thread"))
        .spawn(move || {
            for evt in rx {
                match evt {
                    AppEvent::Command(cmd) => {
                        println!("Processing user command: {:?}", cmd);
                        ctrl.handle_command(cmd);
                    }
                    AppEvent::Playback(evt) => {
                        println!("Processing playback event: {:?}", evt);
                        ctrl.handle_playback_event(evt);
                    }
                }
            }
        })
        .expect("Failed to start event processing thread")
}
