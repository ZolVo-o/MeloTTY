use std::fs;
use std::path::{Path, PathBuf};

use crate::playlist::is_audio_file;

use rand::Rng;

pub struct FileBrowser {
    current_dir: PathBuf,
    entries: Vec<PathBuf>,
    selected: usize,
    scroll: usize,
    bookmarks: Vec<PathBuf>,
}

impl FileBrowser {
    pub fn new(start_dir: PathBuf) -> Self {
        let mut bookmarks = Vec::new();
        
        // Добавляем стандартные закладки
        if let Ok(home) = std::env::var("HOME") {
            let home_path = PathBuf::from(&home);
            bookmarks.push(home_path.clone());
            
            // Проверяем популярные музыкальные папки
            for music_dir in &["Музыка", "Music", "Загрузки", "Downloads", "Documents"] {
                let path = home_path.join(music_dir);
                if path.exists() {
                    bookmarks.push(path);
                }
            }
        }
        
        // Добавляем корень для навигации по всей системе
        bookmarks.push(PathBuf::from("/"));
        
        let mut browser = FileBrowser {
            current_dir: start_dir,
            entries: Vec::new(),
            selected: 0,
            scroll: 0,
            bookmarks,
        };
        browser.refresh();
        browser
    }

    pub fn refresh(&mut self) {
        self.entries.clear();
        
        // Всегда показываем закладки в начале
        self.entries.push(PathBuf::from("─── ЗАКЛАДКИ ───"));
        self.entries.extend(self.bookmarks.clone());
        self.entries.push(PathBuf::from("─── СОДЕРЖИМОЕ ───"));
        
        // Добавляем родительскую папку
        if let Some(parent) = self.current_dir.parent() {
            self.entries.push(parent.to_path_buf());
        }

        // Читаем содержимое текущей папки
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();
                
                // Пропускаем скрытые папки
                if name.starts_with('.') {
                    continue;
                }
                
                if path.is_dir() {
                    dirs.push(path);
                } else if is_audio_file(&path) {
                    files.push(path);
                }
            }

            dirs.sort_by(|a, b| {
                a.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase()
                    .cmp(&b.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_lowercase())
            });
            
            files.sort_by(|a, b| {
                a.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase()
                    .cmp(&b.file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_lowercase())
            });
            
            self.entries.extend(dirs);
            self.entries.extend(files);
        }

        self.selected = 1; // Выбираем первую закладку
        self.scroll = 0;
    }

    pub fn search(&mut self, query: &str) {
        if query.is_empty() {
            return;
        }

        let query = query.to_lowercase();
        
        // Find first match after the bookmark section (index 2 = after "─── СОДЕРЖИМОЕ ───")
        for (i, path) in self.entries.iter().enumerate() {
            // Skip bookmark headers and bookmarks themselves
            if i <= 2 || self.bookmarks.contains(path) {
                continue;
            }
            
            let name = path.file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            
            if name.contains(&query) {
                self.selected = i;
                self.scroll = i.saturating_sub(7);
                return;
            }
        }
    }

    pub fn navigate_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll {
                self.scroll = self.selected;
            }
        }
    }

    pub fn navigate_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
            if self.selected >= self.scroll + 20 {
                self.scroll = self.selected - 19;
            }
        }
    }

    pub fn navigate_to_top(&mut self) {
        self.selected = 1; // First bookmark
        self.scroll = 0;
    }

    pub fn navigate_to_bottom(&mut self) {
        if !self.entries.is_empty() {
            self.selected = self.entries.len() - 1;
            let visible_height = 20;
            if self.selected >= visible_height {
                self.scroll = self.selected - visible_height + 1;
            }
        }
    }

    pub fn enter_directory(&mut self) {
        if let Some(path) = self.entries.get(self.selected).cloned() {
            // Проверяем, не закладка ли это
            if self.bookmarks.contains(&path) || path == PathBuf::from("/") {
                self.current_dir = path.clone();
                self.refresh();
                return;
            }
            
            // Проверяем родительскую папку
            if let Some(parent) = self.current_dir.parent() {
                if path == parent {
                    self.current_dir = parent.to_path_buf();
                    self.refresh();
                    return;
                }
            }
            
            // Обычная папка
            if path.is_dir() {
                self.current_dir = path;
                self.refresh();
            }
        }
    }

    pub fn go_back(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.refresh();
        }
    }

    pub fn current_dir(&self) -> &Path {
        &self.current_dir
    }

    pub fn entries(&self) -> &Vec<PathBuf> {
        &self.entries
    }

    pub fn selected(&self) -> usize {
        self.selected
    }

    pub fn scroll(&self) -> usize {
        self.scroll
    }

    pub fn selected_path(&self) -> Option<&PathBuf> {
        self.entries.get(self.selected)
    }
}
