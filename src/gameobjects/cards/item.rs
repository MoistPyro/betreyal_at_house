use super::{Card, Consumable};
use smelling_salt::SmellingSalt;

pub mod smelling_salt;

pub const JSON_PATH: &str = "./assets/cards/item";

pub fn get_cards() -> Vec<Box<dyn Card>> {
    let mut deck: Vec<Box<dyn Card>> = Vec::new();
    for _ in 0..22 {
        deck.push(Box::new(SmellingSalt::new()));
    }
    deck
}