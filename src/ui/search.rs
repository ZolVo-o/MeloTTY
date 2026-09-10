use ratatui::{
    layout::Rect,
    style::Style,
    symbols::border,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::theme::Theme;

pub fn render(f: &mut Frame, area: Rect, query: &str, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.yellow))
        .border_set(border::ROUNDED)
        .title("🔍 Search")
        .style(Style::default().bg(theme.bg));

    let text = format!("\n  Search: {}_\n\n  Enter — confirm, Esc — cancel", query);

    f.render_widget(
        Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(theme.fg)),
        area,
    );
}
