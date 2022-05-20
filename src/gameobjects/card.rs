use crate::card_render::{CardRenderer, CardRenderSettings};
use super::character::Character;

pub enum LifeTime {OneShot, Consumable, Permanent(PermanentType)}
pub enum PermanentType {Weapon, Companion, Item}

#[derive(Clone, Copy)]
pub enum CardTypes {Event, Item, Omen}

pub struct Card {
    pub title: String,
    pub head: String,
    pub body: String,
    pub lifetime: LifeTime,
    pub card_type: CardTypes,
    pub draw_method: fn(&Card, &mut Character),
    pub activate_method: fn(&Card, &mut Character),
    pub spend_method: fn(&Card, &mut Character),
    pub discard_method: fn(&Card, &mut Character),
    pub drawer: CardRenderer,
}

impl Card {
    pub fn new(
        title:String, head:String, body:String, lifetime: LifeTime, card_type: CardTypes,
        draw_method: fn(&Card, &mut Character),
        activate_method: fn(&Card, &mut Character),
        spend_method: fn(&Card, &mut Character),
        discard_method: fn(&Card, &mut Character),
    ) -> Self {
        let card_render_settings = CardRenderSettings::new(&card_type);
        let drawer = CardRenderer::new(card_render_settings);
        
        Self {
            title, head, body,
            lifetime, card_type,
            draw_method, activate_method,
            spend_method, discard_method,
            drawer,
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

    pub fn dummy_func(&self, _character: &mut Character) {}
}