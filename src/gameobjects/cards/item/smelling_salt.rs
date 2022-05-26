use opengl_graphics::GlGraphics;
use graphics::Context;

use super::{Card, Consumable, JSON_PATH};
use crate::json::get_card_data;
use crate::prelude::{CardData, Character, CardTypes};
use crate::card_render::CardRenderer;

pub struct SmellingSalt {
    pub drawer: CardRenderer,
}

impl Card for SmellingSalt {
    fn get_title(&self) -> String {
        self.drawer.settings.title.join("\n")
    }
    fn get_head(&self) -> String {
        self.drawer.settings.head.join("\n")
    }
    fn get_body(&self) -> String {
        self.drawer.settings.body.join("\n")
    }

    fn set_pos(&mut self, position: Vec<f64>) {
        self.drawer.settings.position = position;
    }
    fn set_visible(&mut self, v: bool) {
        self.drawer.settings.visible = v;
    }

    fn draw(&self, context: &Context, graphics: &mut GlGraphics) {
        self.drawer.draw(context, graphics);
    }
}

impl Consumable for SmellingSalt {
    fn spend_method(&self, character: &mut Character) {
        let starting_knowledge = character.get_starting_stat(3);
        if character.get_stat(3) < starting_knowledge {
            character.set_stat(3, starting_knowledge);
        }
    }
}

impl SmellingSalt {
    pub fn new() -> Self {
        let path: String = JSON_PATH.to_string() + "/smelling_salt.json";
        let txt_data: CardData = get_card_data(&path);
        let drawer: CardRenderer = CardRenderer::new(&CardTypes::Item, txt_data);
        Self { drawer }
    }
}