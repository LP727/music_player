//! UI
//!
//! This module is where the Draw functions live
//! Currently designed to work with ratatui, might
//! change to a GUI in the future so needs to offer
//! an interface to easily swap between.

mod commands;

pub use commands::main_loop;
pub use commands::spawn_input_thread;
