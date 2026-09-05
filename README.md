# MUSIC PLAYER

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