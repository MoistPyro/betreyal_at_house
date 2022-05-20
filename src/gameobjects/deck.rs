use rand::{thread_rng, Rng};

use super::card::Card;

pub struct Deck {
    pub name: String,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(name: &str, cards: Vec<Card>) -> Self {
        Self { name: name.to_string(), cards: cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop().unwrap()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}