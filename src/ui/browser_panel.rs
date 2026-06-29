use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use std::path::PathBuf;

use crate::browser::FileBrowser;
use super::theme::Theme;

pub fn render(f: &mut Frame, area: Rect, browser: &FileBrowser, is_active: bool, theme: &Theme) {
    let border_color = if is_active { theme.blue } else { theme.gray };

    let visible_entries: Vec<&PathBuf> = browser
        .entries()
        .iter()
        .skip(browser.scroll())
        .take(20)
        .collect();

    let home = std::env::var("HOME").unwrap_or_default();
    
    let items: Vec<ListItem> = visible_entries
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let actual_index = i + browser.scroll();
            let is_selected = actual_index == browser.selected() && is_active;
            let name = path.to_string_lossy().to_string();

            let (icon, color) = get_entry_style(&name, path, &home, theme);
            let display_name = get_display_name(&name, path, &home);

            let style = if is_selected {
                Style::default().fg(color).bg(theme.gray).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            ListItem::new(format!(" {} {}", icon, display_name)).style(style)
        })
        .collect();

    let dir_display = browser.current_dir().display().to_string();
    let short_dir = dir_display.replace(&home, "~");

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_set(border::ROUNDED)
        .title(ratatui::text::Span::styled(
            format!(" 📂 {} ", short_dir),
            Style::default().fg(border_color).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg));

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().fg(theme.aqua).add_modifier(Modifier::BOLD));
    f.render_widget(list, area);
}

fn get_entry_style(name: &str, path: &PathBuf, home: &str, theme: &Theme) -> (&'static str, ratatui::style::Color) {
    if name.starts_with("───") {
        ("", theme.gray)
    } else if name == "/" {
        ("💻 ", theme.orange)
    } else if name == home {
        ("🏠 ", theme.yellow)
    } else if name.contains("Музыка") || name.to_lowercase().contains("music") {
        ("🎼 ", theme.green)
    } else if name.contains("Загрузк") || name.to_lowercase().contains("download") {
        ("📥 ", theme.blue)
    } else if name.contains("Документ") || name.to_lowercase().contains("document") {
        ("📄 ", theme.yellow)
    } else if name.contains("Видео") || name.to_lowercase().contains("video") {
        ("🎬 ", theme.purple)
    } else if name.contains("Изображен") || name.to_lowercase().contains("picture") || name.to_lowercase().contains("image") {
        ("🖼 ", theme.orange)
    } else if path.is_dir() {
        ("📁 ", theme.blue)
    } else {
        ("🎵 ", theme.fg)
    }
}

fn get_display_name(name: &str, path: &PathBuf, home: &str) -> String {
    if name == "/" {
        "Root filesystem".to_string()
    } else if name.starts_with('/') {
        path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| name.replace(home, "~"))
    } else {
        name.to_string()
    }
}
