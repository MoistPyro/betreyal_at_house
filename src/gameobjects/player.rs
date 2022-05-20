use super::{card::{Card, LifeTime}, character::Character, deck::Deck};

pub struct Player {
    pub cards: Vec<Card>,
    pub character: Character,
}

impl Player {
    pub fn new(character: Character) -> Self {
        Self { cards: Vec::<Card>::new(), character: character }
    }

    pub fn draw_card(&mut self, deck: &mut Deck) {
        let card: Card = deck.draw();
        card.run_draw_method(&mut self.character);

        match card.lifetime {
            LifeTime::OneShot => (),
            _ => self.cards.push(card),
        }
    }

    pub fn discard_card(&mut self, i: usize) {
        let card = self.cards.remove(i);
        card.run_discard_method(&mut self.character);
    }

    fn spend_consumable(&mut self, card: Card) {
        card.run_spend_method(&mut self.character);
        card.run_discard_method(&mut self.character);
    }

    fn activate_permanent(&mut self, card: Card) {
        card.run_activate_method(&mut self.character);
        self.cards.push(card);
    }

    pub fn activate_card(&mut self, i: usize) {
        let card = self.cards.remove(i);

        match card.lifetime {
            LifeTime::OneShot => (),
            LifeTime::Consumable => self.spend_consumable(card),
            LifeTime::Permanent(_) => self.activate_permanent(card),
        }
    }

    pub fn look_for_card(&self, title: String) -> usize {
        self.cards.iter().position(|x| {x.title == title}).unwrap()
    }
}