use piston::input::GenericEvent;

use crate::json::{card::*, character::*};
use crate::gameobjects::{deck::Deck, player::Player, character::Character};

pub struct GameBoard {
    pub decks: Vec<Deck>,
    pub players: Vec<Player>,
}

impl GameBoard {
    ///setup all the game objects.
    pub fn new() -> Self {
        let mut decks = Vec::<Deck>::new();
        decks.push(Deck::new(
            "Item Deck", get_item_cards().expect("Failed to get cards.")
        ));
        decks.push(Deck::new(
            "Event Deck", get_event_cards().expect("Failed to get cards.")
        ));
        decks.push(Deck::new(
            "Omen Deck", get_omen_cards().expect("Failed to get cards.")
        ));

        let mut characters: Vec<Character> = get_characters().unwrap();
        let mut players = Vec::<Player>::new();
        players.push(Player::new(characters.pop().unwrap()));

        Self { decks: decks, players: players }
    }
}
pub struct GameController {
    pub gameboard: GameBoard,
}


impl GameController {
    pub fn new(gameboard: GameBoard) -> GameController {
        GameController {
            gameboard: gameboard,
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {

    }
}