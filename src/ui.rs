use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::cards::{Card, Suit};
use crate::game::*;

pub fn ui(f: &mut Frame, game: &Game, form: &mut TextArea) {
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

    let title_bar = chunks[0];
    let top = chunks[1];
    let middle = chunks[2];
    let bottom = chunks[3];

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(centered_rect(75, 75, top));

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(centered_rect(75, 75, bottom));

    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title_widget = Paragraph::new(
        Line::from("TUI BLACKJACK")
            .fg(Color::Blue)
            .centered()
            .bold(),
    )
    .block(title_block);
    f.render_widget(title_widget, title_bar);

    // Dealer
    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));
    let dealer_rect = centered_rect(75, 75, top_chunks[1]);
    f.render_widget(dealer_block, dealer_rect);

    // Player
    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));
    let player_rect = centered_rect(75, 75, bottom_chunks[1]);
    f.render_widget(player_block, player_rect);

    let stats_rect = centered_rect(75, 75, bottom_chunks[0]);
    let stats_block = Block::default()
        .title("Player stats")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let stats_widget = Paragraph::new(vec![
        Line::from(format!("Bank: {}", game.bank)),
    ])
    .block(stats_block);
    f.render_widget(stats_widget, stats_rect);

    // Bet form
    let bet_rect = centered_rect(100, 25, player_rect);
    let bet_form = form.widget();

    // Conditional rendering
    let command_hint: &str;

    match game.state {
        GameState::Bet => {
            f.render_widget(bet_form, bet_rect);
            command_hint = "<Enter> to place bet / <Escape> to quit game";
        }
        GameState::Play if game.splittable() => {
            command_hint = "<h> to hit / <s> to stand / <p> to split / <q> to quit game";
            render_player_cards(f, game, player_rect);
            render_upcard(f, game, dealer_rect);

            let stats_block = Block::default()
                .title("Player stats")
                .borders(Borders::ALL)
                .title_alignment(Alignment::Center);
            let stats_widget = Paragraph::new(vec![
                Line::from(format!("Bank: {}", game.bank)),
                Line::from(format!("Current bet: {}", game.active_hand().bet())),
            ])
            .block(stats_block);
            f.render_widget(stats_widget, stats_rect);
        }
        GameState::Play => {
            command_hint = "<h> to hit / <s> to stand / <q> to quit game";
            render_player_cards(f, game, player_rect);
            render_upcard(f, game, dealer_rect);
        }
        GameState::Results => {
            command_hint = "<Enter> to play again / <q> to quit";
            render_player_cards(f, game, player_rect);
            render_dealer_cards(f, game, dealer_rect);
        }
    }

    // Render command hints
    let command_hint = Span::styled(command_hint, Style::default().fg(Color::Yellow));
    let command_hint =
        Paragraph::new(Line::from(command_hint).centered().bold()).block(Block::default());
    f.render_widget(command_hint, middle);
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

fn render_player_cards(f: &mut Frame, game: &Game, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_bottom(format!("Score: {}", game.active_hand().score()))
        .title_alignment(Alignment::Center);
    let cards: Vec<Line> = game
        .active_hand()
        .cards
        .iter()
        .map(|card| display_card(card))
        .collect();
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn render_dealer_cards(f: &mut Frame, game: &Game, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_bottom(format!("Score: {}", game.dealer.score()))
        .title_alignment(Alignment::Center);
    let cards: Vec<Line> = game
        .dealer
        .hand
        .iter()
        .map(|card| display_card(card))
        .collect();
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn render_upcard(f: &mut Frame, game: &Game, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_bottom(format!("Showing: {}", game.dealer.showing()))
        .title_alignment(Alignment::Center);
    let cards = vec![
        Line::from("*****").fg(Color::Gray).bold().centered(),
        display_card(&game.dealer.hand[1]),
    ];
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn display_card(card: &Card) -> Line {
    let color = match card.suit {
        Suit::Hearts => Color::LightRed,
        Suit::Diamonds => Color::LightRed,
        _ => Color::Gray,
    };
    Line::from(format!("{}", card)).fg(color).bold().centered()
}
