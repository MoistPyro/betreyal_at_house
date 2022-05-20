use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, EventLoop, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use graphics::*;

use game_controller::{GameController, GameBoard};

mod gameobjects;
mod json;
mod card_render;
mod game_controller;

const BG_COLOUR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

fn main() {
    start_app();
}

fn start_app() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Betreyal at House on the Hill", [1400, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build().unwrap();

    let mut events = Events::new(EventSettings::new()).lazy(true);
    let mut gl = GlGraphics::new(opengl);

    let gameboard = GameBoard::new();
    let mut controller = GameController::new(gameboard);

    while let Some(e) = events.next(&mut window) {
        controller.event(&e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |context, gl_graphics| {
                clear(BG_COLOUR, gl_graphics);

            });
        }
        if let Some(args) = e.update_args() {
            
        }
    }
}