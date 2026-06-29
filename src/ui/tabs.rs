use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Tabs,
    Frame,
};
use ratatui::text::Span;

use crate::app::Mode;
use super::theme::Theme;

pub fn render(f: &mut Frame, area: Rect, mode: &Mode, theme: &Theme) {
    let titles = vec!["  📁 Browser  ", "  🎵 Playlist  "];
    let selected = match mode {
        Mode::Browser => 0,
        Mode::Playlist => 1,
        Mode::Search => 0,
    };

    let tabs = Tabs::new(titles)
        .select(selected)
        .style(Style::default().fg(theme.gray))
        .highlight_style(
            Style::default()
                .fg(theme.aqua)
                .add_modifier(Modifier::BOLD)
        )
        .divider(Span::raw("│"));

    f.render_widget(tabs, area);
}
