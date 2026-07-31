use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::path::PathBuf;

use crate::browser::FileBrowser;
use super::theme::Theme;

pub fn render_enhanced(f: &mut Frame, area: Rect, browser: &FileBrowser, is_active: bool, theme: &Theme) {
    let border_color = if is_active { theme.cyan } else { theme.gray };
    let title_color = if is_active { theme.gradient_start } else { theme.fg_dim };

    let visible_entries: Vec<&PathBuf> = browser
        .entries()
        .iter()
        .skip(browser.scroll())
        .take((area.height.saturating_sub(2)) as usize)
        .collect();

    let home = std::env::var("HOME").unwrap_or_default();
    
    let mut list_state = ListState::default();
    list_state.select(Some(browser.selected().saturating_sub(browser.scroll())));
    
    let items: Vec<ListItem> = visible_entries
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let name = path.to_string_lossy().to_string();

            let (icon, color) = get_entry_style(&name, path, &home, theme);
            let display_name = get_display_name(&name, path, &home);
            
            let is_selected = i == 0; // Первый элемент - выбранный

            let style = if is_selected && is_active {
                Style::default()
                    .fg(theme.bg_dark)
                    .bg(theme.gradient_end)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            let prefix = if is_selected && is_active { "▸ " } else { "  " };
            ListItem::new(format!("{}{}{}", prefix, icon, display_name)).style(style)
        })
        .collect();

    let dir_display = browser.current_dir().display().to_string();
    let short_dir = dir_display.replace(&home, "~");

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .border_set(border::DOUBLE)
        .title(ratatui::text::Span::styled(
            format!(" 📂 {} ", short_dir),
            Style::default().fg(title_color).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg_dark));

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default()
            .fg(theme.bg_dark)
            .bg(theme.gradient_end)
            .add_modifier(Modifier::BOLD));
    
    f.render_stateful_widget(list, area, &mut list_state);
}

fn get_entry_style(name: &str, path: &PathBuf, home: &str, theme: &Theme) -> (&'static str, ratatui::style::Color) {
    if name.starts_with("───") {
        ("", theme.gray)
    } else if name == "/" {
        ("💻 ", theme.orange)
    } else if name == home || name == "~" {
        ("🏠 ", theme.yellow)
    } else if name.contains("Музыка") || name.to_lowercase().contains("music") {
        ("🎼 ", theme.green)
    } else if name.contains("Загрузк") || name.to_lowercase().contains("download") {
        ("📥 ", theme.blue)
    } else if name.contains("Документ") || name.to_lowercase().contains("document") {
        ("📄 ", theme.yellow)
    } else if name.contains("Видео") || name.to_lowercase().contains("video") {
        ("🎬 ", theme.magenta)
    } else if name.contains("Изображен") || name.to_lowercase().contains("picture") || name.to_lowercase().contains("image") {
        ("🖼 ", theme.orange)
    } else if path.is_dir() {
        ("📁 ", theme.cyan)
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
