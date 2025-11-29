use crate::app::App;
use crate::utils::highlight_anumbers_line;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Tabs
            Constraint::Min(10),   // Content
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    app.detail_tabs_area = None;
    app.detail_content_area = None;

    if let Some(seq) = app.current_sequence.clone() {
        render_title(f, chunks[0], app, &seq, &theme);
        render_tabs(f, chunks[1], app, &theme);

        // Render tab content based on selected tab
        match app.detail_tab {
            6 => render_graph_tab(f, chunks[2], app, &seq, &theme),
            7 => render_export_tab(f, chunks[2], app, &seq, &theme),
            _ => render_content(f, chunks[2], app, &seq, &theme),
        }

        render_help(f, chunks[3], app);
    } else {
        let text = Paragraph::new("No sequence loaded").alignment(Alignment::Center);
        f.render_widget(text, f.area());
    }

    // Render detail help modal if visible
    if app.detail_help_visible {
        render_help_modal(f, app);
    }
}

fn render_title(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &App,
    seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    // Check if sequence is bookmarked
    let is_bookmarked = app.cache.is_bookmarked(seq.number).unwrap_or(false);
    let bookmark_icon = if is_bookmarked { "★ " } else { "☆ " };
    let bookmark_text = if is_bookmarked {
        app.i18n.t("detail-bookmarked")
    } else {
        app.i18n.t("detail-not-bookmarked")
    };

    let title_text = vec![Line::from(vec![
        Span::styled(bookmark_icon, if is_bookmarked { theme.success() } else { theme.muted() }),
        Span::styled(seq.a_number(), theme.accent_bold()),
        Span::raw(": "),
        Span::styled(&seq.name, theme.text()),
    ])];

    let paragraph = Paragraph::new(title_text)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(Span::styled(
                    format!(" {} ", bookmark_text),
                    theme.accent_bold()
                )),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn render_tabs(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &mut App,
    theme: &crate::ui::Theme,
) {
    let tab_titles = vec![
        app.i18n.t("detail-tab-overview"),
        app.i18n.t("detail-tab-formulas"),
        app.i18n.t("detail-tab-code"),
        app.i18n.t("detail-tab-references"),
        app.i18n.t("detail-tab-crossrefs"),
        app.i18n.t("detail-tab-metadata"),
        app.i18n.t("detail-tab-graph"),
        app.i18n.t("detail-tab-export"),
    ];

    let tabs = Tabs::new(tab_titles)
        .select(app.detail_tab)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent()),
        )
        .style(theme.text())
        .highlight_style(theme.highlight_bold());

    f.render_widget(tabs, area);
    app.detail_tabs_area = Some(area);
}

fn render_content(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    app: &mut App,
    seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    app.detail_content_area = Some(area);
    let selected = app.detail_reference_index;
    let mut references = Vec::new();

    let content = match app.detail_tab {
        0 => render_overview_content(app, seq, selected, &mut references, theme),
        1 => render_formulas_content(seq, selected, &mut references, theme),
        2 => render_code_content(seq, selected, &mut references, theme),
        3 => render_references_content(seq, selected, &mut references, theme),
        4 => render_crossrefs_content(seq, selected, &mut references, theme),
        5 => render_metadata_content(seq, selected, &mut references, theme),
        _ => vec![Line::from("Invalid tab")],
    };

    app.set_detail_references(references);

    let paragraph = Paragraph::new(content)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(Span::styled(" Details ", theme.accent_bold())),
        )
        .wrap(Wrap { trim: true })
        .scroll((app.detail_scroll, 0));

    f.render_widget(paragraph, area);
}

pub fn render_overview_content(
    app: &App,
    seq: &crate::api::Sequence,
    selected: Option<usize>,
    references: &mut Vec<String>,
    theme: &crate::ui::Theme,
) -> Vec<Line<'static>> {
    let mut lines = vec![];

    lines.push(Line::from(Span::styled("Data:", theme.highlight_bold())));
    lines.push(Line::from(vec![Span::styled(
        seq.data.clone(),
        theme.text(),
    )]));
    lines.push(Line::from(""));

    if !seq.comment.is_empty() {
        lines.push(Line::from(Span::styled(
            "Comments:",
            theme.highlight_bold(),
        )));
        for comment in &seq.comment {
            lines.push(highlight_anumbers_line(
                comment, references, selected, theme,
            ));
        }
        lines.push(Line::from(""));
    }

    if !seq.example.is_empty() {
        lines.push(Line::from(Span::styled(
            "Examples:",
            theme.highlight_bold(),
        )));
        for example in &seq.example {
            lines.push(highlight_anumbers_line(
                example, references, selected, theme,
            ));
        }
        lines.push(Line::from(""));
    }

    // B-file section (moved to bottom)
    if let Some(ref bfile_data) = app.bfile_data {
        let count = bfile_data.len();
        let mut args = fluent::FluentArgs::new();
        args.set("count", count);
        lines.push(Line::from(Span::styled(
            app.i18n.t_with_args("detail-bfile-loaded", Some(&args)).to_string(),
            theme.success().add_modifier(Modifier::BOLD),
        )));

        // Show first 50 terms
        let display_count = bfile_data.len().min(50);
        for entry in bfile_data.iter().take(display_count) {
            lines.push(Line::from(vec![
                Span::styled(format!("{}: ", entry.index), theme.muted()),
                Span::styled(entry.value.clone(), theme.text()),
            ]));
        }
        if bfile_data.len() > 50 {
            lines.push(Line::from(Span::styled(
                format!("... ({} more terms)", bfile_data.len() - 50),
                theme.muted(),
            )));
        }
        lines.push(Line::from(""));
    } else if let Some(ref error) = app.bfile_error {
        lines.push(Line::from(Span::styled(
            error.clone(),
            theme.danger(),
        )));
        lines.push(Line::from(""));
    } else if app.pending_bfile.is_some() {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", app.get_spinner_char()),
                theme.highlight_bold(),
            ),
            Span::styled(
                app.i18n.t("detail-bfile-loading").to_string(),
                theme.highlight(),
            ),
        ]));
        lines.push(Line::from(""));
    } else {
        // Not loaded yet - show fetch prompt
        lines.push(Line::from(Span::styled(
            app.i18n.t("detail-bfile-available").to_string(),
            theme.highlight_bold(),
        )));
        lines.push(Line::from(Span::styled(
            app.i18n.t("detail-bfile-fetch").to_string(),
            theme.muted(),
        )));
        lines.push(Line::from(""));
    }

    lines
}

pub fn render_formulas_content(
    seq: &crate::api::Sequence,
    selected: Option<usize>,
    references: &mut Vec<String>,
    theme: &crate::ui::Theme,
) -> Vec<Line<'static>> {
    let mut lines = vec![];

    if seq.formula.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "No formulas available",
            theme.muted(),
        )]));
    } else {
        for formula in &seq.formula {
            lines.push(highlight_anumbers_line(
                formula, references, selected, theme,
            ));
            lines.push(Line::from(""));
        }
    }

    lines
}

pub fn render_code_content(
    seq: &crate::api::Sequence,
    selected: Option<usize>,
    references: &mut Vec<String>,
    theme: &crate::ui::Theme,
) -> Vec<Line<'static>> {
    let mut lines = vec![];

    if !seq.maple.is_empty() {
        lines.push(Line::from(Span::styled(
            "Maple:",
            theme.success().add_modifier(Modifier::BOLD),
        )));
        for code in &seq.maple {
            lines.push(highlight_anumbers_line(code, references, selected, theme));
        }
        lines.push(Line::from(""));
    }

    if !seq.mathematica.is_empty() {
        lines.push(Line::from(Span::styled(
            "Mathematica:",
            theme.success().add_modifier(Modifier::BOLD),
        )));
        for code in &seq.mathematica {
            lines.push(highlight_anumbers_line(code, references, selected, theme));
        }
        lines.push(Line::from(""));
    }

    if !seq.program.is_empty() {
        lines.push(Line::from(Span::styled(
            "Other Programs:",
            theme.success().add_modifier(Modifier::BOLD),
        )));
        for code in &seq.program {
            lines.push(highlight_anumbers_line(code, references, selected, theme));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "No code available",
            theme.muted(),
        )]));
    }

    lines
}

pub fn render_references_content(
    seq: &crate::api::Sequence,
    selected: Option<usize>,
    references: &mut Vec<String>,
    theme: &crate::ui::Theme,
) -> Vec<Line<'static>> {
    let mut lines = vec![];

    if !seq.reference.is_empty() {
        for reference in &seq.reference {
            lines.push(highlight_anumbers_line(
                reference, references, selected, theme,
            ));
            lines.push(Line::from(""));
        }
    }

    if !seq.link.is_empty() {
        lines.push(Line::from(Span::styled("Links:", theme.accent_bold())));
        for link in &seq.link {
            lines.push(highlight_anumbers_line(link, references, selected, theme));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "No references available",
            theme.muted(),
        )]));
    }

    lines
}

pub fn render_crossrefs_content(
    seq: &crate::api::Sequence,
    selected: Option<usize>,
    references: &mut Vec<String>,
    theme: &crate::ui::Theme,
) -> Vec<Line<'static>> {
    let mut lines = vec![];

    if seq.xref.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "No cross-references available",
            theme.muted(),
        )]));
    } else {
        for xref in &seq.xref {
            lines.push(highlight_anumbers_line(xref, references, selected, theme));
            lines.push(Line::from(""));
        }
    }

    lines
}

pub fn render_metadata_content(
    seq: &crate::api::Sequence,
    selected: Option<usize>,
    references: &mut Vec<String>,
    theme: &crate::ui::Theme,
) -> Vec<Line<'static>> {
    let mut lines = vec![];

    lines.push(Line::from(vec![
        Span::styled("A-number: ", theme.highlight()),
        Span::styled(seq.a_number(), theme.text()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("ID: ", theme.highlight()),
        Span::styled(seq.id.clone(), theme.text()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Offset: ", theme.highlight()),
        Span::styled(seq.offset.clone(), theme.text()),
    ]));

    let keyword_line = highlight_anumbers_line(&seq.keyword, references, selected, theme);
    let mut keyword_spans = vec![Span::styled("Keywords: ", theme.highlight())];
    keyword_spans.extend(keyword_line.spans);
    lines.push(Line::from(keyword_spans));

    lines.push(Line::from(vec![
        Span::styled("Author: ", theme.highlight()),
        Span::styled(seq.author.clone(), theme.text()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Created: ", theme.highlight()),
        Span::styled(seq.created.clone(), theme.text()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Last modified: ", theme.highlight()),
        Span::styled(seq.time.clone(), theme.text()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("References: ", theme.highlight()),
        Span::styled(format!("{}", seq.references), theme.text()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Revision: ", theme.highlight()),
        Span::styled(format!("{}", seq.revision), theme.text()),
    ]));

    lines
}

fn render_help(f: &mut Frame, area: ratatui::layout::Rect, app: &crate::app::App) {
    let theme = app.theme();

    // Show export-specific help when on Export tab
    let help_text = if app.detail_tab == 7 {
        vec![Line::from(app.i18n.t("export-help").to_string())]
    } else {
        vec![Line::from(vec![
            Span::styled("Tab", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("detail-help-switch-tab"))),
            Span::styled("↑↓", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("detail-help-scroll"))),
            Span::styled("g", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("detail-help-graph"))),
            Span::styled("e", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("detail-help-export"))),
            Span::styled("o", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("detail-help-browser"))),
            Span::styled("Ctrl+H", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("detail-help-more"))),
            Span::styled("Esc", theme.danger().add_modifier(Modifier::BOLD)),
            Span::raw(format!(" {}", app.i18n.t("common-back"))),
        ])]
    };

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

/// Render the detail help modal
pub fn render_help_modal(f: &mut Frame, app: &App) {
    let area = centered_rect(60, 70, f.area());
    f.render_widget(ratatui::widgets::Clear, area);

    let theme = app.theme();
    let paragraph = Paragraph::new(build_detail_help_lines(app, theme))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true })
        .scroll((app.detail_help_scroll, 0))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} ", app.i18n.t("detail-help-modal-title")))
                .title_alignment(Alignment::Center),
        );

    f.render_widget(paragraph, area);
}

fn build_detail_help_lines(app: &App, theme: &crate::ui::theme::Theme) -> Vec<Line<'static>> {
    vec![
        section_header(&app.i18n.t("help-detail"), theme),
        Line::from(format!(
            "  Tab / Shift+Tab - {}",
            app.i18n.t("detail-help-next-link")
        )),
        Line::from(format!(
            "  Ctrl+Tab - {}",
            app.i18n.t("detail-help-switch-tab")
        )),
        Line::from(format!(
            "  Enter / Ctrl+Click - {}",
            app.i18n.t("detail-help-follow-link")
        )),
        Line::from(format!(
            "  ↑/↓ or k/j - {}",
            app.i18n.t("detail-help-scroll")
        )),
        Line::from(format!(
            "  PgUp/PgDn - {}",
            app.i18n.t("detail-help-scroll-fast")
        )),
        Line::from(format!("  g - {}", app.i18n.t("detail-help-graph"))),
        Line::from(format!("  e - {}", app.i18n.t("detail-help-export"))),
        Line::from(format!("  o - {}", app.i18n.t("detail-help-browser"))),
        Line::from(format!("  b - {}", app.i18n.t("detail-help-bookmark"))),
        Line::from(format!("  f - {}", app.i18n.t("detail-help-bfile"))),
        Line::from(format!("  Esc - {}", app.i18n.t("common-back"))),
        Line::from(""),
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

/// Render the Graph tab (embedded graph view)
fn render_graph_tab(
    f: &mut Frame,
    area: Rect,
    app: &App,
    seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    // Delegate to the graph module's render function for embedded view
    crate::ui::graph::render_embedded(f, area, app, seq, theme);
}

/// Render the Export tab (export format selection and preview)
fn render_export_tab(
    f: &mut Frame,
    area: Rect,
    app: &mut App,
    seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    // Delegate to the export module's render function for embedded view
    crate::ui::export::render_embedded(f, area, app, seq, theme);
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
