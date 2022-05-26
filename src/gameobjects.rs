use character::Character;
use cards::*;
use crate::json::get_all_character_data;
use crate::prelude::*;

pub mod deck;
pub mod cards;
pub mod player;
pub mod character;

pub fn get_event_cards() -> Vec<Box<dyn Card>> {
    event::get_cards()
}
pub fn get_item_cards() -> Vec<Box<dyn Card>> {
    item::get_cards()
}
pub fn get_omen_cards() -> Vec<Box<dyn Card>> {
    omen::get_cards()
}

pub fn get_characters() -> Vec<Character> {
    let data_list: Vec<CharacterData> = get_all_character_data();
    data_list.iter().map(|cd| Character::new(cd.to_owned())).collect()
}