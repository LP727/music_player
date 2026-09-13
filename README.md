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

## Running

## Usage