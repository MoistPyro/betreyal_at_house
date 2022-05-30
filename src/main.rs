use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::event_loop::{EventSettings, EventLoop, Events};
use piston::input::{UpdateArgs, UpdateEvent, RenderEvent};
use piston::window::WindowSettings;

use game_controller::GameController;
use json::{Settings, get_settings};

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
    let opengl_ver: OpenGL = OpenGL::V4_5;
    let settings: Settings = get_settings();

    let window_settings: WindowSettings = WindowSettings::new(settings.game_title, settings.window_size)
        .graphics_api(opengl_ver)
        .exit_on_esc(true);
    let mut window: GlutinWindow = window_settings.build().expect("could not create window.");

    let mut events: Events = Events::new(EventSettings::new().lazy(true));

    let mut controller: GameController = GameController::new(opengl_ver);

    while let Some(e) = events.next(&mut window) {
        controller.event(&e);
        if let Some(args) = e.update_args() {}
    }
}