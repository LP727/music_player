# MUSIC PLAYER

## Description

Small Music player app project to learn the basics of Rust. 

Work in Progress.

## Goal

Have a local music player that:
- Works offline
- Doesn't expose me to distraction when I just want to listen music while working (too many work sessions got derailed by youtube)
- Doesn't expose me to adds (unlike spotify)
- Doesn't expose me to a broken autoplay (unlike youtube using whatever add blocking)
- Is feature light
- Is free

## Getting music

Getting music is not integrated in the app yet (we will see about that).

For now, gettingt the music relies on yt-dlp, to install, follow instructions here:
https://github.com/yt-dlp/yt-dlp/wiki/Installation

Note that the binary can be updated using:

```bash
yt-dlp -U
```

The command to obtain the music in a format that has some tagged metadata is:

```bash
yt-dlp -x --audio-format mp3 --audio-quality 0   --embed-metadata --embed-thumbnail   --parse-metadata "title:%(artist)s - %(title)s"   -o "%(artist)s - %(title)s [%(id)s].%(ext)s"   "INSERT_URL_HERE"
```

## Building
Prior to building, set the RUST_LOG environment variable to tune the log level of the app.
By default, the log level will be "error".

```bash
# Builds with RUST_LOG=error by default, here we build in debug
export RUST_LOG=debug
cargo build
```


## Running
Running currently only puts the app on Pause for a whole library play.
```bash
# Builds with RUST_LOG=error by default, here we build in info to get some playing feedback
export RUST_LOG=info
cargo run
```

## Usage
Currently featuring a "Pseudo TUI"
Loads all the songs in the user Music folder and puts itself in pause.

### Commands
- UpArrowKey: _Play_
- DownArrowKey: _Pause_
- LeftArrowKey: _Previous_
- RightArrowKey: _Next_
- 'q': _Quit_
- Backspace: _Stop_
  
Note: As there are no browsing or loading implemented, _Stop_ puts the app in a unuseable position