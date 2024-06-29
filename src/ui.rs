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
            Constraint::Min(2),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
            Constraint::Min(2),
        ])
        .split(f.size());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Command Line Blackjack",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    f.render_widget(title, chunks[0]);

    let command_footer = Paragraph::new("(h) to hit / (s) to stay / (q) to quit")
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(command_footer, chunks[3]);

}
