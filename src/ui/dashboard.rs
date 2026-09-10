use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::time::Duration;

use crate::app::{Mode, Page};
use crate::browser::FileBrowser;
use crate::playlist::{Playlist, RepeatMode};

use super::{progress, search, Theme, CATTPUCCIN};

pub fn render(
    f: &mut Frame,
    mode: &Mode,
    page: Page,
    browser: &FileBrowser,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    volume: f32,
    track_position: Duration,
    track_duration: Option<Duration>,
    spectrum: &[f32],
    search_query: &str,
    cover_ascii: &Option<Vec<(String, Vec<ratatui::style::Color>)>>,
) {
    let theme = &CATTPUCCIN;
    let area = f.area();
    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg_dark)),
        area,
    );

    if page == Page::Settings {
        render_settings_page(f, area, browser, volume, theme);
        return;
    }
    if page == Page::NowPlaying {
        render_now_playing_page(
            f,
            area,
            playlist,
            is_playing,
            has_sink,
            volume,
            track_position,
            track_duration,
            spectrum,
            cover_ascii,
            theme,
        );
        return;
    }
    if page == Page::Library {
        render_library_page(f, area, mode, browser, volume, theme);
        return;
    }
    if page == Page::Queue {
        render_queue_page(f, area, playlist, is_playing, volume, theme);
        return;
    }

    let compact = area.width < 80 || area.height < 24;
    let hero_height = if compact {
        7
    } else if cover_ascii.is_some() {
        10
    } else {
        7
    };
    let layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(hero_height),
        Constraint::Length(1),
        Constraint::Min(8),
        Constraint::Length(2),
    ])
    .split(area);

    render_brand(f, layout[0], playlist, volume, theme);
    render_hero(
        f,
        layout[1],
        playlist,
        is_playing,
        has_sink,
        cover_ascii,
        theme,
    );
    progress::render(
        f,
        layout[2],
        track_position,
        track_duration,
        is_playing,
        spectrum,
        theme,
    );

    if *mode == Mode::Search {
        search::render(f, layout[3], search_query, theme);
    } else if *mode == Mode::Settings {
        render_settings(f, layout[3], browser, volume, theme);
    } else {
        render_library_and_queue(f, layout[3], mode, browser, playlist, is_playing, theme);
    }

    render_command_bar(f, layout[4], mode, playlist, volume, theme);
}

fn render_library_page(
    f: &mut Frame,
    area: Rect,
    mode: &Mode,
    browser: &FileBrowser,
    volume: f32,
    theme: &Theme,
) {
    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg_dark)),
        area,
    );
    render_brand(
        f,
        Rect { height: 2, ..area },
        &Playlist::new(),
        volume,
        theme,
    );
    let content = Rect {
        y: area.y + 2,
        height: area.height.saturating_sub(4),
        ..area
    };
    render_library(f, content, browser, *mode == Mode::Browser, theme);
    render_page_footer(
        f,
        Rect {
            y: area.bottom().saturating_sub(2),
            height: 2,
            ..area
        },
        "1 LIBRARY",
        theme,
    );
}

fn render_queue_page(
    f: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    is_playing: bool,
    volume: f32,
    theme: &Theme,
) {
    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg_dark)),
        area,
    );
    render_brand(f, Rect { height: 2, ..area }, playlist, volume, theme);
    let content = Rect {
        y: area.y + 2,
        height: area.height.saturating_sub(4),
        ..area
    };
    render_queue(f, content, playlist, true, is_playing, theme);
    render_page_footer(
        f,
        Rect {
            y: area.bottom().saturating_sub(2),
            height: 2,
            ..area
        },
        "2 QUEUE",
        theme,
    );
}

fn render_settings_page(
    f: &mut Frame,
    area: Rect,
    browser: &FileBrowser,
    volume: f32,
    theme: &Theme,
) {
    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg_dark)),
        area,
    );
    render_brand(
        f,
        Rect { height: 2, ..area },
        &Playlist::new(),
        volume,
        theme,
    );
    render_settings(
        f,
        Rect {
            y: area.y + 3,
            height: area.height.saturating_sub(5),
            ..area
        },
        browser,
        volume,
        theme,
    );
    render_page_footer(
        f,
        Rect {
            y: area.bottom().saturating_sub(2),
            height: 2,
            ..area
        },
        "4 SETTINGS",
        theme,
    );
}

fn render_now_playing_page(
    f: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    volume: f32,
    position: Duration,
    duration: Option<Duration>,
    spectrum: &[f32],
    cover: &Option<Vec<(String, Vec<ratatui::style::Color>)>>,
    theme: &Theme,
) {
    f.render_widget(
        Block::default().style(Style::default().bg(theme.bg_dark)),
        area,
    );
    render_brand(f, Rect { height: 2, ..area }, playlist, volume, theme);
    let compact = area.width < 80 || area.height < 24;
    let hero = Rect {
        y: area.y + 2,
        height: area.height.saturating_sub(if compact { 6 } else { 7 }),
        ..area
    };
    render_hero(f, hero, playlist, is_playing, has_sink, cover, theme);
    progress::render(
        f,
        Rect {
            y: hero.bottom(),
            height: 1,
            ..area
        },
        position,
        duration,
        is_playing,
        spectrum,
        theme,
    );
    render_page_footer(
        f,
        Rect {
            y: area.bottom().saturating_sub(2),
            height: 2,
            ..area
        },
        "3 NOW PLAYING",
        theme,
    );
}

fn render_page_footer(f: &mut Frame, area: Rect, current: &str, theme: &Theme) {
    let text = if area.width < 72 {
        format!(" {} · O settings · Q quit", current)
    } else {
        format!(
            "  1 LIBRARY   2 QUEUE   3 NOW PLAYING   4 SETTINGS     {}     O settings · +/- volume · Q quit",
            current,
        )
    };
    f.render_widget(
        Paragraph::new(text).style(Style::default().fg(theme.fg_dim).bg(theme.bg_dark)),
        area,
    );
}

fn render_settings(f: &mut Frame, area: Rect, browser: &FileBrowser, volume: f32, theme: &Theme) {
    let block = card("SETTINGS", area, theme.accent, theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let narrow = inner.width < 52;
    let volume_label = if narrow {
        format!("  Volume: {:>3.0}%", volume * 100.0)
    } else {
        format!("  Volume{:>38.0}%", volume * 100.0)
    };
    let hidden_label = if narrow {
        format!(
            "  Hidden files: {}",
            if browser.hidden_files_enabled() {
                "ON"
            } else {
                "OFF"
            }
        )
    } else {
        format!(
            "  Show hidden files{:>27}",
            if browser.hidden_files_enabled() {
                "ON"
            } else {
                "OFF"
            }
        )
    };
    let rows = vec![
        Line::from(vec![Span::styled(
            "  AUDIO",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(volume_label, Style::default().fg(theme.fg))),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  LIBRARY",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::styled(
            hidden_label,
            Style::default().fg(if browser.hidden_files_enabled() {
                theme.green
            } else {
                theme.gray
            }),
        )),
        Line::from(""),
        Line::from(Span::styled(
            if narrow {
                "  H hidden · +/- volume · Esc back"
            } else {
                "  H toggle hidden files    +/- change volume    Esc back"
            },
            Style::default().fg(theme.fg_dim),
        )),
    ];
    f.render_widget(
        Paragraph::new(rows)
            .block(Block::default().padding(ratatui::widgets::Padding::new(1, 1, 1, 1))),
        inner,
    );
}

fn render_brand(f: &mut Frame, area: Rect, playlist: &Playlist, volume: f32, theme: &Theme) {
    let compact = area.width < 60;
    let line = if compact {
        Line::from(vec![
            Span::styled(
                " MELOTTY ",
                Style::default()
                    .fg(theme.bg_dark)
                    .bg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {:02.0}%", volume * 100.0),
                Style::default().fg(theme.gray),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled(
                "  MELOTTY ",
                Style::default()
                    .fg(theme.bg_dark)
                    .bg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  MUSIC WORKSPACE", Style::default().fg(theme.fg_dim)),
            Span::styled("  ·  ", Style::default().fg(theme.gray)),
            Span::styled(
                if playlist.is_empty() {
                    "EMPTY QUEUE"
                } else {
                    "READY"
                },
                Style::default().fg(if playlist.is_empty() {
                    theme.yellow
                } else {
                    theme.green
                }),
            ),
            Span::styled(
                format!(
                    "                                      {:02.0}%",
                    volume * 100.0
                ),
                Style::default().fg(theme.gray),
            ),
        ])
    };
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme.bg_dark)),
        area,
    );
}

fn render_hero(
    f: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    is_playing: bool,
    has_sink: bool,
    cover_ascii: &Option<Vec<(String, Vec<ratatui::style::Color>)>>,
    theme: &Theme,
) {
    let block = card("NOW PLAYING", area, theme.accent, theme);
    let inner = block.inner(area);
    f.render_widget(block, area);

    let cover_width = if inner.width < 60 { 18 } else { 25 };
    let columns =
        Layout::horizontal([Constraint::Length(cover_width), Constraint::Min(0)]).split(inner);

    if let Some(lines) = cover_ascii {
        render_cover(f, columns[0], lines, theme);
    } else {
        render_empty_cover(f, columns[0], theme);
    }

    let (status, color) = if is_playing {
        ("PLAYING", theme.green)
    } else if has_sink {
        ("PAUSED", theme.yellow)
    } else {
        ("STOPPED", theme.gray)
    };
    let title = playlist
        .current_track()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Choose a track to begin".to_string());
    let title = if columns[1].width < 30 && title.chars().count() > 22 {
        format!("{}…", title.chars().take(21).collect::<String>())
    } else {
        title
    };
    let lines = vec![
        Line::from(Span::styled(
            title,
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!(
                "{}   ·   TRACK {}/{}",
                status,
                if playlist.is_empty() {
                    0
                } else {
                    playlist.current_index() + 1
                },
                playlist.len()
            ),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Your personal library, in focus.",
            Style::default().fg(theme.fg_dim),
        )),
    ];
    f.render_widget(
        Paragraph::new(lines)
            .block(Block::default().padding(ratatui::widgets::Padding::new(1, 1, 1, 0))),
        columns[1],
    );
}

fn render_empty_cover(f: &mut Frame, area: Rect, theme: &Theme) {
    let lines = vec![
        Line::from(Span::styled(
            "     ┌─────────┐",
            Style::default().fg(theme.gray),
        )),
        Line::from(Span::styled(
            "     │         │",
            Style::default().fg(theme.gray),
        )),
        Line::from(Span::styled(
            "     │  NO     │",
            Style::default().fg(theme.fg_dim),
        )),
        Line::from(Span::styled(
            "     │ ARTWORK │",
            Style::default().fg(theme.fg_dim),
        )),
        Line::from(Span::styled(
            "     │         │",
            Style::default().fg(theme.gray),
        )),
        Line::from(Span::styled(
            "     └─────────┘",
            Style::default().fg(theme.gray),
        )),
    ];
    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border_inactive))
                .style(Style::default().bg(theme.bg)),
        ),
        area,
    );
}

fn render_cover(
    f: &mut Frame,
    area: Rect,
    lines: &[(String, Vec<ratatui::style::Color>)],
    theme: &Theme,
) {
    let inner = area;
    let mut rendered = Vec::with_capacity(lines.len());

    for (text, colors) in lines {
        let spans = text
            .chars()
            .enumerate()
            .map(|(index, character)| {
                let color = colors.get(index).copied().unwrap_or(theme.purple);
                Span::styled(character.to_string(), Style::default().fg(color))
            })
            .collect::<Vec<_>>();
        rendered.push(Line::from(spans));
    }

    f.render_widget(
        Paragraph::new(rendered).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.purple))
                .style(Style::default().bg(theme.bg)),
        ),
        inner,
    );
}

fn render_library_and_queue(
    f: &mut Frame,
    area: Rect,
    mode: &Mode,
    browser: &FileBrowser,
    playlist: &Playlist,
    is_playing: bool,
    theme: &Theme,
) {
    let vertical = area.width < 100 || area.height < 18;
    let chunks = if vertical {
        Layout::vertical([Constraint::Percentage(52), Constraint::Percentage(48)]).split(area)
    } else {
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).split(area)
    };
    render_library(f, chunks[0], browser, *mode == Mode::Browser, theme);
    render_queue(
        f,
        chunks[1],
        playlist,
        *mode == Mode::Playlist,
        is_playing,
        theme,
    );
}

fn render_library(f: &mut Frame, area: Rect, browser: &FileBrowser, active: bool, theme: &Theme) {
    let items = browser
        .entries()
        .iter()
        .skip(browser.scroll())
        .take(20)
        .enumerate()
        .map(|(i, path)| {
            let index = i + browser.scroll();
            let name = path.to_string_lossy();
            let label = if name.starts_with("───") {
                name.to_string()
            } else {
                path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| name.to_string())
            };
            let style = if index == browser.selected() && active {
                Style::default()
                    .fg(theme.bg_dark)
                    .bg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else if name.starts_with("───") {
                Style::default().fg(theme.gray).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(if path.is_dir() { theme.blue } else { theme.fg })
            };
            ListItem::new(format!("  {}", label)).style(style)
        })
        .collect::<Vec<_>>();
    f.render_widget(
        List::new(items).block(card(
            "LIBRARY",
            area,
            if active {
                theme.accent
            } else {
                theme.border_inactive
            },
            theme,
        )),
        area,
    );
}

fn render_queue(
    f: &mut Frame,
    area: Rect,
    playlist: &Playlist,
    active: bool,
    is_playing: bool,
    theme: &Theme,
) {
    let items = playlist
        .tracks()
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let current = i == playlist.current_index();
            let icon = if current && is_playing {
                "▶"
            } else if current {
                "Ⅱ"
            } else {
                "·"
            };
            let style = if current {
                Style::default()
                    .fg(theme.fg)
                    .bg(theme.highlight_bg)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.fg_dim)
            };
            ListItem::new(format!(
                " {}  {:02}  {}",
                icon,
                i + 1,
                path.file_name().unwrap_or_default().to_string_lossy()
            ))
            .style(style)
        })
        .collect::<Vec<_>>();
    f.render_widget(
        List::new(items).block(card(
            "QUEUE",
            area,
            if active {
                theme.accent
            } else {
                theme.border_inactive
            },
            theme,
        )),
        area,
    );
}

fn render_command_bar(
    f: &mut Frame,
    area: Rect,
    mode: &Mode,
    playlist: &Playlist,
    volume: f32,
    theme: &Theme,
) {
    let repeat = match playlist.repeat {
        RepeatMode::Off => "OFF",
        RepeatMode::All => "ALL",
        RepeatMode::One => "ONE",
    };
    let focus = if matches!(mode, Mode::Playlist) {
        "QUEUE"
    } else {
        "LIBRARY"
    };
    let line = Line::from(vec![
        Span::styled(
            format!(" {} ", focus),
            Style::default()
                .fg(theme.bg_dark)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "  1 LIBRARY  2 QUEUE  3 NOW PLAYING  4 SETTINGS  ",
            Style::default().fg(theme.fg_dim),
        ),
        Span::styled(
            format!(
                "O settings  SPACE play  N/P track  / search  S shuffle  R repeat:{}  VOL:{:02.0}%  Q quit",
                repeat,
                volume * 100.0
            ),
            Style::default().fg(theme.fg),
        ),
    ]);
    f.render_widget(
        Paragraph::new(line).style(Style::default().bg(theme.bg_dark)),
        area,
    );
}

fn card<'a>(
    title: &'a str,
    _area: Rect,
    border: ratatui::style::Color,
    theme: &Theme,
) -> Block<'a> {
    Block::default()
        .title(format!(" {} ", title))
        .title_style(Style::default().fg(border).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(theme.bg))
}
