use super::Card;
use funeral::Funeral;

//registering all the cards in /event
pub mod funeral;

pub const JSON_PATH: &str = "./assets/cards/event";

pub fn get_cards() -> Vec<Box<dyn Card>> {
    let mut deck: Vec<Box<dyn Card>> = Vec::new();
    for _ in 0..45 {
        deck.push(Box::new(Funeral::new()));
    }
    deck
}