use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Gauge, Block, Borders},
    symbols::border,
    Frame,
};
use ratatui::text::{Line, Span};
use std::time::{Duration, Instant};

use super::theme::Theme;

pub fn render_with_spectrum(
    f: &mut Frame,
    area: Rect,
    track_start: Option<Instant>,
    track_duration: Option<Duration>,
    is_playing: bool,
    spectrum: &[f32],
    theme: &Theme,
) {
    // Вычисляем прогресс и временные метки
    let (progress, elapsed_str, total_str) = if let (Some(start), Some(duration)) = (track_start, track_duration) {
        let elapsed = start.elapsed();
        let ratio = (elapsed.as_secs_f64() / duration.as_secs_f64()).min(1.0);
        let elapsed_str = format_duration(elapsed.min(duration));
        let total_str = format_duration(duration);
        (ratio, elapsed_str, total_str)
    } else {
        (0.0, "0:00".to_string(), "0:00".to_string())
    };

    // Цвет прогресс-бара в зависимости от состояния
    let gauge_color = if is_playing { 
        theme.gradient_start 
    } else { 
        theme.gray 
    };

    // Создаем мини-визуализацию спектра
    let spectrum_display = if is_playing && !spectrum.is_empty() {
        render_mini_spectrum(spectrum, area.width.saturating_sub(10) as usize, theme)
    } else {
        String::new()
    };

    // Формируем лейбл с временем
    let time_label = format!(" {} │{}│ {} ", 
        elapsed_str,
        if !spectrum_display.is_empty() { &spectrum_display } else { "" },
        total_str
    );

    // Создаем кастомный gauge с современным стилем
    let gauge = Gauge::default()
        .gauge_style(Style::default()
            .fg(gauge_color)
            .bg(theme.bg_darker)
            .add_modifier(if is_playing { Modifier::BOLD } else { Modifier::DIM }))
        .label(Span::from(Span::styled(
            time_label,
            Style::default().fg(theme.fg_dim),
        )))
        .ratio(progress)
        .use_unicode(true);

    // Блок вокруг прогресс-бара
    let block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default().bg(theme.bg_dark));

    let gauge_with_block = gauge.block(block);

    f.render_widget(gauge_with_block, area);
}

fn render_mini_spectrum(spectrum: &[f32], width: usize, theme: &Theme) -> String {
    if spectrum.is_empty() || width == 0 {
        return String::new();
    }

    let bars = ["▁", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
    let step = (spectrum.len() as f32 / width as f32).max(1.0) as usize;
    
    let mut result = String::new();
    for i in (0..spectrum.len()).step_by(step.max(1)) {
        let value = spectrum[i].min(1.0).max(0.0);
        let bar_index = (value * (bars.len() - 1) as f32) as usize;
        result.push_str(bars[bar_index]);
        
        if result.len() >= width {
            break;
        }
    }
    
    result
}

fn format_duration(d: Duration) -> String {
    let mins = d.as_secs() / 60;
    let secs = d.as_secs() % 60;
    format!("{}:{:02}", mins, secs)
}
