use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::app::*;
use crate::cards::{Card, Suit};

pub fn ui(f: &mut Frame, app: &App, form: &mut TextArea) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Percentage(50),
            Constraint::Min(3),
            Constraint::Percentage(50),
        ])
        .split(f.size());

    let title_rect = chunks[0];
    let dealer_rect = centered_rect(75, 75, chunks[1]);
    let command_rect = chunks[2];
    let player_rect = centered_rect(75, 75, chunks[3]);

    let dealer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(dealer_rect);

    let player_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3); 3])
        .split(player_rect);

    let dealer_cards_rect = centered_rect(75, 75, dealer_chunks[1]);

    let player_cards_rect = centered_rect(75, 75, player_chunks[1]);
    let player_stats_rect = centered_rect(75, 75, player_chunks[0]);

    // Title bar
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(
        Line::from("COMMAND LINE BLACKJACK")
            .fg(Color::Blue)
            .centered()
            .bold(),
    )
    .block(title_block);

    f.render_widget(title, title_rect);

    let command_hint = {
        match app.state {
            GameState::EnterBet => "<Enter> to place bet / <Escape> to quit game",
            GameState::PlayerTurn => "<h> to hit / <s> to stand / <q> to quit game",
            GameState::DealerTurn => "Dealer's play...",
            _ => "<Enter> to play again / <q> to quit",
        }
    };

    let command_hint = Span::styled(command_hint, Style::default().fg(Color::Yellow));
    let command_hint =
        Paragraph::new(Line::from(command_hint).centered().bold()).block(Block::default());

    f.render_widget(command_hint, command_rect);

    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));
    f.render_widget(dealer_block, dealer_rect);

    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));
    f.render_widget(player_block, player_rect);

    render_player_stats(f, app, player_stats_rect);

    match app.state {
        GameState::EnterBet => {
            let bet_rect = centered_rect(100, 25, player_cards_rect);
            let bet_form = form.widget();
            f.render_widget(bet_form, bet_rect);
        }
        GameState::PlayerTurn => {
            render_player_cards(f, app, player_cards_rect);
            render_upcard(f, app, dealer_cards_rect);
        }
        GameState::DealerTurn => {
            render_player_cards(f, app, player_cards_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
        }
        GameState::Win => {
            let win_text = Paragraph::new(vec![
                Line::from(format!("You win! +${}", app.current_bet)).fg(Color::LightGreen).bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
            .centered();
            render_player_cards(f, app, player_cards_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, command_rect);
            f.render_widget(win_text, command_rect);
        }
        GameState::Lose => {
            let lose_text = Paragraph::new(vec![
                Line::from(format!("Better luck next time. -${}", app.current_bet))
                    .fg(Color::LightRed)
                    .bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
            .centered();
            render_player_cards(f, app, player_cards_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, command_rect);
            f.render_widget(lose_text, command_rect);
        }
        GameState::Blackjack => {
            let win_text = Paragraph::new(vec![
                Line::from(format!("Blackjack! +${}", app.blackjack_payout)).fg(Color::LightGreen).bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
            .centered();
            render_player_cards(f, app, player_cards_rect);
            render_upcard(f, app, dealer_cards_rect);
            f.render_widget(Clear, command_rect);
            f.render_widget(win_text, command_rect);
        }
        GameState::Draw => {
            let draw_text = Paragraph::new(vec![
                Line::from("Draw!").fg(Color::Blue).bold(),
                Line::from(""),
                Line::from("Press <Enter> to play again / <q> to quit").fg(Color::Yellow),
            ])
            .centered();
            render_player_cards(f, app, player_cards_rect);
            render_dealer_cards(f, app, dealer_cards_rect);
            f.render_widget(Clear, command_rect);
            f.render_widget(draw_text, command_rect);
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

fn render_player_cards(f: &mut Frame, app: &App, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_bottom(format!("Score: {}", app.player_score()))
        .title_alignment(Alignment::Center);
    let cards: Vec<Line> = app
        .player_hand
        .iter()
        .map(|card| display_card(card))
        .collect();
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn render_dealer_cards(f: &mut Frame, app: &App, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_bottom(format!("Score: {}", app.dealer_score()))
        .title_alignment(Alignment::Center);
    let cards: Vec<Line> = app
        .dealer_hand
        .iter()
        .map(|card| display_card(card))
        .collect();
    let card_view = Paragraph::new(cards).block(block);
    f.render_widget(card_view, rect);
}

fn render_player_stats(f: &mut Frame, app: &App, rect: Rect) {
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

fn render_upcard(f: &mut Frame, app: &App, rect: Rect) {
    let block = Block::default()
        .title("Current hand")
        .borders(Borders::ALL)
        .title_bottom(format!("Showing: {}", app.dealer_showing()))
        .title_alignment(Alignment::Center);
    let upcard = display_card(app.dealer_hand.first().unwrap());
    let hole = Line::from("| HOLE CARD |");
    let dealer_view = Paragraph::new(vec![upcard, hole])
        .block(block)
        .centered()
        .alignment(Alignment::Center);
    f.render_widget(dealer_view, rect);
}

fn display_card(card: &Card) -> Line {
    let color = match card.suit {
        Suit::Hearts => Color::LightRed,
        Suit::Diamonds => Color::LightRed,
        _ => Color::Gray,
    };
    Line::from(format!("{}", card)).fg(color).bold().centered()
}
