use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    Frame,
    Terminal,
    backend::{Backend, CrosstermBackend},
};

use tui_textarea::TextArea;

use crate::component::*;

struct Screen<'a, B: Backend> {
    terminal: &'a mut Terminal<B>,
    chunks: Vec<Rect>,
    title: Rect,
    dealer_area: Rect,
    dealer_cards: Rect,
    player_area: Rect,
    player_cards: Rect,
    player_stats: Rect,
    command_hints: Rect,
}

impl<'a, B: Backend> Screen<'a, B> {
    pub fn new(terminal: &'a mut Terminal<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Percentage(50),
                Constraint::Min(3),
                Constraint::Percentage(50),
            ])
            .split(terminal.get_frame().size());
        let title = chunks[0];
        let dealer_area = centered_rect(75, 75, chunks[1]);
        let command_hints = chunks[2];
        let player_area = centered_rect(75, 75, chunks[3]);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
