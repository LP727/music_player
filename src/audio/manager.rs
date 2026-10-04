//! Manager
//!
//! AudioManager, struct to handle the playback state and relevant data (Source, Since, Track)

use std::io::BufReader;
use std::{fs::File, time::Duration};

use crate::library::Track;
use rodio::{Decoder, Player};

pub struct AudioManager {
    player: Player,
}

/// Describe this function.
///
/// # Returns
///
/// - `Self` - Describe the return value.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let man = AudioManager::new();
/// man.play(track);
/// ```
impl AudioManager {
    pub fn new() -> Self {
        let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open sink");
        let player = rodio::Player::connect_new(sink.mixer());
        Self { player }
    }

    /// Starts Playing a new song or resumes playing one, caller must be aware if something is
    /// loaded
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`undefined`) - AudioManager
    /// - `track` (`Option<Track>`) - Song to start playing if playing a new song
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = play();
    /// ```
    pub fn play(&mut self, track: Option<&Track>) {
        if !track.is_none() {
            let file = BufReader::new(File::open(track.unwrap().file_path.clone()).unwrap());
            let sound_source = Decoder::new(file).unwrap();

            println!(
                "Playing: {} from {}",
                track.clone().unwrap().artist,
                track.clone().unwrap().album
            );

            self.player.append(sound_source);
        }

        // This will have no effect if no source was previously loaded
        self.player.play();
        self.player.sleep_until_end();
        return;
    }

    pub fn pause(&self) {
        // no effect if no track is playing
        self.player.pause();
    }

    /// Stop the audio and empty the player's queue, will need to be provided a source to start
    /// # Arguments
    ///
    /// - `&self` (`undefined`) - Self
    ///
    /// # Examples
    /// TODO: Fill with AI
    /// ```
    /// use crate::...;
    ///
    /// let _ = stop();
    /// ```
    pub fn stop(&self) {
        // stop the audio and empty the player's queue, will need to be provided a source to start
        // playing again
        self.player.stop();
    }

    pub fn position(&self) -> Duration {
        self.player.get_pos()
    }

    pub fn seek(&self, pos: Duration) -> Result<(), rodio::source::SeekError> {
        self.player.try_seek(pos)
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
        man.play(Some(&track));
        return Ok(());
    }
}
