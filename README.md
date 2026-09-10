# MeloTTY

MeloTTY is a focused terminal music player written in Rust. It provides a
keyboard-driven interface with separate Library, Queue, Now Playing, and
Settings pages, embedded album-art support, and a compact layout for narrow
terminals.

## Features

- MP3, FLAC, WAV, OGG, AAC, M4A, Opus, and other formats supported by Rodio
- Embedded and sidecar album artwork (`cover.jpg`, `folder.png`, and similar)
- Separate pages opened with `1`–`4`
- Search, queue management, shuffle, and repeat modes
- Persistent playlist and configuration
- Catppuccin-inspired TUI with responsive narrow-terminal layout
- Pure terminal interface; no graphical desktop required

## Requirements

- Rust stable toolchain
- ALSA development libraries on Linux
- A real interactive terminal

On Arch Linux:

```bash
sudo pacman -S rustup alsa-lib
```

## Build and install

```bash
git clone <your-melotty-repository-url>
cd melotty
cargo build --release
sudo install -Dm755 target/release/melotty /usr/local/bin/melotty
```

Optional Nerd Font symbols:

```bash
sudo pacman -S ttf-nerd-fonts-symbols
```

## Usage

```bash
melotty
melotty ~/Music
melotty ~/Music/song.mp3
melotty --help
```

## Controls

### Pages

| Key | Action |
| --- | --- |
| `1` | Library |
| `2` | Queue |
| `3` | Now Playing |
| `4` or `O` | Settings |
| `Esc` / `B` | Return to Library from Settings |

### Playback

| Key | Action |
| --- | --- |
| `Space` | Play / pause |
| `N` / `P` | Next / previous track |
| `+` / `-` | Increase / decrease volume |
| `5` / `0` | Set volume to 50% / 100% |
| `S` | Toggle shuffle |
| `R` | Cycle repeat mode |
| `/` | Search |
| `H` | Open help |
| `Q` | Quit |

### Library and Queue

| Key | Action |
| --- | --- |
| `Up` / `Down`, `J` / `K` | Navigate |
| `Enter` / `Right` / `L` | Open directory or play track |
| `Backspace` / `Left` | Go to parent directory |
| `A` | Add selected item to the queue |
| `D` | Remove the selected queue item |
| `C` | Clear the queue |
| `Tab` | Switch Library / Queue focus |

## Configuration

MeloTTY stores its configuration at `~/.config/melotty.conf`:

```ini
start_dir =
default_volume = 0.7
show_hidden_files = false
```

The playlist is saved at `~/.melotty_playlist.m3u`.

## Project layout

```text
src/
├── main.rs       # CLI, terminal session, and keyboard handling
├── app.rs        # Application state and playback actions
├── audio.rs      # Rodio audio engine
├── browser.rs    # File browser
├── playlist.rs   # Queue management
├── config.rs     # Persistent settings
├── cover.rs      # Embedded and sidecar artwork loading
├── ascii_art.rs  # Terminal artwork conversion
└── ui/           # Pages, widgets, theme, progress, and search
```

## Development

```bash
cargo fmt
cargo test
cargo check
```

## Contributing

Bug reports, improvements, and pull requests are welcome. Please keep changes
focused and run the formatting, test, and check commands before submitting a
pull request.

## License

MeloTTY is licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE).
