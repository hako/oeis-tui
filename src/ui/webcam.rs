use crate::app::{App, WebcamFocus, WebcamInterval};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(8), // Controls
            Constraint::Min(10),   // Sequence display
            Constraint::Length(4), // Help
        ])
        .split(f.area());

    render_title(f, chunks[0], app, &theme);
    render_controls(f, chunks[1], app, &theme);
    render_sequence(f, chunks[2], app, &theme);
    render_help(f, chunks[3], app);
}

fn render_title(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    let title_text = vec![Line::from(Span::styled(
        "OEIS Webcam - Sequence Browser",
        theme.accent_bold(),
    ))];

    let paragraph = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(format!(
                    " {} {}",
                    app.get_spinner_char(),
                    app.i18n.t("webcam-title")
                ))
                .title_alignment(Alignment::Left),
        );

    f.render_widget(paragraph, area);
}

fn render_controls(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Categories
            Constraint::Percentage(50), // Intervals
        ])
        .split(area);

    render_categories(f, chunks[0], app, theme);
    render_intervals(f, chunks[1], app, theme);
}

fn render_categories(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    let categories = [
        (
            app.i18n.t("webcam-category-all"),
            app.i18n.t("webcam-category-all-desc"),
        ),
        (
            app.i18n.t("webcam-category-best"),
            app.i18n.t("webcam-category-best-desc"),
        ),
        (
            app.i18n.t("webcam-category-needing"),
            app.i18n.t("webcam-category-needing-desc"),
        ),
        (
            app.i18n.t("webcam-category-recent"),
            app.i18n.t("webcam-category-recent-desc"),
        ),
    ];

    let items: Vec<ListItem> = categories
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let is_selected = i == app.webcam_category;
            let style = if is_selected {
                theme.highlight_bold()
            } else {
                theme.text()
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(name.as_str(), style),
                ]),
                Line::from(vec![Span::raw("    "), Span::styled(desc.as_str(), theme.muted())]),
            ])
        })
        .collect();

    let border_style = if matches!(app.webcam_focus, WebcamFocus::Categories) {
        theme.selected_border()
    } else {
        theme.accent()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" {} ", app.i18n.t("webcam-category"))),
        )
        .highlight_style(theme.highlight_bold());

    let mut state = ListState::default()
        .with_selected(Some(app.webcam_category))
        .with_offset(app.webcam_category_scroll as usize);

    f.render_stateful_widget(list, area, &mut state);
}

fn render_intervals(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    let intervals = vec![
        (
            WebcamInterval::Manual,
            app.i18n.t("webcam-interval-manual"),
            app.i18n.t("webcam-interval-manual-desc"),
        ),
        (
            WebcamInterval::FiveSeconds,
            app.i18n.t("webcam-interval-5s"),
            app.i18n.t("webcam-interval-5s-desc"),
        ),
        (
            WebcamInterval::TenSeconds,
            app.i18n.t("webcam-interval-10s"),
            app.i18n.t("webcam-interval-10s-desc"),
        ),
        (
            WebcamInterval::TwentySeconds,
            app.i18n.t("webcam-interval-20s"),
            app.i18n.t("webcam-interval-20s-desc"),
        ),
        (
            WebcamInterval::ThirtySeconds,
            app.i18n.t("webcam-interval-30s"),
            app.i18n.t("webcam-interval-30s-desc"),
        ),
        (
            WebcamInterval::OneMinute,
            app.i18n.t("webcam-interval-1m"),
            app.i18n.t("webcam-interval-1m-desc"),
        ),
    ];

    let items: Vec<ListItem> = intervals
        .iter()
        .map(|(interval, name, desc)| {
            let is_selected = app.webcam_interval == Some(*interval);
            let style = if is_selected {
                theme.success().add_modifier(Modifier::BOLD)
            } else {
                theme.text()
            };

            let prefix = if is_selected { "● " } else { "○ " };

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(prefix, style),
                    Span::styled(name.as_str(), style),
                ]),
                Line::from(vec![Span::raw("  "), Span::styled(desc.as_str(), theme.muted())]),
            ])
        })
        .collect();

    let border_style = if matches!(app.webcam_focus, WebcamFocus::Intervals) {
        theme.selected_border()
    } else {
        theme.accent()
    };

    // Find the index of the selected interval
    let selected_index = intervals
        .iter()
        .position(|(interval, _, _)| app.webcam_interval == Some(*interval))
        .unwrap_or(0);

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" {} ", app.i18n.t("webcam-interval"))),
        )
        .highlight_style(theme.success().add_modifier(Modifier::BOLD));

    let mut state = ListState::default()
        .with_selected(Some(selected_index))
        .with_offset(app.webcam_interval_scroll as usize);

    f.render_stateful_widget(list, area, &mut state);
}

fn render_sequence(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    if let Some(ref seq) = app.current_sequence {
        let mut lines = vec![];

        // Header
        lines.push(Line::from(vec![
            Span::styled(seq.a_number(), theme.accent_bold()),
            Span::raw(": "),
            Span::styled(&seq.name, theme.text().add_modifier(Modifier::BOLD)),
        ]));
        lines.push(Line::from(""));

        // Metadata
        lines.push(Line::from(vec![
            Span::styled("Offset: ", theme.highlight()),
            Span::styled(&seq.offset, theme.text()),
            Span::raw(" | "),
            Span::styled("Keywords: ", theme.highlight()),
            Span::styled(&seq.keyword, theme.text()),
        ]));
        lines.push(Line::from(""));

        // Data
        lines.push(Line::from(Span::styled(
            "Sequence Data:",
            theme.success().add_modifier(Modifier::BOLD),
        )));

        // Split data into lines of reasonable length
        let data_values: Vec<&str> = seq.data.split(',').collect();
        let chunk_size = 15;
        for chunk in data_values.chunks(chunk_size) {
            lines.push(Line::from(vec![Span::styled(
                chunk.join(", "),
                theme.text(),
            )]));
        }
        lines.push(Line::from(""));

        // Comments preview
        if !seq.comment.is_empty() {
            lines.push(Line::from(Span::styled("Comments:", theme.accent_bold())));
            for (i, comment) in seq.comment.iter().take(3).enumerate() {
                if i > 0 {
                    lines.push(Line::from(""));
                }
                lines.push(Line::from(vec![Span::styled(
                    comment.clone(),
                    theme.text(),
                )]));
            }
            if seq.comment.len() > 3 {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("... and {} more comments", seq.comment.len() - 3),
                    theme.muted(),
                )));
            }
            lines.push(Line::from(""));
        }

        // Refresh status
        if let Some(interval) = app.webcam_interval {
            if let Some(last_update) = app.webcam_last_update {
                let elapsed = last_update.elapsed().as_secs();
                let interval_secs = interval.as_duration().map(|d| d.as_secs()).unwrap_or(0);

                if interval_secs > 0 {
                    let remaining = interval_secs.saturating_sub(elapsed);
                    lines.push(Line::from(""));
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("{} ", app.get_spinner_char()),
                            theme.highlight_bold(),
                        ),
                        Span::styled(
                            format!("Next refresh in {} seconds...", remaining),
                            theme.highlight(),
                        ),
                    ]));
                }
            }
        }

        let border_style = if matches!(app.webcam_focus, WebcamFocus::Sequence) {
            theme.selected_border()
        } else {
            theme.accent()
        };

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(format!(" {} ", app.i18n.t("webcam-current-sequence"))),
            )
            .wrap(Wrap { trim: true })
            .scroll((0, 0));

        f.render_widget(paragraph, area);
    } else {
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(app.i18n.t("webcam-no-sequence"), theme.muted())),
            Line::from(""),
            Line::from(app.i18n.t("webcam-load-first")),
        ];

        let border_style = if matches!(app.webcam_focus, WebcamFocus::Sequence) {
            theme.selected_border()
        } else {
            theme.accent()
        };

        let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(format!(" {} ", app.i18n.t("webcam-current-sequence"))),
        );

        f.render_widget(paragraph, area);
    }
}

fn render_help(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let help_text = vec![Line::from(vec![
        Span::styled("Space/Enter", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("webcam-help-next"))),
        Span::styled("←→", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("webcam-help-category"))),
        Span::styled("↑↓", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("webcam-help-interval"))),
        Span::styled("0-5", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("webcam-help-quick"))),
        Span::styled("d", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("webcam-help-detail"))),
        Span::styled("Esc", theme.danger().add_modifier(Modifier::BOLD)),
        Span::raw(format!(" {}", app.i18n.t("webcam-help-back"))),
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
