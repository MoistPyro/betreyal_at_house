use opengl_graphics::{OpenGL, GlGraphics};
use piston::input::GenericEvent;
use graphics::*;
use graphics::types::Color;
use std::collections::HashMap;

use crate::gameobjects::{cards::Card, deck::Deck, player::Player, character::Character};
use crate::gameobjects::*;

const BG_COLOUR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];

pub struct GameBoard {
    pub decks: Vec<Deck>,
    pub players: Vec<Player>,
    pub cards: HashMap<usize, Box<dyn Card>>,
}

impl GameBoard {
    ///setup all the game objects.
    pub fn new() -> Self {
        let mut decks = Vec::<Deck>::new();
        decks.push(
            Deck::new("Event Deck", get_event_cards())
        );
        decks.push(
            Deck::new("Item Deck", get_item_cards())
        );
        decks.push(
            Deck::new("Omen Deck", get_omen_cards())
        );

        let mut characters: Vec<Character> = get_characters();
        let mut players: Vec<Player> = Vec::new();
        let cards: HashMap<usize, Box<dyn Card>> = HashMap::new();

        players.push(Player::new(characters.pop().unwrap()));

        Self { decks, players, cards }
    }

    fn slot_is_open(&self, i: &usize) -> bool {
        match self.cards.get(i) {
            Some(_) => false,
            None => true,
        }
    }

    pub fn card_into_slot(&mut self, mut card: Box<dyn Card>, i: usize) {
        if self.slot_is_open(&i) && i >= 0 && i < 3 {
            let position = vec![i as f64 * 350.0 + 350.0, 400.0];
            card.set_pos(position);
            card.set_visible(true);
            self.cards.insert(i, card);
        };
    }
    pub fn remove_card_from_slot(&mut self, i: usize) -> Box<dyn Card> {
        let mut card = self.cards.remove(&i).unwrap();
        card.set_visible(false);
        card
    }
}
pub struct GameController {
    pub gameboard: GameBoard,
    open_gl: GlGraphics,
    background_colour: Color,
    pub cursor_pos: [f64; 2],
}


impl GameController {
    pub fn new(opengl_ver: OpenGL) -> Self {
        let gameboard = GameBoard::new();
        let open_gl: GlGraphics = GlGraphics::new(opengl_ver);
        let background_colour: Color = [0.3, 0.3, 0.3, 1.0];

        Self {
            gameboard,
            open_gl,
            background_colour,
            cursor_pos: [0.0; 2]
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(args) = e.render_args() {
            self.open_gl.draw(args.viewport(), |context, gl_graphics| {
                clear(BG_COLOUR, gl_graphics);
                for (_i, card) in &self.gameboard.cards {
                    card.draw(&context, gl_graphics)
                }
            });
        }
    }
}