pub use glutin_window::GlutinWindow as Window;
pub use crate::gameobjects::character::Character;

//types:
pub type CardData = (Vec<String>, Vec<String>, Vec<String>);
pub type CharacterData = (
    String, u64, u64, u64,
    Vec<String>, Vec<u64>,
    Vec<Vec<u8>>,
    Vec<u8>
);

//enums:
#[derive(Clone, Copy)]
pub enum CardTypes {Event, Item, Omen}