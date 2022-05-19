use rand::{thread_rng, Rng};

pub struct Character {
    pub name: String,
    pub age: u8,
    pub height: u16,
    pub weight: u16,
    pub hobbies: [String; 2],
    pub birthday: [u8; 2],
    stats: [[u8; 9]; 4],
    starting_stats: [u8; 4],
    pub stat_indexes: [u8; 4],
}

impl Character {
    pub fn new(
        name: String, age: u8, height: u16, weight: u16, hobbies: [String; 2] , birthday: [u8; 2],
        might: [u8; 9], speed: [u8; 9], sanity: [u8; 9], knowledge: [u8; 9], starting_stats: [u8; 4]
    ) -> Self {
        let stats: [[u8; 9]; 4] = [might, speed, sanity, knowledge];
        Self {
            name, age, height, weight, hobbies, birthday, stats, starting_stats,
            stat_indexes: starting_stats,
        }
    }

    pub fn get_stat(&self, i:usize) -> u8 {
        self.stat_indexes[i].into()
    }
    pub fn modify_stat(&mut self, i: usize, value: i8) {
        if value > 0 {
            let positive: u8 = value.try_into().unwrap();
            self.stat_indexes[i] += positive;
        } else {
            let negative: u8 = (value * -1).try_into().unwrap();
            self.stat_indexes[i] -= negative;
        }
    }
    pub fn set_stat(&mut self, i: usize, value: u8) {
        self.stats[i][0] = value;
    }

    pub fn take_dmg_physical(&mut self, dmg: u8) {}
    pub fn take_dmg_mental(&mut self, dmg: u8) {}
}

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

        match card.card_type {
            CardTypes::OneShot => (),
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

        match card.card_type {
            CardTypes::OneShot => (),
            CardTypes::Consumable => self.spend_consumable(card),
            CardTypes::Permanent(_) => self.activate_permanent(card),
        }
    }

    pub fn look_for_card(&self, title: String) -> usize {
        self.cards.iter().position(|x| {x.title == title}).unwrap()
    }
}

pub enum CardTypes {OneShot, Consumable, Permanent(PermanentType)}
pub enum PermanentType {Weapon, Companion, Item}

pub struct Card {
    pub title: String,
    pub head: String,
    pub body: String,
    pub card_type: CardTypes,
    pub draw_method: fn(&Card, &mut Character),
    pub activate_method: fn(&Card, &mut Character),
    pub spend_method: fn(&Card, &mut Character),
    pub discard_method: fn(&Card, &mut Character),
}

impl Card {
    pub fn new(
        title:String, head:String, body:String, card_type: CardTypes,
        draw_method: fn(&Card, &mut Character),
        activate_method: fn(&Card, &mut Character),
        spend_method: fn(&Card, &mut Character),
        discard_method: fn(&Card, &mut Character),
    ) -> Self {
        Self {
            title: title,
            head: head,
            body: body,
            card_type: card_type,
            draw_method: draw_method,
            activate_method: activate_method,
            spend_method: spend_method,
            discard_method: discard_method
        }
    }

    pub fn run_draw_method(&self, character: &mut Character) {
        (self.draw_method)(self, character)
    }
    pub fn run_activate_method(&self, character: &mut Character) {
        (self.activate_method)(self, character)
    }
    pub fn run_spend_method(&self, character: &mut Character) {
        (self.spend_method)(self, character)
    }
    pub fn run_discard_method(&self, character: &mut Character) {
        (self.discard_method)(self, character)
    }

    pub fn smelling_salts_func(&self, character: &mut Character) {
        let current_knowledge = character.get_stat(3);
        let starting_knowledge = character.starting_stats[3];

        if current_knowledge < starting_knowledge {character.set_stat(3, starting_knowledge)}
    }

    pub fn dummy_func(&self, _character: &mut Character) {}
}

pub struct Deck {
    pub name: String,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(name: String) -> Self {
        Self { name: name, cards: Vec::<Card>::new() }
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