use std::{fs, path::PathBuf};

use anyhow::Result;

/// Resolve the app's config directory (e.g. ~/.config/oeis-tui) and create it if needed.
pub fn ensure_config_dir() -> Result<PathBuf> {
    let dir = config_dir()?;

    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Resolve the app's config directory (e.g. ~/.config/oeis-tui) without creating it.
pub fn config_dir() -> Result<PathBuf> {
    Ok(preferred_base().join("oeis-tui"))
}

fn preferred_base() -> PathBuf {
    // Force ~/.config on macOS to keep it hidden instead of ~/Library/Application Support.
    if cfg!(target_os = "macos") {
        if let Some(home) = dirs::home_dir() {
            return home.join(".config");
        }
    }

    if let Some(base) = dirs::config_dir() {
        return base;
    }

    if let Some(home) = dirs::home_dir() {
        return home.join(".config");
    }

    PathBuf::from(".config")
}

/// Helper to build a path inside the config directory (ensures the dir exists).
pub fn config_file(name: &str) -> Result<PathBuf> {
    Ok(ensure_config_dir()?.join(name))
}
