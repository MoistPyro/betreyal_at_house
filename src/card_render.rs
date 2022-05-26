use opengl_graphics::{GlGraphics, Filter, GlyphCache, TextureSettings};
use graphics::text::Text;
use graphics::types::Rectangle as Rect;
use graphics::{context::Context, Graphics, types::Color, Transformed, Rectangle};
use graphics::character::{CharacterCache, Character};

use crate::prelude::CardTypes;
use crate::prelude::CardData;


const BORDER_COLOUR: Color = [0.0, 0.0, 0.0, 1.0];
const TEXT_COLOUR: Color = [0.0, 0.0, 0.0, 1.0];
const EVENT_CARD_BG_COLOUR: Color = [0.7, 0.6, 0.3, 1.0];
const ITEM_CARD_BG_COLOUR: Color = [0.7, 0.5, 0.2, 1.0];
const OMEN_CARD_BG_COLOUR: Color = [0.5, 0.7, 0.2, 1.0];

pub struct CardRenderSettings {
    pub visible: bool,
    pub position: Vec<f64>,
    size: [f64; 2],
    border_colour: Color,
    background_colour: Color,
    text_colour: Color,
    pub title: Vec<String>,
    title_font: String,
    title_pos: Vec<f64>,
    pub head: Vec<String>,
    head_font: String,
    head_pos: Vec<f64>,
    pub body: Vec<String>,
    body_font: String,
    body_pos: Vec<f64>,
}

impl CardRenderSettings {
    pub fn new(card_type: &CardTypes, txt_data: CardData) -> Self {
        let visible = false;
        let position: Vec<f64> = vec![0.0, 0.0];
        let size: [f64; 2] = [230.0, 460.0];
        let border_colour: Color = BORDER_COLOUR;
        let text_colour: Color = TEXT_COLOUR;

        let background_colour: Color = match card_type {
            CardTypes::Event => EVENT_CARD_BG_COLOUR,
            CardTypes::Item => ITEM_CARD_BG_COLOUR,
            CardTypes::Omen => OMEN_CARD_BG_COLOUR,
        };

        let title: Vec<String> = txt_data.0;
        let title_pos = vec![0.0, - size[1] / 4.0];
        let title_font = "assets/Amatic-Bold.ttf".to_string();

        let head: Vec<String> = txt_data.1;
        let head_pos = vec![0.0, - size[1] / 6.0];
        let head_font = "assets/Amatic-Bold.ttf".to_string();

        let body: Vec<String> = txt_data.2;
        let body_pos = vec![0.0, 0.0];
        let body_font = "assets/AmaticSC-Regular.ttf".to_string();

        Self {
            visible,
            position,
            size,
            border_colour,
            background_colour,
            text_colour,
            title, title_pos, title_font,
            head, head_pos, head_font,
            body, body_pos, body_font
        }
    }

    pub fn get_rect(&self) -> Rect {
        [self.position[0] - self.size[0] / 2.0,
        self.position[1] - self.size[1] / 2.0,
        self.size[0],
        self.size[1]]
    }
}

pub struct CardRenderer {
    pub settings: CardRenderSettings
}

impl CardRenderer {
    pub fn new(card_type: &CardTypes, txt_data: CardData) -> Self {
        let settings = CardRenderSettings::new(card_type, txt_data);
        Self { settings }
    }

    pub fn draw(&self, context: &Context, graphics: &mut GlGraphics) {
        if self.settings.visible {
            let texture_settings = TextureSettings::new().filter(Filter::Nearest);

            let ref mut title_glyphs: GlyphCache = GlyphCache::new(&self.settings.title_font, (), texture_settings)
                .expect("Failed to load font");
            let ref mut head_glyphs: GlyphCache = GlyphCache::new(&self.settings.head_font, (), texture_settings)
                .expect("Failed to load font");
            let ref mut body_glyphs: GlyphCache = GlyphCache::new(&self.settings.body_font, (), texture_settings)
                .expect("Failed to load font");
            
            self.draw_background(context, graphics);
            self.draw_text(title_glyphs, head_glyphs, body_glyphs, context, graphics);
        }
    }

    fn draw_background(&self, context: &Context, graphics: &mut impl Graphics) {
        let settings = &self.settings;
        let card_rect = settings.get_rect();

        Rectangle::new(settings.background_colour)
            .draw(card_rect, &context.draw_state, context.transform, graphics);
        Rectangle::new_round_border(settings.border_colour, 4.0, 10.0)
            .draw(card_rect, &context.draw_state, context.transform, graphics);
    }

    fn draw_text<G, C>(&self, title_glyphs: &mut C, head_glyphs: &mut C, body_glyphs: &mut C, context: &Context, graphics: &mut G)
    where
        G: Graphics,
        C: CharacterCache<Texture = G::Texture>,
    {
        let settings = &self.settings;

        self.write_stuff(&settings.title, &settings.title_pos, 34, title_glyphs, context, graphics);
        self.write_stuff(&settings.head, &settings.head_pos, 20, head_glyphs, context, graphics);
        self.write_stuff(&settings.body, &settings.body_pos, 20, body_glyphs, context, graphics);
    }

    fn write_stuff<G, C>(&self, text: &Vec<String>, pos: &Vec<f64>, font_size: u32, glyphs: &mut C, context: &Context, graphics: &mut G)
    where
    G: Graphics,
    C: CharacterCache<Texture = G::Texture>,
    {
        for (i, t) in text.iter().enumerate() {
            let line_girth = get_line_girth(t, font_size, glyphs, graphics).unwrap();

            let x = pos[0] + self.settings.position[0] - line_girth[0] / 2.0;
            let y = pos[1] + self.settings.position[1] + i as f64 * line_girth[1];

            let transform = context.transform.trans(x, y);
            Text::new_color(self.settings.text_colour, font_size)
                .draw(t, glyphs, &context.draw_state, transform, graphics)
                .ok().unwrap();
        }
    }
}

fn get_line_girth<G, C>(text: &str, font_size: u32, cache: &mut C, _g: &mut G) -> Option<Vec<f64>>
where
    G: Graphics,
    C: CharacterCache<Texture = G::Texture>,
{
        let mut width: f64 = 0.0;
        let mut highest: f64 = 0.0;
        for ch in text.chars() {
            let character: Character<<G as Graphics>::Texture> = cache.character(font_size, ch).ok().unwrap();
            width += character.advance_width();
            let height = character.atlas_size[1];
            if height > highest { highest = height };
        }
        Some(vec![width, highest])
}