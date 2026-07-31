use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders},
    symbols::border,
    Frame,
};
use ratatui::text::{Line, Span};

use crate::app::Mode;
use super::theme::Theme;

pub fn render_modern(f: &mut Frame, area: Rect, mode: &Mode, theme: &Theme) {
    let active_style = Style::default()
        .fg(theme.bg_dark)
        .bg(theme.gradient_start)
        .add_modifier(Modifier::BOLD);
    
    let inactive_style = Style::default()
        .fg(theme.fg_dim)
        .bg(theme.bg_darker);

    let browser_tab = match mode {
        Mode::Browser => Span::styled(" 📁 Browser ", active_style),
        _ => Span::styled(" 📁 Browser ", inactive_style),
    };

    let playlist_tab = match mode {
        Mode::Playlist => Span::styled(" 🎵 Playlist ", active_style),
        _ => Span::styled(" 🎵 Playlist ", inactive_style),
    };

    let search_tab = match mode {
        Mode::Search => Span::styled(" 🔍 Search ", active_style),
        _ => Span::styled(" 🔍 Search ", inactive_style),
    };

    let tabs_line = Line::from(vec![
        browser_tab,
        Span::raw(" "),
        playlist_tab,
        Span::raw(" "),
        search_tab,
    ]);

    f.render_widget(tabs_line, area);
}
