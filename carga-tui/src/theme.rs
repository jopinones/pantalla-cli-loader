use ratatui::style::Color;
use crate::app::Theme;

pub struct ThemeColors {
    pub term_bg:      Color,
    pub term_fg:      Color,
    pub term_dim:     Color,
    pub term_mute:    Color,
    pub rule:         Color,
    pub rule_strong:  Color,
    pub accent:       Color,
    pub ok:           Color,
    pub info:         Color,
    pub warn:         Color,
    pub err:          Color,
    pub selection_bg: Color,
}

pub static DARK: ThemeColors = ThemeColors {
    term_bg:      Color::Rgb(27,  26,  25),
    term_fg:      Color::Rgb(232, 226, 214),
    term_dim:     Color::Rgb(138, 132, 122),
    term_mute:    Color::Rgb(92,  86,  78),
    rule:         Color::Rgb(57,  52,  46),
    rule_strong:  Color::Rgb(74,  68,  59),
    accent:       Color::Rgb(226, 125, 96),
    ok:           Color::Rgb(143, 184, 122),
    info:         Color::Rgb(127, 169, 196),
    warn:         Color::Rgb(217, 183, 106),
    err:          Color::Rgb(199, 120, 115),
    selection_bg: Color::Rgb(60,  40,  32),
};

pub static LIGHT: ThemeColors = ThemeColors {
    term_bg:      Color::Rgb(251, 248, 242),
    term_fg:      Color::Rgb(31,  30,  28),
    term_dim:     Color::Rgb(111, 106, 96),
    term_mute:    Color::Rgb(160, 154, 142),
    rule:         Color::Rgb(226, 221, 211),
    rule_strong:  Color::Rgb(207, 200, 187),
    accent:       Color::Rgb(192, 73,  43),
    ok:           Color::Rgb(63,  140, 73),
    info:         Color::Rgb(61,  110, 148),
    warn:         Color::Rgb(156, 122, 31),
    err:          Color::Rgb(163, 58,  51),
    selection_bg: Color::Rgb(245, 234, 230),
};

pub static AUBERGINE: ThemeColors = ThemeColors {
    term_bg:      Color::Rgb(48,  10,  36),
    term_fg:      Color::Rgb(255, 255, 255),
    term_dim:     Color::Rgb(200, 174, 194),
    term_mute:    Color::Rgb(133, 98,  139),
    rule:         Color::Rgb(74,  27,  61),
    rule_strong:  Color::Rgb(94,  42,  79),
    accent:       Color::Rgb(233, 84,  32),
    ok:           Color::Rgb(115, 195, 109),
    info:         Color::Rgb(117, 161, 255),
    warn:         Color::Rgb(217, 183, 106),
    err:          Color::Rgb(199, 120, 115),
    selection_bg: Color::Rgb(80,  30,  55),
};

impl ThemeColors {
    pub fn get(theme: Theme) -> &'static ThemeColors {
        match theme {
            Theme::Dark      => &DARK,
            Theme::Light     => &LIGHT,
            Theme::Aubergine => &AUBERGINE,
        }
    }
}
