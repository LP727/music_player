//! Manager
//!
//! AudioManager, struct to handle the playback state and relevant data (Source, Since, Track)

use std::fs::File;
use std::io::BufReader;

use crate::library::Track;
use rodio::{Decoder, MixerDeviceSink, Player};

pub struct AudioManager {
    // TODO: Assess if needed
    sink: MixerDeviceSink,
    player: Player,
    current_track: Option<Track>,
}

impl AudioManager {
    pub fn new() -> Self {
        let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open sink");
        let player = rodio::Player::connect_new(sink.mixer());
        Self {
            sink,
            player,
            current_track: None,
        }
    }

    pub fn play(&mut self, track: Option<Track>) {
        if self.current_track.is_none() && track.is_none() {
            return;
        }

        if self.current_track != track && track != None {
            self.current_track = track;
        }

        let file =
            BufReader::new(File::open(self.current_track.clone().unwrap().file_path).unwrap());
        let sound_source = Decoder::new(file).unwrap();

        println!(
            "Playing: {} from {}",
            self.current_track.clone().unwrap().artist,
            self.current_track.clone().unwrap().album
        );

        self.player.append(sound_source);
        // self.player.play(); might be necessary?

        self.player.sleep_until_end();
    }

    pub fn pause(&self) {
        match &self.current_track {
            Some(_) => {
                self.player.pause();
            }
            None => return,
        }
    }
}

mod test {

    use crate::{audio::manager::AudioManager, library::Track};
    use std::path::Path;

    #[test]
    fn play_test() -> anyhow::Result<(), String> {
        let track = Track::from_path(Path::new(
            "assets/The Beatles - Revolution (Remastered 2009) [6MbqzDm1uCo].mp3",
        ))
        .expect("Failed to create the Track");

        let mut man = AudioManager::new();
        man.play(Some(track));
        return Ok(());
    }
}
