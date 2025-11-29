use crate::app::{App, SettingsFocus};
use crate::i18n::Language;
use crate::ui::animation::WelcomeAnimationMode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Settings list
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    render_title(f, chunks[0], app);
    render_settings(f, chunks[1], app);
    render_help(f, chunks[2], app);
}

fn render_title(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let title_text = vec![Line::from(Span::styled(
        app.i18n.t("settings-title"),
        theme.accent_bold(),
    ))];

    let paragraph = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    format!(" {} ", app.i18n.t("settings-block-settings")),
                    theme.accent_bold(),
                ))
                .border_style(theme.accent()),
        );

    f.render_widget(paragraph, area);
}

fn render_settings(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(35),
            Constraint::Percentage(25),
        ])
        .split(area);

    render_language_settings(f, chunks[0], app);
    render_theme_settings(f, chunks[1], app);
    render_animation_settings(f, chunks[2], app);
}

fn render_language_settings(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let current_language = app.i18n.get_current_language();
    let languages = Language::all();
    let focused = matches!(app.settings_focus, SettingsFocus::Language);

    let language_items: Vec<ListItem> = languages
        .iter()
        .enumerate()
        .map(|(idx, lang)| {
            let is_current = *lang == current_language;
            let is_selected = idx == app.settings_selected_language;

            let highlight_style = if is_selected {
                theme.highlight_bold()
            } else if is_current {
                theme.success()
            } else {
                theme.text()
            };

            let arrow = if is_selected { "▶ " } else { "  " };
            let indicator = if is_current { "● " } else { "○ " };

            ListItem::new(Line::from(vec![
                Span::styled(
                    arrow,
                    if is_selected {
                        theme.highlight()
                    } else {
                        theme.muted()
                    },
                ),
                Span::styled(
                    indicator,
                    if is_current {
                        theme.success()
                    } else {
                        theme.muted()
                    },
                ),
                Span::styled(lang.name(), highlight_style),
                Span::raw(" ("),
                Span::styled(lang.code(), theme.muted()),
                Span::raw(")"),
            ]))
        })
        .collect();

    let settings_list = List::new(language_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Language / 言語 / Langue / Idioma / 언어 / 语言 ",
                    theme.accent_bold(),
                ))
                .border_style(if focused {
                    theme.selected_border()
                } else {
                    theme.accent()
                }),
        )
        .highlight_style(theme.highlight());

    let mut state = ListState::default()
        .with_selected(Some(app.settings_selected_language))
        .with_offset(app.settings_language_scroll as usize);

    f.render_stateful_widget(settings_list, area, &mut state);
}

fn render_theme_settings(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let focused = matches!(app.settings_focus, SettingsFocus::Theme);

    let theme_items: Vec<ListItem> = app
        .themes
        .iter()
        .enumerate()
        .map(|(idx, candidate)| {
            let is_active = idx == app.active_theme;
            let is_selected = idx == app.settings_selected_theme;

            let arrow = if is_selected { "▶ " } else { "  " };
            let indicator = if is_active { "● " } else { "○ " };

            ListItem::new(Line::from(vec![
                Span::styled(
                    arrow,
                    if is_selected {
                        theme.highlight()
                    } else {
                        theme.muted()
                    },
                ),
                Span::styled(
                    indicator,
                    if is_active {
                        theme.success()
                    } else {
                        theme.muted()
                    },
                ),
                Span::styled(
                    candidate.name(),
                    if is_selected {
                        theme.highlight_bold()
                    } else if is_active {
                        theme.accent_bold()
                    } else {
                        theme.text()
                    },
                ),
            ]))
        })
        .collect();

    let theme_list = List::new(theme_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    format!(" {} ", app.i18n.t("settings-block-themes")),
                    theme.accent_bold(),
                ))
                .border_style(if focused {
                    theme.selected_border()
                } else {
                    theme.accent()
                }),
        )
        .highlight_style(theme.highlight());

    let mut state = ListState::default()
        .with_selected(Some(app.settings_selected_theme))
        .with_offset(app.settings_theme_scroll as usize);

    f.render_stateful_widget(theme_list, area, &mut state);
}

fn render_animation_settings(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let focused = matches!(app.settings_focus, SettingsFocus::Animation);
    let modes = WelcomeAnimationMode::modes();

    let items: Vec<ListItem> = modes
        .iter()
        .enumerate()
        .map(|(idx, mode)| {
            let is_active = *mode == app.welcome_animation_mode;
            let is_selected = idx == app.settings_selected_animation;

            let arrow = if is_selected { "▶ " } else { "  " };
            let indicator = if is_active { "● " } else { "○ " };

            ListItem::new(Line::from(vec![
                Span::styled(
                    arrow,
                    if is_selected {
                        theme.highlight()
                    } else {
                        theme.muted()
                    },
                ),
                Span::styled(
                    indicator,
                    if is_active {
                        theme.success()
                    } else {
                        theme.muted()
                    },
                ),
                Span::styled(
                    mode.label(),
                    if is_selected {
                        theme.highlight_bold()
                    } else if is_active {
                        theme.accent_bold()
                    } else {
                        theme.text()
                    },
                ),
                Span::raw(" — "),
                Span::styled(mode.description(), theme.muted()),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    format!(" {} ", app.i18n.t("settings-block-animation")),
                    theme.accent_bold(),
                ))
                .border_style(if focused {
                    theme.selected_border()
                } else {
                    theme.accent()
                }),
        )
        .highlight_style(theme.highlight());

    let mut state = ListState::default()
        .with_selected(Some(app.settings_selected_animation))
        .with_offset(app.settings_animation_scroll as usize);

    f.render_stateful_widget(list, area, &mut state);
}

fn render_help(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let help_text = vec![Line::from(vec![
        Span::styled("Tab", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("settings-help-switch"))),
        Span::styled("↑↓", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("settings-help-navigate"))),
        Span::styled("Enter", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("settings-help-apply"))),
        Span::styled("Ctrl+T", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("settings-help-cycle-theme"))),
        Span::styled("Esc", theme.danger().add_modifier(Modifier::BOLD)),
        Span::raw(format!(" {}", app.i18n.t("settings-help-back"))),
    ])];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent()),
        );

    f.render_widget(help, area);
}
