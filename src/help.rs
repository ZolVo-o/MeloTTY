use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols::border,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render_help(f: &mut Frame) {
    let area = f.area();

    let bg_block = Block::default().style(Style::default().bg(Color::Rgb(29, 32, 33)));
    f.render_widget(bg_block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .horizontal_margin(4)
        .vertical_margin(2)
        .split(area);

    let title = Paragraph::new("🎵 MeloTTY — Справка по управлению")
        .style(
            Style::default()
                .fg(Color::Rgb(142, 192, 124))
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED),
        );
    f.render_widget(title, chunks[0]);

    let help_text = vec![
        ("", "═ СТРАНИЦЫ ═", ""),
        ("1", "Открыть Library — браузер музыкальных файлов", ""),
        ("2", "Открыть Queue — очередь воспроизведения", ""),
        ("3", "Открыть Now Playing — текущий трек и обложка", ""),
        ("4 / O", "Открыть Settings", ""),
        ("Esc / B", "Вернуться из Settings в Library", ""),
        ("", "", ""),
        ("", "═ ВОСПРОИЗВЕДЕНИЕ ═", ""),
        ("Пробел", "Воспроизведение / пауза", ""),
        ("N / P", "Следующий / предыдущий трек", ""),
        ("+ / =", "Увеличить громкость", ""),
        ("-", "Уменьшить громкость", ""),
        ("5 / 0", "Установить громкость 50% / 100%", ""),
        ("S", "Включить или выключить перемешивание", ""),
        ("R", "Цикл повтора: выключен → все → один", ""),
        ("Q", "Выйти из MeloTTY", ""),
        ("H", "Открыть эту справку", ""),
        ("", "", ""),
        ("", "═ LIBRARY ═", ""),
        ("↑↓ / J/K", "Навигация по файлам и папкам", ""),
        ("Enter / → / L", "Войти в папку или воспроизвести файл", ""),
        ("Backspace / ←", "Вернуться в родительскую папку", ""),
        ("A", "Добавить выбранный файл или каталог в Queue", ""),
        ("/", "Поиск по библиотеке", ""),
        ("", "", ""),
        ("", "═ QUEUE ═", ""),
        ("↑↓ / J/K", "Навигация по трекам", ""),
        ("Enter", "Воспроизвести выбранный трек", ""),
        ("D", "Удалить текущий трек из Queue", ""),
        ("C", "Очистить Queue", ""),
        ("Tab", "Переключить фокус Library / Queue", ""),
        ("", "", ""),
        ("", "═ ФАЙЛЫ MELOTTY ═", ""),
        ("Config", "~/.config/melotty.conf", ""),
        ("Playlist", "~/.melotty_playlist.m3u", ""),
    ];

    let mut lines = Vec::new();
    for (key, desc, _) in help_text {
        if key.is_empty() && desc.starts_with("═") {
            lines.push(ratatui::text::Line::from(ratatui::text::Span::styled(
                format!("  {}", desc),
                Style::default()
                    .fg(Color::Rgb(250, 189, 47))
                    .add_modifier(Modifier::BOLD),
            )));
        } else if key.is_empty() {
            lines.push(ratatui::text::Line::from(""));
        } else {
            lines.push(ratatui::text::Line::from(vec![
                ratatui::text::Span::styled(
                    format!("  {:12}", key),
                    Style::default()
                        .fg(Color::Rgb(131, 165, 152))
                        .add_modifier(Modifier::BOLD),
                ),
                ratatui::text::Span::styled(desc, Style::default().fg(Color::Rgb(235, 219, 178))),
            ]));
        }
    }

    let help_paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(help_paragraph, chunks[1]);
}
