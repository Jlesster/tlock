use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::time::Duration;

mod app;
mod auth;
mod logo;
mod ui;

use app::{App, AppState, TICK_RATE_MS};
use auth::{AuthResult, try_authenticate};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let result = run(&mut terminal, &mut app);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> Result<()> {
    let username = whoami::username();
    loop {
        terminal.draw(|frame| ui::render(app, frame))?;
        if event::poll(Duration::from_millis(TICK_RATE_MS))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_key_event(app, key, &username);
                }
            }
        } else {
            app.tick();
        }
        if app.should_quit() {
            return Ok(());
        }
    }
}

fn handle_key_event(app: &mut App, key: KeyEvent, username: &str) {
    if app.state == AppState::Authenticating {
        return;
    }

    match key.code {
        KeyCode::Enter => {
            if !app.password.is_empty() {
                let password = app.begin_auth();
                match try_authenticate(username, password) {
                    AuthResult::Success => {
                        app.on_auth_success();
                    }
                    AuthResult::Failure(_msg) => {
                        app.on_auth_failure();
                    }
                }
            }
        }
        KeyCode::Backspace => {
            app.pop_char();
        }
        KeyCode::Esc => {
            app.password.clear();
            app.fail_count = 0;
            app.state = AppState::Locked;
        }
        KeyCode::Char(c) => {
            app.push_char(c);
        }
        _ => {}
    }
}

fn render(frame: &mut Frame, _app: &App) {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::Block;

    frame.render_widget(
        Block::default().style(Style::default().bg(Color::Black)),
        frame.area(),
    );
}
