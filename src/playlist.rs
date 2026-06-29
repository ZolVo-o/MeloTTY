use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct Playlist {
    tracks: Vec<PathBuf>,
    current_index: usize,
    pub shuffle: bool,
    pub repeat: RepeatMode,
}

#[derive(Clone, PartialEq)]
pub enum RepeatMode {
    Off,
    One,
    All,
}

impl Playlist {
    pub fn new() -> Self {
        Playlist {
            tracks: Vec::new(),
            current_index: 0,
            shuffle: false,
            repeat: RepeatMode::Off,
        }
    }

    pub fn add_track(&mut self, path: PathBuf) {
        if !self.tracks.contains(&path) {
            self.tracks.push(path);
        }
    }

    pub fn add_directory(&mut self, dir: &Path) -> io::Result<()> {
        if !dir.is_dir() {
            return Ok(());
        }

        let mut paths: Vec<PathBuf> = Vec::new();
        self.collect_audio_files(dir, &mut paths)?;
        paths.sort();

        for path in paths {
            if !self.tracks.contains(&path) {
                self.tracks.push(path);
            }
        }
        Ok(())
    }

    fn collect_audio_files(&self, dir: &Path, paths: &mut Vec<PathBuf>) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.collect_audio_files(&path, paths)?;
            } else if is_audio_file(&path) {
                paths.push(path);
            }
        }
        Ok(())
    }

    pub fn remove_track(&mut self, index: usize) {
        if index < self.tracks.len() {
            self.tracks.remove(index);
            if self.current_index >= self.tracks.len() && !self.tracks.is_empty() {
                self.current_index = self.tracks.len() - 1;
            }
        }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
        self.current_index = 0;
    }

    pub fn next(&mut self) -> Option<usize> {
        if self.tracks.is_empty() {
            return None;
        }

        if self.shuffle {
            use std::time::{SystemTime, UNIX_EPOCH};
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos();
            self.current_index = (nanos as usize) % self.tracks.len();
        } else {
            self.current_index += 1;
            if self.current_index >= self.tracks.len() {
                self.current_index = if self.repeat == RepeatMode::All {
                    0
                } else {
                    self.tracks.len() - 1
                };
            }
        }

        Some(self.current_index)
    }

    pub fn previous(&mut self) -> Option<usize> {
        if self.tracks.is_empty() {
            return None;
        }

        if self.current_index == 0 {
            self.current_index = self.tracks.len() - 1;
        } else {
            self.current_index -= 1;
        }

        Some(self.current_index)
    }

    pub fn select(&mut self, index: usize) -> Option<usize> {
        if index < self.tracks.len() {
            self.current_index = index;
            Some(index)
        } else {
            None
        }
    }

    pub fn current_track(&self) -> Option<&PathBuf> {
        self.tracks.get(self.current_index)
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn tracks(&self) -> &Vec<PathBuf> {
        &self.tracks
    }

    pub fn len(&self) -> usize {
        self.tracks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tracks.is_empty()
    }

    pub fn save_to_file(&self, path: &Path) -> io::Result<()> {
        let mut file = File::create(path)?;
        for track in &self.tracks {
            writeln!(file, "{}", track.display())?;
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, path: &Path) -> io::Result<()> {
        if !path.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(path)?;
        self.tracks.clear();
        
        for line in content.lines() {
            let track_path = PathBuf::from(line);
            if track_path.exists() && is_audio_file(&track_path) {
                self.tracks.push(track_path);
            }
        }
        
        Ok(())
    }
}

pub fn is_audio_file(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "mp3" | "wav" | "flac" | "ogg" | "aac" | "m4a" | "opus" | "wma"
        ),
        None => false,
    }
}
