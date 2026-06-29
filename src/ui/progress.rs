use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Gauge,
    Frame,
};
use ratatui::text::Span;
use std::time::{Duration, Instant};

use super::theme::Theme;

pub fn render(
    f: &mut Frame,
    area: Rect,
    track_start: Option<Instant>,
    track_duration: Option<Duration>,
    is_playing: bool,
    theme: &Theme,
) {
    let (progress, label) = if let (Some(start), Some(duration)) = (track_start, track_duration) {
        let elapsed = start.elapsed();
        let ratio = (elapsed.as_secs_f64() / duration.as_secs_f64()).min(1.0);
        let elapsed_str = format_duration(elapsed.min(duration));
        let total_str = format_duration(duration);
        (ratio, format!(" {} / {} ", elapsed_str, total_str))
    } else {
        (0.0, " 0:00 / 0:00 ".to_string())
    };

    let gauge_color = if is_playing { theme.aqua } else { theme.gray };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(gauge_color).bg(theme.bg))
        .label(Span::styled(label, Style::default().fg(theme.fg).add_modifier(Modifier::BOLD)))
        .ratio(progress)
        .use_unicode(true);

    f.render_widget(gauge, area);
}

fn format_duration(d: Duration) -> String {
    let mins = d.as_secs() / 60;
    let secs = d.as_secs() % 60;
    format!("{}:{:02}", mins, secs)
}
