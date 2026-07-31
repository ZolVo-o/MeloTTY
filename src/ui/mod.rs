mod theme;
mod tabs;
mod now_playing;
mod progress;
mod browser_panel;
mod playlist_panel;
mod search;
mod status_bar;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::text::{Line, Span};
use std::time::{Duration, Instant};

use crate::app::Mode;
use crate::browser::FileBrowser;
use crate::playlist::Playlist;

pub use theme::Theme;
pub use theme::CYBERPUNK as DEFAULT_THEME;

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
    spectrum: &[f32],
    search_query: &str,
) {
    let theme = &DEFAULT_THEME;

    // Рендеринг фонового градиента
    render_background(f, f.area(), theme);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // header с tabs
            Constraint::Length(5),  // now playing dashboard
            Constraint::Length(3),  // progress bar с визуализацией
            Constraint::Min(8),     // main area
            Constraint::Length(3),  // status bar расширенный
        ])
        .split(f.area());

    // Header с табами и заголовком
    render_header(f, main_layout[0], mode, theme);
    
    // Dashboard "Now Playing"
    now_playing::render_dashboard(f, main_layout[1], playlist, is_playing, has_sink, theme);
    
    // Progress bar с визуализацией спектра
    progress::render_with_spectrum(f, main_layout[2], track_start, track_duration, is_playing, spectrum, theme);
    
    // Основная область
    render_main_area(f, main_layout[3], mode, browser, playlist, is_playing, search_query, theme);
    
    // Расширенный status bar
    status_bar::render_enhanced(f, main_layout[4], playlist, volume, theme);
}

fn render_background(f: &mut Frame, area: Rect, theme: &Theme) {
    let bg_block = Block::default()
        .style(Style::default().bg(theme.bg));
    f.render_widget(bg_block, area);
}

fn render_header(f: &mut Frame, area: Rect, mode: &Mode, theme: &Theme) {
    let title = Span::styled(
        "🎵 TERMUSIC ",
        Style::default()
            .fg(theme.gradient_start)
            .add_modifier(Modifier::BOLD),
    );
    
    let subtitle = Span::styled(
        format!("v{} | Modern Edition", env!("CARGO_PKG_VERSION")),
        Style::default().fg(theme.fg_dim),
    );
    
    let header_text = Line::from(vec![title, subtitle]);
    
    let header_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.gradient_start))
        .border_set(border::ROUNDED)
        .title(header_text)
        .style(Style::default().bg(theme.bg_dark));
    
    f.render_widget(header_block, area);
    
    // Рендерим табы внутри header
    let tabs_area = Rect::new(
        area.x + 2,
        area.y + 1,
        area.width.saturating_sub(4),
        1,
    );
    tabs::render_modern(f, tabs_area, mode, theme);
}

fn render_main_area(
    f: &mut Frame,
    area: Rect,
    mode: &Mode,
    browser: &FileBrowser,
    playlist: &Playlist,
    is_playing: bool,
    search_query: &str,
    theme: &Theme,
) {
    if *mode == Mode::Search {
        search::render_enhanced(f, area, search_query, theme);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(45),
            Constraint::Percentage(55),
        ])
        .split(area);

    browser_panel::render_enhanced(f, chunks[0], browser, *mode == Mode::Browser, theme);
    playlist_panel::render_enhanced(f, chunks[1], playlist, *mode == Mode::Playlist, is_playing, theme);
}
