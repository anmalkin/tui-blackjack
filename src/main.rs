mod app;
mod cards;
mod errors;
mod ui;

use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use tui_textarea::TextArea;

use crate::app::*;
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    let res = run_app(&mut app, &mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub fn run_app<B: Backend>(app: &mut App, terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut bet_form = TextArea::default();

    loop {
        terminal.draw(|f| ui(f, app, &mut bet_form));

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.state {
                GameState::EnterBet => match key.code {
                    // TODO: Use tui-textarea for bet inputting
                    KeyCode::Char(c) => {
                        if c.is_digit(10) {
                            todo!()
                        }
                    }
                    _ => continue,
                },
                GameState::PlayerTurn => todo!(),
                GameState::Win => todo!(),
                GameState::Lose => todo!(),
                GameState::Quit => todo!(),
            }
        }
        break; // TODO: Finish main loop implementation
    }
    Ok(())
}
