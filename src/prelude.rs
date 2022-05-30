use serde::{Deserialize, Serialize};
pub use crate::gameobjects::character::Character;

pub const SETTINGS_PATH: &str = "settings.json";

//types:
pub type CharacterData = (
    String, u64, u64, u64,
    Vec<String>, Vec<u64>,
    Vec<Vec<u8>>,
    Vec<u8>
);

#[derive(Serialize, Deserialize)]
pub struct CardFields {
    ///this struct represents the datastructure of json-files in the card assets folder.
    pub title: Vec<String>,
    pub head: Vec<String>,
    pub body: Vec<String>,
}

//enums:
#[derive(Clone, Copy)]
pub enum CardTypes {Event, Item, Omen}