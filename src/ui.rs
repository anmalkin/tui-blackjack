use ratatui::{
    layout::{self, Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::cards::{Card, Suit};
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

    // Title bar
    let title_bar = chunks[0];
    let title = Paragraph::new(
        Line::from("COMMAND LINE BLACKJACK")
            .fg(Color::Blue)
            .centered()
            .bold(),
    )
    .block(Block::default()
        .borders(Borders::ALL)
        .style(Style::default()));
    f.render_widget(title, title_bar);

    // Dealer
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
            State::Play => {
                "<h> to hit / <s> to stand / <q> to quit game"
            }
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
    let player_stats_rect = centered_rect(75, 75, bottom[0]);
    let player_hands_rect = centered_rect(75, 75, bottom[1]);
    let constraints: Vec<Constraint> = Vec::new();
    let hand_count = app.player.len() as u32;
    if hand_count > 1 {
        for _ in 0..hand_count {
            constraints.push(Constraint::Ratio(1, hand_count));
        }
    }
    let player_hands = Layout::default()
        .constraints(constraints)
        .split(player_hands_rect);

    // Bet form
    let bet_rect = centered_rect(100, 25, player_hands_rect);
    let bet_form = form.widget();

    // Dealer
    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));

    // Player
    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));

    render_player_stats(f, app, player_stats_rect);

    // Conditional rendering
    match app.state {
        State::Bet => {
            f.render_widget(bet_form, bet_rect);
        }
        State::Play => {
            render_player_cards(f, app, player_hands_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
        }
        State::Win => {
            let win_text = Paragraph::new(vec![
                Line::from(format!("You win! +${}", app.current_bet))
                .fg(Color::LightGreen)
                .bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
                .centered();
            render_player_cards(f, app, player_hands_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, middle);
            f.render_widget(win_text, middle);
        }
        State::Lose => {
            let lose_text = Paragraph::new(vec![
                Line::from(format!("Better luck next time. -${}", app.current_bet))
                .fg(Color::LightRed)
                .bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
                .centered();
            render_player_cards(f, app, player_hands_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, middle);
            f.render_widget(lose_text, middle);
        }
        State::Blackjack => {
            let win_text = Paragraph::new(vec![
                Line::from(format!("Blackjack! +${}", app.blackjack_payout))
                .fg(Color::LightGreen)
                .bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
                .centered();
            render_player_cards(f, app, player_hands_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, middle);
            f.render_widget(win_text, middle);
        }
        State::Draw => {
            let draw_text = Paragraph::new(vec![
                Line::from("Draw!").fg(Color::Blue).bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
                .centered();
            render_player_cards(f, app, player_hands_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, middle);
            f.render_widget(draw_text, middle);
        }
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
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(r);
    (layout[0], layout[1])
}

fn hand_widget(cards: &[Card], block: Block) -> Paragraph {
    let cards = cards.iter().map(|card| display_card(card)).collect();
    Paragraph::new(cards).block(block)
}

fn upcard_widget(upcard: Card, block:Block) -> Paragraph {
    let upcard_str = "| HOLE CARD |";
    Paragraph::new([
        Line::from(upcard_str),
        Line::from(display_card(&upcard))
    ]).block(block)
}

fn render_player_cards(f: &mut Frame, game: &Game, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    match game.state {
        State::Bet => {},
        State::Play => block = block.title_bottom(format!("Score: {}", game.player.score())
    }
    let cards: Vec<Line> = game
        .player_hand
        .iter()
        .map(|card| display_card(card))
        .collect();
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn render_dealer_cards(f: &mut Frame, app: &Game, rect: Rect) {
    let mut block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    if app.dealer_hand[0].down {
        block = block.title_bottom(format!("Showing: {}", app.dealer_showing()));
    } else {
        block = block.title_bottom(format!("Score: {}", app.dealer_score()));
    }
    let cards: Vec<Line> = app
        .dealer_hand
        .iter()
        .map(|card| display_card(card))
        .collect();
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn render_player_stats(f: &mut Frame, app: &Game, rect: Rect) {
    let block = Block::default()
        .title("Player stats")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let stats = Paragraph::new(vec![
        Line::from(format!("Bank: {}", app.bank)),
        Line::from(format!("Current bet: {}", app.current_bet)),
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
