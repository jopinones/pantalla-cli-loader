use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use crate::theme::ThemeColors;

pub fn render(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let left = "▲ carga  v0.4.2  ~ / datos / contribuciones";
    let right = "? ayuda  ·  : comando  ·  q salir";
    let left_w = left.chars().count();
    let right_w = right.chars().count();
    let pad = (area.width as usize).saturating_sub(left_w + right_w);

    let line = Line::from(vec![
        Span::styled("▲ carga", Style::default().fg(th.accent).add_modifier(Modifier::BOLD)),
        Span::styled("  v0.4.2", Style::default().fg(th.term_mute)),
        Span::styled("  ~ / datos / contribuciones", Style::default().fg(th.term_dim)),
        Span::raw(" ".repeat(pad)),
        Span::styled(right, Style::default().fg(th.term_dim)),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.term_bg)),
        area,
    );
}

pub fn render_sep(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let line = "─".repeat(area.width as usize);
    frame.render_widget(
        Paragraph::new(line).style(Style::default().fg(th.rule_strong).bg(th.term_bg)),
        area,
    );
}

pub fn render_right_title(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let line = Line::from(vec![
        Span::styled("● ", Style::default().fg(th.accent)),
        Span::styled("Panel de ejecución", Style::default().fg(th.term_dim)),
        Span::styled("  proceso_id 42 · running", Style::default().fg(th.term_mute)),
    ]);
    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.term_bg)),
        area,
    );
}

pub fn render_hotkeys(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let hints: &[(&str, &str)] = &[
        ("E", "ejecutar  "),
        ("D", "dry-run  "),
        ("V", "validar  "),
        ("L", "log  "),
        ("C", "copiar SQL  "),
        ("R", "reintentar  "),
        (":", "comando  "),
        ("q", "salir"),
    ];

    let mut spans = Vec::new();
    for (key, label) in hints {
        spans.push(Span::styled(*key, Style::default().fg(th.accent).add_modifier(Modifier::BOLD)));
        spans.push(Span::styled(*label, Style::default().fg(th.term_dim)));
    }

    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(th.term_bg)),
        area,
    );
}
