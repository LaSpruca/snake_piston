mod util;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::util::{Color, Cell, GameState};

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    grid: Vec<Vec<Cell>>,
    delta_total: f64,
    state: GameState,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 25.0);
        let grid = &self.grid;

        let state = self.state;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(Color::BLACK.rgb(), gl);

            let transform = c
                .transform
                .trans(25.0, 25.0);


            for (x, row) in grid.iter().enumerate() {
                for (y, cell) in row.iter().enumerate() {
                    let transformed = transform.trans(25.0 * x as f64, 25.0 * y as f64);
                    match cell {
                        Cell::Empty => rectangle(Color::BLUE.rgb(), square, transformed, gl),
                        Cell::Food => rectangle(Color::RED.rgb(), square, transformed, gl),
                        Cell::Head => rectangle(Color::GREEN.rgb(), square, transformed, gl),
                        Cell::Tail => rectangle(Color::YELLOW.rgb(), square, transformed, gl),
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        match self.state {
            GameState::Playing => {
                self.delta_total += args.dt;
                if self.delta_total > 0.200 {
                    println!("Update");
                    self.delta_total = 0.0;

                    let mut tail = vec!();
                    let mut head = (-1, -1);

                    for (x, row) in self.grid.clone().iter().enumerate() {
                        for (y, cell) in row.iter().enumerate() {
                            match cell {
                                Cell::Head => head = (x as i32, y as i32),
                                Cell::Tail => tail.push((x, y)),
                                _ => {}
                            }
                            self.grid[x][y] = Cell::Empty;
                        }
                    }

                    head.1 += 1;

                    if head.1 >= 0 && head.0 >= 0 {
                        if head.1 < 15 && head.0 < 15 {
                            self.grid[head.0 as usize][(head.1) as usize] = Cell::Head;
                        } else {
                            self.state = GameState::Over;
                        }
                    } else {
                        self.state = GameState::Over;
                    }
                }
            }
            GameState::Over => {}
        }
    }

    fn init(&mut self) {
        self.grid[0][0] = Cell::Head;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [425, 425])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: (0..15).map(|_| (0..15).map(|_| Cell::Empty).collect()).collect(),
        delta_total: 0.0,
        state: GameState::Playing,
    };

    app.init();

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
