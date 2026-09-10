use ratatui::style::Color;

pub struct Theme {
    pub bg: Color,
    pub bg_dark: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub gray: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub purple: Color,
    pub accent: Color,
    pub border_inactive: Color,
    pub progress_bg: Color,
    pub progress_fg: Color,
    pub highlight_bg: Color,
}

pub const CATTPUCCIN: Theme = Theme {
    bg: Color::Rgb(30, 30, 46),
    bg_dark: Color::Rgb(24, 24, 37),
    fg: Color::Rgb(205, 214, 244),
    fg_dim: Color::Rgb(166, 173, 200),
    gray: Color::Rgb(88, 91, 112),
    green: Color::Rgb(166, 227, 161),
    yellow: Color::Rgb(249, 226, 175),
    blue: Color::Rgb(137, 180, 250),
    purple: Color::Rgb(203, 166, 247),
    accent: Color::Rgb(203, 166, 247),
    border_inactive: Color::Rgb(88, 91, 112),
    progress_bg: Color::Rgb(49, 50, 68),
    progress_fg: Color::Rgb(148, 226, 213),
    highlight_bg: Color::Rgb(88, 91, 112),
};
