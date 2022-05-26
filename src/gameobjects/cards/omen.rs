use super::{Card, Companion};
use girl::Girl;

pub mod girl;

pub const JSON_PATH: &str = "./cards/omen";

pub fn get_cards() -> Vec<Box<dyn Card>> {
    let mut deck: Vec<Box<dyn Card>> = Vec::new();
    for _ in 0..13 {
        deck.push(Box::new(Girl::new()));
    }
    deck
}

pub fn haunt_roll() {}