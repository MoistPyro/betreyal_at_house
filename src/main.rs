use prelude::Window;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, EventLoop, Events};
use piston::input::{UpdateArgs, UpdateEvent, RenderEvent};
use piston::window::WindowSettings;

use game_controller::GameController;
use die_roller::roll_die;

mod prelude;
mod gameobjects;
mod json;
mod card_render;
mod game_controller;
mod die_roller;

fn main() {
    start_app();
}

fn start_app() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Betreyal at House on the Hill", [1400, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut controller = GameController::new(opengl);

    let card = controller.gameboard.players[0].draw_card(&mut controller.gameboard.decks[roll_die() as usize]);
    controller.gameboard.card_into_slot(card, 1);

    while let Some(e) = events.next(&mut window) {
        controller.event(&e);
        if let Some(args) = e.update_args() {}
    }
}