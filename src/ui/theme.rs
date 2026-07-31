use ratatui::style::Color;

pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub bg_dark: Color,
    pub bg_darker: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub gray: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub cyan: Color,
    pub magenta: Color,
    pub orange: Color,
    pub pink: Color,
    pub gradient_start: Color,
    pub gradient_end: Color,
}

// Современная тема в стиле Cyberpunk/Neon
pub const CYBERPUNK: Theme = Theme {
    name: "Cyberpunk",
    bg: Color::Rgb(15, 15, 35),
    bg_dark: Color::Rgb(10, 10, 25),
    bg_darker: Color::Rgb(5, 5, 15),
    fg: Color::Rgb(240, 240, 255),
    fg_dim: Color::Rgb(150, 150, 180),
    gray: Color::Rgb(80, 80, 100),
    red: Color::Rgb(255, 85, 100),
    green: Color::Rgb(80, 255, 120),
    yellow: Color::Rgb(255, 220, 80),
    blue: Color::Rgb(80, 180, 255),
    cyan: Color::Rgb(80, 240, 255),
    magenta: Color::Rgb(255, 100, 200),
    orange: Color::Rgb(255, 150, 80),
    pink: Color::Rgb(255, 120, 180),
    gradient_start: Color::Rgb(120, 80, 255),
    gradient_end: Color::Rgb(80, 200, 255),
};

// Тема в стиле Synthwave
pub const SYNTHWAVE: Theme = Theme {
    name: "Synthwave",
    bg: Color::Rgb(25, 20, 40),
    bg_dark: Color::Rgb(18, 15, 30),
    bg_darker: Color::Rgb(10, 8, 20),
    fg: Color::Rgb(255, 240, 250),
    fg_dim: Color::Rgb(180, 160, 200),
    gray: Color::Rgb(90, 80, 110),
    red: Color::Rgb(255, 100, 120),
    green: Color::Rgb(100, 255, 150),
    yellow: Color::Rgb(255, 230, 100),
    blue: Color::Rgb(100, 200, 255),
    cyan: Color::Rgb(100, 255, 240),
    magenta: Color::Rgb(255, 120, 220),
    orange: Color::Rgb(255, 170, 100),
    pink: Color::Rgb(255, 140, 200),
    gradient_start: Color::Rgb(255, 100, 180),
    gradient_end: Color::Rgb(180, 100, 255),
};

// Минималистичная современная тема
pub const MODERN_MINIMAL: Theme = Theme {
    name: "Modern Minimal",
    bg: Color::Rgb(30, 30, 35),
    bg_dark: Color::Rgb(20, 20, 25),
    bg_darker: Color::Rgb(10, 10, 15),
    fg: Color::Rgb(235, 235, 240),
    fg_dim: Color::Rgb(140, 140, 150),
    gray: Color::Rgb(90, 90, 100),
    red: Color::Rgb(255, 100, 100),
    green: Color::Rgb(100, 255, 130),
    yellow: Color::Rgb(255, 240, 120),
    blue: Color::Rgb(100, 180, 255),
    cyan: Color::Rgb(100, 240, 240),
    magenta: Color::Rgb(240, 140, 240),
    orange: Color::Rgb(255, 180, 100),
    pink: Color::Rgb(255, 150, 180),
    gradient_start: Color::Rgb(100, 180, 255),
    gradient_end: Color::Rgb(100, 255, 200),
};

// Обновленная Gruvbox с современными акцентами
pub const GRUVBOX: Theme = Theme {
    name: "Gruvbox Modern",
    bg: Color::Rgb(40, 40, 40),
    bg_dark: Color::Rgb(29, 32, 33),
    bg_darker: Color::Rgb(20, 22, 23),
    fg: Color::Rgb(235, 219, 178),
    fg_dim: Color::Rgb(168, 153, 132),
    gray: Color::Rgb(102, 92, 84),
    red: Color::Rgb(251, 73, 52),
    green: Color::Rgb(184, 187, 38),
    yellow: Color::Rgb(250, 189, 47),
    blue: Color::Rgb(131, 165, 152),
    cyan: Color::Rgb(142, 192, 124),
    magenta: Color::Rgb(211, 134, 155),
    orange: Color::Rgb(254, 128, 25),
    pink: Color::Rgb(255, 140, 170),
    gradient_start: Color::Rgb(254, 128, 25),
    gradient_end: Color::Rgb(184, 187, 38),
};

pub const NORD: Theme = Theme {
    name: "Nord",
    bg: Color::Rgb(46, 52, 64),
    bg_dark: Color::Rgb(41, 45, 56),
    bg_darker: Color::Rgb(35, 39, 48),
    fg: Color::Rgb(216, 222, 233),
    fg_dim: Color::Rgb(160, 170, 185),
    gray: Color::Rgb(76, 86, 106),
    red: Color::Rgb(191, 97, 106),
    green: Color::Rgb(163, 190, 140),
    yellow: Color::Rgb(235, 203, 139),
    blue: Color::Rgb(129, 161, 193),
    cyan: Color::Rgb(136, 192, 208),
    magenta: Color::Rgb(180, 142, 173),
    orange: Color::Rgb(208, 135, 112),
    pink: Color::Rgb(220, 150, 180),
    gradient_start: Color::Rgb(129, 161, 193),
    gradient_end: Color::Rgb(163, 190, 140),
};

pub const DRACULA: Theme = Theme {
    name: "Dracula",
    bg: Color::Rgb(40, 42, 54),
    bg_dark: Color::Rgb(33, 34, 44),
    bg_darker: Color::Rgb(25, 26, 35),
    fg: Color::Rgb(248, 248, 242),
    fg_dim: Color::Rgb(180, 180, 175),
    gray: Color::Rgb(98, 114, 164),
    red: Color::Rgb(255, 85, 85),
    green: Color::Rgb(80, 250, 123),
    yellow: Color::Rgb(241, 250, 140),
    blue: Color::Rgb(189, 147, 249),
    cyan: Color::Rgb(139, 233, 253),
    magenta: Color::Rgb(255, 121, 198),
    orange: Color::Rgb(255, 184, 108),
    pink: Color::Rgb(255, 150, 200),
    gradient_start: Color::Rgb(255, 121, 198),
    gradient_end: Color::Rgb(189, 147, 249),
};

impl Theme {
    pub fn all_themes() -> Vec<&'static Theme> {
        vec![&CYBERPUNK, &SYNTHWAVE, &MODERN_MINIMAL, &GRUVBOX, &NORD, &DRACULA]
    }
}
