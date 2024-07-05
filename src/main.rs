#![allow(dead_code, unused_imports)]

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
    style::{Color, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tui_textarea::{Input, Key, TextArea};

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
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    textarea.set_style(Style::default());
    textarea.set_block(Block::default().borders(Borders::ALL).title("Place bet"));
    let mut is_valid = true;

    loop {
        terminal.draw(|f| ui(f, app, &mut textarea))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.state {
                GameState::EnterBet => {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter if is_valid => {
                            let bet = textarea.lines()[0].parse::<u32>().unwrap();
                            app.place_bet(bet);
                        }
                        _ => {
                            // TextArea::input returns if the input modified its text
                            if textarea.input(key) {
                                is_valid = validate(&mut textarea);
                            }
                        }
                    }
                }
                GameState::PlayerTurn => {
                    if let Event::Key(key) = event::read()? {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('h') => app.run(Command::Hit),
                            KeyCode::Char('s') => app.run(Command::Stand),
                            _ => {}
                        }
                    }
                }
                GameState::Win => todo!(),
                GameState::Lose => todo!(),
                GameState::Quit => todo!(),
            }
        }
    }
    Ok(())
}

fn validate(textarea: &mut TextArea) -> bool {
    if textarea.lines()[0].parse::<u32>().is_err() {
        textarea.set_style(Style::default().fg(Color::LightRed));
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Error: Invalid input")
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
