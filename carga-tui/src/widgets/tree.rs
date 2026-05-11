use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, TREE};
use crate::theme::ThemeColors;

pub fn render(
    frame: &mut Frame,
    th: &ThemeColors,
    app: &App,
    title_area: Rect,
    list_area: Rect,
    foot_area: Rect,
) {
    render_title(frame, th, title_area);
    render_list(frame, th, app, list_area);
    render_foot(frame, th, foot_area);
}

fn render_title(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let line = Line::from(vec![
        Span::styled("● ", Style::default().fg(th.accent)),
        Span::styled("Árbol de carpetas", Style::default().fg(th.term_dim)),
        Span::styled("  3 dirs · 1 sel", Style::default().fg(th.term_mute)),
    ]);
    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.term_bg)),
        area,
    );
}

fn render_list(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let items: Vec<ListItem> = TREE.iter().map(|e| {
        let indent = "  ".repeat(e.indent as usize);
        let chevron = if e.is_dir {
            if e.is_open { "▾ " } else { "▸ " }
        } else {
            "· "
        };

        let mut spans: Vec<Span> = Vec::new();

        if e.selected {
            spans.push(Span::styled("▌", Style::default().fg(th.accent)));
        } else {
            spans.push(Span::raw(" "));
        }

        spans.push(Span::raw(indent));
        spans.push(Span::styled(chevron, Style::default().fg(th.term_mute)));

        let name_style = if e.selected {
            Style::default().fg(th.accent).add_modifier(Modifier::BOLD)
        } else if e.is_dir {
            Style::default().fg(th.info)
        } else {
            Style::default().fg(th.term_fg)
        };

        if e.checked {
            spans.push(Span::styled(e.name, name_style));
            spans.push(Span::styled(" ✓", Style::default().fg(th.ok).add_modifier(Modifier::BOLD)));
        } else {
            spans.push(Span::styled(e.name, name_style));
        }

        if e.cursor {
            let block = if app.cursor_visible { "█" } else { " " };
            spans.push(Span::styled(block, Style::default().fg(th.accent)));
        }

        if !e.meta.is_empty() {
            spans.push(Span::styled(format!("  {}", e.meta), Style::default().fg(th.term_mute)));
        }

        let line_style = if e.selected {
            Style::default().bg(th.selection_bg)
        } else {
            Style::default().bg(th.term_bg)
        };

        ListItem::new(Line::from(spans)).style(line_style)
    }).collect();

    let list = List::new(items).style(Style::default().bg(th.term_bg));
    frame.render_widget(list, area);
}

fn render_foot(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let hints: &[(&str, &str)] = &[
        ("[↑↓]", " navegar  "),
        ("[⏎]",  " abrir  "),
        ("[␣]",  " marcar  "),
        ("[/]",  " buscar"),
    ];

    let mut spans = Vec::new();
    for (key, label) in hints {
        spans.push(Span::styled(*key, Style::default().fg(th.term_fg)));
        spans.push(Span::styled(*label, Style::default().fg(th.term_dim)));
    }

    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(th.term_bg)),
        area,
    );
}
