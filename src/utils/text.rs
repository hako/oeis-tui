use once_cell::sync::Lazy;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};
use regex::{Regex, RegexBuilder};

use crate::ui::Theme;

/// Regex that matches canonical OEIS sequence identifiers (A-number format).
static ANUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"A\d{6}").expect("valid regex"));

/// Highlight OEIS A-number references within `text`, registering each discovered
/// identifier in `references`. The currently selected reference is highlighted
/// with an accent background to indicate focus.
pub fn highlight_anumbers_line(
    text: &str,
    references: &mut Vec<String>,
    selected: Option<usize>,
    theme: &Theme,
) -> Line<'static> {
    let mut spans = Vec::new();
    let mut cursor = 0;

    for mat in ANUMBER_PATTERN.find_iter(text) {
        if mat.start() > cursor {
            spans.push(Span::styled(
                text[cursor..mat.start()].to_string(),
                theme.text(),
            ));
        }

        let index = references.len();
        let matched = text[mat.start()..mat.end()].to_string();
        references.push(matched.clone());

        let style = if Some(index) == selected {
            Style::default()
                .fg(theme.text_color())
                .bg(theme.highlight_bg_color())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(theme.accent_color())
                .add_modifier(Modifier::UNDERLINED)
        };

        spans.push(Span::styled(matched, style));
        cursor = mat.end();
    }

    if cursor < text.len() {
        spans.push(Span::styled(text[cursor..].to_string(), theme.text()));
    }

    if spans.is_empty() {
        spans.push(Span::styled(text.to_string(), theme.text()));
    }

    Line::from(spans)
}

/// Highlight any of the `terms` (case-insensitive) that appear in `text`.
/// Returns a [`Line`] with styled spans for matches.
pub fn highlight_terms_line(text: &str, terms: &[String], theme: &Theme) -> Line<'static> {
    let Some(regex) = build_terms_regex(terms) else {
        return Line::from(vec![Span::styled(text.to_string(), theme.text())]);
    };

    let mut spans = Vec::new();
    let mut cursor = 0;

    for mat in regex.find_iter(text) {
        if mat.start() > cursor {
            spans.push(Span::styled(
                text[cursor..mat.start()].to_string(),
                theme.text(),
            ));
        }

        spans.push(Span::styled(
            text[mat.start()..mat.end()].to_string(),
            theme.highlight_bold(),
        ));

        cursor = mat.end();
    }

    if cursor < text.len() {
        spans.push(Span::styled(text[cursor..].to_string(), theme.text()));
    }

    if spans.is_empty() {
        spans.push(Span::styled(text.to_string(), theme.text()));
    }

    Line::from(spans)
}

fn build_terms_regex(terms: &[String]) -> Option<Regex> {
    let pattern = terms
        .iter()
        .filter(|term| !term.is_empty())
        .map(|term| regex::escape(term))
        .collect::<Vec<_>>()
        .join("|");

    if pattern.is_empty() {
        return None;
    }

    RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .ok()
}

/// Parse a raw search query into highlight-able tokens.
pub fn parse_search_terms(query: &str) -> Vec<String> {
    let mut terms: Vec<String> = query
        .split([';', '\n'])
        .flat_map(|chunk| chunk.split_whitespace())
        .flat_map(|token| token.split(','))
        .filter_map(|token| {
            let cleaned = token.trim_matches(|c: char| !c.is_ascii_alphanumeric());
            if cleaned.contains(':') || cleaned.is_empty() {
                return None;
            }

            if cleaned.len() == 1 && !cleaned.chars().all(|c| c.is_ascii_digit()) {
                return None;
            }

            Some(cleaned.to_string())
        })
        .collect();

    let trimmed = query.trim();
    if !trimmed.is_empty() {
        terms.push(trimmed.to_string());
    }

    terms.sort();
    terms.dedup();
    terms
}

pub fn highlight_sequence_line(text: &str, query: &str, theme: &Theme) -> Option<Line<'static>> {
    let parts: Vec<&str> = query
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    if parts.len() < 2 {
        return None;
    }

    let pattern = parts
        .iter()
        .map(|part| regex::escape(part))
        .collect::<Vec<_>>()
        .join(r"\s*,\s*");

    let regex = RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .ok()?;

    let mut spans = Vec::new();
    let mut cursor = 0;

    for mat in regex.find_iter(text) {
        if mat.start() > cursor {
            spans.push(Span::styled(
                text[cursor..mat.start()].to_string(),
                theme.text(),
            ));
        }

        spans.push(Span::styled(
            text[mat.start()..mat.end()].to_string(),
            theme.highlight_bold(),
        ));

        cursor = mat.end();
    }

    if cursor < text.len() {
        spans.push(Span::styled(text[cursor..].to_string(), theme.text()));
    }

    if spans.len() <= 1 {
        return None;
    }

    Some(Line::from(spans))
}
