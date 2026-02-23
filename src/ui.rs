use chrono::Local;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use whoami;

use crate::app::{App, AppState, MASK_CHAR};
use crate::logo::{LOGO, LOGO_HEIGHT, LOGO_WIDTH};

const BG: Color = Color::Rgb(24, 24, 37); // Catppuccin Mocha base
const ACCENT: Color = Color::Rgb(203, 166, 247); // Catppuccin Mauve
const SUBTEXT: Color = Color::Rgb(166, 173, 200); // Catppuccin Subtext1
const MASK_COLOR: Color = Color::Rgb(137, 180, 250); // Catppuccin Blue
const ERR_COLOR: Color = Color::Rgb(243, 139, 168); // Catppuccin Red
const OK_COLOR: Color = Color::Rgb(166, 227, 161);
const DIM: Color = Color::Rgb(88, 91, 112);

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();

    frame.render_widget(Block::default().style(Style::default().bg(BG)), area);

    let content_area = centred_rect(area, LOGO_WIDTH + 4, 22);

    let chunks = Layout::vertical([
        Constraint::Length(LOGO_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Length(3),
        Constraint::Min(0),
    ])
    .split(content_area);

    render_logo(frame, chunks[0]);
    render_time(frame, chunks[2]);
    render_dateinfo(frame, chunks[3]);
    render_distro(frame, chunks[4]);
    render_password(app, frame, chunks[5]);
    render_status(app, frame, chunks[6]);
}

fn render_logo(frame: &mut Frame, area: Rect) {
    let logo_widget = Paragraph::new(LOGO)
        .alignment(Alignment::Center)
        .style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD));

    frame.render_widget(logo_widget, area);
}

fn render_time(frame: &mut Frame, area: Rect) {
    let now = Local::now();
    let time_str = now.format("%H:%M:%S").to_string();
    let line = Line::from(vec![
        Span::styled(" ", Style::default().fg(ACCENT)),
        Span::styled(
            time_str,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

fn render_dateinfo(frame: &mut Frame, area: Rect) {
    let now = Local::now();
    let date_str = now.format("%a %d %b %Y").to_string();

    let user = whoami::username();
    let host = whoami::fallible::hostname().unwrap_or_else(|_| "UNKNOWN".to_string());
    let info_line = Line::from(vec![
        Span::styled(date_str, Style::default()),
        Span::styled("  󱙵   ", Style::default().fg(SUBTEXT)),
        Span::styled(user, Style::default().fg(ACCENT)),
        Span::styled("@", Style::default().fg(SUBTEXT)),
        Span::styled(host, Style::default().fg(Color::White)),
    ]);

    let info_widget = Paragraph::new(info_line).alignment(Alignment::Center);

    frame.render_widget(info_widget, area);
}

fn render_sysinfo(frame: &mut Frame, area: Rect) {
    let user = whoami::username();
    let host = whoami::fallible::hostname().unwrap_or_else(|_| "UNKNOWN".to_string());

    let info_line = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled("  󱙵   ", Style::default().fg(SUBTEXT)),
        Span::styled(&user, Style::default().fg(ACCENT)),
        Span::styled("@", Style::default().fg(SUBTEXT)),
        Span::styled(&host, Style::default().fg(Color::White)),
    ]);

    let info_widget = Paragraph::new(info_line).alignment(Alignment::Center);

    frame.render_widget(info_widget, area);
}

fn render_distro(frame: &mut Frame, area: Rect) {
    let distro = whoami::distro();
    let distro_line = Line::from(vec![Span::styled(&distro, Style::default().fg(ACCENT))]);

    let distro_widget = Paragraph::new(distro_line).alignment(Alignment::Center);

    frame.render_widget(distro_widget, area);
}

fn render_password(app: &App, frame: &mut Frame, area: Rect) {
    let area = apply_shake(area, app.shake_offset());
    let border_color = match &app.state {
        AppState::Locked => ACCENT,
        AppState::Authenticating => DIM,
        AppState::Failed(_) => ERR_COLOR,
        AppState::Unlocked => OK_COLOR,
    };

    let content: Line = match &app.state {
        AppState::Authenticating => Line::from(Span::styled(
            " Checking..",
            Style::default().fg(DIM).add_modifier(Modifier::ITALIC),
        )),
        _ => {
            let dots: String = std::iter::repeat(MASK_CHAR)
                .take(app.password.len())
                .collect();

            let display = if app.password.is_empty() {
                " ".to_string()
            } else {
                format!("  {}", dots)
            };
            Line::from(Span::styled(
                display,
                Style::default().fg(MASK_COLOR).add_modifier(Modifier::BOLD),
            ))
        }
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .title(" Password ")
        .title_alignment(Alignment::Center);

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

fn render_status(app: &App, frame: &mut Frame, area: Rect) {
    let (text, style) = match &app.state {
        AppState::Locked => ("".to_string(), Style::default()),
        AppState::Authenticating => (
            "Verifying Credentials..".to_string(),
            Style::default().fg(DIM).add_modifier(Modifier::ITALIC),
        ),
        AppState::Failed(n) => (
            format!("X Wrong Password (Attempt {})", n),
            Style::default().fg(ERR_COLOR),
        ),
        AppState::Unlocked => (
            " Unlocked ".to_string(),
            Style::default().fg(OK_COLOR).add_modifier(Modifier::BOLD),
        ),
    };

    frame.render_widget(
        Paragraph::new(text)
            .style(style)
            .alignment(Alignment::Center),
        area,
    );
}

fn centred_rect(area: Rect, width: u16, height: u16) -> Rect {
    let width = width.min(area.width);
    let height = height.min(area.height);

    let h_padding = (area.width.saturating_sub(width)) / 2;
    let [_, h_centre, _] = Layout::horizontal([
        Constraint::Length(h_padding),
        Constraint::Length(width),
        Constraint::Min(0),
    ])
    .areas(area);

    let v_padding = (area.height.saturating_sub(height)) / 2;
    let [_, v_centre, _] = Layout::vertical([
        Constraint::Length(v_padding),
        Constraint::Length(height),
        Constraint::Min(0),
    ])
    .areas(h_centre);

    v_centre
}

fn apply_shake(r: Rect, offset: i32) -> Rect {
    if offset == 0 {
        return r;
    }

    let new_x = (r.x as i32 + offset).clamp(0, r.x as i32 + offset.abs()) as u16;
    let new_x = new_x.min(r.x.saturating_add(2));
    Rect { x: new_x, ..r }
}
