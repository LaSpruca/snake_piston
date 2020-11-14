mod util;
mod app;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston_window::{PistonWindow as Window, Glyphs};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::util::{Color, Cell, GameState, Direction};
use piston::{PressEvent, Button, Key};
use rand::prelude::ThreadRng;
use rand::Rng;
use graphics::color::WHITE;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake", [425, 425])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let glyphs = window.load_font(assets.join("pixeboy.ttf")).unwrap();

    // Create a new game and run it.
    let mut app = app::App::new(GlGraphics::new(opengl), glyphs);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.press_args() {
            match args {
                Button::Keyboard(Key::W) | Button::Keyboard(Key::Up) => {
                    app.set_direction(Direction::Up)
                }
                Button::Keyboard(Key::A) | Button::Keyboard(Key::Left) => {
                    app.set_direction(Direction::Left)
                }
                Button::Keyboard(Key::S) | Button::Keyboard(Key::Down) => {
                    app.set_direction(Direction::Down)
                }
                Button::Keyboard(Key::D) | Button::Keyboard(Key::Right) => {
                    app.set_direction(Direction::Right)
                }
                Button::Keyboard(Key::R) => {
                    app.reset()
                }
                _ => {}
            }
        }
    }
}
