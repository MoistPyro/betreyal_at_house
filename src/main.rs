use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;

mod gameobject;
mod json_handler;
use gameobject::{Character, Player, Card, CardTypes, Deck};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const BG_COLOUR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BG_COLOUR, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //runs every frame
        
    }
}

fn main() {
    let mut item_deck = Deck::new("Item Deck".to_string());
    let mut event_deck = Deck::new("Event Deck".to_string());
    let mut omen_deck = Deck::new("Omen Deck".to_string());

    let smelling_salt = Card::new(
        "Smelling Salts".to_string(),
        "Whew, that's a lungful.".to_string(),
        "If your Knowledge is below its starting value,
            you can raise that trait to its starting value by using this.".to_string(),
        CardTypes::Consumable,
        Card::dummy_func,
        Card::dummy_func,
        Card::smelling_salts_func,
        Card::dummy_func
    );

    let bellows = Character::new(
        "Ox Bellows".to_string(),
        23,
        193,
        130,
        ["Football".to_string(), "Shiny Objects".to_string()],
        [18, 10],
        [0, 4, 5, 5, 6, 6, 7, 8, 8],
        [0, 2, 2, 2, 3, 4, 5, 5, 6],
        [0, 2, 2, 3, 4, 5, 5, 6, 7],
        [0, 2, 2, 3, 3, 5, 5, 6, 6],
        [4, 6, 4, 4]
    );

    let mut player = Player::new(bellows);

    item_deck.add_card(smelling_salt);
    item_deck.shuffle();
    player.draw_card(&mut item_deck);

    player.character.modify_stat(3, -2);

    let index: usize = player.look_for_card("Smelling Salts".to_string());
    player.activate_card(index);

    start_app();
}

fn start_app() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Betreyal at House on the Hill", [1400, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}