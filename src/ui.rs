use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use crate::app::*;
use crate::cards::Suit;

pub fn ui(f: &mut Frame, app: &App, form: &mut TextArea) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Percentage(50),
            Constraint::Min(1),
            Constraint::Percentage(50),
        ])
        .split(f.size());

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

    f.render_widget(title, chunks[0]);

    // Dealer block
    let dealer_area = centered_rect(50, 75, chunks[1]);
    let dealer_block = Block::default()
        .title("Dealer")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let command_hint = {
        match app.state {
            GameState::EnterBet => "<Enter> to place bet / <Escape> to quit game",
            GameState::PlayerTurn => "<h> to hit / <s> to stand / <q> to quit game",
            _ => "<Enter> to play again / <q> to quit",
        }
    };

    let command_hint = Span::styled(command_hint, Style::default().fg(Color::Yellow));
    let command_hint =
        Paragraph::new(Line::from(command_hint).centered().bold()).block(Block::default());

    f.render_widget(command_hint, chunks[2]);

    // Player block
    let player_area = centered_rect(50, 75, chunks[3]);
    let player_block = Block::default()
        .title("Player")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    match app.state {
        GameState::EnterBet => {
            let bet_area = centered_rect(50, 25, player_area);
            let bet_form = form.widget();
            f.render_widget(player_block, player_area);
            f.render_widget(dealer_block, dealer_area);
            f.render_widget(bet_form, bet_area);
        }
        GameState::PlayerTurn => {
            let upcard = app.dealer_hand.first().unwrap();
            let color = match upcard.suit {
                Suit::Hearts => Color::LightRed,
                Suit::Diamonds => Color::LightRed,
                _ => Color::Gray,
            };
            let upcard = Line::from(format!("{}", upcard)).fg(color).bold();
            let hole = Line::from("| HOLE CARD |");
            let dealer_cards = Paragraph::new(vec![upcard, hole]).block(dealer_block);
            f.render_widget(dealer_cards, dealer_area);

            let mut player_cards: Vec<Line> = Vec::new();
            for card in &app.player_hand {
                let color = match card.suit {
                    Suit::Hearts => Color::LightRed,
                    Suit::Diamonds => Color::LightRed,
                    _ => Color::Gray,
                };
                player_cards.push(Line::from(format!("{card}")).fg(color).bold());
            }
            player_cards.push(Line::from(" "));
            player_cards.push(Line::from(format!("Score: {}", app.player_score())));
            let player_view = Paragraph::new(player_cards).block(player_block);
            f.render_widget(player_view, player_area);
        }
        // TODO: Implement UI for winning/losing
        GameState::Win => todo!(),
        GameState::Lose => todo!(),
    }

    // Bank balance
    //
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
