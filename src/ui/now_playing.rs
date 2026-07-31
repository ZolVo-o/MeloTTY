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

pub fn render_dashboard(
    f: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    theme: &Theme,
) {
    // Определяем статус и цвета
    let (main_icon, status_text, primary_color, secondary_color) = if is_playing {
        ("🎵", "NOW PLAYING", theme.green, theme.gradient_start)
    } else if has_sink {
        ("⏸", "PAUSED", theme.yellow, theme.orange)
    } else {
        ("⏹", "STOPPED", theme.gray, theme.fg_dim)
    };

    // Получаем информацию о треке
    let track_name = playlist
        .current_track()
        .map(|p| p.file_name().unwrap_or_default().to_string_lossy().to_string())
        .unwrap_or_else(|| "No track loaded".to_string());

    // Создаем визуальный индикатор воспроизведения
    let visualizer_bars = if is_playing {
        create_visualizer_bars(theme)
    } else {
        "─".repeat(5)
    };

    // Формируем содержимое дашборда
    let mut lines = Vec::new();

    // Заголовок статуса
    lines.push(Line::from(vec![
        Span::styled(main_icon, Style::default().fg(primary_color).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(status_text, Style::default().fg(primary_color).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(visualizer_bars, Style::default().fg(secondary_color)),
    ]));

    // Разделитель
    lines.push(Line::from(Span::styled(
        "─".repeat(area.width.saturating_sub(2) as usize),
        Style::default().fg(theme.gray),
    )));

    // Название трека
    lines.push(Line::from(Span::styled(
        format!("🎶 {}", truncate_str(&track_name, area.width.saturating_sub(4) as usize)),
        Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
    )));

    // Информация о позиции в плейлисте
    if !playlist.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("Track {:02} / {:02}", playlist.current_index() + 1, playlist.len()),
            Style::default().fg(theme.fg_dim),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "Empty playlist",
            Style::default().fg(theme.gray),
        )));
    }

    // Режимы воспроизведения
    let mode_info = format!(
        "{} {}",
        if playlist.shuffle { "🔀" } else { "·" },
        match playlist.repeat {
            crate::playlist::RepeatMode::Off => "🔁 off",
            crate::playlist::RepeatMode::All => "🔁 all",
            crate::playlist::RepeatMode::One => "🔂 one",
        }
    );
    lines.push(Line::from(Span::styled(
        mode_info,
        Style::default().fg(theme.fg_dim),
    )));

    // Создаем блок с современным дизайном
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(primary_color))
        .border_set(border::ROUNDED)
        .title(Span::styled(
            " Dashboard ",
            Style::default().fg(primary_color).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg_dark));

    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn create_visualizer_bars(theme: &Theme) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let bar_patterns = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    let index = (seed % 8) as usize;
    format!("{}{}{}{}{}", 
        bar_patterns[(index) % 8],
        bar_patterns[(index + 1) % 8],
        bar_patterns[(index + 2) % 8],
        bar_patterns[(index + 3) % 8],
        bar_patterns[(index + 4) % 8]
    )
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
