mod app;
mod ascii_art;
mod audio;
mod browser;
mod config;
mod cover;
mod error;
mod help;
mod playlist;
mod ui;

use anyhow::{bail, Context};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, IsTerminal};
use std::time::Duration;

use crate::app::{App, Mode, Page};

#[derive(Parser)]
#[command(name = "melotty")]
#[command(about = "A focused terminal music player with album art")]
struct Cli {
    path: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = config::Config::load();

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        bail!(
            "melotty requires an interactive terminal. Run it from a real terminal, for example: cargo run -- ~/Music"
        );
    }

    let mut terminal = TerminalSession::start()?;

    let start_path = cli.path.clone().or(config.start_dir.clone());
    let mut app = App::new(start_path, config.clone())?;
    app.set_volume(config.default_volume);

    let run_result = run(terminal.terminal_mut(), &mut app);
    let save_result = app.save_playlist().context("failed to save playlist");

    run_result.and(save_result)
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl TerminalSession {
    fn start() -> anyhow::Result<Self> {
        enable_raw_mode().context("failed to enable raw mode")?;
        let mut stdout = io::stdout();

        if let Err(error) = execute!(stdout, EnterAlternateScreen) {
            disable_raw_mode().ok();
            return Err(error.into());
        }

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = match Terminal::new(backend) {
            Ok(terminal) => terminal,
            Err(error) => {
                disable_raw_mode().ok();
                return Err(error.into());
            }
        };

        if let Err(error) = terminal.hide_cursor() {
            disable_raw_mode().ok();
            execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
            return Err(error.into());
        }

        Ok(Self { terminal })
    }

    fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).ok();
        self.terminal.show_cursor().ok();
    }
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> anyhow::Result<()> {
    let mut show_help = false;

    loop {
        app.update();

        if show_help {
            terminal.draw(|f| help::render_help(f))?;
        } else {
            let search = app.search_query.clone();
            let cover = app.cover_ascii.clone();
            terminal.draw(|f| {
                ui::render(
                    f,
                    &app.mode,
                    app.page,
                    &app.browser,
                    &app.playlist,
                    app.is_playing,
                    app.audio.has_sink(),
                    app.audio.volume(),
                    app.audio.position(),
                    app.track_duration,
                    &app.spectrum,
                    &search,
                    &cover,
                )
            })?;
        }

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if show_help {
                        match key.code {
                            KeyCode::Char('h') | KeyCode::Char('q') | KeyCode::Esc => {
                                show_help = false
                            }
                            _ => {}
                        }
                        continue;
                    }

                    if app.mode == Mode::Search {
                        match key.code {
                            KeyCode::Esc => app.cancel_search(),
                            KeyCode::Enter => app.search_confirm(),
                            KeyCode::Backspace => app.search_backspace(),
                            KeyCode::Char(c) => app.search_add_char(c),
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('h') => {
                            show_help = true;
                            continue;
                        }
                        KeyCode::Char('o') => {
                            app.open_settings();
                            continue;
                        }
                        KeyCode::Char('1') => {
                            app.open_page(Page::Library);
                            continue;
                        }
                        KeyCode::Char('2') => {
                            app.open_page(Page::Queue);
                            continue;
                        }
                        KeyCode::Char('3') => {
                            app.open_page(Page::NowPlaying);
                            continue;
                        }
                        KeyCode::Char('4') => {
                            app.open_page(Page::Settings);
                            continue;
                        }
                        KeyCode::Char('/') => {
                            app.start_search();
                            continue;
                        }
                        KeyCode::Char('5') => {
                            app.set_volume_preset(0.5);
                            continue;
                        }
                        KeyCode::Char('0') => {
                            app.set_volume_preset(1.0);
                            continue;
                        }
                        _ => handle_key(key.code, app)?,
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_key(key: KeyCode, app: &mut App) -> anyhow::Result<()> {
    match key {
        KeyCode::Char('q') => {
            app.should_quit = true;
            return Ok(());
        }
        KeyCode::Tab => {
            app.toggle_mode();
            return Ok(());
        }
        KeyCode::Char(' ') => {
            app.toggle_playback()?;
            return Ok(());
        }
        KeyCode::Char('n') => {
            app.next_track()?;
            return Ok(());
        }
        KeyCode::Char('p') => {
            app.prev_track()?;
            return Ok(());
        }
        KeyCode::Char('+') | KeyCode::Char('=') => {
            app.volume_up();
            return Ok(());
        }
        KeyCode::Char('-') => {
            app.volume_down();
            return Ok(());
        }
        KeyCode::Char('s') => {
            app.toggle_shuffle();
            return Ok(());
        }
        KeyCode::Char('r') => {
            app.cycle_repeat();
            return Ok(());
        }
        _ => {}
    }

    match app.mode {
        Mode::Browser => handle_browser_keys(key, app)?,
        Mode::Playlist => handle_playlist_keys(key, app)?,
        Mode::Settings => handle_settings_keys(key, app),
        _ => {}
    }

    Ok(())
}

fn handle_settings_keys(key: KeyCode, app: &mut App) {
    match key {
        KeyCode::Esc | KeyCode::Char('b') => app.open_page(Page::Library),
        KeyCode::Char('h') => app.toggle_hidden_files(),
        KeyCode::Char('+') | KeyCode::Char('=') => app.volume_up(),
        KeyCode::Char('-') => app.volume_down(),
        _ => {}
    }
}

fn handle_browser_keys(key: KeyCode, app: &mut App) -> anyhow::Result<()> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => app.browser.navigate_up(),
        KeyCode::Down | KeyCode::Char('j') => app.browser.navigate_down(),
        KeyCode::Enter | KeyCode::Right | KeyCode::Char('l') => {
            if let Some(path) = app.browser.selected_path().cloned() {
                if path.is_dir() {
                    app.browser.enter_directory();
                } else {
                    app.play_selected()?;
                }
            }
        }
        KeyCode::Backspace | KeyCode::Left => app.browser.go_back(),
        KeyCode::Char('a') => app.add_selected_to_playlist(),
        _ => {}
    }
    Ok(())
}

fn handle_playlist_keys(key: KeyCode, app: &mut App) -> anyhow::Result<()> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.playlist.current_index() > 0 {
                app.playlist.select(app.playlist.current_index() - 1);
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            if app.playlist.current_index() + 1 < app.playlist.len() {
                app.playlist.select(app.playlist.current_index() + 1);
            }
        }
        KeyCode::Enter => app.play_track()?,
        KeyCode::Char('d') => app.remove_current_track(),
        KeyCode::Char('c') => app.clear_playlist(),
        _ => {}
    }
    Ok(())
}
