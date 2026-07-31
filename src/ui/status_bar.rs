use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::text::{Line, Span};

use crate::playlist::{Playlist, RepeatMode};
use super::theme::Theme;

pub fn render_enhanced(f: &mut Frame, area: Rect, playlist: &Playlist, volume: f32, theme: &Theme) {
    // Индикатор shuffle с градиентом
    let shuffle_icon = if playlist.shuffle { "🔀" } else { "⊘" };
    let shuffle_style = if playlist.shuffle {
        Style::default()
            .fg(theme.bg_dark)
            .bg(theme.green)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.fg_dim)
    };

    // Индикатор repeat с разными цветами для режимов
    let (repeat_icon, repeat_style) = match playlist.repeat {
        RepeatMode::Off => ("🔁", Style::default().fg(theme.fg_dim)),
        RepeatMode::All => ("🔁", 
            Style::default()
                .fg(theme.bg_dark)
                .bg(theme.blue)
                .add_modifier(Modifier::BOLD)),
        RepeatMode::One => ("🔂", 
            Style::default()
                .fg(theme.bg_dark)
                .bg(theme.magenta)
                .add_modifier(Modifier::BOLD)),
    };

    // Иконка громкости с цветом
    let (vol_icon, vol_percent) = if volume > 0.7 {
        ("🔊", Style::default().fg(theme.green).add_modifier(Modifier::BOLD))
    } else if volume > 0.3 {
        ("🔉", Style::default().fg(theme.yellow))
    } else if volume > 0.0 {
        ("🔈", Style::default().fg(theme.orange))
    } else {
        ("🔇", Style::default().fg(theme.red))
    };

    // Ползунок громкости визуальный
    let vol_bar_len = ((area.width.saturating_sub(40) / 2) as f32 * volume) as usize;
    let vol_bar_full = "█".repeat(vol_bar_len.min(20));
    let vol_bar_empty = "░".repeat((20 - vol_bar_len).max(0));
    let vol_bar = format!("[{}{}]", vol_bar_full, vol_bar_empty);

    // Горячие клавиши в компактном формате
    let hotkey_style = Style::default().fg(theme.bg_dark).bg(theme.gray);
    let separator = Span::styled(" ▐ ", Style::default().fg(theme.fg_dim));

    let line = Line::from(vec![
        // Левая секция - навигация
        Span::styled(" 📁 Browser ", hotkey_style),
        Span::raw("Tab"),
        separator.clone(),
        
        Span::styled(" ▶ Play ", hotkey_style),
        Span::raw("Space"),
        separator.clone(),
        
        Span::styled(" ⏭ Next ", hotkey_style),
        Span::raw("n"),
        separator.clone(),
        
        Span::styled(" ⏮ Prev ", hotkey_style),
        Span::raw("p"),
        separator.clone(),
        
        Span::styled(" 🔍 Search ", hotkey_style),
        Span::raw("/"),
        separator.clone(),
        
        // Центральная секция - режимы
        Span::styled(format!(" {} Shuffle ", shuffle_icon), shuffle_style),
        Span::raw(" "),
        Span::styled(format!(" {} Repeat ", repeat_icon), repeat_style),
        separator.clone(),
        
        // Правая секция - громкость
        Span::styled(format!(" {} ", vol_icon), vol_percent),
        Span::styled(format!("{} ", vol_bar), Style::default().fg(theme.cyan)),
        Span::styled(format!("{:.0}%", volume * 100.0), Style::default().fg(theme.fg_dim)),
        separator.clone(),
        
        // Quit кнопка
        Span::styled(" Q Quit ", Style::default().fg(theme.bg_dark).bg(theme.red).add_modifier(Modifier::BOLD)),
    ]);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.gradient_end))
        .border_set(border::ROUNDED)
        .style(Style::default().bg(theme.bg_dark));

    f.render_widget(Paragraph::new(line).block(block), area);
}
