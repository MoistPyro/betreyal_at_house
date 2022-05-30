use opengl_graphics::{GlGraphics, Filter, GlyphCache, TextureSettings};
use graphics::text::Text;
use graphics::types::Rectangle as Rect;
use graphics::{context::Context, types::Color, Transformed, Rectangle};
use graphics::character::{CharacterCache};
use std::cmp::max;

use crate::prelude::*;
use crate::json::{get_settings};

pub struct CardRenderSettings {
    pub visible: bool,
    pub position: [f64; 2],
    size: [f64; 2],
    margin: f64,
    border_colour: Color,
    background_colour: Color,
    text_colour: Color,
    pub title: TxtField,
    pub head: TxtField,
    pub body: TxtField,
}

impl CardRenderSettings {
    pub fn new(card_type: &CardTypes, txt_data: CardFields) -> Self {
        let settings = get_settings();
        let visible = false;
        let position: [f64; 2] = [0.0, 0.0];
        let size: [f64; 2] = [230.0, 460.0];
        let margin: f64 = 20.0;
        let border_colour: Color = settings.card_border_colour;
        let text_colour: Color = settings.card_text_colour;

        let background_colour: Color = match card_type {
            CardTypes::Event => settings.card_event_bg_colour,
            CardTypes::Item => settings.card_item_bg_colour,
            CardTypes::Omen => settings.card_omen_bg_colour,
        };

        let title = TxtField::new(txt_data.title, settings.card_title_size, settings.card_title_font_path, true);
        let head = TxtField::new(txt_data.head, settings.card_head_size, settings.card_head_font_path, true);
        let body = TxtField::new(txt_data.body, settings.card_body_size, settings.card_body_font_path, false);

        Self {
            visible,
            position,
            size,
            margin,
            border_colour,
            background_colour,
            text_colour,
            title, head, body,
        }
    }

    pub fn get_card_rect(&self) -> Rect {
        [self.position[0] - self.size[0] / 2.0,
        self.position[1] - self.size[1] / 2.0,
        self.size[0],
        self.size[1]]
    }

    pub fn get_txt_rect(&mut self) -> Rect {
        let title_height = self.title.get_height();
        let head_height = self.head.get_height();
        let body_height = self.body.get_height();

        let width = self.size[0] - self.margin * 2.0;
        let total_height: f64 = title_height + head_height + body_height;

        [self.position[0] - width / 2.0,
        self.position[1] - total_height / 2.0,
        width,
        total_height]
    }
}

pub struct TxtField {
    pub text: Vec<String>,
    size: u32,
    glyphs: GlyphCache<'static>,
    pub centered: bool,
}

impl TxtField {
    pub fn new(text: Vec<String>, size: u32, font: String, centered: bool) -> Self {
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let glyphs = GlyphCache::new(&font, (), texture_settings)
            .expect("Failed to load font");
        
        Self { text, size, glyphs, centered }
    }

    ///returns the size of this field if drawn.
    pub fn get_height(&mut self) -> f64 {
        let mut height: f64 = 0.0;
        for t in &self.text {
            height += get_line_girth(&t, self.size, &mut self.glyphs).unwrap()[1];
        }
        height
    }
}

pub struct CardRenderer {
    pub settings: CardRenderSettings
}

impl CardRenderer {
    pub fn new(card_type: &CardTypes, txt_data: CardFields) -> Self {
        let settings = CardRenderSettings::new(card_type, txt_data);
        Self { settings }
    }

    pub fn draw(&mut self, context: &Context, graphics: &mut GlGraphics) {
        if self.settings.visible {
            self.draw_background(context, graphics);
            self.draw_text(context, graphics);
        }
    }

    fn draw_background(&self, context: &Context, graphics: &mut GlGraphics) {
        let settings = &self.settings;
        let card_rect = settings.get_card_rect();

        Rectangle::new(settings.background_colour)
            .draw(card_rect, &context.draw_state, context.transform, graphics);
        Rectangle::new_round_border(settings.border_colour, 4.0, 10.0)
            .draw(card_rect, &context.draw_state, context.transform, graphics);
    }

    fn draw_text(&mut self, context: &Context, graphics: &mut GlGraphics) {
        let settings: &mut CardRenderSettings = &mut self.settings;
        let txt_rect: [f64; 4] = settings.get_txt_rect();

        let mut spent_height: f64 = 0.0;
        for field in [&mut settings.title, &mut settings.head, &mut settings.body] {
            for t in field.text.iter() {
                let ref mut glyphs = field.glyphs;
                let girth: [f64; 2] = get_line_girth(t, field.size, glyphs).unwrap();

                let x: f64 = match field.centered {
                    true => settings.position[0] - girth[0] / 2.0,
                    false => txt_rect[0],
                };
                let y: f64 = txt_rect[1] + spent_height;

                let transform = context.transform.trans(x, y);
                let text_obj = Text::new_color(settings.text_colour, field.size);
                text_obj.draw(t, glyphs, &context.draw_state, transform, graphics)
                    .unwrap();

                spent_height += girth[1];
            }
        }
    }
}

fn get_line_girth(text: &str, font_size: u32, cache: &mut GlyphCache) -> Option<[f64; 2]> {
        let mut width: f64 = 0.0;
        let mut highest: f64 = 0.0;
        for ch in text.chars() {
            let height = match ch {
                ' ' => cache.character(font_size, 'A').ok().unwrap().atlas_size[1] * 0.5,
                _ => cache.character(font_size, 'A').ok().unwrap().atlas_size[1] * 1.3,
            };
            width += cache.character(font_size, ch).ok().unwrap().advance_width();
            if height > highest { highest = height };
        }
        Some([width, highest])
}