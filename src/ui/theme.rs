use ratatui::style::Color;

pub struct Theme {
    pub bg: Color,
    pub bg_dark: Color,
    pub fg: Color,
    pub gray: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub aqua: Color,
    pub purple: Color,
    pub orange: Color,
}

pub const GRUVBOX: Theme = Theme {
    bg: Color::Rgb(40, 40, 40),
    bg_dark: Color::Rgb(29, 32, 33),
    fg: Color::Rgb(235, 219, 178),
    gray: Color::Rgb(102, 92, 84),
    red: Color::Rgb(251, 73, 52),
    green: Color::Rgb(184, 187, 38),
    yellow: Color::Rgb(250, 189, 47),
    blue: Color::Rgb(131, 165, 152),
    aqua: Color::Rgb(142, 192, 124),
    purple: Color::Rgb(211, 134, 155),
    orange: Color::Rgb(254, 128, 25),
};

pub const NORD: Theme = Theme {
    bg: Color::Rgb(46, 52, 64),
    bg_dark: Color::Rgb(41, 45, 56),
    fg: Color::Rgb(216, 222, 233),
    gray: Color::Rgb(76, 86, 106),
    red: Color::Rgb(191, 97, 106),
    green: Color::Rgb(163, 190, 140),
    yellow: Color::Rgb(235, 203, 139),
    blue: Color::Rgb(129, 161, 193),
    aqua: Color::Rgb(136, 192, 208),
    purple: Color::Rgb(180, 142, 173),
    orange: Color::Rgb(208, 135, 112),
};

pub const DRACULA: Theme = Theme {
    bg: Color::Rgb(40, 42, 54),
    bg_dark: Color::Rgb(33, 34, 44),
    fg: Color::Rgb(248, 248, 242),
    gray: Color::Rgb(98, 114, 164),
    red: Color::Rgb(255, 85, 85),
    green: Color::Rgb(80, 250, 123),
    yellow: Color::Rgb(241, 250, 140),
    blue: Color::Rgb(189, 147, 249),
    aqua: Color::Rgb(139, 233, 253),
    purple: Color::Rgb(255, 121, 198),
    orange: Color::Rgb(255, 184, 108),
};
