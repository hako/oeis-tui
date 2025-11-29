use crate::app::App;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn render_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(Clear, area);

    let theme = app.theme();
    let paragraph = Paragraph::new(build_help_lines(app, theme))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((app.help_scroll, 0))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.i18n.t("help-title")))
                .title_alignment(Alignment::Center),
        );

    f.render_widget(paragraph, area);
}

fn build_help_lines(app: &App, theme: &crate::ui::theme::Theme) -> Vec<Line<'static>> {
    vec![
        section_header(&app.i18n.t("help-global"), theme),
        Line::from(format!(
            "  Ctrl+Q / Ctrl+C - {}",
            app.i18n.t("help-global-quit")
        )),
        Line::from(format!("  Ctrl+H - {}", app.i18n.t("help-global-help"))),
        Line::from("  Ctrl+A - Show about"),
        Line::from(format!("  Esc - {}", app.i18n.t("help-global-back"))),
        Line::from(""),
        section_header(&app.i18n.t("help-search"), theme),
        Line::from(format!("  i / / - {}", app.i18n.t("help-search-input"))),
        Line::from(format!("  Enter - {}", app.i18n.t("help-search-view"))),
        Line::from(format!(
            "  ↑/↓ or k/j - {}",
            app.i18n.t("help-search-navigate")
        )),
        Line::from(format!("  ←/→ or h/l - {}", app.i18n.t("help-search-page"))),
        Line::from(format!("  p - {}", app.i18n.t("help-search-preview"))),
        Line::from(format!(
            "  Tab / 1-6 - {}",
            app.i18n.t("help-search-preview-tabs")
        )),
        Line::from(format!("  r - {}", app.i18n.t("help-search-random"))),
        Line::from(format!("  w - {}", app.i18n.t("help-search-webcam"))),
        Line::from(format!(
            "  Click - {}",
            app.i18n.t("help-search-mouse-select")
        )),
        Line::from(format!(
            "  Click x2 - {}",
            app.i18n.t("help-search-mouse-open")
        )),
        Line::from(format!(
            "  Scroll - {}",
            app.i18n.t("help-search-mouse-scroll")
        )),
        Line::from(""),
        section_header(&app.i18n.t("help-detail"), theme),
        Line::from(format!(
            "  Tab / Shift+Tab - {}",
            app.i18n.t("help-detail-tabs")
        )),
        Line::from(format!(
            "  ←/→ or h/l - {}",
            app.i18n.t("help-detail-links")
        )),
        Line::from(format!(
            "  Enter / Ctrl+Click - {}",
            app.i18n.t("help-detail-open-link")
        )),
        Line::from(format!(
            "  ↑/↓ or k/j - {}",
            app.i18n.t("help-detail-scroll")
        )),
        Line::from(format!(
            "  PgUp/PgDn - {}",
            app.i18n.t("help-detail-scroll-fast")
        )),
        Line::from(format!("  g - {}", app.i18n.t("help-detail-graph"))),
        Line::from(format!("  e - {}", app.i18n.t("help-detail-export"))),
        Line::from(format!("  o - {}", app.i18n.t("help-detail-browser"))),
        Line::from(""),
        section_header(&app.i18n.t("help-graph"), theme),
        Line::from(format!("  1/2/3/4 - {}", app.i18n.t("help-graph-types"))),
        Line::from(""),
        section_header(&app.i18n.t("help-webcam"), theme),
        Line::from(format!(
            "  Space/Enter - {}",
            app.i18n.t("help-webcam-next")
        )),
        Line::from(format!("  ←/→ - {}", app.i18n.t("help-webcam-category"))),
        Line::from(format!("  ↑/↓ - {}", app.i18n.t("help-webcam-interval"))),
        Line::from(format!("  0-5 - {}", app.i18n.t("help-webcam-quick"))),
        Line::from(format!("  d - {}", app.i18n.t("help-webcam-detail"))),
        Line::from(""),
        Line::from(format!("Esc - {}", app.i18n.t("help-global-back"))),
    ]
}

fn section_header(label: &str, theme: &crate::ui::theme::Theme) -> Line<'static> {
    Line::from(vec![Span::styled(
        label.to_string(),
        Style::default()
            .fg(theme.accent_color())
            .add_modifier(Modifier::BOLD),
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
