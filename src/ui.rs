#![allow(dead_code, unused_imports)]

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{self, Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};
use tui_textarea::TextArea;

use crate::app::*;

pub fn ui(f: &mut Frame, app: &App, form: &mut TextArea) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(4),
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            Constraint::Min(4),
        ])
        .split(f.size());

    // Title bar
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(
        Line::from("Command Line Blackjack").alignment(Alignment::Center).fg(Color::Green),
    )
    .block(title_block);

    f.render_widget(title, chunks[0]);

    // Dealer view
    let dealer_area = centered_rect(50, 75, chunks[1]);
    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(dealer_block, dealer_area);

    // Player view
    let player_area = centered_rect(50, 75, chunks[2]);
    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(player_block, player_area);

    // Input bet
    let bet_area = centered_rect(50, 25, player_area);
    let bet_block = Block::default()
        .title("Place bet")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Yellow).fg(Color::Black));
    form.set_block(bet_block);
    let bet_form = form.widget();
    f.render_widget(bet_form, bet_area);

    // Footer with allowed commands
    let current_keys_hint = {
        match app.state {
            GameState::EnterBet => {
                Span::styled("Input bet and press enter", Style::default().fg(Color::Red))
            }
            GameState::PlayerTurn => Span::styled(
                "(h) to hit / (s) to stand / (q) to quit game",
                Style::default().fg(Color::Red),
            ),
            GameState::Win => Span::styled(
                "Press Enter to play again / (q) to quit",
                Style::default().fg(Color::Red),
            ),
            GameState::Lose => Span::styled(
                "Press Enter to play again / (q) to quit",
                Style::default().fg(Color::Red),
            ),
            GameState::Quit => Span::styled("Exiting game...", Style::default().fg(Color::Red)),
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint).alignment(Alignment::Center))
            .block(Block::default().borders(Borders::ALL));

    f.render_widget(key_notes_footer, chunks[3]);
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
