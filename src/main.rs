mod cards;
mod game;
mod ui;

use std::{error::Error, io};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tui_textarea::TextArea;

use crate::game::*;
use crate::ui::ui;

fn main() -> Result<(), Box<dyn Error>> {
    // set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = Game::default();
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

pub fn run_app<B: Backend>(game: &mut Game, terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut textarea = TextArea::default();

    loop {
        let is_valid = validate(&mut textarea, game);
        terminal.draw(|f| ui(f, game, &mut textarea))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match game.state {
                State::Bet => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Enter if is_valid => {
                        let bet = textarea.lines()[0].parse::<u32>().unwrap();
                        game.place_bet(bet);
                    }
                    KeyCode::Enter => {}
                    _ => {
                        textarea.input(key);
                    }
                },

                State::Play => {
                    let splittable = game.splittable();
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('h') => game.execute(Command::Hit),
                        KeyCode::Char('s') => game.execute(Command::Stand),
                        KeyCode::Char('p') if splittable => game.execute(Command::Split),
                        _ => {}
                    }
                }

                State::Results => match key.code {
                    KeyCode::Enter => game.reset(),
                    KeyCode::Char('q') => break,
                    _ => {}
                },
            }
        }
    }
    Ok(())
}

fn validate(textarea: &mut TextArea, app: &Game) -> bool {
    let bet = textarea.lines()[0].parse::<u32>();

    if textarea.is_empty() {
        textarea.set_cursor_line_style(Style::default());
        textarea.set_style(Style::default());
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Place bet")
                .border_style(Style::default().fg(Color::Yellow)),
        );
        return false;
    }

    match bet {
        Ok(bet) => {
            if bet > app.bank {
                textarea.set_style(Style::default().fg(Color::LightRed));
                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Error: Too big!")
                        .border_style(Style::default().fg(Color::LightRed)),
                );
                false
            } else if bet == 0 {
                textarea.set_style(Style::default().fg(Color::LightRed));
                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Error: Bet must be greater than 0")
                        .border_style(Style::default().fg(Color::LightRed)),
                );
                false
            } else {
                textarea.set_style(Style::default().fg(Color::LightGreen));
                textarea.set_block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("OK")
                        .border_style(Style::default().fg(Color::LightGreen)),
                );
                true
            }
        }
        Err(_) => {
            textarea.set_style(Style::default().fg(Color::LightRed));
            textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Error: Invalid input")
                    .border_style(Style::default().fg(Color::LightRed)),
            );
            false
        }
    }
}
