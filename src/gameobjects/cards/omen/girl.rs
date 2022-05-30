use opengl_graphics::GlGraphics;
use graphics::Context;

use super::{Card, Companion, JSON_PATH, haunt_roll};
use crate::json::get_card_data;
use crate::prelude::*;
use crate::card_render::CardRenderer;

pub struct Girl {
    pub drawer: CardRenderer,
}

impl Card for Girl {
    fn get_title(&self) -> String {
        self.drawer.settings.title.text.join("\n")
    }
    fn get_head(&self) -> String {
        self.drawer.settings.head.text.join("\n")
    }
    fn get_body(&self) -> String {
        self.drawer.settings.body.text.join("\n")
    }

    fn draw_method(&self, character: &mut Character) {
        character.modify_stat(2, 1);
        character.modify_stat(3, 1);
        haunt_roll();
    }
    fn discard_method(&self, character: &mut Character) {
        character.modify_stat(2, -1);
        character.modify_stat(3, -1);
    }

    fn draw(&mut self, context: &Context, graphics: &mut GlGraphics) {
        self.drawer.draw(context, graphics);
    }

    fn set_pos(&mut self, position: [f64; 2]) {
        self.drawer.settings.position = position;
    }
    fn set_visible(&mut self, v: bool) {
        self.drawer.settings.visible = v;
    }
}

impl Companion for Girl {}

impl Girl {
    pub fn new() -> Self {
        let path: String = JSON_PATH.to_string() + "/girl.json";
        let txt_data: CardFields = get_card_data(&path);
        let drawer: CardRenderer = CardRenderer::new(&CardTypes::Omen, txt_data);
        Self { drawer }
    }
}