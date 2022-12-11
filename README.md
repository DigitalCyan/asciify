# ASCIIFY

## About

ASCIIFY is a CLI program that converts videos into playable ASCII art.

## Dependencies

Make sure you have `ffmpeg` installed on your system and make sure it's available on your system globally.

## How to use

### Convert

```bash
user@machine:~$ asciify video.mp4 convert 30 5 video.asciivid
```

### Play

```bash
user@machine:~$ asciify video.asciivid play
```

## TODO

- Refactoring
- Add `stream` option.
- Better error handleing
