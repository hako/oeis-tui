use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    #[allow(dead_code)]
    pub name: &'static str,
    palette: Palette,
}

#[derive(Debug, Clone)]
struct Palette {
    accent: Color,
    #[allow(dead_code)]
    accent_dim: Color,
    highlight: Color,
    highlight_bg: Color,
    text: Color,
    muted: Color,
    success: Color,
    #[allow(dead_code)]
    warning: Color,
    danger: Color,
    background: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            name: "Dark",
            palette: Palette {
                accent: Color::Cyan,
                accent_dim: Color::Rgb(120, 200, 220),
                highlight: Color::Yellow,
                highlight_bg: Color::Rgb(60, 60, 60),
                text: Color::White,
                muted: Color::DarkGray,
                success: Color::Green,
                warning: Color::LightYellow,
                danger: Color::Red,
                background: Color::Black,
            },
        }
    }

    pub fn light() -> Self {
        Self {
            name: "Light",
            palette: Palette {
                accent: Color::Blue,
                accent_dim: Color::Rgb(40, 100, 180),
                highlight: Color::Rgb(210, 140, 0),
                highlight_bg: Color::Rgb(240, 220, 150),
                text: Color::Black,
                muted: Color::Gray,
                success: Color::Green,
                warning: Color::Rgb(200, 120, 0),
                danger: Color::Red,
                background: Color::White,
            },
        }
    }

    pub fn dracula() -> Self {
        Self {
            name: "Dracula",
            palette: Palette {
                accent: Color::Rgb(189, 147, 249),
                accent_dim: Color::Rgb(98, 114, 164),
                highlight: Color::Rgb(255, 121, 198),
                highlight_bg: Color::Rgb(68, 71, 90),
                text: Color::Rgb(248, 248, 242),
                muted: Color::Rgb(98, 114, 164),
                success: Color::Rgb(80, 250, 123),
                warning: Color::Rgb(241, 250, 140),
                danger: Color::Rgb(255, 85, 85),
                background: Color::Rgb(40, 42, 54),
            },
        }
    }

    pub fn nord() -> Self {
        Self {
            name: "Nord",
            palette: Palette {
                accent: Color::Rgb(143, 188, 187),
                accent_dim: Color::Rgb(94, 129, 172),
                highlight: Color::Rgb(191, 97, 106),
                highlight_bg: Color::Rgb(67, 76, 94),
                text: Color::Rgb(216, 222, 233),
                muted: Color::Rgb(136, 192, 208),
                success: Color::Rgb(163, 190, 140),
                warning: Color::Rgb(235, 203, 139),
                danger: Color::Rgb(191, 97, 106),
                background: Color::Rgb(46, 52, 64),
            },
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            name: "Gruvbox",
            palette: Palette {
                accent: Color::Rgb(215, 153, 33),
                accent_dim: Color::Rgb(204, 136, 34),
                highlight: Color::Rgb(231, 111, 81),
                highlight_bg: Color::Rgb(60, 56, 54),
                text: Color::Rgb(235, 219, 178),
                muted: Color::Rgb(168, 153, 132),
                success: Color::Rgb(152, 151, 26),
                warning: Color::Rgb(250, 189, 47),
                danger: Color::Rgb(204, 36, 29),
                background: Color::Rgb(40, 40, 40),
            },
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            name: "Solarized Dark",
            palette: Palette {
                accent: Color::Rgb(38, 139, 210),
                accent_dim: Color::Rgb(7, 54, 66),
                highlight: Color::Rgb(181, 137, 0),
                highlight_bg: Color::Rgb(0, 43, 54),
                text: Color::Rgb(253, 246, 227),
                muted: Color::Rgb(133, 153, 0),
                success: Color::Rgb(133, 153, 0),
                warning: Color::Rgb(203, 75, 22),
                danger: Color::Rgb(220, 50, 47),
                background: Color::Rgb(0, 43, 54),
            },
        }
    }

    pub fn solarized_light() -> Self {
        Self {
            name: "Solarized Light",
            palette: Palette {
                accent: Color::Rgb(38, 139, 210),
                accent_dim: Color::Rgb(131, 148, 150),
                highlight: Color::Rgb(181, 137, 0),
                highlight_bg: Color::Rgb(238, 232, 213),
                text: Color::Rgb(101, 123, 131),
                muted: Color::Rgb(147, 161, 161),
                success: Color::Rgb(133, 153, 0),
                warning: Color::Rgb(203, 75, 22),
                danger: Color::Rgb(211, 54, 130),
                background: Color::Rgb(253, 246, 227),
            },
        }
    }

    pub fn monokai() -> Self {
        Self {
            name: "Monokai",
            palette: Palette {
                accent: Color::Rgb(249, 38, 114),
                accent_dim: Color::Rgb(102, 217, 239),
                highlight: Color::Rgb(253, 151, 31),
                highlight_bg: Color::Rgb(39, 40, 34),
                text: Color::Rgb(248, 248, 242),
                muted: Color::Rgb(117, 113, 94),
                success: Color::Rgb(166, 226, 46),
                warning: Color::Rgb(253, 151, 31),
                danger: Color::Rgb(249, 38, 114),
                background: Color::Rgb(39, 40, 34),
            },
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha",
            palette: Palette {
                accent: Color::Rgb(137, 180, 250),
                accent_dim: Color::Rgb(108, 112, 134),
                highlight: Color::Rgb(245, 169, 127),
                highlight_bg: Color::Rgb(49, 50, 68),
                text: Color::Rgb(205, 214, 244),
                muted: Color::Rgb(127, 132, 156),
                success: Color::Rgb(166, 227, 161),
                warning: Color::Rgb(249, 226, 175),
                danger: Color::Rgb(243, 139, 168),
                background: Color::Rgb(30, 30, 46),
            },
        }
    }

    pub fn one_dark() -> Self {
        Self {
            name: "One Dark",
            palette: Palette {
                accent: Color::Rgb(97, 175, 239),
                accent_dim: Color::Rgb(92, 99, 112),
                highlight: Color::Rgb(229, 192, 123),
                highlight_bg: Color::Rgb(40, 44, 52),
                text: Color::Rgb(171, 178, 191),
                muted: Color::Rgb(76, 82, 99),
                success: Color::Rgb(152, 195, 121),
                warning: Color::Rgb(229, 192, 123),
                danger: Color::Rgb(224, 108, 117),
                background: Color::Rgb(40, 44, 52),
            },
        }
    }

    pub fn night_owl() -> Self {
        Self {
            name: "Night Owl",
            palette: Palette {
                accent: Color::Rgb(120, 220, 232),
                accent_dim: Color::Rgb(89, 158, 173),
                highlight: Color::Rgb(255, 203, 107),
                highlight_bg: Color::Rgb(13, 32, 48),
                text: Color::Rgb(199, 221, 236),
                muted: Color::Rgb(102, 129, 153),
                success: Color::Rgb(130, 200, 160),
                warning: Color::Rgb(255, 203, 107),
                danger: Color::Rgb(255, 88, 116),
                background: Color::Rgb(1, 22, 39),
            },
        }
    }

    pub fn phosphor_night() -> Self {
        Self {
            name: "Phosphor Night",
            palette: Palette {
                // Authentic P3 phosphor CRT - black and green
                accent: Color::Rgb(51, 255, 51), // #33FF33 - authentic P3 phosphor green
                accent_dim: Color::Rgb(40, 200, 40), // #28C828 - dimmed phosphor
                highlight: Color::Rgb(255, 220, 100), // #FFDC64 - warm amber glow
                highlight_bg: Color::Rgb(15, 20, 15), // #0F140F - near-black CRT
                text: Color::Rgb(200, 255, 200), // #C8FFC8 - phosphor white with green tint
                muted: Color::Rgb(100, 150, 100), // #649664 - darker phosphor
                success: Color::Rgb(80, 255, 120), // #50FF78 - bright phosphor
                warning: Color::Rgb(255, 200, 80), // #FFC850 - amber warning
                danger: Color::Rgb(255, 100, 80), // #FF6450 - warm red glow
                background: Color::Black,
            },
        }
    }

    pub fn punchcard_light() -> Self {
        Self {
            name: "Punchcard Light",
            palette: Palette {
                accent: Color::Rgb(15, 98, 254), // #0F62FE - IBM Blue 60
                accent_dim: Color::Rgb(78, 70, 180), // #4E46B4 - IBM Purple 60
                highlight: Color::Rgb(198, 120, 30), // #C6781E - warmer orange
                highlight_bg: Color::Rgb(255, 252, 245), // #FFFCF5 - warm ivory paper
                text: Color::Rgb(22, 22, 22),    // #161616 - IBM Carbon Gray 100
                muted: Color::Rgb(110, 110, 110), // #6E6E6E - IBM Gray 60
                success: Color::Rgb(36, 161, 72), // #24A148 - IBM Green 50
                warning: Color::Rgb(255, 131, 43), // #FF832B - IBM Orange 40
                danger: Color::Rgb(218, 30, 40), // #DA1E28 - IBM Red 60
                background: Color::Rgb(255, 252, 245), // #FFFCF5 - warm ivory paper
            },
        }
    }

    pub fn terminal_trove() -> Self {
        Self {
            name: "Terminal Trove",
            palette: Palette {
                accent: Color::Rgb(185, 255, 172), // #B9FFAC - Terminal Trove brand green
                accent_dim: Color::Rgb(119, 255, 96), // #77FF60 - hover state
                highlight: Color::Rgb(255, 255, 255), // #FFFFFF - white for contrast
                highlight_bg: Color::Rgb(30, 35, 30), // #1E231E - dark with green tint
                text: Color::Rgb(230, 255, 225),   // #E6FFE1 - green-tinted white
                muted: Color::Rgb(120, 140, 120),  // #788C78 - muted green-gray
                success: Color::Rgb(185, 255, 172), // #B9FFAC - brand green
                warning: Color::Rgb(255, 220, 120), // #FFDC78 - warm yellow
                danger: Color::Rgb(255, 100, 120), // #FF6478 - softer red
                background: Color::Black,
            },
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn accent(&self) -> Style {
        Style::default().fg(self.palette.accent)
    }

    pub fn accent_bold(&self) -> Style {
        self.accent().add_modifier(Modifier::BOLD)
    }

    #[allow(dead_code)]
    pub fn accent_dim(&self) -> Style {
        Style::default().fg(self.palette.accent_dim)
    }

    pub fn highlight(&self) -> Style {
        Style::default().fg(self.palette.highlight)
    }

    pub fn highlight_bold(&self) -> Style {
        self.highlight().add_modifier(Modifier::BOLD)
    }

    pub fn highlight_bg(&self) -> Style {
        Style::default()
            .bg(self.palette.highlight_bg)
            .add_modifier(Modifier::BOLD)
    }

    pub fn text(&self) -> Style {
        Style::default().fg(self.palette.text)
    }

    pub fn muted(&self) -> Style {
        Style::default().fg(self.palette.muted)
    }

    pub fn success(&self) -> Style {
        Style::default().fg(self.palette.success)
    }

    #[allow(dead_code)]
    pub fn warning(&self) -> Style {
        Style::default().fg(self.palette.warning)
    }

    pub fn danger(&self) -> Style {
        Style::default().fg(self.palette.danger)
    }

    pub fn selected_border(&self) -> Style {
        Style::default().fg(self.palette.highlight)
    }

    pub fn placeholder(&self) -> Style {
        self.muted()
    }

    pub fn accent_color(&self) -> Color {
        self.palette.accent
    }

    pub fn highlight_color(&self) -> Color {
        self.palette.highlight
    }

    pub fn highlight_bg_color(&self) -> Color {
        self.palette.highlight_bg
    }

    pub fn text_color(&self) -> Color {
        self.palette.text
    }

    pub fn muted_color(&self) -> Color {
        self.palette.muted
    }

    pub fn success_color(&self) -> Color {
        self.palette.success
    }

    #[allow(dead_code)]
    pub fn warning_color(&self) -> Color {
        self.palette.warning
    }

    pub fn danger_color(&self) -> Color {
        self.palette.danger
    }

    pub fn background(&self) -> Style {
        Style::default().bg(self.palette.background)
    }
}
