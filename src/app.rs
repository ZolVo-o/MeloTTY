use std::path::PathBuf;
use std::time::Duration;

use crate::audio::AudioEngine;
use crate::browser::FileBrowser;
use crate::config::Config;
use crate::cover;
use crate::error::Result;
use crate::playlist::{Playlist, RepeatMode};

#[derive(PartialEq)]
pub enum Mode {
    Browser,
    Playlist,
    Search,
    Settings,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Library,
    Queue,
    NowPlaying,
    Settings,
}

pub struct App {
    pub browser: FileBrowser,
    pub playlist: Playlist,
    pub audio: AudioEngine,
    pub mode: Mode,
    pub page: Page,
    pub is_playing: bool,
    pub track_duration: Option<Duration>,
    pub should_quit: bool,
    pub search_query: String,
    pub spectrum: Vec<f32>,
    pub cover_ascii: Option<Vec<(String, Vec<ratatui::style::Color>)>>,
    pub config: Config,
    playlist_path: PathBuf,
}

impl App {
    pub fn new(start_path: Option<String>, config: Config) -> Result<Self> {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let playlist_path = PathBuf::from(&home).join(".melotty_playlist.m3u");

        let start_dir = if let Some(p) = start_path {
            let path = PathBuf::from(p);
            if path.exists() {
                if path.is_dir() {
                    path
                } else {
                    path.parent()
                        .unwrap_or(std::path::Path::new(&home))
                        .to_path_buf()
                }
            } else {
                PathBuf::from(&home)
            }
        } else {
            PathBuf::from(&home)
        };

        let mut app = App {
            browser: FileBrowser::new(start_dir, config.show_hidden_files),
            playlist: Playlist::new(),
            audio: AudioEngine::new()?,
            mode: Mode::Browser,
            page: Page::Library,
            is_playing: false,
            track_duration: None,
            should_quit: false,
            search_query: String::new(),
            spectrum: vec![0.0; 32],
            cover_ascii: None,
            config,
            playlist_path,
        };

        app.load_playlist();
        Ok(app)
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.audio.set_volume(volume);
        self.config.default_volume = self.audio.volume();
        self.config.save();
    }

    pub fn set_volume_preset(&mut self, preset: f32) {
        self.set_volume(preset);
    }

    pub fn save_playlist(&self) -> Result<()> {
        self.playlist.save_to_file(&self.playlist_path)?;
        Ok(())
    }

    fn load_playlist(&mut self) {
        self.playlist.load_from_file(&self.playlist_path).ok();
    }

    fn load_cover(&mut self) {
        self.cover_ascii = None;
        if let Some(path) = self.playlist.current_track() {
            if let Ok(Some(img)) = cover::extract_cover(path) {
                self.cover_ascii = Some(crate::ascii_art::image_to_ascii(&img, 20, 6));
            }
        }
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

    pub fn open_settings(&mut self) {
        self.page = Page::Settings;
        self.mode = Mode::Settings;
    }

    pub fn open_page(&mut self, page: Page) {
        self.page = page;
        self.mode = match page {
            Page::Library => Mode::Browser,
            Page::Queue => Mode::Playlist,
            Page::NowPlaying => Mode::Browser,
            Page::Settings => Mode::Settings,
        };
    }

    pub fn toggle_hidden_files(&mut self) {
        self.browser.toggle_hidden_files();
        self.config.show_hidden_files = self.browser.hidden_files_enabled();
        self.config.save();
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
                let idx = self
                    .playlist
                    .tracks()
                    .iter()
                    .position(|p| p == path)
                    .unwrap_or(0);
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
            self.track_duration = duration;
            self.load_cover();
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
            self.is_playing = false;
            self.audio.stop();
        }
        Ok(())
    }

    pub fn prev_track(&mut self) -> Result<()> {
        if self.playlist.is_empty() {
            return Ok(());
        }

        if let Some(_) = self.playlist.previous() {
            self.play_track()?;
        }
        Ok(())
    }

    pub fn remove_current_track(&mut self) {
        let idx = self.playlist.current_index();
        let was_playing = self.is_playing || self.audio.has_sink();
        self.playlist.remove_track(idx);

        if was_playing {
            self.audio.stop();
            self.is_playing = false;
            self.track_duration = None;
            self.cover_ascii = None;
        }
    }

    pub fn clear_playlist(&mut self) {
        self.audio.stop();
        self.is_playing = false;
        self.track_duration = None;
        self.playlist.clear();
    }

    pub fn toggle_shuffle(&mut self) {
        self.playlist.shuffle = !self.playlist.shuffle;
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
            if let Some(duration) = self.track_duration {
                if self.audio.position() >= duration {
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
