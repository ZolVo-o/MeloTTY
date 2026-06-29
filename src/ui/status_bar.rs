use ratatui::{
    layout::Rect,
    style::Style,
    widgets::Paragraph,
    Frame,
};
use ratatui::text::{Line, Span};

use crate::playlist::{Playlist, RepeatMode};
use super::theme::Theme;

pub fn render(f: &mut Frame, area: Rect, playlist: &Playlist, volume: f32, theme: &Theme) {
    let shuffle = if playlist.shuffle {
        Span::styled("🔀", Style::default().fg(theme.green))
    } else {
        Span::styled("🔀", Style::default().fg(theme.gray))
    };

    let repeat = match playlist.repeat {
        RepeatMode::Off => Span::styled("🔁", Style::default().fg(theme.gray)),
        RepeatMode::All => Span::styled("🔁", Style::default().fg(theme.green)),
        RepeatMode::One => Span::styled("🔂", Style::default().fg(theme.blue)),
    };

    let vol_icon = if volume > 0.7 { "🔊" } else if volume > 0.3 { "🔉" } else if volume > 0.0 { "🔈" } else { "🔇" };

    let tab_style = Style::default().bg(theme.gray).fg(theme.bg_dark);
    let quit_style = Style::default().bg(theme.red).fg(theme.bg_dark);

    let line = Line::from(vec![
        Span::styled(" Tab ", tab_style),
        Span::raw(" switch "),
        Span::styled(" Space ", tab_style),
        Span::raw(" play "),
        Span::styled(" n/p ", tab_style),
        Span::raw(" next/prev "),
        Span::styled(" / ", tab_style),
        Span::raw(" search "),
        Span::styled(" 0/1/5 ", tab_style),
        Span::raw(" vol "),
        Span::raw(" │ "),
        shuffle,
        Span::raw(" "),
        repeat,
        Span::raw(" │ "),
        Span::raw(vol_icon),
        Span::raw(format!(" {:.0}% ", volume * 100.0)),
        Span::styled(" q ", quit_style),
        Span::raw(" quit"),
    ]);

    f.render_widget(Paragraph::new(line).style(Style::default().fg(theme.gray)), area);
}
