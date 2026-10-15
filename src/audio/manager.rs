//! Manager
//!
//! AudioManager, struct to handle the playback state and relevant data (Source, Since, Track)

use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::thread;
use std::{fs::File, time::Duration};

use log::debug;
use rodio::{Decoder, MixerDeviceSink, Player};

use crate::event::{AppEvent, PlaybackEvent};
use crate::library::Track;

pub struct AudioManager {
    player: Arc<Player>,
    // dropping would clear the queue, we need to keep this
    _device: MixerDeviceSink,
    // atomic counter to keep track of what is playing and what has ended for the thread
    append_count: Arc<AtomicU64>,
    event_channel: Sender<AppEvent>,
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
    pub fn new(event_channel: Sender<AppEvent>) -> Self {
        let sink = rodio::DeviceSinkBuilder::open_default_sink().expect("Failed to open sink");
        let player = Arc::<Player>::new(rodio::Player::connect_new(sink.mixer()));
        Self {
            player,
            // We need to hold the sink, it holds the audio thread, dropping kills the audio
            _device: sink,
            append_count: Arc::new(AtomicU64::new(0)),
            event_channel,
        }
    }

    /// Starts Playing a new song or resumes playing one, caller must be aware if something is
    /// loaded. Providing a new song clears the existing queue and loads that song.
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
        if let Some(t) = track {
            let file = File::open(&t.file_path).unwrap();
            let len = file.metadata().unwrap().len();

            // With seekable needed for the previous functionality to seek begining
            let sound_source = Decoder::builder()
                .with_data(BufReader::new(file))
                .with_byte_len(len)
                .with_seekable(true)
                .build()
                .unwrap();
            debug!(
                "Playing file at {}",
                &t.file_path.to_str().unwrap_or("bad path")
            );

            debug!(
                "Playing: {} from {}",
                track.clone().unwrap().artist,
                track.clone().unwrap().album
            );
            // bump the append count
            self.append_count.fetch_add(1, Ordering::AcqRel);
            // clear exisiting queue
            self.player.clear();
            // add new song
            self.player.append(sound_source);
        }

        debug!("Currently {} songs loaded", self.player.len());

        // This will have no effect if no source was previously loaded
        self.player.play();

        self.monitor_playing();
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

    pub fn is_paused(&self) -> bool {
        self.player.is_paused() && !self.player.empty()
    }

    pub fn is_playing(&self) -> bool {
        !self.player.is_paused() && !self.player.empty()
    }

    pub fn is_stopped(&self) -> bool {
        self.player.empty()
    }

    fn monitor_playing(&self) {
        let tx_channel = self.event_channel.clone();
        let player = Arc::clone(&self.player);
        let start_count = self.append_count.load(Ordering::Acquire);
        let append_count = Arc::clone(&self.append_count);
        thread::Builder::new()
            .name(String::from("Wait for song end"))
            .spawn(move || {
                player.sleep_until_end();

                // this allows to only send an event if the end event wasn't already event generated
                if start_count == append_count.load(Ordering::Acquire) {
                    // On quit, the rx will be gone first so failure here is accepted
                    let _ = tx_channel.send(AppEvent::Playback(PlaybackEvent::TrackFinished));
                }
            })
            .expect("Failed to start song end monitoring");
    }
}

#[cfg(test)]
mod test {

    use crate::{audio::manager::AudioManager, library::Track};
    use std::path::Path;
    use std::sync::mpsc;

    #[test]
    fn play_test() -> anyhow::Result<(), String> {
        let track = Track::from_path(Path::new(
            "assets/The Beatles - Revolution (Remastered 2009) [6MbqzDm1uCo].mp3",
        ))
        .expect("Failed to create the Track");

        let (tx, _rx) = mpsc::channel();
        let mut man = AudioManager::new(tx);
        man.play(Some(&track));
        return Ok(());
    }
}
