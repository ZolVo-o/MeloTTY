use ratatui::text::Span;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::Gauge,
    Frame,
};
use std::time::Duration;

use super::theme::Theme;

pub fn render(
    f: &mut Frame,
    area: Rect,
    track_position: Duration,
    track_duration: Option<Duration>,
    is_playing: bool,
    spectrum: &[f32],
    theme: &Theme,
) {
    let (progress, label) = if let Some(duration) = track_duration {
        let elapsed = track_position.min(duration);
        let ratio = if duration.is_zero() {
            0.0
        } else {
            (elapsed.as_secs_f64() / duration.as_secs_f64()).min(1.0)
        };
        let elapsed_str = format_duration(elapsed.min(duration));
        let total_str = format_duration(duration);
        let visualizer = if is_playing && area.width >= 56 {
            mini_spectrum(spectrum, area.width)
        } else {
            String::new()
        };
        (
            ratio,
            if visualizer.is_empty() {
                format!("{} / {}", elapsed_str, total_str)
            } else {
                format!("{}  {}  {}", elapsed_str, visualizer, total_str)
            },
        )
    } else {
        (0.0, " --:-- • --:-- ".to_string())
    };

    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(theme.progress_fg)
                .bg(theme.progress_bg)
                .add_modifier(Modifier::BOLD),
        )
        .label(Span::styled(
            label,
            Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
        ))
        .ratio(progress)
        .use_unicode(true);

    f.render_widget(gauge, area);
}

fn mini_spectrum(values: &[f32], width: u16) -> String {
    const BARS: &[char] = &['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    let available = width.saturating_sub(18) as usize;

    values
        .iter()
        .take(available.min(16))
        .map(|value| {
            let index = (value.clamp(0.0, 1.0) * (BARS.len() - 1) as f32) as usize;
            BARS[index]
        })
        .collect()
}

fn format_duration(d: Duration) -> String {
    let total_secs = d.as_secs();
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{}:{:02}", mins, secs)
    }
}
