use crate::app::{App, ExportFormat};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

fn render_format_selection(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::theme::Theme) {
    let formats = [
        (
            ExportFormat::Json,
            format!("{} - {}", app.i18n.t("export-json"), app.i18n.t("export-json-desc")),
        ),
        (
            ExportFormat::Csv,
            format!("{} - {}", app.i18n.t("export-csv"), app.i18n.t("export-csv-desc")),
        ),
        (
            ExportFormat::Txt,
            format!("{} - {}", app.i18n.t("export-txt"), app.i18n.t("export-txt-desc")),
        ),
        (
            ExportFormat::Markdown,
            format!("{} - {}", app.i18n.t("export-markdown"), app.i18n.t("export-markdown-desc")),
        ),
        (
            ExportFormat::BFile,
            format!("{} - {}", app.i18n.t("export-bfile"), app.i18n.t("export-bfile-desc")),
        ),
    ];

    let items: Vec<ListItem> = formats
        .iter()
        .map(|(format, description)| {
            let is_selected = format == &app.export_format;
            let style = if is_selected {
                Style::default()
                    .fg(theme.highlight_color())
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let prefix = if is_selected { "▶ " } else { "  " };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(format.as_str(), style),
                Span::raw(" - "),
                Span::styled(description.clone(), Style::default().fg(theme.muted_color())),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", app.i18n.t("export-select-format"))),
    );

    f.render_widget(list, area);
}

fn render_preview(f: &mut Frame, area: Rect, app: &App, theme: &crate::ui::theme::Theme) {
    if let Some(ref seq) = app.current_sequence {
        let preview = generate_export_preview(seq, &app.export_format);

        let text = Paragraph::new(preview)
            .block(Block::default().borders(Borders::ALL).title(format!(" {} ", app.i18n.t("export-preview"))))
            .style(Style::default().fg(theme.muted_color()))
            .scroll((0, 0));

        f.render_widget(text, area);
    } else {
        let text = Paragraph::new(app.i18n.t("export-no-sequence").to_string()).alignment(Alignment::Center);
        f.render_widget(text, area);
    }
}

fn generate_export_preview(
    seq: &crate::api::Sequence,
    format: &ExportFormat,
) -> Vec<Line<'static>> {
    match format {
        ExportFormat::Json => generate_json_preview(seq),
        ExportFormat::Csv => generate_csv_preview(seq),
        ExportFormat::Txt => generate_txt_preview(seq),
        ExportFormat::Markdown => generate_markdown_preview(seq),
        ExportFormat::BFile => generate_bfile_preview(seq),
    }
}

fn generate_json_preview(seq: &crate::api::Sequence) -> Vec<Line<'static>> {
    vec![
        Line::from("{"),
        Line::from(format!("  \"number\": {},", seq.number)),
        Line::from(format!("  \"name\": \"{}\",", seq.name)),
        Line::from(format!("  \"data\": \"{}\",", seq.data)),
        Line::from(format!("  \"offset\": \"{}\",", seq.offset)),
        Line::from(format!("  \"keywords\": \"{}\",", seq.keyword)),
        Line::from(format!("  \"author\": \"{}\",", seq.author)),
        Line::from("  ..."),
        Line::from("}"),
    ]
}

fn generate_csv_preview(seq: &crate::api::Sequence) -> Vec<Line<'static>> {
    let values: Vec<&str> = seq.data.split(',').take(10).collect();

    vec![
        Line::from("# A-number,Name,Values"),
        Line::from(format!(
            "{},\"{}\",\"{}{}\"",
            seq.a_number(),
            seq.name,
            values.join(","),
            if seq.data.split(',').count() > 10 {
                ",..."
            } else {
                ""
            }
        )),
    ]
}

fn generate_txt_preview(seq: &crate::api::Sequence) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("{}: {}", seq.a_number(), seq.name)),
        Line::from(""),
        Line::from(format!("Offset: {}", seq.offset)),
        Line::from(format!("Keywords: {}", seq.keyword)),
        Line::from(""),
        Line::from("Data:"),
        Line::from(seq.data.clone()),
        Line::from(""),
        Line::from("..."),
    ]
}

fn generate_markdown_preview(seq: &crate::api::Sequence) -> Vec<Line<'static>> {
    vec![
        Line::from(format!("# {}: {}", seq.a_number(), seq.name)),
        Line::from(""),
        Line::from("## Sequence Data"),
        Line::from(""),
        Line::from(format!("`{}`", seq.data)),
        Line::from(""),
        Line::from("## Metadata"),
        Line::from(""),
        Line::from(format!("- **Offset**: {}", seq.offset)),
        Line::from(format!("- **Keywords**: {}", seq.keyword)),
        Line::from(format!("- **Author**: {}", seq.author)),
        Line::from(""),
        Line::from("..."),
    ]
}

fn generate_bfile_preview(seq: &crate::api::Sequence) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from(format!("# {} - {}", seq.a_number(), seq.name)),
        Line::from(""),
    ];

    // Show first 10 terms from sequence data as preview
    for (i, value) in seq.data.split(',').enumerate().take(10) {
        lines.push(Line::from(format!("{} {}", i, value.trim())));
    }

    lines.push(Line::from(""));
    lines.push(Line::from("..."));

    lines
}

/// Generate full export content for a sequence
pub fn export_sequence(
    seq: &crate::api::Sequence,
    format: &ExportFormat,
    bfile_data: Option<&Vec<crate::api::models::BFileEntry>>,
) -> String {
    match format {
        ExportFormat::Json => export_to_json(seq),
        ExportFormat::Csv => export_to_csv(seq),
        ExportFormat::Txt => export_to_txt(seq),
        ExportFormat::Markdown => export_to_markdown(seq),
        ExportFormat::BFile => export_to_bfile(seq, bfile_data),
    }
}

fn export_to_json(seq: &crate::api::Sequence) -> String {
    serde_json::to_string_pretty(seq).unwrap_or_else(|_| "Error serializing to JSON".to_string())
}

fn export_to_csv(seq: &crate::api::Sequence) -> String {
    format!(
        "A-number,Index,Value\n{}",
        seq.data
            .split(',')
            .enumerate()
            .map(|(i, val)| format!("{},{},{}", seq.a_number(), i, val.trim()))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn export_to_txt(seq: &crate::api::Sequence) -> String {
    let mut output = String::new();

    output.push_str(&format!("{}: {}\n", seq.a_number(), seq.name));
    output.push_str(&"=".repeat(seq.name.len() + seq.a_number().len() + 2));
    output.push_str("\n\n");

    output.push_str(&format!("Offset: {}\n", seq.offset));
    output.push_str(&format!("Keywords: {}\n", seq.keyword));
    if !seq.author.is_empty() {
        output.push_str(&format!("Author: {}\n", seq.author));
    }
    output.push('\n');

    output.push_str("Data:\n");
    output.push_str(&format!("{}\n\n", seq.data));

    if !seq.comment.is_empty() {
        output.push_str("Comments:\n");
        for comment in &seq.comment {
            output.push_str(&format!("  {}\n", comment));
        }
        output.push('\n');
    }

    if !seq.formula.is_empty() {
        output.push_str("Formulas:\n");
        for formula in &seq.formula {
            output.push_str(&format!("  {}\n", formula));
        }
        output.push('\n');
    }

    if !seq.reference.is_empty() {
        output.push_str("References:\n");
        for reference in &seq.reference {
            output.push_str(&format!("  {}\n", reference));
        }
        output.push('\n');
    }

    if !seq.link.is_empty() {
        output.push_str("Links:\n");
        for link in &seq.link {
            output.push_str(&format!("  {}\n", link));
        }
        output.push('\n');
    }

    if !seq.xref.is_empty() {
        output.push_str("Cross-references:\n");
        for xref in &seq.xref {
            output.push_str(&format!("  {}\n", xref));
        }
        output.push('\n');
    }

    output.push_str(&format!("\nSource: {}\n", seq.url()));

    output
}

fn export_to_markdown(seq: &crate::api::Sequence) -> String {
    let mut output = String::new();

    output.push_str(&format!("# {}: {}\n\n", seq.a_number(), seq.name));

    output.push_str("## Sequence Data\n\n");
    output.push_str(&format!("`{}`\n\n", seq.data));

    output.push_str("## Metadata\n\n");
    output.push_str(&format!("- **A-number**: {}\n", seq.a_number()));
    output.push_str(&format!("- **Offset**: {}\n", seq.offset));
    output.push_str(&format!("- **Keywords**: {}\n", seq.keyword));
    if !seq.author.is_empty() {
        output.push_str(&format!("- **Author**: {}\n", seq.author));
    }
    if !seq.created.is_empty() {
        output.push_str(&format!("- **Created**: {}\n", seq.created));
    }
    if !seq.time.is_empty() {
        output.push_str(&format!("- **Last Modified**: {}\n", seq.time));
    }
    output.push('\n');

    if !seq.comment.is_empty() {
        output.push_str("## Comments\n\n");
        for comment in &seq.comment {
            output.push_str(&format!("{}\n\n", comment));
        }
    }

    if !seq.formula.is_empty() {
        output.push_str("## Formulas\n\n");
        for formula in &seq.formula {
            output.push_str(&format!("- {}\n", formula));
        }
        output.push('\n');
    }

    if !seq.example.is_empty() {
        output.push_str("## Examples\n\n");
        for example in &seq.example {
            output.push_str(&format!("```\n{}\n```\n\n", example));
        }
    }

    if !seq.maple.is_empty() || !seq.mathematica.is_empty() || !seq.program.is_empty() {
        output.push_str("## Code\n\n");

        if !seq.maple.is_empty() {
            output.push_str("### Maple\n\n");
            for code in &seq.maple {
                output.push_str(&format!("```maple\n{}\n```\n\n", code));
            }
        }

        if !seq.mathematica.is_empty() {
            output.push_str("### Mathematica\n\n");
            for code in &seq.mathematica {
                output.push_str(&format!("```mathematica\n{}\n```\n\n", code));
            }
        }

        if !seq.program.is_empty() {
            output.push_str("### Other Programs\n\n");
            for code in &seq.program {
                output.push_str(&format!("```\n{}\n```\n\n", code));
            }
        }
    }

    if !seq.reference.is_empty() {
        output.push_str("## References\n\n");
        for reference in &seq.reference {
            output.push_str(&format!("- {}\n", reference));
        }
        output.push('\n');
    }

    if !seq.link.is_empty() {
        output.push_str("## Links\n\n");
        for link in &seq.link {
            output.push_str(&format!("- {}\n", link));
        }
        output.push('\n');
    }

    if !seq.xref.is_empty() {
        output.push_str("## Cross-references\n\n");
        for xref in &seq.xref {
            output.push_str(&format!("- {}\n", xref));
        }
        output.push('\n');
    }

    output.push_str("---\n\n");
    output.push_str(&format!(
        "**Source**: [{}]({})\n\n",
        seq.a_number(),
        seq.url()
    ));
    output.push_str("*Data from the On-Line Encyclopedia of Integer Sequences (OEIS)*\n");

    output
}

fn export_to_bfile(
    seq: &crate::api::Sequence,
    bfile_data: Option<&Vec<crate::api::models::BFileEntry>>,
) -> String {
    let mut output = String::new();

    // Add header comment with A-number and name
    output.push_str(&format!("# {} - {}\n", seq.a_number(), seq.name));

    // If B-file data is available, export it
    if let Some(data) = bfile_data {
        for entry in data {
            output.push_str(&format!("{} {}\n", entry.index, entry.value));
        }
    } else {
        // Fallback: export sequence data from the main data field
        output.push_str("# Note: B-file data not loaded, showing sequence data instead\n");
        for (i, value) in seq.data.split(',').enumerate() {
            output.push_str(&format!("{} {}\n", i, value.trim()));
        }
    }

    output
}

/// Render export embedded in detail view tab (no title/help bars)
pub fn render_embedded(
    f: &mut Frame,
    area: Rect,
    app: &mut App,
    _seq: &crate::api::Sequence,
    theme: &crate::ui::Theme,
) {
    // Split into format selection and preview
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Format selection
            Constraint::Min(5),     // Preview
        ])
        .split(area);

    render_format_selection(f, chunks[0], app, theme);
    render_preview(f, chunks[1], app, theme);
}
