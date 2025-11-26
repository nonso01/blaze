# BLAZE

A fast, native, Spotify-inspired music player built in Rust for Linux.

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat&logo=linux&logoColor=black)
![egui](https://img.shields.io/badge/GUI-egui-brightgreen)

### Features (in progress)
- Pure Rust audio playback (rodio + symphonia)  
- Embedded album art & metadata (lofty)  
- Smooth dark UI with GPU rendering (egui + eframe)  
- Background library scanning  
- Desktop notifications & media-key support  
- Zero Electron, zero bloat

### Tech stack
```rs
egui = "0.33.2"
eframe = "0.33.2"
rodio = "0.21.1"
symphonia = "0.5.5"
lofty = "0.22.4"
image = "0.25.9"
walkdir = "2.5.0"
poll-promise = "0.3.0"
zbus = "5.12.0"
notify-rust = "4.11.7"