use graphics::{context::Context, Graphics, types::Color};
use crate::{game_controller::GameController, gameobjects::card::CardTypes};


const BORDER_COLOUR: Color = [0.0, 0.0, 0.0, 1.0];
const TEXT_COLOUR: Color = [0.0, 0.0, 0.0, 1.0];
const ITEM_CARD_BG_COLOUR: Color = [0.3, 0.7, 0.0, 1.0];
const EVENT_CARD_BG_COLOUR: Color = [0.7, 0.3, 0.0, 1.0];
const OMEN_CARD_BG_COLOUR: Color = [0.3, 0.7, 0.3, 1.0];

pub struct CardRenderSettings {
    position: [f64; 2],
    size: [f64; 2],
    border_colour: Color,
    background_colour: Color,
    text_colour: Color,
}

impl CardRenderSettings {
    pub fn new(card_type: &CardTypes) -> Self {
        let position = [700.0, 400.0];
        let size = [150.0, 300.0];
        let border_colour = BORDER_COLOUR;
        let text_colour = BORDER_COLOUR;

        let background_colour = match card_type {
            CardTypes::Event => EVENT_CARD_BG_COLOUR,
            CardTypes::Item => ITEM_CARD_BG_COLOUR,
            CardTypes::Omen => OMEN_CARD_BG_COLOUR,
        };

        Self { position, size, border_colour, background_colour, text_colour }
    }
}

pub struct CardRenderer {
    pub settings: CardRenderSettings
}

impl CardRenderer {
    pub fn new(settings: CardRenderSettings) -> Self {
        Self { settings }
    }

    pub fn draw<G: Graphics>(&self, controller: &GameController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};

        let settings = &self.settings;
        let card_rect = [
            settings.position[0], settings.position[1], settings.size[0], settings.size[1]
        ];

        Rectangle::new(settings.background_colour)
            .draw(card_rect, &c.draw_state, c.transform, g)
    }
}