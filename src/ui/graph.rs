use crate::app::{App, GraphType};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};

pub fn render(f: &mut Frame, app: &mut App) {
    let theme = app.theme().clone();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Graph
            Constraint::Length(5), // Help
        ])
        .split(f.area());

    render_title(f, chunks[0], app, &theme);

    if let Some(ref seq) = app.current_sequence {
        render_graph(f, chunks[1], app, seq, &theme);
    } else {
        let text = Paragraph::new("No sequence loaded").alignment(Alignment::Center);
        f.render_widget(text, chunks[1]);
    }

    render_help(f, chunks[2], app, &theme);
}

fn render_title(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    let title = if let Some(ref seq) = app.current_sequence {
        format!("{}: {} - Graph View", seq.a_number(), seq.name)
    } else {
        "Graph View".to_string()
    };

    let title_text = vec![Line::from(Span::styled(title, theme.accent_bold()))];

    let paragraph = Paragraph::new(title_text)
        .alignment(Alignment::Center)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(Span::styled(" Graph ", theme.accent_bold())),
        );

    f.render_widget(paragraph, area);
}

fn render_graph(
    f: &mut Frame,
    area: Rect,
    app: &App,
    seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    // Parse sequence data into points
    let data_points = parse_sequence_data(seq);

    if data_points.is_empty() {
        let text = Paragraph::new("No numeric data to plot").alignment(Alignment::Center);
        f.render_widget(text, area);
        return;
    }

    // Use Ratatui's canvas for simple visualization
    render_simple_graph(f, area, app, &data_points, theme);
}

fn parse_sequence_data(seq: &crate::api::Sequence) -> Vec<(f64, f64)> {
    let (start_index, _) = seq.parse_offset();

    seq.data
        .split(',')
        .enumerate()
        .filter_map(|(i, val)| {
            let trimmed = val.trim();
            // Try to parse as integer or float
            if let Ok(num) = trimmed.parse::<i64>() {
                Some(((start_index + i as i32) as f64, num as f64))
            } else if let Ok(num) = trimmed.parse::<f64>() {
                Some(((start_index + i as i32) as f64, num))
            } else {
                None
            }
        })
        .take(100) // Limit to 100 points for rendering
        .collect()
}

fn render_simple_graph(
    f: &mut Frame,
    area: Rect,
    app: &App,
    data: &[(f64, f64)],
    theme: &crate::ui::Theme,
) {
    if data.is_empty() {
        return;
    }

    // Find bounds
    let x_min = data.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let x_max = data
        .iter()
        .map(|(x, _)| *x)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_min = data.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let y_max = data
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, f64::max);

    // Add padding to bounds
    let x_padding = (x_max - x_min) * 0.1;
    let y_padding = (y_max - y_min) * 0.1;
    let x_min = x_min - x_padding;
    let x_max = x_max + x_padding;
    let y_min = y_min - y_padding;
    let y_max = y_max + y_padding;

    // Handle edge cases
    let (x_min, x_max) = if x_max == x_min {
        (x_min - 1.0, x_max + 1.0)
    } else {
        (x_min, x_max)
    };
    let (y_min, y_max) = if y_max == y_min {
        (y_min - 1.0, y_max + 1.0)
    } else {
        (y_min, y_max)
    };

    match app.graph_type {
        GraphType::Line | GraphType::Scatter => {
            render_standard_plot(
                f,
                area,
                data,
                x_min,
                x_max,
                y_min,
                y_max,
                &app.graph_type,
                theme,
            );
        }
        GraphType::LogScatter => {
            render_log_plot(f, area, data, x_min, x_max, theme);
        }
        GraphType::PinPlot => {
            render_pin_plot(f, area, data, x_min, x_max, y_min, y_max, theme);
        }
    }
}

fn render_standard_plot(
    f: &mut Frame,
    area: Rect,
    data: &[(f64, f64)],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    graph_type: &GraphType,
    theme: &crate::ui::Theme,
) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(Span::styled(
                    format!(
                        " {} ",
                        match graph_type {
                            GraphType::Line => "Line Chart",
                            GraphType::Scatter => "Scatter Plot",
                            _ => "Plot",
                        }
                    ),
                    theme.accent_bold(),
                )),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .marker(ratatui::symbols::Marker::Braille)
        .paint(|ctx| {
            // Draw points
            for (x, y) in data {
                ctx.draw(&Points {
                    coords: &[(*x, *y)],
                    color: theme.accent_color(),
                });
            }

            // For line chart, connect points
            if matches!(graph_type, GraphType::Line) {
                for window in data.windows(2) {
                    let (x1, y1) = window[0];
                    let (x2, y2) = window[1];

                    // Draw line between points by drawing multiple points
                    let steps = 20;
                    for i in 0..=steps {
                        let t = i as f64 / steps as f64;
                        let x = x1 + (x2 - x1) * t;
                        let y = y1 + (y2 - y1) * t;
                        ctx.draw(&Points {
                            coords: &[(x, y)],
                            color: theme.success_color(),
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, area);
}

fn render_log_plot(
    f: &mut Frame,
    area: Rect,
    data: &[(f64, f64)],
    x_min: f64,
    x_max: f64,
    theme: &crate::ui::Theme,
) {
    // Filter positive values for log plot
    let log_data: Vec<(f64, f64)> = data
        .iter()
        .filter(|(_, y)| *y > 0.0)
        .map(|(x, y)| (*x, y.ln()))
        .collect();

    if log_data.is_empty() {
        let text = Paragraph::new("No positive values to plot on logarithmic scale")
            .alignment(Alignment::Center);
        f.render_widget(text, area);
        return;
    }

    let y_min = log_data
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::INFINITY, f64::min);
    let y_max = log_data
        .iter()
        .map(|(_, y)| *y)
        .fold(f64::NEG_INFINITY, f64::max);
    let y_padding = (y_max - y_min) * 0.1;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(Span::styled(
                    " Logarithmic Scatter Plot ",
                    theme.accent_bold(),
                )),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min - y_padding, y_max + y_padding])
        .marker(ratatui::symbols::Marker::Braille)
        .paint(|ctx| {
            for (x, y) in &log_data {
                ctx.draw(&Points {
                    coords: &[(*x, *y)],
                    color: theme.accent_color(),
                });
            }
        });

    f.render_widget(canvas, area);
}

fn render_pin_plot(
    f: &mut Frame,
    area: Rect,
    data: &[(f64, f64)],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    theme: &crate::ui::Theme,
) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent())
                .title(Span::styled(" Pin Plot ", theme.accent_bold())),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .marker(ratatui::symbols::Marker::Braille)
        .paint(|ctx| {
            // Draw vertical lines (pins) from x-axis to each point
            for (x, y) in data {
                let steps = 20;
                for i in 0..=steps {
                    let t = i as f64 / steps as f64;
                    let pin_y = y_min + (*y - y_min) * t;
                    ctx.draw(&Points {
                        coords: &[(*x, pin_y)],
                        color: theme.highlight_color(),
                    });
                }

                // Draw point at top
                ctx.draw(&Points {
                    coords: &[(*x, *y)],
                    color: theme.danger_color(),
                });
            }
        });

    f.render_widget(canvas, area);
}

fn render_help(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::Theme) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("1", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("graph-help-line"))),
            Span::styled("2", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("graph-help-scatter"))),
            Span::styled("3", theme.accent_bold()),
            Span::raw(format!(" {} | ", app.i18n.t("graph-help-log"))),
            Span::styled("4", theme.accent_bold()),
            Span::raw(format!(" {}", app.i18n.t("graph-help-pin"))),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw(format!("{}: ", app.i18n.t("graph-current"))),
            Span::styled(format!("{:?}", app.graph_type), theme.highlight_bold()),
            Span::raw(" | "),
            Span::styled("Esc", theme.danger().add_modifier(Modifier::BOLD)),
            Span::raw(format!(" {}", app.i18n.t("graph-help-back"))),
        ]),
    ];

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

/// Render graph embedded in detail view tab (with help bar)
pub fn render_embedded(
    f: &mut Frame,
    area: Rect,
    app: &App,
    seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    // Split area into graph and help sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),   // Graph
            Constraint::Length(3), // Help
        ])
        .split(area);

    // Render the graph
    render_graph(f, chunks[0], app, seq, theme);

    // Render help bar
    let help_text = vec![Line::from(vec![
        Span::styled("1", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("graph-help-line"))),
        Span::styled("2", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("graph-help-scatter"))),
        Span::styled("3", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("graph-help-log"))),
        Span::styled("4", theme.accent_bold()),
        Span::raw(format!(" {} | ", app.i18n.t("graph-help-pin"))),
        Span::styled("g", theme.accent_bold()),
        Span::raw(" Full-screen"),
    ])];

    let help = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .style(theme.text())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.accent()),
        );

    f.render_widget(help, chunks[1]);
}
