mod app;
mod game;
mod ui;

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;

use app::App;
use game::Difficulty;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| {
        let mut app = App::new(Difficulty::Easy);
        run_app(terminal, &mut app)
    })
}

fn run_app(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| ui::render(frame, app))?;
        handle_events(app)?;
    }
    Ok(())
}

fn handle_events(app: &mut App) -> io::Result<()> {
    if let Event::Key(key) = event::read()?
        && key.kind == KeyEventKind::Press
    {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => app.exit = true,
            KeyCode::Char('r') => app.new_game(),
            KeyCode::Char('d') => app.change_difficulty(),
            KeyCode::Char('h') => app.hint(),
            KeyCode::Up => app.move_up(),
            KeyCode::Down => app.move_down(),
            KeyCode::Left => app.move_left(),
            KeyCode::Right => app.move_right(),
            KeyCode::Char(c @ '1'..='9') => {
                if let Some(n) = c.to_digit(10) {
                    app.input_number(n as u8);
                }
            }
            KeyCode::Backspace | KeyCode::Delete => app.clear_cell(),
            _ => {}
        }
    }
    Ok(())
}
