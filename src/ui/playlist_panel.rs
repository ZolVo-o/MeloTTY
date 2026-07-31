use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::playlist::Playlist;
use super::theme::Theme;

pub fn render_enhanced(f: &mut Frame, area: Rect, playlist: &Playlist, is_active: bool, is_playing: bool, theme: &Theme) {
    let border_color = if is_active { theme.gradient_end } else { theme.gray };
    let title_color = if is_active { theme.green } else { theme.fg_dim };

    let current_idx = playlist.current_index();
    let tracks = playlist.tracks();
    
    // Calculate scroll offset to keep current track visible with padding
    let visible_height = area.height.saturating_sub(2) as usize;
    let scroll_offset = if current_idx >= visible_height.saturating_sub(3) {
        current_idx.saturating_sub(visible_height.saturating_sub(3))
    } else {
        0
    };

    let items: Vec<ListItem> = tracks
        .iter()
        .skip(scroll_offset)
        .take(visible_height)
        .enumerate()
        .map(|(i, path)| {
            let actual_idx = i + scroll_offset;
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            let is_current = actual_idx == current_idx;

            let (prefix, style) = if is_current && is_playing {
                ("▶ ", Style::default()
                    .fg(theme.bg_dark)
                    .bg(theme.green)
                    .add_modifier(Modifier::BOLD))
            } else if is_current {
                ("⏸ ", Style::default()
                    .fg(theme.bg_dark)
                    .bg(theme.yellow)
                    .add_modifier(Modifier::BOLD))
            } else {
                (format!("{:02}.", actual_idx + 1), Style::default().fg(theme.fg_dim))
            };

            let display_style = if is_current {
                style
            } else {
                Style::default().fg(theme.fg)
            };

            ListItem::new(format!(" {} {}", prefix, name)).style(display_style)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_set(border::DOUBLE)
        .title(ratatui::text::Span::styled(
            format!(" 🎵 Playlist ({}) ", playlist.len()),
            Style::default().fg(title_color).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg_dark));

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default()
            .fg(theme.bg_dark)
            .bg(theme.gradient_end)
            .add_modifier(Modifier::BOLD));
    
    f.render_widget(list, area);
}
