use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::playlist::Playlist;
use super::theme::Theme;

pub fn render(f: &mut Frame, area: Rect, playlist: &Playlist, is_active: bool, is_playing: bool, theme: &Theme) {
    let border_color = if is_active { theme.green } else { theme.gray };

    let items: Vec<ListItem> = playlist
        .tracks()
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            let is_current = i == playlist.current_index();

            let (prefix, style) = if is_current && is_playing {
                ("▶ ", Style::default().fg(theme.green).add_modifier(Modifier::BOLD))
            } else if is_current {
                ("⏸ ", Style::default().fg(theme.yellow).add_modifier(Modifier::BOLD))
            } else {
                let num = format!("{:02} ", i + 1);
                let leaked: &'static str = Box::leak(num.into_boxed_str());
                (leaked, Style::default().fg(theme.gray))
            };

            ListItem::new(format!(" {}{}", prefix, name)).style(style)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_set(border::ROUNDED)
        .title(ratatui::text::Span::styled(
            format!(" 🎵 Playlist ({}) ", playlist.len()),
            Style::default().fg(border_color).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg));

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().fg(theme.aqua).add_modifier(Modifier::BOLD));
    f.render_widget(list, area);
}
