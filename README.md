# 🎵 termusic

![Rust](https://img.shields.io/badge/rust-1.96+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-linux%20%7C%20arch-brightgreen.svg)

**termusic** — высококачественный консольный аудиоплеер для Linux, написанный на Rust. Работает в любом терминале, включая TTY, и не требует X-сервера или Wayland.

![screenshot](https://user-images.githubusercontent.com/placeholder/termusic.png)

## ✨ Возможности

- 🎧 **Поддержка форматов**: MP3, FLAC, WAV, OGG, AAC, M4A, Opus, WMA
- 🖥 **Работа в TTY**: в голой консоли без графического сервера
- 📁 **Файловый браузер** с закладками и поиском
- 🎵 **Умный плейлист** с автосохранением
- 🔀 **Режимы воспроизведения**: перемешивание, повтор одного/всех
- 🎨 **Современный TUI**: темы, иконки, прогресс-бар
- ⚡ **Мгновенный запуск**: скомпилирован в нативный бинарник
- 🔍 **Поиск** по файлам (`/`)
- ⌨ **Vim-подобное управление**: `h/j/k/l`, `/`, цифры
- 💾 **Лёгкий**: 5-10 МБ памяти

## 📦 Установка

### Arch Linux (AUR)

```bash
yay -S termusic

Из исходников
bash

git clone https://github.com/zolvo/termusic.git
cd termusic
cargo build --release
sudo cp target/release/termusic /usr/local/bin/

Зависимости
bash

# Arch Linux
sudo pacman -S rustup alsa-lib
rustup default stable

🚀 Использование
bash

termusic                  # запуск (откроет домашнюю папку)
termusic ~/Music          # указать папку
termusic track.mp3        # указать файл
termusic --help           # справка по аргументам

🎮 Управление
Глобальные клавиши
Клавиша	Действие
Tab	Переключение браузер / плейлист
Space	Play / Pause
N / P	Следующий / Предыдущий трек
S	Перемешивание
R	Режим повтора (Выкл → Все → Один)
+ / -	Громкость
1 / 5 / 0	Громкость 10% / 50% / 100%
/	Поиск файла
H	Справка
Q	Выход
Браузер
Клавиша	Действие
↑↓ / J/K	Навигация
Enter / → / L	Войти в папку / играть файл
Backspace / ← / H	Назад
A	Добавить в плейлист
Плейлист
Клавиша	Действие
↑↓ / J/K	Навигация по трекам
Enter	Играть выбранный
D	Удалить трек
C	Очистить всё
⚙️ Конфигурация

Файл ~/.config/termusic.conf:
ini

# Стартовая директория (пусто = авто)
start_dir = 

# Громкость по умолчанию (0.0 - 1.0)
default_volume = 0.7

# Показывать скрытые файлы
show_hidden_files = false

🏗 Структура проекта
text

src/
├── main.rs          # точка входа, обработка клавиш
├── app.rs           # логика приложения
├── audio.rs         # движок воспроизведения (rodio)
├── browser.rs       # файловый браузер
├── playlist.rs      # управление плейлистом
├── config.rs        # конфигурация
├── error.rs         # типы ошибок
├── help.rs          # экран справки
└── ui/              # интерфейс
    ├── mod.rs       # главный layout
    ├── theme.rs     # цветовые схемы
    ├── tabs.rs      # вкладки браузер/плейлист
    ├── now_playing.rs
    ├── progress.rs
    ├── browser_panel.rs
    ├── playlist_panel.rs
    ├── search.rs
    └── status_bar.rs

🔧 Технологии

    Rust — язык

    rodio — воспроизведение аудио

    ratatui — терминальный UI

    crossterm — управление терминалом

    clap — парсинг аргументов

📝 TODO

    Пакет для AUR

    Поддержка MPRIS (управление с телефона)

    Эквалайзер

    Last.fm скробблинг

    Интернет-радио

    Vim-подобные команды (:q, :play, etc)

🤝 Участие

Pull request'ы приветствуются! Для серьёзных изменений — сначала откройте issue.
📄 Лицензия

MIT © 2026 zolvo
