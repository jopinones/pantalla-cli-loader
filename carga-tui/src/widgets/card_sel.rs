use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::theme::ThemeColors;

pub fn render(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let title = Line::from(vec![
        Span::styled("01", Style::default().fg(th.accent)),
        Span::styled(" Selección", Style::default().fg(th.term_dim)),
    ]);
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(th.rule))
        .title(title)
        .style(Style::default().bg(th.term_bg));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split inner into kv area + button row
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);
    let kv_area  = sections[0];
    let btn_area = sections[1];

    render_kv(frame, th, kv_area);
    render_buttons(frame, th, btn_area);
}

fn render_kv(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let key_style = Style::default().fg(th.term_mute);
    let val_style = Style::default().fg(th.term_fg);
    let dim_style = Style::default().fg(th.term_dim);
    let inf_style = Style::default().fg(th.info);

    let rows = vec![
        Line::from(vec![
            Span::styled("archivo  ", key_style),
            Span::styled("rol_2024_s2.csv", val_style),
            Span::styled("  · 4.2 MB · 18 cols · 12 500 filas", dim_style),
        ]),
        Line::from(vec![
            Span::styled("tabla    ", key_style),
            Span::styled("contribuciones_staging", val_style),
            Span::styled("  [", dim_style),
            Span::styled("staging", inf_style),
            Span::styled("]", dim_style),
        ]),
        Line::from(vec![
            Span::styled("mapeo    ", key_style),
            Span::styled("configs/rol_cobro.toml", val_style),
            Span::styled("  · sha 8f3a…91", dim_style),
        ]),
        Line::from(vec![
            Span::styled("modo     ", key_style),
            Span::styled("INSERT IGNORE", val_style),
            Span::styled("  · batch 1 000 · paralelo 4", dim_style),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(rows).style(Style::default().bg(th.term_bg)),
        area,
    );
}

fn render_buttons(frame: &mut Frame, th: &ThemeColors, area: Rect) {
    let primary_bg = th.accent;
    let primary_fg = th.term_bg;
    let normal_bg  = th.rule;
    let normal_fg  = th.term_fg;
    let ghost_fg   = th.term_dim;

    let line = Line::from(vec![
        Span::styled(" [E] Ejecutar ", Style::default()
            .fg(primary_fg).bg(primary_bg).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(" [D] Dry-run ", Style::default().fg(normal_fg).bg(normal_bg)),
        Span::raw("  "),
        Span::styled(" [V] Validar ", Style::default().fg(normal_fg).bg(normal_bg)),
        Span::raw("  "),
        Span::styled(" [R] Reintentar ", Style::default().fg(ghost_fg).bg(th.term_bg)),
    ]);

    frame.render_widget(
        Paragraph::new(line).style(Style::default().bg(th.term_bg)),
        area,
    );
}
