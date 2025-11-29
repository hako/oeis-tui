use crate::app::{App, InputMode, SearchFocus, PLACEHOLDER_EXAMPLES};
use crate::utils::{highlight_sequence_line, highlight_terms_line};
use fluent::FluentArgs;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    if app.show_welcome_modal {
        render_welcome_modal(f, app);
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Length(2), // Status line
            Constraint::Min(10),   // Results (or results + preview split)
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    app.search_input_area = Some(chunks[0]);
    app.search_results_area = None;
    app.search_preview_area = None;
    app.search_preview_tabs_area = None;
    app.history_area = None;

    render_search_input(f, chunks[0], app);
    render_status(f, chunks[1], app);

    // If preview is enabled, split the results area horizontally
    if app.show_preview && !app.search_results.is_empty() {
        let results_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Results table
                Constraint::Percentage(50), // Preview pane
            ])
            .split(chunks[2]);

        app.search_results_area = Some(results_chunks[0]);
        app.search_preview_area = Some(results_chunks[1]);

        render_results(f, results_chunks[0], app);
        render_preview(f, results_chunks[1], app);
    } else {
        // Split vertically: results on top, history on bottom
        let results_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60), // Results table
                Constraint::Percentage(40), // History panel
            ])
            .split(chunks[2]);

        app.search_results_area = Some(results_chunks[0]);
        render_results(f, results_chunks[0], app);

        // Show bookmarks or history based on toggle
        if app.show_bookmarks {
            render_bookmarks_panel(f, results_chunks[1], app);
        } else {
            render_history_panel(f, results_chunks[1], app);
        }
    }

    render_help(f, chunks[3], app);
}

fn render_search_input(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let (display_text, placeholder) = if app.search_input.is_empty() {
        let example = PLACEHOLDER_EXAMPLES[app.placeholder_index];
        (String::new(), Some(example.to_string()))
    } else if app.input_mode == InputMode::Editing || app.search_focus == SearchFocus::Input {
        (format!("{}█", &app.search_input), None)
    } else {
        (app.search_input.clone(), None)
    };

    let style = if app.input_mode == InputMode::Editing {
        theme.highlight()
    } else {
        theme.text()
    };

    // Highlight border when focused
    let border_style = if app.search_focus == SearchFocus::Input {
        theme.highlight()
    } else {
        theme.accent()
    };

    let mut block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(format!(" {} ", app.i18n.t("search-title")))
        .title_alignment(Alignment::Left);

    if let Some(placeholder) = placeholder {
        block = block.title_bottom(Span::styled(
            format!(" {} ", placeholder),
            theme.placeholder(),
        ));
    }

    let input = Paragraph::new(display_text).style(style).block(block);

    f.render_widget(input, area);

    if app.input_mode == InputMode::Editing {
        f.set_cursor_position((area.x + app.search_cursor as u16 + 1, area.y + 1));
    }
}

fn render_welcome_modal(f: &mut Frame, app: &mut App) {
    let area = welcome_modal_area(f.area());
    let theme = app.theme().clone();

    f.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            format!(" {} ", app.i18n.t("welcome-title")),
            theme.accent_bold(),
        ))
        .border_style(theme.accent());
    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Top spacing
            Constraint::Length(3), // Title
            Constraint::Length(2), // Subtitle
            Constraint::Length(3), // Search input
            Constraint::Length(2), // Action text
            Constraint::Length(6), // Body
            Constraint::Min(1),    // Bottom spacing
        ])
        .split(inner);

    let title = Paragraph::new(Line::from(vec![Span::styled(
        app.i18n.t("welcome-subtitle"),
        theme.highlight_bold(),
    )]))
    .alignment(Alignment::Center);
    f.render_widget(title, layout[1]);

    let subtitle = Paragraph::new(Line::from(app.i18n.t("welcome-prompt")))
        .alignment(Alignment::Center)
        .style(theme.muted());
    f.render_widget(subtitle, layout[2]);

    app.search_input_area = Some(layout[3]);
    render_search_input(f, layout[3], app);

    let action_text = Paragraph::new(Line::from(vec![
        Span::styled(app.i18n.t("welcome-enter-hint"), theme.accent_bold()),
        Span::raw("  •  "),
        Span::styled(app.i18n.t("welcome-esc-hint"), theme.accent_bold()),
    ]))
    .alignment(Alignment::Center)
    .style(theme.muted());
    f.render_widget(action_text, layout[4]);

    let mut body_lines = vec![Line::from(app.i18n.t("welcome-hero-subtitle"))];
    body_lines.push(Line::from(""));
    body_lines.push(Line::from(app.i18n.t("welcome-hero-tips")));
    body_lines.push(Line::from(app.i18n.t("welcome-hero-search-hint")));

    let body = Paragraph::new(body_lines)
        .alignment(Alignment::Center)
        .style(theme.muted())
        .wrap(Wrap { trim: true });
    f.render_widget(body, layout[5]);
}

pub fn welcome_modal_area(area: Rect) -> Rect {
    centered_rect(70, 60, area)
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

fn render_status(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let mut status_spans = vec![];

    if app.searching {
        status_spans.push(Span::styled(
            format!(
                "{} {}",
                app.get_spinner_char(),
                app.i18n.t("search-status-loading")
            ),
            theme.highlight(),
        ));
    } else if app.search_results.is_empty()
        && !app.search_input.is_empty()
        && app.current_query.is_some()
    {
        status_spans.push(Span::styled(app.i18n.t("search-no-results"), theme.danger()));
    } else if !app.search_results.is_empty() {
        let page_size = app.results_per_page as i32;
        let current_page = if let Some(ref query) = app.current_query {
            (query.start / page_size) + 1
        } else {
            1
        };

        // Display result count - when count is 100, show "X+ results" since it's estimated
        let results_text = if app.result_count == 1 {
            app.i18n.t("search-result-one")
        } else if app.result_count == 100 && app.search_results.len() == app.results_per_page {
            // Count of 100 with exactly page_size results means "many more results exist"
            let mut args = FluentArgs::new();
            args.set("count", app.results_per_page);
            app.i18n.t_with_args("search-result-many-plus", Some(&args))
        } else {
            let mut args = FluentArgs::new();
            args.set("count", app.result_count);
            app.i18n.t_with_args("search-result-many", Some(&args))
        };

        status_spans.push(Span::styled(results_text, theme.success()));

        status_spans.push(Span::raw(" • "));

        status_spans.push(Span::styled(
            format!("Page {}", current_page),
            theme.accent(),
        ));

        if let Some(time) = app.last_search_time {
            status_spans.push(Span::raw(" • "));
            status_spans.push(Span::styled(format!("{:.3}s", time), theme.muted()));
        }
    }

    if let Some(ref error) = app.error_message {
        status_spans = vec![Span::styled(
            format!("{}: {}", app.i18n.t("common-error"), error),
            theme.danger(),
        )];
    }

    let status = Paragraph::new(Line::from(status_spans))
        .style(theme.text())
        .alignment(Alignment::Left);

    f.render_widget(status, area);
}

fn render_results(f: &mut Frame, area: Rect, app: &mut App) {
    app.search_results_area = Some(area);
    let theme = app.theme().clone();

    // Show loading indicator when searching
    if app.searching {
        let text = vec![
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                format!("{} {}", app.get_spinner_char(), app.i18n.t("search-status-loading")),
                theme.highlight_bold(),
            )),
            Line::from(""),
            Line::from(Span::styled(
                app.i18n.t("search-status-fetching"),
                theme.muted(),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .style(theme.text())
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        format!(" {} ", app.i18n.t("search-results-title")),
                        theme.accent_bold(),
                    ))
                    .title_alignment(Alignment::Left)
                    .border_style(theme.accent()),
            );

        f.render_widget(paragraph, area);
    } else if app.search_results.is_empty() {
        // Show empty state with helpful tips
        let text = vec![
            Line::from(""),
            Line::from(Span::styled(
                app.i18n.t("search-empty-title"),
                theme.muted(),
            )),
            Line::from(""),
            Line::from(app.i18n.t("search-tips-title")),
            Line::from(app.i18n.t("search-tip-terms")),
            Line::from(app.i18n.t("search-tip-anumber")),
            Line::from(app.i18n.t("search-tip-keyword")),
            Line::from(app.i18n.t("search-tip-prefixes")),
            Line::from(""),
            Line::from(Span::styled(
                app.i18n.t("search-start-hint"),
                theme.highlight(),
            )),
        ];

        let paragraph = Paragraph::new(text)
            .style(theme.text())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        format!(" {} ", app.i18n.t("search-results-title")),
                        theme.accent_bold(),
                    ))
                    .title_alignment(Alignment::Left)
                    .border_style(theme.accent()),
            );

        f.render_widget(paragraph, area);
    } else {
        // Render results as a table
        let anumber_text = app.i18n.t("search-table-anumber");
        let name_text = app.i18n.t("search-table-name");
        let data_text = app.i18n.t("search-table-data");

        let header_cells = vec!["#", &anumber_text, &name_text, &data_text]
            .into_iter()
            .map(|h| ratatui::widgets::Cell::from(h).style(theme.accent_bold()));
        let header = Row::new(header_cells).height(1).bottom_margin(1);

        let rows: Vec<Row> = app
            .search_results
            .iter()
            .enumerate()
            .map(|(i, seq)| {
                let is_selected = i == app.selected_result;

                let style = if is_selected {
                    theme.highlight_bg()
                } else {
                    theme.text()
                };

                let number = format!("{}", i + 1);
                let a_number = seq.a_number();
                let name = if seq.name.len() > 50 {
                    format!("{}...", &seq.name[..47])
                } else {
                    seq.name.clone()
                };

                // Show first few terms of the sequence
                let data_values: Vec<String> = seq
                    .data
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .take(15)
                    .collect();
                let data = if data_values.len() >= 15 {
                    format!("{},...", data_values.join(", "))
                } else {
                    data_values.join(", ")
                };

                let name_cell = ratatui::widgets::Cell::from(highlight_terms_line(
                    &name,
                    &app.search_terms,
                    &theme,
                ));
                let data_line = highlight_sequence_line(&data, &app.search_input, &theme)
                    .unwrap_or_else(|| highlight_terms_line(&data, &app.search_terms, &theme));
                let data_cell = ratatui::widgets::Cell::from(data_line);

                let highlight_a_number = app
                    .search_terms
                    .iter()
                    .any(|term| term.eq_ignore_ascii_case(&a_number))
                    || app.search_input.to_ascii_uppercase().contains(&a_number);

                let a_number_style = if highlight_a_number {
                    theme.highlight_bold()
                } else {
                    theme.accent_bold().add_modifier(Modifier::UNDERLINED)
                };

                let a_number_cell =
                    ratatui::widgets::Cell::from(Span::styled(a_number.clone(), a_number_style));

                let cells = vec![
                    ratatui::widgets::Cell::from(number),
                    a_number_cell,
                    name_cell,
                    data_cell,
                ];

                Row::new(cells).style(style).height(1)
            })
            .collect();

        let is_results_focused = app.search_focus == SearchFocus::Results;

        let table = Table::new(
            rows,
            [
                Constraint::Length(3),
                Constraint::Length(10),
                Constraint::Percentage(40),
                Constraint::Percentage(47),
            ],
        )
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " Results ",
                    if is_results_focused {
                        theme.highlight_bold()
                    } else {
                        theme.muted().add_modifier(Modifier::BOLD)
                    },
                ))
                .title_alignment(Alignment::Left)
                .border_style(if is_results_focused {
                    theme.selected_border()
                } else {
                    theme.accent()
                }),
        )
        .column_spacing(1);

        f.render_widget(table, area);
    }
}

fn render_help(f: &mut Frame, area: Rect, app: &App) {
    let theme = app.theme();
    let help_text = vec![Line::from(vec![
        Span::styled("i", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-search"))),
        Span::styled("↑↓", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-navigate"))),
        Span::styled("←→", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-page"))),
        Span::styled("Enter", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-view"))),
        Span::styled("p", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-preview"))),
        Span::styled("b", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-bookmarks"))),
        Span::styled("r", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-random"))),
        Span::styled("w", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-webcam"))),
        Span::styled("s", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-settings"))),
        Span::styled("Ctrl+H", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("search-help-help"))),
        Span::styled("q", theme.danger().add_modifier(Modifier::BOLD)),
        Span::raw(format!(" {}", app.i18n.t("search-help-quit"))),
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

fn render_preview(f: &mut Frame, area: Rect, app: &mut App) {
    app.search_preview_area = Some(area);

    if let Some(seq) = app.current_sequence.clone() {
        // Split preview area into title, tabs, and content
        let preview_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // Tabs
                Constraint::Min(0),    // Content
            ])
            .split(area);

        // Render title
        render_preview_title(f, preview_chunks[0], app, &seq);

        // Render tabs
        render_preview_tabs(f, preview_chunks[1], app);

        // Render content based on selected tab
        render_preview_content(f, preview_chunks[2], app, &seq);
    } else {
        app.search_preview_tabs_area = None;
        let theme = app.theme();
        let text = Paragraph::new("No preview available")
            .alignment(Alignment::Center)
            .style(theme.muted())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(" Preview ", theme.accent_bold()))
                    .border_style(theme.accent()),
            );
        f.render_widget(text, area);
    }
}

fn render_preview_title(f: &mut Frame, area: Rect, app: &App, seq: &crate::api::Sequence) {
    let theme = app.theme();
    let title_text = vec![Line::from(vec![
        Span::styled(seq.a_number(), theme.accent_bold()),
        Span::raw(": "),
        Span::styled(&seq.name, theme.text()),
    ])];

    let paragraph = Paragraph::new(title_text)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Preview ", theme.accent_bold()))
                .border_style(theme.accent()),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn render_preview_tabs(f: &mut Frame, area: Rect, app: &mut App) {
    let tab_titles = vec![
        app.i18n.t("detail-tab-overview"),
        app.i18n.t("detail-tab-formulas"),
        app.i18n.t("detail-tab-code"),
        app.i18n.t("detail-tab-references"),
        app.i18n.t("detail-tab-crossrefs"),
        app.i18n.t("detail-tab-metadata"),
    ];

    let theme = app.theme();
    let tabs = Tabs::new(tab_titles)
        .select(app.preview_tab)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent()),
        )
        .style(theme.text())
        .highlight_style(theme.highlight_bold());

    f.render_widget(tabs, area);
    app.search_preview_tabs_area = Some(area);
}

fn render_preview_content(f: &mut Frame, area: Rect, app: &App, seq: &crate::api::Sequence) {
    // Reuse the detail screen's rendering logic
    let selected = None;
    let mut references = Vec::new();
    let theme = app.theme();

    let content = match app.preview_tab {
        0 => crate::ui::detail::render_overview_content(app, seq, selected, &mut references, theme),
        1 => crate::ui::detail::render_formulas_content(seq, selected, &mut references, theme),
        2 => crate::ui::detail::render_code_content(seq, selected, &mut references, theme),
        3 => crate::ui::detail::render_references_content(seq, selected, &mut references, theme),
        4 => crate::ui::detail::render_crossrefs_content(seq, selected, &mut references, theme),
        5 => crate::ui::detail::render_metadata_content(seq, selected, &mut references, theme),
        _ => vec![Line::from("Invalid tab")],
    };

    let paragraph = Paragraph::new(content)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(" Details ", theme.accent_bold()))
                .border_style(theme.accent()),
        )
        .wrap(Wrap { trim: true })
        .scroll((app.preview_scroll, 0));

    f.render_widget(paragraph, area);
}

fn render_history_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let theme = app.theme();

    if app.recent_sequences.is_empty() {
        let is_history_focused = app.search_focus == SearchFocus::History;
        let empty_text = Paragraph::new(app.i18n.t("search-history-empty"))
            .alignment(Alignment::Center)
            .style(theme.muted())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        format!(" {} ", app.i18n.t("search-recently-viewed")),
                        if is_history_focused {
                            theme.highlight_bold()
                        } else {
                            theme.muted().add_modifier(Modifier::BOLD)
                        },
                    ))
                    .border_style(if is_history_focused {
                        theme.selected_border()
                    } else {
                        theme.accent()
                    }),
            );
        f.render_widget(empty_text, area);
        return;
    }

    // Create table rows from recent sequences
    let rows: Vec<Row> = app
        .recent_sequences
        .iter()
        .enumerate()
        .map(|(i, (number, name, view_count, _viewed_at))| {
            let is_selected = i == app.history_selected;
            let style = if is_selected {
                theme.highlight()
            } else {
                theme.text()
            };

            // Format A-number
            let a_number = format!("A{:06}", number);

            // Truncate name if too long
            let display_name = if name.len() > 50 {
                format!("{}...", &name[..47])
            } else {
                name.clone()
            };

            // Format view count
            let mut args = FluentArgs::new();
            args.set("count", *view_count);
            let views = app.i18n.t_with_args("search-view-count", Some(&args));

            Row::new(vec![a_number, display_name, views]).style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(10), // A-number
        Constraint::Min(30),    // Name (flexible)
        Constraint::Length(10), // Views
    ];

    let is_history_focused = app.search_focus == SearchFocus::History;

    let anumber_text = app.i18n.t("search-table-anumber");
    let name_text = app.i18n.t("search-table-name");
    let views_text = app.i18n.t("search-table-views");

    let table = Table::new(rows, widths)
        .header(
            Row::new(vec![anumber_text.as_str(), name_text.as_str(), views_text.as_str()])
                .style(theme.accent_bold())
                .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    format!(" {} ", app.i18n.t("search-recently-viewed")),
                    if is_history_focused {
                        theme.highlight_bold()
                    } else {
                        theme.muted().add_modifier(Modifier::BOLD)
                    },
                ))
                .border_style(if is_history_focused {
                    theme.selected_border()
                } else {
                    theme.accent()
                }),
        )
        .row_highlight_style(theme.highlight_bold());

    f.render_widget(table, area);
    app.history_area = Some(area);
}

fn render_bookmarks_panel(f: &mut Frame, area: Rect, app: &mut App) {
    let theme = app.theme();

    if app.bookmarks.is_empty() {
        let is_bookmarks_focused = app.search_focus == SearchFocus::Bookmarks;
        let empty_text = Paragraph::new(app.i18n.t("search-bookmarks-empty"))
            .alignment(Alignment::Center)
            .style(theme.muted())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        format!(" {} ", app.i18n.t("search-bookmarks-title")),
                        if is_bookmarks_focused {
                            theme.highlight_bold()
                        } else {
                            theme.muted().add_modifier(Modifier::BOLD)
                        },
                    ))
                    .border_style(if is_bookmarks_focused {
                        theme.selected_border()
                    } else {
                        theme.accent()
                    }),
            );
        f.render_widget(empty_text, area);
        return;
    }

    // Create table rows from bookmarks
    let rows: Vec<Row> = app
        .bookmarks
        .iter()
        .enumerate()
        .map(|(i, (number, notes))| {
            let is_selected = i == app.bookmarks_selected;
            let style = if is_selected {
                theme.highlight()
            } else {
                theme.text()
            };

            // Format A-number
            let a_number = format!("A{:06}", number);

            // Get sequence name from cache (if available)
            let name = if let Ok(Some(seq)) = app.cache.get_cached_sequence(*number, 30) {
                if seq.name.len() > 40 {
                    format!("{}...", &seq.name[..37])
                } else {
                    seq.name.clone()
                }
            } else {
                app.i18n.t("search-bookmarks-loading").to_string()
            };

            // Format notes (future use)
            let notes_display = notes
                .as_ref()
                .map(|n| {
                    if n.len() > 20 {
                        format!("{}...", &n[..17])
                    } else {
                        n.clone()
                    }
                })
                .unwrap_or_else(|| String::from("-"));

            Row::new(vec![a_number, name, notes_display]).style(style)
        })
        .collect();

    let widths = [
        Constraint::Length(10), // A-number
        Constraint::Min(30),    // Name (flexible)
        Constraint::Length(20), // Notes
    ];

    let is_bookmarks_focused = app.search_focus == SearchFocus::Bookmarks;

    let anumber_text = app.i18n.t("search-table-anumber");
    let name_text = app.i18n.t("search-table-name");
    let notes_text = app.i18n.t("search-bookmarks-notes");

    let table = Table::new(rows, widths)
        .header(
            Row::new(vec![
                anumber_text.as_str(),
                name_text.as_str(),
                notes_text.as_str(),
            ])
            .style(theme.accent_bold())
            .bottom_margin(1),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    format!(" {} ", app.i18n.t("search-bookmarks-title")),
                    if is_bookmarks_focused {
                        theme.highlight_bold()
                    } else {
                        theme.muted().add_modifier(Modifier::BOLD)
                    },
                ))
                .border_style(if is_bookmarks_focused {
                    theme.selected_border()
                } else {
                    theme.accent()
                }),
        )
        .row_highlight_style(theme.highlight_bold());

    f.render_widget(table, area);
    app.bookmarks_area = Some(area);
}
