#![allow(dead_code, unused_imports)]

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Percentage(35),
            Constraint::Percentage(35),
            Constraint::Min(3),
        ])
        .split(f.size());

    // Title bar
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Command Line Blackjack",
        Style::default().fg(Color::Green),
    ))
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

    // Footer with allowed commands
    let command_footer = Paragraph::new("(h) to hit / (s) to stay / (q) to quit")
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(command_footer, chunks[3]);
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
