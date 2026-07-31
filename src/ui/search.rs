use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::text::{Line, Span};

use super::theme::Theme;

pub fn render_enhanced(f: &mut Frame, area: Rect, query: &str, theme: &Theme) {
    // Создаем анимированный курсор
    let cursor = if (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() / 500) % 2 == 0
    {
        "█"
    } else {
        " "
    };

    // Формируем строку поиска с подсветкой
    let search_prompt = Line::from(vec![
        Span::styled("🔍 ", Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD)),
        Span::styled("Search: ", Style::default().fg(theme.fg_dim)),
        Span::styled(query, Style::default().fg(theme.yellow).add_modifier(Modifier::BOLD)),
        Span::styled(cursor, Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD)),
    ]);

    // Инструкция
    let instructions = Line::from(vec![
        Span::styled("Enter", Style::default().fg(theme.green).add_modifier(Modifier::BOLD)),
        Span::raw(" — confirm, "),
        Span::styled("Esc", Style::default().fg(theme.red).add_modifier(Modifier::BOLD)),
        Span::raw(" — cancel"),
    ]);

    let lines = vec![
        Line::from(""),
        search_prompt,
        Line::from(""),
        instructions,
        Line::from(""),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.cyan))
        .border_set(border::ROUNDED)
        .title(Span::styled(
            " 🔍 Search Mode ",
            Style::default().fg(theme.cyan).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg_dark));

    f.render_widget(Paragraph::new(lines).block(block).style(Style::default().fg(theme.fg)), area);
}
