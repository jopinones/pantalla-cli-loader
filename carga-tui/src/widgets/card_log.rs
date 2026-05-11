use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::app::{App, LogLevel, INITIAL_LOGS, LOG_POOL};
use crate::theme::ThemeColors;

pub fn render(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let title = Line::from(vec![
        Span::styled("03", Style::default().fg(th.accent)),
        Span::styled(" Log en vivo", Style::default().fg(th.term_dim)),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(th.rule))
        .title(title)
        .style(Style::default().bg(th.term_bg));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    render_log_lines(frame, th, app, inner);
}

fn level_color(level: LogLevel, th: &ThemeColors) -> (ratatui::style::Color, &'static str) {
    match level {
        LogLevel::Info => (th.info, "INFO "),
        LogLevel::Ok   => (th.ok,   "OK   "),
        LogLevel::Warn => (th.warn, "WARN "),
        LogLevel::Err  => (th.err,  "ERR  "),
    }
}

fn render_log_lines(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let max_lines = area.height as usize;

    // Build visible log window: initial logs + rotating pool entry
    let mut all_lines: Vec<(&str, LogLevel, &str)> = INITIAL_LOGS
        .iter()
        .map(|l| (l.ts, l.level, l.msg))
        .collect();

    // Append rotating pool lines up to log_head
    for i in 0..=app.log_head {
        let e = &LOG_POOL[i % LOG_POOL.len()];
        all_lines.push((e.ts, e.level, e.msg));
    }

    // Keep only last max_lines entries
    let start = all_lines.len().saturating_sub(max_lines);
    let visible = &all_lines[start..];

    let items: Vec<ListItem> = visible.iter().enumerate().map(|(idx, (ts, level, msg))| {
        let (color, label) = level_color(*level, th);
        let is_last = idx == visible.len() - 1;

        let mut spans = vec![
            Span::styled(*ts, Style::default().fg(th.term_mute)),
            Span::raw("  "),
            Span::styled(label, Style::default().fg(color).add_modifier(Modifier::BOLD)),
            Span::raw("  "),
            Span::styled(*msg, Style::default().fg(th.term_fg)),
        ];

        if is_last {
            let cursor = if app.cursor_visible { "█" } else { " " };
            spans.push(Span::styled(cursor, Style::default().fg(th.term_fg)));
        }

        ListItem::new(Line::from(spans)).style(Style::default().bg(th.term_bg))
    }).collect();

    frame.render_widget(
        List::new(items).style(Style::default().bg(th.term_bg)),
        area,
    );
}

pub fn render_statusbar(frame: &mut Frame, th: &ThemeColors, app: &App, area: Rect) {
    let pip = if app.cursor_visible { "● " } else { "○ " };
    let left = "MySQL  localhost:3306 / contribuciones  user  etl_user  trx  #42 · running";
    let right = "throughput  5 921 filas/s  mem  42.1 MB  NORMAL";
    let pip_w = 2usize;
    let left_w = left.chars().count();
    let right_w = right.chars().count();
    let pad = (area.width as usize).saturating_sub(pip_w + left_w + right_w + 2);

    let line = Line::from(vec![
        Span::styled(pip, Style::default().fg(th.ok)),
        Span::styled("MySQL  ", Style::default().fg(th.term_mute)),
        Span::styled("localhost:3306 / contribuciones  ", Style::default().fg(th.term_fg)),
        Span::styled("user  ", Style::default().fg(th.term_mute)),
        Span::styled("etl_user  ", Style::default().fg(th.term_fg)),
        Span::styled("trx  ", Style::default().fg(th.term_mute)),
        Span::styled("#42 · running", Style::default().fg(th.term_fg)),
        Span::raw(" ".repeat(pad)),
        Span::styled("throughput  ", Style::default().fg(th.term_mute)),
        Span::styled("5 921 filas/s  ", Style::default().fg(th.term_fg)),
        Span::styled("mem  ", Style::default().fg(th.term_mute)),
        Span::styled("42.1 MB  ", Style::default().fg(th.term_fg)),
        Span::styled("NORMAL", Style::default().fg(th.accent).add_modifier(Modifier::BOLD)),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.rule)),
        area,
    );
}
