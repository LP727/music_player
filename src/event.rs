#[derive(Debug, Clone)]
pub enum AppEvent {
    Command(Command),
    Playback(PlaybackEvent),
}

#[derive(Debug, Clone)]
pub enum Command {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    Quit,
    // ...
}

// "Playback" name to be reconsidered, might be audio, or something else
// These events could be produced by any other asynchrounonous process, like a db search
// download, scan, that this app MIGHT add in the future
#[derive(Debug, Clone)]
pub enum PlaybackEvent {
    TrackFinished,
    // ...
}
