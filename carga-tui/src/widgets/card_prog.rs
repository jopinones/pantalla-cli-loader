use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::App;
use crate::theme::ThemeColors;

const PROGRESS: u8 = 67;

pub fn render(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let title = Line::from(vec![
        Span::styled("02", Style::default().fg(th.accent)),
        Span::styled(" Progreso", Style::default().fg(th.term_dim)),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(th.rule))
        .title(title)
        .style(Style::default().bg(th.term_bg));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(inner);

    render_meta(frame, th, rows[0]);
    render_bar(frame, th, app, rows[1]);
    render_stages(frame, th, app, rows[2]);
}

fn render_meta(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let left = "Cargando · batch 8 / 13";
    let right = "67%  · 8 420 / 12 500 filas · ETA 00:00:21";
    let left_w = left.chars().count();
    let right_w = right.chars().count();
    let pad = (area.width as usize).saturating_sub(left_w + right_w);

    let line = Line::from(vec![
        Span::styled(left, Style::default().fg(th.term_dim)),
        Span::raw(" ".repeat(pad)),
        Span::styled("67%", Style::default().fg(th.accent).add_modifier(Modifier::BOLD)),
        Span::styled("  · 8 420 / 12 500 filas · ETA 00:00:21", Style::default().fg(th.term_fg)),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.term_bg)),
        area,
    );
}

fn render_bar(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let width = area.width as usize;
    let fill_len = (width * PROGRESS as usize) / 100;
    let empty_len = width.saturating_sub(fill_len);

    let offset = (app.tick / 3) as usize % 4;
    let fill_chars = ['▓', '▓', '▒', '▓'];
    let filled: String = (0..fill_len)
        .map(|i| fill_chars[(i + offset) % fill_chars.len()])
        .collect();
    let empty: String = "░".repeat(empty_len);

    let line = Line::from(vec![
        Span::styled(filled, Style::default().fg(th.accent).bg(th.rule)),
        Span::styled(empty,  Style::default().fg(th.rule_strong).bg(th.rule)),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.term_bg)),
        area,
    );
}

fn render_stages(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let pip_active = if app.cursor_visible { "●" } else { "○" };

    let stages: &[(&str, u8)] = &[
        ("Schema",       0), // 0 = done
        ("Validación",   0),
        ("Inserción",    1), // 1 = active
        ("Verificación", 2), // 2 = pending
        ("Commit",       2),
    ];

    let sep = Span::styled("  →  ", Style::default().fg(th.rule_strong));

    let mut spans: Vec<Span> = Vec::new();
    for (i, (name, kind)) in stages.iter().enumerate() {
        let (pip, color) = match kind {
            0 => ("✓", th.ok),
            1 => (pip_active, th.accent),
            _ => ("·", th.term_mute),
        };

        spans.push(Span::styled(pip, Style::default().fg(color)));
        spans.push(Span::raw(" "));
        spans.push(Span::styled(*name, Style::default().fg(color)));

        if i < stages.len() - 1 {
            spans.push(sep.clone());
        }
    }

    frame.render_widget(
        Paragraph::new(Line::from(spans)).style(Style::default().bg(th.term_bg)),
        area,
    );
}
