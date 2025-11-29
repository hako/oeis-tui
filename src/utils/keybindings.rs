use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

/// User-editable keybinding configuration (stored as strings in settings.json).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyBindingsConfig {
    pub quit: Option<Vec<String>>,
    pub help: Option<Vec<String>>,
    pub about: Option<Vec<String>>,
    pub cycle_theme: Option<Vec<String>>,
    pub toggle_keybindings_preset: Option<Vec<String>>,
    pub preset: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub quit: Vec<KeyChord>,
    pub help: Vec<KeyChord>,
    pub about: Vec<KeyChord>,
    pub cycle_theme: Vec<KeyChord>,
    pub toggle_keybindings_preset: Vec<KeyChord>,
}

impl KeyBindings {
    pub fn from_config(config: Option<&KeyBindingsConfig>, preset: Option<&str>) -> Self {
        let default = Self::preset_or_default(preset);

        let resolve = |custom: Option<&Vec<String>>, fallback: &[KeyChord]| -> Vec<KeyChord> {
            if let Some(list) = custom {
                let parsed = list
                    .iter()
                    .filter_map(|s| KeyChord::parse(s))
                    .collect::<Vec<_>>();
                if !parsed.is_empty() {
                    return parsed;
                }
            }
            fallback.to_vec()
        };

        let fallback_config = KeyBindingsConfig::default();
        let config = config.unwrap_or(&fallback_config);

        Self {
            quit: resolve(config.quit.as_ref(), &default.quit),
            help: resolve(config.help.as_ref(), &default.help),
            about: resolve(config.about.as_ref(), &default.about),
            cycle_theme: resolve(config.cycle_theme.as_ref(), &default.cycle_theme),
            toggle_keybindings_preset: resolve(
                config.toggle_keybindings_preset.as_ref(),
                &default.toggle_keybindings_preset,
            ),
        }
    }

    pub fn is_quit(&self, key: &KeyEvent) -> bool {
        self.quit.iter().any(|c| c.matches(key))
    }

    pub fn is_help(&self, key: &KeyEvent) -> bool {
        self.help.iter().any(|c| c.matches(key))
    }

    pub fn is_about(&self, key: &KeyEvent) -> bool {
        self.about.iter().any(|c| c.matches(key))
    }

    pub fn is_cycle_theme(&self, key: &KeyEvent) -> bool {
        self.cycle_theme.iter().any(|c| c.matches(key))
    }

    pub fn is_toggle_keybindings_preset(&self, key: &KeyEvent) -> bool {
        self.toggle_keybindings_preset
            .iter()
            .any(|c| c.matches(key))
    }

    pub fn preset_or_default(preset: Option<&str>) -> Self {
        match preset {
            Some("vim") => Self::vim(),
            _ => Self::default(),
        }
    }

    pub fn vim() -> Self {
        Self {
            quit: vec![
                KeyChord::new(KeyCode::Char('q'), KeyModifiers::NONE),
                KeyChord::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            ],
            help: vec![KeyChord::new(KeyCode::Char('?'), KeyModifiers::NONE)],
            about: vec![KeyChord::new(KeyCode::Char('a'), KeyModifiers::NONE)],
            cycle_theme: vec![KeyChord::new(KeyCode::Char('t'), KeyModifiers::NONE)],
            toggle_keybindings_preset: vec![KeyChord::new(
                KeyCode::Char('v'),
                KeyModifiers::CONTROL,
            )],
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit: vec![
                KeyChord::new(KeyCode::Char('q'), KeyModifiers::CONTROL),
                KeyChord::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            ],
            help: vec![KeyChord::new(KeyCode::Char('h'), KeyModifiers::CONTROL)],
            about: vec![KeyChord::new(KeyCode::Char('a'), KeyModifiers::CONTROL)],
            cycle_theme: vec![KeyChord::new(KeyCode::Char('t'), KeyModifiers::CONTROL)],
            toggle_keybindings_preset: vec![KeyChord::new(
                KeyCode::Char('v'),
                KeyModifiers::CONTROL,
            )],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyChord {
    code: KeyCode,
    modifiers: KeyModifiers,
}

impl KeyChord {
    pub fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    pub fn matches(&self, key: &KeyEvent) -> bool {
        key.code == self.code && key.modifiers.contains(self.modifiers)
    }

    fn parse(input: &str) -> Option<Self> {
        let mut modifiers = KeyModifiers::empty();
        let mut code_str: Option<String> = None;

        for raw in input.split('+') {
            let part = raw.trim().to_lowercase();
            match part.as_str() {
                "ctrl" | "control" => {
                    modifiers.insert(KeyModifiers::CONTROL);
                }
                "alt" | "meta" => {
                    modifiers.insert(KeyModifiers::ALT);
                }
                "shift" => {
                    modifiers.insert(KeyModifiers::SHIFT);
                }
                "" => {}
                other => {
                    code_str = Some(other.to_string());
                }
            }
        }

        let code = parse_key_code(code_str.as_deref().unwrap_or_default())?;
        Some(Self::new(code, modifiers))
    }
}

fn parse_key_code(s: &str) -> Option<KeyCode> {
    match s {
        "enter" => Some(KeyCode::Enter),
        "esc" | "escape" => Some(KeyCode::Esc),
        "tab" => Some(KeyCode::Tab),
        "backtab" | "shift+tab" => Some(KeyCode::BackTab),
        "up" => Some(KeyCode::Up),
        "down" => Some(KeyCode::Down),
        "left" => Some(KeyCode::Left),
        "right" => Some(KeyCode::Right),
        "home" => Some(KeyCode::Home),
        "end" => Some(KeyCode::End),
        "pageup" | "page_up" => Some(KeyCode::PageUp),
        "pagedown" | "page_down" => Some(KeyCode::PageDown),
        _ => {
            if s.len() == 1 {
                return s.chars().next().map(KeyCode::Char);
            }
            None
        }
    }
}
