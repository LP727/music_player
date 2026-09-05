//! Main
//!
//! Main module that takes in arguments, initializes various modules
//! Start the app and handles the teardown.

use std::println;

use anyhow::Ok;
use lofty::tag::Accessor;
use lofty::{file::TaggedFileExt, read_from_path};
use music_player::library::scan_assets;

fn main() -> anyhow::Result<()> {
    // TODO: Scan for music
    let songs_path = scan_assets();
    for song in songs_path {
        println!("{}", song.to_str().unwrap());

        let taggedfile = read_from_path(song);
        println!("Tag types:");
        for tag in taggedfile.unwrap().tags() {
            println!("{:?}", tag.tag_type());
            println!("{:?}", tag.title());
            println!("{:?}", tag.artist());
            for item in tag.items() {
                println!("{:?} = {:?}", item.key(), item.value());
            }
        }
    }

    // TODO: Set state

    // TODO: Start UI

    Ok(())
}
