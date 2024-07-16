use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    Frame,
};
use tui_textarea::TextArea;

use crate::cards::{Card, Suit};
use crate::component::*;
use crate::game::*;

pub fn ui(f: &mut Frame, app: &Game, form: &mut TextArea) {
    // Global layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Percentage(50),
            Constraint::Min(3),
            Constraint::Percentage(50),
        ])
        .split(f.size());

    // Title
    let mut title = Component::default();
    title.set_widget(Paragraph::new(
        Line::from("COMMAND LINE BLACKJACK")
            .fg(Color::Blue)
            .centered()
            .bold(),
    ));
    title.set_block(Block::default()
    .borders(Borders::ALL)
    .style(Style::default()));
    title.set_rect(chunks[0]);
    title.render(f);

    // Dealer
    let dealer = Component::default();
    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));

    let top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(centered_rect(75, 75, chunks[1]));
    let dealer_rect = centered_rect(75, 75, top[1]);

    // Commands
    let middle = chunks[2];
    let command_hint = {
        match app.state {
            State::Bet => "<Enter> to place bet / <Escape> to quit game",
            State::Play if app.active_hand().splittable() => {
                "<h> to hit / <s> to stand / <p> to split / <q> to quit game"
            }
            State::Play => "<h> to hit / <s> to stand / <q> to quit game",
            State::Results => "<Enter> to play again / <q> to quit",
        }
    };
    let command_hint = Span::styled(command_hint, Style::default().fg(Color::Yellow));
    let command_hint =
        Paragraph::new(Line::from(command_hint).centered().bold()).block(Block::default());
    f.render_widget(command_hint, middle);

    // Player
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(centered_rect(75, 75, chunks[3]));
    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));
    let player_hand = centered_rect(75, 75, bottom[1]);
    let split_hand = centered_rect(75, 75, bottom[2]);

    let player_stats = centered_rect(75, 75, bottom[0]);
    render_player_stats(f, app, player_stats);

    // Bet form
    let bet_rect = centered_rect(100, 25, player_hand);
    let bet_form = form.widget();

    // Conditional rendering
    match app.state {
        State::Bet => {
            f.render_widget(bet_form, bet_rect);
        }
        State::Play => {}
        State::Results => {}
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

fn split_rect(r: Rect, direction: Direction) -> (Rect, Rect) {
    let layout = Layout::default()
        .direction(direction)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(r);
    (layout[0], layout[1])
}

fn hand_widget(cards: &[Card], block: Block) -> Paragraph {
    let cards = cards.iter().map(|card| display_card(card)).collect();
    Paragraph::new(cards).block(block)
}

fn upcard_widget(upcard: Card, block: Block) -> Paragraph {
    let upcard_str = "| HOLE CARD |";
    Paragraph::new([Line::from(upcard_str), Line::from(display_card(&upcard))]).block(block)
}

fn render_player_stats(f: &mut Frame, app: &Game, rect: Rect) {
    let block = Block::default()
        .title("Player stats")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let stats = Paragraph::new(vec![
        Line::from(format!("Bank: {}", app.bank)),
        Line::from(format!("Current bet: {}", app.active_hand().bet())),
    ])
    .block(block);
    f.render_widget(stats, rect);
}

fn display_card(card: &Card) -> Line {
    let color = match card.suit {
        Suit::Hearts if !card.down => Color::LightRed,
        Suit::Diamonds => Color::LightRed,
        _ => Color::Gray,
    };
    Line::from(format!("{}", card)).fg(color).bold().centered()
}
