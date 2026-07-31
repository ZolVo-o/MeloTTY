use std::path::PathBuf;
use std::time::{Duration, Instant};

use crate::audio::AudioEngine;
use crate::browser::FileBrowser;
use crate::error::Result;
use crate::playlist::{Playlist, RepeatMode};

#[derive(PartialEq)]
pub enum Mode {
    Browser,
    Playlist,
    Search,
}

pub struct App {
    pub browser: FileBrowser,
    pub playlist: Playlist,
    pub audio: AudioEngine,
    pub mode: Mode,
    pub is_playing: bool,
    pub track_start: Option<Instant>,
    pub track_duration: Option<Duration>,
    pub should_quit: bool,
    pub search_query: String,
    pub spectrum: Vec<f32>,
    playlist_path: PathBuf,
}

impl App {
    pub fn new(start_path: Option<String>) -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let playlist_path = PathBuf::from(&home).join(".termusic_playlist.m3u");

        // Всегда начинаем с домашней папки если не указан путь
        let start_dir = if let Some(p) = start_path {
            let path = PathBuf::from(p);
            if path.exists() {
                if path.is_dir() {
                    path
                } else {
                    path.parent().unwrap_or(std::path::Path::new(&home)).to_path_buf()
                }
            } else {
                PathBuf::from(&home)
            }
        } else {
            PathBuf::from(&home)
        };

        let mut app = App {
            browser: FileBrowser::new(start_dir),
            playlist: Playlist::new(),
            audio: AudioEngine::new()?,
            mode: Mode::Browser,
            is_playing: false,
            track_start: None,
            track_duration: None,
            should_quit: false,
            search_query: String::new(),
            spectrum: vec![0.0; 32],
            playlist_path,
        };

        app.load_playlist();
        Ok(app)
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.audio.set_volume(volume);
    }

    pub fn set_volume_preset(&mut self, preset: f32) {
        self.audio.set_volume(preset);
    }

    pub fn save_playlist(&self) {
        self.playlist.save_to_file(&self.playlist_path).ok();
    }

    fn load_playlist(&mut self) {
        self.playlist.load_from_file(&self.playlist_path).ok();
    }

    pub fn toggle_mode(&mut self) {
        if self.mode == Mode::Search {
            self.mode = Mode::Browser;
            return;
        }
        self.mode = match self.mode {
            Mode::Browser => Mode::Playlist,
            Mode::Playlist => Mode::Browser,
            _ => Mode::Browser,
        };
    }

    pub fn start_search(&mut self) {
        self.search_query.clear();
        self.mode = Mode::Search;
    }

    pub fn search_add_char(&mut self, c: char) {
        self.search_query.push(c);
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
    }

    pub fn search_confirm(&mut self) {
        let query = self.search_query.clone();
        self.mode = Mode::Browser;
        self.browser.search(&query);
    }

    pub fn cancel_search(&mut self) {
        self.search_query.clear();
        self.mode = Mode::Browser;
    }

    pub fn add_selected_to_playlist(&mut self) {
        if let Some(path) = self.browser.selected_path().cloned() {
            if path.is_dir() {
                self.playlist.add_directory(&path).ok();
            } else {
                self.playlist.add_track(path);
            }
        }
    }

    pub fn play_selected(&mut self) -> Result<()> {
        if let Some(path) = self.browser.selected_path() {
            if !path.is_dir() {
                self.playlist.add_track(path.clone());
                let idx = self.playlist.tracks().iter().position(|p| p == path).unwrap_or(0);
                self.playlist.select(idx);
                return self.play_track();
            }
        }
        Ok(())
    }

    pub fn play_track(&mut self) -> Result<()> {
        if let Some(path) = self.playlist.current_track() {
            let duration = self.audio.play_file(path)?;
            self.is_playing = true;
            self.track_start = Some(Instant::now());
            self.track_duration = duration;
        }
        Ok(())
    }

    pub fn toggle_playback(&mut self) -> Result<()> {
        if !self.audio.has_sink() {
            return self.play_track();
        }

        if self.is_playing {
            self.audio.pause();
            self.is_playing = false;
        } else {
            self.audio.resume();
            self.is_playing = true;
            self.track_start = Some(Instant::now());
        }
        Ok(())
    }

    pub fn next_track(&mut self) -> Result<()> {
        if self.playlist.is_empty() {
            return Ok(());
        }

        if let Some(_) = self.playlist.next() {
            self.play_track()?;
        } else {
            // End of playlist (no repeat all)
            self.is_playing = false;
            self.audio.stop();
            self.track_start = None;
        }
        Ok(())
    }

    pub fn prev_track(&mut self) -> Result<()> {
        if self.playlist.is_empty() {
            return Ok(());
        }

        // If more than 3 seconds into the track, restart it
        if let (Some(start), Some(_)) = (self.track_start, self.track_duration) {
            if start.elapsed().as_secs() > 3 {
                self.track_start = Some(Instant::now());
                self.audio.stop();
                return self.play_track();
            }
        }

        if let Some(_) = self.playlist.previous() {
            self.play_track()?;
        }
        Ok(())
    }

    pub fn remove_current_track(&mut self) {
        let idx = self.playlist.current_index();
        self.playlist.remove_track(idx);
    }

    pub fn clear_playlist(&mut self) {
        self.audio.stop();
        self.is_playing = false;
        self.playlist.clear();
    }

    pub fn toggle_shuffle(&mut self) {
        self.playlist.shuffle = !self.playlist.shuffle;
        if self.playlist.shuffle {
            self.playlist.shuffle_playlist();
        } else {
            self.playlist.sort_playlist();
        }
    }

    pub fn cycle_repeat(&mut self) {
        self.playlist.repeat = match self.playlist.repeat {
            RepeatMode::Off => RepeatMode::All,
            RepeatMode::All => RepeatMode::One,
            RepeatMode::One => RepeatMode::Off,
        };
    }

    pub fn volume_up(&mut self) {
        let new_vol = (self.audio.volume() + 0.05).min(1.0);
        self.audio.set_volume(new_vol);
    }

    pub fn volume_down(&mut self) {
        let new_vol = (self.audio.volume() - 0.05).max(0.0);
        self.audio.set_volume(new_vol);
    }

    pub fn update(&mut self) {
        if self.is_playing {
            if let (Some(start), Some(duration)) = (self.track_start, self.track_duration) {
                if start.elapsed() >= duration {
                    if self.playlist.repeat == RepeatMode::One {
                        self.play_track().ok();
                    } else {
                        self.next_track().ok();
                    }
                }
            }
        }
    }
}
