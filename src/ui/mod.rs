mod theme;
mod tabs;
mod now_playing;
mod progress;
mod browser_panel;
mod playlist_panel;
mod search;
mod status_bar;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Block,
    Frame,
};
use std::time::{Duration, Instant};

use crate::app::Mode;
use crate::browser::FileBrowser;
use crate::playlist::Playlist;

pub use theme::Theme;
pub use theme::GRUVBOX;

pub fn render(
    f: &mut Frame,
    mode: &Mode,
    browser: &FileBrowser,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    volume: f32,
    track_start: Option<Instant>,
    track_duration: Option<Duration>,
    _spectrum: &[f32],
    search_query: &str,
) {
    let theme = &GRUVBOX;

    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg_dark).fg(theme.fg)),
        f.area(),
    );

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // tabs
            Constraint::Length(3),  // now playing
            Constraint::Length(1),  // progress
            Constraint::Min(0),     // main area
            Constraint::Length(1),  // status
        ])
        .split(f.area());

    tabs::render(f, main_layout[0], mode, theme);
    now_playing::render(f, main_layout[1], playlist, is_playing, has_sink, theme);
    progress::render(f, main_layout[2], track_start, track_duration, is_playing, theme);
    render_main_area(f, main_layout[3], mode, browser, playlist, is_playing, search_query, theme);
    status_bar::render(f, main_layout[4], playlist, volume, theme);
}

fn render_main_area(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    mode: &Mode,
    browser: &FileBrowser,
    playlist: &Playlist,
    is_playing: bool,
    search_query: &str,
    theme: &Theme,
) {
    if *mode == Mode::Search {
        search::render(f, area, search_query, theme);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    browser_panel::render(f, chunks[0], browser, *mode == Mode::Browser, theme);
    playlist_panel::render(f, chunks[1], playlist, *mode == Mode::Playlist, is_playing, theme);
}
