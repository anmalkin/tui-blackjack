mod game;
mod cards;
mod ui;

use std::{error::Error, io, thread::sleep, time::Duration};

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

pub fn run_app<B: Backend>(app: &mut Game, terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut textarea = TextArea::default();

    loop {
        let is_valid = validate(&mut textarea, app);
        terminal.draw(|f| ui(f, app, &mut textarea))?;

        // Run dealer animation
        if let State::Dealer = app.state {
            sleep(Duration::from_secs(1));
            app.execute(Command::AdvanceDealer);
            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.state {
                State::Bet => {
                    match key.code {
                        KeyCode::Esc => break,
                        KeyCode::Enter if is_valid => {
                            let bet = textarea.lines()[0].parse::<u32>().unwrap();
                            app.place_bet(bet);
                            app.start();
                        }
                        KeyCode::Enter => {},
                        _ => {
                            // TextArea::input returns if the input modified its text
                            textarea.input(key); 
                            
                        }
                    }
                }
                State::Play => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => app.execute(Command::Hit),
                    KeyCode::Char('s') => app.execute(Command::Stand),
                    _ => {}
                },
                State::Dealer => {
                    if let KeyCode::Char('q') = key.code {
                        break;
                    }
                }
                // Handle both win and lose cases
                _ => match key.code {
                    KeyCode::Enter => app.reset(),
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
        return false
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
