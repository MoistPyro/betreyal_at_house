use rand::{thread_rng, Rng};

use super::cards::{Card, Consumable, Activatable};
use super::{character::Character, deck::Deck};

pub struct Player {
    pub cards: Vec<Box<dyn Card>>,
    pub character: Character,
}

impl Player {
    pub fn new(character: Character) -> Self {
        Self { cards: Vec::<Box<dyn Card>>::new(), character }
    }

    fn shuffle_hand(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }

    pub fn draw_card(&mut self, deck: &mut Deck) -> Box<dyn Card> {
        let card: Box<dyn Card> = deck.draw();
        card.draw_method(&mut self.character);
        card
    }

    pub fn discard_card(&mut self, index: usize) {
        let card: Box<dyn Card> = self.cards.remove(index);
        card.discard_method(&mut self.character);
    }

    pub fn give_card(&mut self, index: usize) -> Box<dyn Card> {
        self.cards.remove(index)
    }
    pub fn give_card_random(&mut self) -> Box<dyn Card> {
        self.shuffle_hand();
        self.cards.pop().unwrap()
    }

    fn spend_consumable(&mut self, card: impl Consumable) {
        card.spend_method(&mut self.character);
    }

    fn activate_permanent(&mut self, card: impl Activatable) -> impl Activatable {
        card.activate_method(&mut self.character);
        card
    }

    pub fn look_for_card(&self, title: String) -> usize {
        self.cards.iter().position(|x| x.get_title() == title).unwrap()
    }
}