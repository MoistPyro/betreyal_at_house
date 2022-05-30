use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::prelude::*;

pub mod event;
pub mod item;
pub mod omen;

pub trait Card {
    fn get_title(&self) -> String;
    fn get_head(&self) -> String;
    fn get_body(&self) -> String;

    fn draw_method(&self, character: &mut Character) {}
    fn discard_method(&self, character: &mut Character) {}

    fn set_pos(&mut self, position: [f64; 2]);
    fn set_visible(&mut self, v: bool);
    fn draw(&mut self, context: &Context, graphics: &mut GlGraphics);
}

pub trait Consumable: Card {
    fn spend_method(&self, character: &mut Character);
}

pub trait Weapon: Card {
    fn attack_method(&self, character: &mut Character, target: &mut Character);
}

pub trait Companion: Card {}
pub trait Activatable: Card {
    fn activate_method(&self, character: &mut Character);
}