use opengl_graphics::GlGraphics;
use graphics::Context;

use super::{Card, JSON_PATH};
use crate::json::get_card_data;
use crate::prelude::*;
use crate::card_render::CardRenderer;
use crate::die_roller::roll_multiple;

pub struct Funeral {
    pub drawer: CardRenderer,
}

impl Funeral {
    pub fn new() -> Self {
        let path: String = JSON_PATH.to_string() + "/funeral.json";
        let txt_data: CardFields = get_card_data(&path);
        let drawer: CardRenderer = CardRenderer::new(&CardTypes::Event, txt_data);
        Self { drawer }
    }
}

impl Card for Funeral {
    fn get_title(&self) -> String {
        self.drawer.settings.title.text.join("\n")
    }
    fn get_head(&self) -> String {
        self.drawer.settings.head.text.join("\n")
    }
    fn get_body(&self) -> String {
        self.drawer.settings.body.text.join("\n")
    }

    fn set_pos(&mut self, position: [f64; 2]) {
        self.drawer.settings.position = position;
    }
    fn set_visible(&mut self, v: bool) {
        self.drawer.settings.visible = v;
    }

    fn draw(&mut self, context: &Context, graphics: &mut GlGraphics) {
        self.drawer.draw(context, graphics);
    }

    fn draw_method(&self, character: &mut Character) {
        let sanity = character.get_stat(2);
        match roll_multiple(sanity) {
            4.. => big_hit(character),
            2|3 => small_hit(character),
            0|1 => miss(character),
        }
    }
}

fn big_hit(character: &mut Character) {
    character.modify_stat(2, 1);
}
fn small_hit(character: &mut Character) {
    character.modify_stat(2, -1);
}
fn miss(character: &mut Character) {
    panic!("not implemented!")
}