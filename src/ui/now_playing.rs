use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::text::{Line, Span};

use crate::playlist::Playlist;
use super::theme::Theme;

pub fn render(
    f: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    theme: &Theme,
) {
    let (icon, color, status) = if is_playing {
        ("▶", theme.green, "PLAYING")
    } else if has_sink {
        ("⏸", theme.yellow, "PAUSED")
    } else {
        ("⏹", theme.gray, "STOPPED")
    };

    let track_name = playlist
        .current_track()
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
        .unwrap_or_else(|| "No track selected".to_string());

    let track_info = if playlist.is_empty() {
        vec![Line::from(Span::styled(
            "No tracks in playlist",
            Style::default().fg(theme.gray),
        ))]
    } else {
        vec![
            Line::from(Span::styled(
                format!("{}  {}", icon, track_name),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                format!("{}  Track {} of {}", status, playlist.current_index() + 1, playlist.len()),
                Style::default().fg(theme.gray),
            )),
        ]
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.gray))
        .border_set(border::ROUNDED)
        .style(Style::default().bg(theme.bg));

    f.render_widget(Paragraph::new(track_info).block(block), area);
}
