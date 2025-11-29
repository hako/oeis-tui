use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let theme = app.theme();
    let paragraph = Paragraph::new(build_about_lines(app, theme))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.i18n.t("about-title")))
                .title_alignment(Alignment::Center),
        );

    f.render_widget(paragraph, area);
}

fn build_about_lines(app: &App, theme: &crate::ui::theme::Theme) -> Vec<Line<'static>> {
    vec![
        Line::from(""),
        // Application name
        Line::from(Span::styled(
            "OEIS TUI",
            theme.accent_bold().add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "A beautiful TUI for the On-Line Encyclopedia of Integer Sequences",
            theme.text(),
        )),
        Line::from(""),
        Line::from(""),
        // Version
        Line::from(vec![
            Span::styled(
                format!("{}: ", app.i18n.t("about-version")),
                theme.accent_bold(),
            ),
            Span::styled("1.0.0", theme.text()),
        ]),
        Line::from(""),
        // Author
        Line::from(vec![
            Span::styled(
                format!("{}: ", app.i18n.t("about-author")),
                theme.accent_bold(),
            ),
            Span::styled("Wesley Hill (@hako / @hakobyte)", theme.text()),
        ]),
        Line::from(""),
        // Links section
        section_header(&app.i18n.t("about-links"), theme),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{}: ", app.i18n.t("about-repository")),
                theme.accent_bold(),
            ),
            Span::styled("https://github.com/hako/oeis-tui", theme.highlight()),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{}: ", app.i18n.t("about-oeis-link")),
                theme.accent_bold(),
            ),
            Span::styled("https://oeis.org", theme.highlight()),
        ]),
        Line::from(""),
        Line::from(""),
        // Disclaimer
        Line::from(Span::styled(app.i18n.t("about-disclaimer"), theme.muted())),
        Line::from(""),
        Line::from(""),
        // Close instruction
        Line::from(Span::styled(
            format!("Esc - {}", app.i18n.t("help-global-back")),
            theme.muted(),
        )),
    ]
}

fn section_header(label: &str, theme: &crate::ui::theme::Theme) -> Line<'static> {
    Line::from(vec![Span::styled(
        label.to_string(),
        theme.accent_bold().add_modifier(Modifier::BOLD),
    )])
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(horizontal[1])[1]
}
