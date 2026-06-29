use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub start_dir: Option<String>,
    pub default_volume: f32,
    pub show_hidden_files: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            start_dir: None,
            default_volume: 0.7,
            show_hidden_files: false,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = get_config_path();
        
        if !config_path.exists() {
            let config = Config::default();
            config.save();
            return config;
        }

        let content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => return Config::default(),
        };

        let mut config = Config::default();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim();

            match key {
                "start_dir" => config.start_dir = Some(value.to_string()),
                "default_volume" => {
                    if let Ok(vol) = value.parse::<f32>() {
                        config.default_volume = vol.clamp(0.0, 1.0);
                    }
                }
                "show_hidden_files" => {
                    config.show_hidden_files = value == "true" || value == "1";
                }
                _ => {}
            }
        }

        config
    }

    pub fn save(&self) {
        let config_path = get_config_path();
        
        let content = format!(
            r#"# termusic configuration file
# Редактируйте этот файл для настройки плеера

# Начальная директория при запуске (оставьте пустым для автоопределения)
start_dir = {}

# Громкость по умолчанию (от 0.0 до 1.0)
default_volume = {}

# Показывать скрытые файлы и папки
show_hidden_files = {}
"#,
            self.start_dir.as_deref().unwrap_or(""),
            self.default_volume,
            self.show_hidden_files,
        );

        fs::write(config_path, content).ok();
    }
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".config/termusic.conf")
}
