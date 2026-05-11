use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::Block,
    Frame,
};
use crate::app::App;
use crate::theme::ThemeColors;
use crate::widgets;

pub fn render(frame: &mut Frame, app: &App) {
    let th = ThemeColors::get(app.theme);

    // Full-screen background
    frame.render_widget(
        Block::default().style(Style::default().bg(th.term_bg)),
        frame.area(),
    );

    // Top-level vertical split
    let main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // header
            Constraint::Length(1), // separator
            Constraint::Min(0),    // body
            Constraint::Length(1), // hotkeys
            Constraint::Length(1), // status bar
        ])
        .split(frame.area());

    let header_area  = main[0];
    let sep_area     = main[1];
    let body_area    = main[2];
    let hotkeys_area = main[3];
    let status_area  = main[4];

    // Body: horizontal split (left tree + right panel)
    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(34),
            Constraint::Min(0),
        ])
        .split(body_area);

    let left_area  = body[0];
    let right_area = body[1];

    // Left panel vertical split
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // pane title
            Constraint::Min(0),    // tree list
            Constraint::Length(2), // keyboard hints
        ])
        .split(left_area);

    // Right panel vertical split
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // pane title
            Constraint::Length(8), // card 01: Selección
            Constraint::Length(6), // card 02: Progreso
            Constraint::Min(0),    // card 03: Log en vivo
        ])
        .split(right_area);

    widgets::header::render(frame, th, header_area);
    widgets::header::render_sep(frame, th, sep_area);
    widgets::tree::render(frame, th, app, left[0], left[1], left[2]);
    widgets::header::render_right_title(frame, th, right[0]);
    widgets::card_sel::render(frame, th, right[1]);
    widgets::card_prog::render(frame, th, app, right[2]);
    widgets::card_log::render(frame, th, app, right[3]);
    widgets::header::render_hotkeys(frame, th, hotkeys_area);
    widgets::card_log::render_statusbar(frame, th, app, status_area);
}
