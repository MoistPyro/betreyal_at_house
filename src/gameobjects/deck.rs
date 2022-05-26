use rand::{thread_rng, Rng};

use super::cards::Card;

pub struct Deck {
    name: String,
    cards: Vec<Box<dyn Card>>,
}

impl Deck {
    pub fn new(name: &str, cards: Vec<Box<dyn Card>>) -> Self {
        let mut r = Self { name: name.to_string(), cards };
        r.shuffle();
        r
    }

    pub fn get_title(&self) -> String {
        self.name.to_string()
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }

    pub fn draw(&mut self) -> Box<dyn Card> {
        self.cards.pop().unwrap()
    }

    pub fn add_card(&mut self, card: Box<dyn Card>) {
        self.cards.push(card);
    }
}