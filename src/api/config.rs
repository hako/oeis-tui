use std::fs;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{
    i18n::Language,
    ui::animation::WelcomeAnimationMode,
    utils::{
        keybindings::{KeyBindings, KeyBindingsConfig},
        paths::{config_file, ensure_config_dir},
    },
};

/// User settings stored on disk.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserSettings {
    pub language: Option<String>,
    pub theme: Option<usize>,
    pub welcome_animation: Option<String>,
    pub welcome_animation_played: Option<bool>,
    #[serde(default)]
    pub keybindings: Option<KeyBindingsConfig>,
    pub keybindings_preset: Option<String>,
}

impl UserSettings {
    pub fn load() -> Self {
        if ensure_config_dir().is_err() {
            return Self::default();
        }

        let path = match config_file("settings.json") {
            Ok(path) => path,
            Err(_) => return Self::default(),
        };

        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }

        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = config_file("settings.json")?;
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents).context("Failed to write settings file")?;
        Ok(())
    }

    pub fn language(&self) -> Option<Language> {
        self.language.as_deref().and_then(Language::from_code)
    }

    pub fn welcome_animation_mode(&self) -> Option<WelcomeAnimationMode> {
        self.welcome_animation
            .as_deref()
            .and_then(WelcomeAnimationMode::from_key)
    }

    pub fn keybindings(&self) -> KeyBindings {
        KeyBindings::from_config(
            self.keybindings.as_ref(),
            self.keybindings_preset.as_deref(),
        )
    }
}
